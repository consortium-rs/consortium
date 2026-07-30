// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Portable UIO interrupt dispatch: the `[fd readiness → doorbell fan-out]` half of
//! the protocol.
//!
//! Kept out from under the crate's Linux gate (this module is `cfg(unix)`) so its
//! socketpair-backed tests build and run on macOS as well as Linux. The real
//! `/dev/uioN` device wiring lives in `uio`, which is Linux-only.
//!
//! `irq_loop` is the user-space half of the UIO IRQ protocol. The kernel arms
//! the device interrupt when `/dev/uioN` is opened; from then on user space owns
//! the mask/re-arm handshake. Each M→A interrupt drives one loop iteration in a
//! fixed order:
//!
//! 1. **Wait** for the fd to become readable. When the interrupt fires the
//!    kernel masks it and bumps a 4-byte counter, which makes the fd readable.
//! 2. **Drain** the 4-byte count so the fd is no longer readable. The value (an
//!    interrupt sequence number) is discarded; only the edge matters.
//! 3. **Fan out** to every registered doorbell `Service`. A masking doorbell
//!    (IPCC, MU) masks its asserted lines here and then wakes its
//!    [`tokio::sync::Notify`] waiters; a plain doorbell just calls
//!    `notify_waiters()`.
//! 4. **Re-arm** the interrupt with a 4-byte write.
//!
//! The ordering is the whole point. Masking in step 3 happens *before* the
//! re-arm in step 4, so a level-sensitive line that is still asserted does not
//! immediately re-fire the UIO IRQ the instant it is unmasked. Equally
//! important, the re-arm is the *last* thing each iteration does — this loop
//! never writes to the fd before its first wait, because the kernel already
//! armed the line at open and a redundant enable can unbalance a UIO driver's
//! enable/disable accounting (leaving the interrupt wedged off).
//!
//! Wakeups ride entirely on tokio's `tokio::sync::Notify`: each
//! `Service` ultimately calls `notify_waiters()` on the doorbell's shared
//! `Notify`, which is the same object the doorbell's `wait()` future subscribes
//! to. The loop itself holds no per-channel state — it does not know which line
//! fired, so it services every doorbell and lets each re-check (and acknowledge)
//! its own hardware. A spurious wake is therefore always safe.
//!
//! This module depends only on `std` plus Tokio's `AsyncFd` — never the
//! Linux-only `uio` crate — so it lives under `cfg(unix)` and its
//! socketpair-backed tests exercise the `[fd readiness → user-space fan-out]`
//! slice on both macOS and Linux. The real `/dev/uioN` mapping and device
//! lifecycle stay in the `uio` module (Linux only), which drives this loop with a
//! `File` opened on the actual UIO character device.

// The only non-test consumer of these items is [`crate::uio`], which is built
// only on Linux with `ipc-shm`. Other builds still compile this module so its
// socketpair-backed tests can exercise the portable IRQ loop, but a normal
// library build sees the items as unused. Allow dead code whenever that
// production consumer is absent rather than gating the logic away from tests.
#![cfg_attr(not(all(target_os = "linux", feature = "ipc-shm")), allow(dead_code))]

use std::{
    fs::File,
    io::{Read as _, Write as _},
    sync::{Arc, Mutex},
};

use tokio::io::unix::AsyncFd;

/// A doorbell's IRQ service callback: run once per M→A interrupt to wake the
/// doorbell's pending `wait()` futures. A masking doorbell (IPCC, MU) masks its
/// asserted lines here *before* waking so a level-sensitive line does not
/// immediately re-fire the UIO IRQ; a plain doorbell just calls
/// `notify_waiters()` on its shared [`Notify`](tokio::sync::Notify).
pub(crate) type Service = Arc<dyn Fn() + Send + Sync>;

/// Shared registry of doorbell IRQ services the loop fans out to.
///
/// [`irq_loop`] invokes every registered [`Service`] on each M→A interrupt, so
/// all pending `wait()` futures re-check their hardware status. Doorbells are
/// added through [`UioDevice::bind_service`](crate::uio::UioDevice::bind_service)
/// / [`UioDevice::bind_notifier`](crate::uio::UioDevice::bind_notifier).
pub(crate) type Services = Arc<Mutex<Vec<Service>>>;

/// Background IRQ listener body.
///
/// Runs the wait → drain → fan-out → re-arm cycle described in the module docs
/// until an unrecoverable fd error tears the listener down.
/// `async_fd` wraps the UIO character device (or, in tests, a socketpair
/// standing in for it). [`crate::uio::UioDevice`] aborts the spawned task on
/// drop, so the loop does not outlive the mmap it wakes waiters against.
pub(crate) async fn irq_loop(async_fd: AsyncFd<File>, services: Services) {
    loop {
        consortium_log::trace!("irq_loop: waiting for M→A interrupt");

        // 1. Wait for an interrupt. We do *not* write to the fd first: the
        //    kernel armed the line at device open, and each iteration re-arms as
        //    its final step (below), so the line is always armed on entry here.
        let mut guard = match async_fd.readable().await {
            Ok(guard) => guard,
            Err(e) => {
                consortium_log::error!(
                    "irq_loop: readable() failed ({e}); tearing down IRQ listener"
                );
                return;
            }
        };

        // 2. Drain the 4-byte interrupt count. `try_io` runs the read and, when
        //    it would block, clears the readiness itself and reports it as a
        //    spurious wake — in that case no interrupt actually fired, so we
        //    re-wait without servicing doorbells or touching the re-arm/mask
        //    accounting.
        match guard.try_io(|inner| drain_count(inner.get_ref())) {
            Ok(Ok(())) => {
                // A real interrupt was consumed. Clear the readiness so the next
                // `readable()` re-registers with the reactor rather than
                // returning the stale, now-drained readiness immediately.
                guard.clear_ready();
            }
            Ok(Err(e)) => {
                // The fd was readable but the drain failed for something other
                // than `WouldBlock`. Treat it as an interrupt anyway: servicing
                // every doorbell just re-checks idle hardware (safe), and we
                // still owe the kernel a re-arm. Consume the readiness first.
                consortium_log::error!("irq_loop: draining UIO count failed: {e}");
                guard.clear_ready();
            }
            Err(_would_block) => {
                consortium_log::trace!("irq_loop: spurious readiness; no interrupt to service");
                continue;
            }
        }

        // 3. Fan the interrupt out to every registered doorbell. Each masking
        //    doorbell masks its asserted lines here, *before* the re-arm below,
        //    then wakes its `Notify` waiters.
        run_services(&services);

        // 4. Re-arm the interrupt now that any level-sensitive lines are masked,
        //    so unmasking the UIO IRQ cannot immediately re-fire it.
        if let Err(e) = rearm_irq(async_fd.get_ref()) {
            consortium_log::error!("irq_loop: re-arm failed ({e}); tearing down IRQ listener");
            return;
        }
    }
}

/// Run every registered doorbell IRQ service.
///
/// We do not know which channel fired, so each service masks its own asserted
/// lines and re-wakes its `wait()` futures; a spurious run is harmless. The
/// registry is snapshotted under the lock and the services run with the lock
/// released, so a panicking service cannot poison the registry. A lock that is
/// *already* poisoned (a registrar panicked mid-`push`) is recovered rather than
/// propagated: a dead IRQ listener would stall every channel on the device, a
/// far worse outcome than one lost registration.
pub(crate) fn run_services(services: &Services) {
    let snapshot = {
        let guard = services
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        guard.clone()
    };
    consortium_log::trace!(
        "run_services: servicing {} registered doorbell(s)",
        snapshot.len()
    );
    for service in &snapshot {
        service();
    }
}

/// Drain the UIO 4-byte interrupt count from `fd`.
///
/// The value is an interrupt sequence number that this edge-driven loop does not
/// use; the read exists to consume the fd's readiness. `Read` is implemented for
/// `&File`, so the read goes through the borrowed fd without duplicating it (the
/// `mut fd` binding lets `read_exact` take `&mut &File`).
fn drain_count(mut fd: &File) -> std::io::Result<()> {
    let mut count = [0u8; 4];
    fd.read_exact(&mut count)?;
    consortium_log::trace!("irq_loop: UIO count = {}", u32::from_ne_bytes(count));
    Ok(())
}

/// Re-arm (unmask) the UIO interrupt by writing a 4-byte `1`.
///
/// `Write` is implemented for `&File`, so — unlike the previous implementation,
/// which `try_clone`d the fd on every interrupt — this writes through the
/// borrowed fd directly, with no per-IRQ `dup(2)`.
fn rearm_irq(mut fd: &File) -> std::io::Result<()> {
    consortium_log::trace!("irq_loop: re-arming UIO interrupt");
    fd.write_all(&1u32.to_ne_bytes())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::io::FromRawFd as _;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use tokio::sync::Notify;

    /// Build a single-threaded runtime with timers enabled. Avoids needing the
    /// tokio `macros` feature for `#[tokio::test]`.
    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("build runtime")
    }

    /// Like [`runtime`] but also enables the IO driver so [`AsyncFd`] can
    /// register the socketpair fd with the reactor.
    fn runtime_io() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("build io runtime")
    }

    /// Set `O_NONBLOCK` on a raw fd, panicking on failure.
    fn set_nonblocking(fd: libc::c_int) {
        // SAFETY: `fd` is a live, owned descriptor; F_GETFL/F_SETFL only read and
        // rewrite its flags. Both results are checked.
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        assert_ne!(flags, -1, "fcntl F_GETFL failed");
        // SAFETY: same live descriptor; adds O_NONBLOCK to its existing flags.
        let rc = unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
        assert_ne!(rc, -1, "fcntl F_SETFL O_NONBLOCK failed");
    }

    /// A connected non-blocking `SOCK_STREAM` socketpair, returned as owned
    /// `File`s.
    ///
    /// `dev` stands in for `/dev/uioN` exactly as [`irq_loop`] uses it: the peer
    /// (`kernel`) writing 4 bytes makes `dev` readable (an asserted interrupt),
    /// `irq_loop` reads 4 bytes to drain the count, and `irq_loop`'s re-arm write
    /// lands in `kernel`'s buffer. Both ends are non-blocking: `dev` so
    /// `AsyncFd`'s `try_io` sees `WouldBlock` on spurious readiness instead of
    /// blocking the reactor thread, and `kernel` so a test (or an in-service
    /// probe) can read the re-arm write without blocking when it is absent.
    fn socketpair() -> (File, File) {
        let mut fds = [0 as libc::c_int; 2];
        // SAFETY: `fds` is a valid, writable 2-element array; `socketpair` either
        // fills both descriptors and returns 0 or returns -1 without touching
        // them. We check the return value before adopting the fds.
        let rc = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        assert_eq!(
            rc,
            0,
            "socketpair failed: {}",
            std::io::Error::last_os_error()
        );
        set_nonblocking(fds[0]);
        set_nonblocking(fds[1]);
        // SAFETY: both descriptors are freshly created and uniquely owned here;
        // wrapping them in `File` transfers ownership so each is closed once on
        // drop.
        unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
    }

    /// "Kernel" side asserts an interrupt: write the 4-byte UIO count to the peer
    /// so `dev` becomes readable. `Write` is implemented for `&File`, so no `mut`
    /// binding is needed.
    fn assert_interrupt(kernel: &File, count: u32) {
        (&mut &*kernel)
            .write_all(&count.to_ne_bytes())
            .expect("assert simulated interrupt");
    }

    /// Spin the executor a few times so a just-spawned task can register its fd
    /// and park on `readable()` before the caller asserts an interrupt — making
    /// the wake a real edge rather than pre-satisfied readiness.
    async fn let_task_park() {
        for _ in 0..8 {
            tokio::task::yield_now().await;
        }
    }

    /// End-to-end `[fd readiness → user-space fan-out]`: a socketpair fakes the
    /// UIO device, a simulated interrupt (a 4-byte write from the peer) drives
    /// one `irq_loop` iteration, and the bound service's `notify_waiters()` wakes
    /// a pending waiter — the same wake a `RecvFut` awaits.
    #[test]
    fn irq_loop_wakes_bound_service_on_simulated_interrupt() {
        runtime_io().block_on(async {
            let (dev, kernel) = socketpair();
            let async_fd = AsyncFd::new(dev).expect("register dev fd with reactor");

            let services: Services = Arc::new(Mutex::new(Vec::new()));
            let woke = Arc::new(Notify::new());
            let woke_svc = Arc::clone(&woke);
            services
                .lock()
                .unwrap()
                .push(Arc::new(move || woke_svc.notify_waiters()) as Service);

            // Arm the waiter before the interrupt fires, as a pending receiver
            // would (mirrors `RecvFut` calling `Notified::enable`).
            let notified = woke.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();

            let task = tokio::spawn(irq_loop(async_fd, Arc::clone(&services)));
            let_task_park().await;

            assert_interrupt(&kernel, 7);

            tokio::time::timeout(Duration::from_secs(1), notified)
                .await
                .expect("bound service should wake on simulated interrupt");

            task.abort();
        });
    }

    /// Regression test for the ordering fix: doorbell services must run *before*
    /// the loop re-arms the UIO interrupt, so a masking doorbell can mask its
    /// level-sensitive lines before they are unmasked and re-fire.
    ///
    /// The re-arm is a 4-byte write to the `kernel` end of the socketpair, so the
    /// service probes that end non-blockingly when it runs: if the re-arm bytes
    /// are already present, the loop re-armed too early. The probe must see
    /// `WouldBlock`.
    #[test]
    fn irq_loop_services_doorbells_before_rearming() {
        runtime_io().block_on(async {
            let (dev, kernel) = socketpair();
            let async_fd = AsyncFd::new(dev).expect("register dev fd with reactor");

            // The service reads the `kernel` fd (via a clone that shares the same
            // non-blocking description) to check whether the re-arm write already
            // landed. Assert the interrupt with a *distinct* count from the
            // re-arm's `1`, so an accidental read of our own asserted byte cannot
            // masquerade as a premature re-arm.
            let probe = kernel.try_clone().expect("clone kernel fd for probe");
            let rearm_before_service = Arc::new(AtomicBool::new(false));
            let woke = Arc::new(Notify::new());

            let flag = Arc::clone(&rearm_before_service);
            let woke_svc = Arc::clone(&woke);
            let services: Services = Arc::new(Mutex::new(Vec::new()));
            services.lock().unwrap().push(Arc::new(move || {
                let mut buf = [0u8; 4];
                match (&mut &probe).read(&mut buf) {
                    // Bytes waiting on the kernel end mean the loop re-armed
                    // before running us — the ordering bug.
                    Ok(n) if n > 0 => flag.store(true, Ordering::SeqCst),
                    // WouldBlock (expected) or a clean EOF: no premature re-arm.
                    _ => {}
                }
                woke_svc.notify_waiters();
            }) as Service);

            let notified = woke.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();

            let task = tokio::spawn(irq_loop(async_fd, Arc::clone(&services)));
            let_task_park().await;

            assert_interrupt(&kernel, 0xABCD_1234);

            tokio::time::timeout(Duration::from_secs(1), notified)
                .await
                .expect("service should run on the simulated interrupt");

            assert!(
                !rearm_before_service.load(Ordering::SeqCst),
                "irq_loop re-armed the interrupt before running doorbell services"
            );

            task.abort();
        });
    }

    /// `run_services` runs every doorbell service registered before the
    /// interrupt. Each service here is the `bind_notifier` wrapper's
    /// `notify_waiters()` closure, mirroring `RecvFut` arming its waker (via
    /// `Notified::enable`) before the M→A IRQ arrives.
    #[test]
    fn run_services_wakes_all_registered_doorbells() {
        runtime().block_on(async {
            let services: Services = Arc::new(Mutex::new(Vec::new()));
            let a = Arc::new(Notify::new());
            let b = Arc::new(Notify::new());
            let a_svc = Arc::clone(&a);
            let b_svc = Arc::clone(&b);
            services
                .lock()
                .unwrap()
                .push(Arc::new(move || a_svc.notify_waiters()) as Service);
            services
                .lock()
                .unwrap()
                .push(Arc::new(move || b_svc.notify_waiters()) as Service);

            // Register both wakers before firing, as a pending receiver would.
            let wait_a = a.notified();
            let wait_b = b.notified();
            tokio::pin!(wait_a, wait_b);
            wait_a.as_mut().enable();
            wait_b.as_mut().enable();

            run_services(&services);

            tokio::time::timeout(Duration::from_secs(1), wait_a)
                .await
                .expect("doorbell a should wake");
            tokio::time::timeout(Duration::from_secs(1), wait_b)
                .await
                .expect("doorbell b should wake");
        });
    }

    /// An empty registry is a no-op: an IRQ before any channel is set up must not
    /// panic or otherwise disturb the loop.
    #[test]
    fn run_services_empty_list_is_ok() {
        let services: Services = Arc::new(Mutex::new(Vec::new()));
        run_services(&services);
    }

    /// A poisoned registry (a registrar panicked while holding the lock) is
    /// recovered, not propagated: previously registered services still run, so a
    /// single panicking registration cannot wedge the whole IRQ listener.
    #[test]
    fn run_services_recovers_from_poisoned_list() {
        let services: Services = Arc::new(Mutex::new(Vec::new()));
        let ran = Arc::new(AtomicBool::new(false));
        let ran_svc = Arc::clone(&ran);
        services
            .lock()
            .unwrap()
            .push(Arc::new(move || ran_svc.store(true, Ordering::SeqCst)) as Service);

        // Poison the lock: a thread panics while holding it.
        let clone = Arc::clone(&services);
        let _ = std::thread::spawn(move || {
            let _guard = clone.lock().unwrap();
            panic!("poison the lock");
        })
        .join();

        // Recovery: the service registered before poisoning still runs.
        run_services(&services);
        assert!(
            ran.load(Ordering::SeqCst),
            "run_services must recover from a poisoned registry and still fan out"
        );
    }
}

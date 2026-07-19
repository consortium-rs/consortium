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

//! Portable UIO interrupt dispatch.
//!
//! [`irq_loop`] is the user-space half of the UIO IRQ protocol: wait for the
//! device fd to become readable (an M→A interrupt fired and the kernel masked
//! it), drain the 4-byte interrupt count, re-arm the interrupt with a 4-byte
//! write, then fan the wakeup out to every registered doorbell [`Service`].
//!
//! This module depends only on `std` plus Tokio's [`AsyncFd`] — never the
//! Linux-only `uio` crate — so it lives under `cfg(unix)` and its
//! socketpair-backed tests exercise the `[fd readiness → user-space fan-out]`
//! slice on both macOS and Linux. The real `/dev/uioN` mapping and device
//! lifecycle stay in [`crate::uio`] (Linux only), which drives this loop with a
//! `File` opened on the actual UIO character device.

// The only non-test consumer of these items is [`crate::uio`], which is
// Linux-only. On other Unix hosts (macOS) a plain `cargo build` therefore sees
// them as unused; they are still exercised by the test module below, so allow
// dead code off-Linux rather than gating the logic away from its own tests.
#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::{
    fs::File,
    io::Write as _,
    sync::{Arc, Mutex},
};

use tokio::io::unix::AsyncFd;

/// A doorbell's IRQ service callback: run once per M→A interrupt to wake the
/// doorbell's pending `wait()` futures. A masking doorbell (IPCC, MU) masks its
/// asserted lines here *before* waking so a level-sensitive line does not
/// immediately re-fire the UIO IRQ; a plain doorbell just calls
/// `notify_waiters()`.
pub(crate) type Service = Arc<dyn Fn() + Send + Sync>;

/// Shared list of doorbell IRQ services the IRQ task fans out to.
///
/// [`irq_loop`] invokes every registered [`Service`] whenever an M→A interrupt
/// fires, so all pending `wait()` futures re-check their hardware status.
pub(crate) type Services = Arc<Mutex<Vec<Service>>>;

/// Background IRQ listener body: block on `async_fd` readiness, drain the UIO
/// interrupt count, re-arm, and fan out to every registered [`Service`].
///
/// `async_fd` wraps the UIO character device (or, in tests, a socketpair
/// standing in for it). Runs until an unrecoverable error tears the listener
/// down; [`crate::uio::UioDevice`] aborts the spawned task on drop.
pub(crate) async fn irq_loop(async_fd: AsyncFd<File>, services: Services) {
    loop {
        consortium_log::trace!("irq_loop: waiting for M→A interrupt");
        // UIO protocol: re-arm the interrupt by writing 1, then block until the
        // kernel reports it readable (an IRQ has fired and been masked).
        if unmask_irq(async_fd.get_ref()).is_err() {
            consortium_log::error!("irq_loop: unmask_irq failed; tearing down IRQ listener");
            break;
        }
        match async_fd.readable().await {
            Err(e) => {
                consortium_log::error!(
                    "irq_loop: readable() failed ({}); tearing down IRQ listener",
                    e
                );
                break;
            }
            Ok(mut guard) => {
                // Consume the 4-byte interrupt count so the fd is no longer
                // readable; the value itself is unused (edge, not level).
                // `&File: Read`, so read through the borrowed fd rather than
                // duping it every interrupt.
                let mut count = [0u8; 4];
                consortium_log::trace!("irq_loop: draining UIO count");
                let read = guard.try_io(|fd| {
                    use std::io::Read;
                    // The reborrow is deliberate: `Read` is implemented for
                    // `&File`, so read through a fresh `&File` place without
                    // duping the fd.
                    #[allow(clippy::borrow_deref_ref)]
                    (&mut &*fd.get_ref()).read_exact(&mut count)
                });
                // `try_io` returns `Err(WouldBlock)` as a spurious-readiness
                // signal (retry the loop); any inner `Err` is a real read
                // failure worth logging before we re-arm.
                if let Ok(Err(e)) = read {
                    consortium_log::error!("irq_loop: draining UIO count failed: {}", e);
                }
                consortium_log::trace!("irq_loop: UIO count = {}", u32::from_ne_bytes(count));
                guard.clear_ready();
                consortium_log::trace!("irq_loop: UIO interrupt handled; notifying all doorbells");
            }
        }
        // Fan the interrupt out to every registered doorbell. A poisoned lock
        // means a registering thread panicked; tear the listener down.
        if !run_services(&services) {
            break;
        }
    }
}

/// Run every registered doorbell IRQ service. We do not know which channel
/// fired, so each service masks its own asserted lines and re-wakes its `wait()`
/// futures; a spurious run is harmless. Returns `false` if the service list is
/// poisoned.
pub(crate) fn run_services(services: &Services) -> bool {
    consortium_log::trace!("run_services: servicing all registered doorbells");
    let snapshot = match services.lock() {
        Ok(list) => {
            consortium_log::trace!("run_services: {} doorbells registered", list.len());
            list.clone()
        }
        Err(_) => {
            consortium_log::error!("run_services: failed to acquire service list lock");
            return false;
        }
    };
    for service in &snapshot {
        consortium_log::trace!("run_services: servicing a doorbell");
        service();
    }
    true
}

fn unmask_irq(f: &File) -> std::io::Result<()> {
    consortium_log::trace!("unmask_irq: writing 1 to UIO fd to re-arm interrupt.");
    f.try_clone()?.write_all(&1u32.to_ne_bytes())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::io::FromRawFd as _;
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

    /// A connected `SOCK_STREAM` socketpair, returned as owned `File`s.
    ///
    /// `dev` stands in for `/dev/uioN` exactly as [`irq_loop`] uses it: the
    /// peer (`kernel`) writing 4 bytes makes `dev` readable (an asserted
    /// interrupt), `irq_loop` reads 4 bytes to drain the count, and `irq_loop`'s
    /// re-arm write lands in the peer's buffer (which the test ignores). `dev`
    /// is set non-blocking so `AsyncFd`'s `try_io` sees `WouldBlock` on spurious
    /// readiness instead of blocking the reactor thread.
    fn socketpair() -> (File, File) {
        let mut fds = [0 as libc::c_int; 2];
        // SAFETY: `fds` is a valid, writable 2-element array; `socketpair`
        // either fills both descriptors and returns 0 or returns -1 without
        // touching them. We check the return value before adopting the fds.
        let rc = unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr()) };
        assert_eq!(
            rc,
            0,
            "socketpair failed: {}",
            std::io::Error::last_os_error()
        );
        // SAFETY: `fds[0]` is a fresh, exclusively owned descriptor; setting
        // O_NONBLOCK on it is sound and we check the result.
        let flags = unsafe { libc::fcntl(fds[0], libc::F_GETFL) };
        assert_ne!(flags, -1, "fcntl F_GETFL failed");
        // SAFETY: same freshly owned descriptor; adding O_NONBLOCK to its flags.
        let rc = unsafe { libc::fcntl(fds[0], libc::F_SETFL, flags | libc::O_NONBLOCK) };
        assert_ne!(rc, -1, "fcntl F_SETFL O_NONBLOCK failed");
        // SAFETY: both descriptors are freshly created and uniquely owned here;
        // wrapping them in `File` transfers ownership so each is closed once on
        // drop.
        unsafe { (File::from_raw_fd(fds[0]), File::from_raw_fd(fds[1])) }
    }

    /// End-to-end `[fd readiness → user-space fan-out]`: a socketpair fakes the
    /// UIO device, a simulated interrupt (a 4-byte write from the peer) drives
    /// one `irq_loop` iteration, and the bound service's `notify_waiters()`
    /// wakes a pending waiter — the same wake a `RecvFut` awaits.
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

            // Let the spawned task register the fd and park on `readable()`
            // before we assert the interrupt, so the wakeup is a real edge.
            for _ in 0..8 {
                tokio::task::yield_now().await;
            }

            // "Kernel" side asserts the interrupt: write the 4-byte UIO count.
            // `Write` is implemented for `&File`, so no `mut` binding is needed.
            {
                use std::io::Write as _;
                (&mut &kernel)
                    .write_all(&7u32.to_ne_bytes())
                    .expect("assert simulated interrupt");
            }

            tokio::time::timeout(Duration::from_secs(1), notified)
                .await
                .expect("bound service should wake on simulated interrupt");

            task.abort();
        });
    }

    // NOTE: the "receiver arms after the edge" lost-wakeup is fixed one layer
    // down, in `consortium_ipc_transport_memory::poll_recv` (it re-scans the
    // slot after subscribing), not here — the `notify_waiters()` fan-out is
    // still intentionally lossy. Its regression test lives with that fix:
    // `recv_delivers_when_remote_publishes_during_subscribe_window` in
    // `consortium-ipc-transport-memory/tests/async_tokio/recv.rs`.

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

            assert!(run_services(&services), "healthy list must report success");

            tokio::time::timeout(Duration::from_secs(1), wait_a)
                .await
                .expect("doorbell a should wake");
            tokio::time::timeout(Duration::from_secs(1), wait_b)
                .await
                .expect("doorbell b should wake");
        });
    }

    /// An empty registry is a no-op success: an IRQ before any channel is set
    /// up must not tear the listener down.
    #[test]
    fn run_services_empty_list_is_ok() {
        let services: Services = Arc::new(Mutex::new(Vec::new()));
        assert!(run_services(&services));
    }

    /// A poisoned registry reports failure so the IRQ loop can stop.
    #[test]
    fn run_services_poisoned_list_fails() {
        let services: Services = Arc::new(Mutex::new(Vec::new()));
        let clone = Arc::clone(&services);
        let _ = std::thread::spawn(move || {
            let _guard = clone.lock().unwrap();
            panic!("poison the lock");
        })
        .join();

        assert!(!run_services(&services));
    }
}

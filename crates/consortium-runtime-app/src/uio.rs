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

//! UIO device management for the A-core Linux runtime.
//!
//! [`UioDevice`] opens `/dev/uioN`, mmaps the DMA-coherent shared region,
//! parses the consortium descriptor, and spawns a background Tokio task that
//! waits for M→A hardware interrupts and broadcasts wakeups to all pending
//! [`SharedMemoryTransport`] receivers via a shared [`Arc<Notify>`].
//!
//! Callers obtain per-channel transports via [`UioDevice::channel_transport`]
//! and wrap them in [`Channel`](consortium_ipc::Channel).

use std::{
    fs::File,
    io::Write as _,
    sync::{Arc, Mutex},
};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_ipc::{Chan, Doorbell};
use consortium_ipc_transport_memory::{
    ChannelDirection, Descriptor, DescriptorError, SharedMemoryTransport,
    SharedMemoryTransportable, config, descriptor_ack,
};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum UioDeviceError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("uio: {0:?}")]
    Uio(UioError),

    #[error("descriptor: {0}")]
    Descriptor(#[from] DescriptorError),

    #[error("channel {0} out of range (descriptor has {1} channels)")]
    ChannelOutOfRange(u8, u8),

    #[error("channel pair for chan {0} not found in descriptor")]
    ChannelPairNotFound(u8),

    #[error("no UIO map named {0:?}")]
    MapNotFound(String),
}

/// Upper bound on UIO maps scanned by [`UioDevice::map_peripheral_by_name`].
///
/// `uio_pdrv_genirq` platform devices expose a handful of maps at most; this
/// caps the sysfs scan so a missing name fails fast instead of looping.
const MAX_UIO_MAPS: usize = 8;

/// Shared list of doorbell wakeups that the IRQ task fans out to.
///
/// Each registered [`Notify`] belongs to a doorbell (its [`notifier()`] handle);
/// `irq_loop` calls `notify_waiters()` on every entry whenever an M→A interrupt
/// fires, so all pending `wait()` futures re-check their hardware status.
///
/// [`notifier()`]: consortium_ipc::Doorbell
type Notifiers = Arc<Mutex<Vec<Arc<Notify>>>>;

impl From<UioError> for UioDeviceError {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

// ── UioDevice ─────────────────────────────────────────────────────────────────

struct Inner {
    region_base: *mut u8,
    descriptor: Descriptor,
    /// Doorbell wakeups fired on every M→A IRQ. All pending `RecvFut`s and
    /// `SendFut`s wake and re-check their slots. Populated through
    /// [`UioDevice::register_doorbell`].
    notifiers: Notifiers,
}

// SAFETY: region_base points into a DMA-coherent mmap that outlives `Inner`.
// Concurrent access is synchronised by the slot protocol and irq_notify.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

/// Entry point for the consortium UIO device (`/dev/uioN`).
///
/// Parses the shared-memory descriptor (the descriptor must already be in
/// `Ready` state, i.e. M-core init is complete; see
/// [`mmap::wait_for_ready`](crate::mmap::wait_for_ready)) and spawns a
/// background Tokio task that listens for M→A IRQs and broadcasts wakeups to
/// all waiting [`SharedMemoryTransport`] futures.
///
/// Obtain per-channel transports via [`channel_transport`](Self::channel_transport).
pub struct UioDevice {
    inner: Arc<Inner>,
    /// Owns the mmap'd resources; also used by [`map_peripheral`] to map the
    /// doorbell register block.
    ///
    /// [`map_peripheral`]: Self::map_peripheral
    uio: uio::UioDevice,
    /// Background IRQ listener. Aborted on drop so the task does not outlive
    /// the device (and the mmap it wakes waiters against).
    irq_task: tokio::task::JoinHandle<()>,
}

impl Drop for UioDevice {
    fn drop(&mut self) {
        self.irq_task.abort();
    }
}

impl UioDevice {
    /// Open the configured UIO device, mmap `map0`, parse the descriptor, and
    /// start the IRQ listener task.
    ///
    /// The device index comes from [`config::UIO`] — the `CONSORTIUM_IPC_UIO`
    /// value injected from the `[ipc]` section of `Consortium.toml` — so it is
    /// not passed by the caller. Building this crate with `ipc-shm` but without
    /// that UIO value is a compile error (see the guard in `lib.rs`), so by the
    /// time `open` runs the index is always real. The mapped address comes from
    /// mmap, not from `CONSORTIUM_IPC_BASE`.
    pub fn open(num_channels: usize, transfer_size: usize) -> Result<Self, UioDeviceError> {
        let uio_num = config::UIO as usize;
        let uio = uio::UioDevice::blocking_new(uio_num)?;

        // map_mapping reads the size from sysfs and mmaps `/dev/uioN` at the
        // map0 offset (the CIPC shared region) in the uio_pdrv_genirq namespace.
        let region_base = uio.map_mapping(0)? as *mut u8;

        // SAFETY: region_base is valid for the lifetime of `uio`; descriptor
        // must already be in Ready state - call mmap::wait_for_ready first.
        let descriptor = unsafe { descriptor_ack(region_base, num_channels, transfer_size) }?;

        let notifiers: Notifiers = Arc::new(Mutex::new(Vec::new()));

        // Open the device fd a second time for async IRQ listening so we do
        // not disturb the mmap held by `uio`.
        let irq_file = File::open(format!("/dev/uio{uio_num}"))?;
        let async_fd = AsyncFd::new(irq_file)?;

        let irq_notifiers = Arc::clone(&notifiers);
        let irq_task = tokio::spawn(async move { irq_loop(async_fd, irq_notifiers).await });

        Ok(Self {
            inner: Arc::new(Inner {
                region_base,
                descriptor,
                notifiers,
            }),
            uio,
            irq_task,
        })
    }

    /// Map a UIO memory mapping by index and return its virtual base address.
    ///
    /// Maps live in the `uio_pdrv_genirq` maps namespace
    /// (`/sys/class/uio/uioN/maps/mapN`); `map0` is the `CIPC` shared-memory
    /// region (mapped at [`open`]). The doorbell peripheral register block
    /// (MU / HSEM / IPCC) is a later map — typically `map1` — described in the
    /// device tree. Pass the returned base into the matching doorbell
    /// constructor, e.g. `MuDoorbell::new([base])`, `HsemDoorbell::new(base)`,
    /// or `IpccDoorbell::new(base)`.
    ///
    /// Prefer [`map_peripheral_by_name`] when map ordering is not guaranteed.
    ///
    /// # Safety (caller obligation)
    ///
    /// The returned address is only valid while this `UioDevice` is alive (it
    /// owns the mapping). The caller must treat it as MMIO: only the doorbell's
    /// typed volatile register accesses may dereference it.
    ///
    /// [`open`]: Self::open
    /// [`map_peripheral_by_name`]: Self::map_peripheral_by_name
    pub fn map_peripheral(&self, index: usize) -> Result<usize, UioDeviceError> {
        Ok(self.uio.map_mapping(index)? as usize)
    }

    /// Map a UIO memory mapping by its device-tree name and return its virtual
    /// base address.
    ///
    /// Resolves `name` against `/sys/class/uio/uioN/maps/*/name` (the `reg-names`
    /// entries from the device tree) so callers do not depend on map ordering,
    /// then maps it like [`map_peripheral`]. The same MMIO safety obligation
    /// applies.
    ///
    /// [`map_peripheral`]: Self::map_peripheral
    pub fn map_peripheral_by_name(&self, name: &str) -> Result<usize, UioDeviceError> {
        let index = self.find_map(name)?;
        self.map_peripheral(index)
    }

    /// Find the map index whose name matches `name`, scanning up to
    /// [`MAX_UIO_MAPS`]. Scanning stops at the first absent map.
    fn find_map(&self, name: &str) -> Result<usize, UioDeviceError> {
        for index in 0..MAX_UIO_MAPS {
            match self.uio.map_name(index) {
                Ok(found) if found == name => return Ok(index),
                Ok(_) => continue,
                // An absent map ends the contiguous map range.
                Err(_) => break,
            }
        }
        Err(UioDeviceError::MapNotFound(name.to_string()))
    }

    /// Register a doorbell's [`Notify`] so the IRQ task wakes its `wait()`
    /// futures on every M→A interrupt.
    ///
    /// The std doorbells each own an internal `Notify` and expose it through
    /// `notifier()`; without this registration their `wait()` futures never
    /// resolve on the Linux side, because the wakeup arrives as a UIO interrupt
    /// rather than via polled MMIO. Call this once per doorbell, before the
    /// first receive:
    ///
    /// ```ignore
    /// let base = dev.map_peripheral(1)?;
    /// let mu = MuDoorbell::new([base; 8]);
    /// dev.register_doorbell(mu.notifier());
    /// let transport = dev.channel_transport(chan, mu, region)?;
    /// ```
    pub fn register_doorbell(&self, notifier: Arc<Notify>) {
        self.inner
            .notifiers
            .lock()
            .expect("notifier list poisoned")
            .push(notifier);
    }

    /// Number of channels declared in the descriptor.
    pub fn num_channels(&self) -> u8 {
        self.inner.descriptor.num_channels
    }

    /// Create a [`SharedMemoryTransport`] for the given logical channel.
    ///
    /// The descriptor channel table is expected to have paired entries for
    /// each logical channel: the entry at index `2 * chan.id()` is the A→M
    /// (transmit from A-core) slot, and `2 * chan.id() + 1` is the M→A
    /// (receive on A-core) slot.
    ///
    /// `doorbell` is rung after each successful send to notify M-core.
    /// `region`   governs cache-coherency operations (use [`DdrRegion`] for
    ///            DDR-backed shared memory).
    ///
    /// [`DdrRegion`]: consortium_ipc::DdrRegion
    pub fn channel_transport<D, R>(
        &self,
        chan: Chan,
        doorbell: D,
        region: R,
    ) -> Result<SharedMemoryTransport<D, R>, UioDeviceError>
    where
        D: Doorbell,
        R: SharedMemoryTransportable,
    {
        let id = chan.id() as usize;
        let num_ch = self.inner.descriptor.num_channels as usize;

        // Each logical channel occupies two consecutive descriptor entries:
        //   even index → AToM (A-core transmits)
        //   odd  index → MToA (A-core receives)
        let tx_idx = 2 * id;
        let rx_idx = 2 * id + 1;

        if rx_idx >= num_ch {
            return Err(UioDeviceError::ChannelOutOfRange(chan.id(), num_ch as u8));
        }

        let tx_entry = &self.inner.descriptor.channels[tx_idx];
        let rx_entry = &self.inner.descriptor.channels[rx_idx];

        if tx_entry.direction != ChannelDirection::AToM
            || rx_entry.direction != ChannelDirection::MToA
        {
            return Err(UioDeviceError::ChannelPairNotFound(chan.id()));
        }

        let base = self.inner.region_base;

        // SAFETY: slot pointers and sizes come from the validated descriptor;
        // this UioDevice exclusively owns the mmap.
        Ok(unsafe { SharedMemoryTransport::new(chan, doorbell, region, base) })
    }
}

// ── IRQ background task ───────────────────────────────────────────────────────

async fn irq_loop(async_fd: AsyncFd<File>, notifiers: Notifiers) {
    loop {
        // UIO protocol: re-arm the interrupt by writing 1, then block until the
        // kernel reports it readable (an IRQ has fired and been masked).
        if unmask_irq(async_fd.get_ref()).is_err() {
            break;
        }
        match async_fd.readable().await {
            Err(_) => break,
            Ok(mut guard) => {
                // Consume the 4-byte interrupt count so the fd is no longer
                // readable; the value itself is unused (edge, not level).
                let mut count = [0u8; 4];
                let _ = guard.try_io(|fd| {
                    use std::io::Read;
                    fd.get_ref().try_clone()?.read_exact(&mut count)
                });
                guard.clear_ready();
            }
        }
        // Fan the interrupt out to every registered doorbell. A poisoned lock
        // means a registering thread panicked; tear the listener down.
        if !notify_all(&notifiers) {
            break;
        }
    }
}

/// Wake every registered doorbell. We do not know which channel fired, so each
/// doorbell's `wait()` re-checks its own status register; a spurious wake is
/// harmless. Returns `false` if the notifier list is poisoned.
fn notify_all(notifiers: &Notifiers) -> bool {
    let snapshot = match notifiers.lock() {
        Ok(list) => list.clone(),
        Err(_) => return false,
    };
    for notify in &snapshot {
        notify.notify_waiters();
    }
    true
}

fn unmask_irq(f: &File) -> std::io::Result<()> {
    f.try_clone()?.write_all(&1u32.to_ne_bytes())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Build a single-threaded runtime with timers enabled. Avoids needing the
    /// tokio `macros` feature for `#[tokio::test]`.
    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("build runtime")
    }

    /// `notify_all` wakes every doorbell registered before the interrupt,
    /// mirroring `RecvFut` arming its waker (via `Notified::enable`) before the
    /// M→A IRQ arrives.
    #[test]
    fn notify_all_wakes_all_registered_doorbells() {
        runtime().block_on(async {
            let notifiers: Notifiers = Arc::new(Mutex::new(Vec::new()));
            let a = Arc::new(Notify::new());
            let b = Arc::new(Notify::new());
            notifiers.lock().unwrap().push(Arc::clone(&a));
            notifiers.lock().unwrap().push(Arc::clone(&b));

            // Register both wakers before firing, as a pending receiver would.
            let wait_a = a.notified();
            let wait_b = b.notified();
            tokio::pin!(wait_a, wait_b);
            wait_a.as_mut().enable();
            wait_b.as_mut().enable();

            assert!(notify_all(&notifiers), "healthy list must report success");

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
    fn notify_all_empty_list_is_ok() {
        let notifiers: Notifiers = Arc::new(Mutex::new(Vec::new()));
        assert!(notify_all(&notifiers));
    }

    /// A poisoned registry reports failure so the IRQ loop can stop.
    #[test]
    fn notify_all_poisoned_list_fails() {
        let notifiers: Notifiers = Arc::new(Mutex::new(Vec::new()));
        let clone = Arc::clone(&notifiers);
        let _ = std::thread::spawn(move || {
            let _guard = clone.lock().unwrap();
            panic!("poison the lock");
        })
        .join();

        assert!(!notify_all(&notifiers));
    }
}

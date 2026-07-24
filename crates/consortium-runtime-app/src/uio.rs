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
    sync::{Arc, Mutex},
};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_ipc_transport_memory::DescriptorError;

use crate::irq::{Service, Services, irq_loop};

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

impl From<UioError> for UioDeviceError {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

// ── UioDevice ─────────────────────────────────────────────────────────────────

struct Inner {
    /// Doorbell IRQ services run on every M→A IRQ. Each masks its asserted
    /// lines (if applicable) and wakes the doorbell's pending `RecvFut`s /
    /// `SendFut`s. Populated through [`UioDevice::bind_service`] /
    /// [`UioDevice::bind_notifier`].
    services: Services,
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
    irq_task: Option<tokio::task::JoinHandle<()>>,
}

impl Drop for UioDevice {
    fn drop(&mut self) {
        if let Some(irq_task) = &self.irq_task {
            irq_task.abort();
        }
    }
}

impl UioDevice {
    /// Open the configured UIO device, mmap `map0`, parse the descriptor, and
    /// optionally start the IRQ listener task.
    ///
    /// The device index comes from [`config::UIO`] — the `CONSORTIUM_IPC_UIO`
    /// value injected from the `[ipc]` section of `Consortium.toml` — so it is
    /// not passed by the caller. Building this crate with `ipc-shm` but without
    /// that UIO value is a compile error (see the guard in `lib.rs`), so by the
    /// time `open` runs the index is always real. The mapped address comes from
    /// mmap, not from `CONSORTIUM_IPC_BASE`.
    ///
    /// Set `enable_irq` only for UIO devices backed by an interrupt source.
    /// Memory-only UIO devices may not support the UIO IRQ read/write protocol.
    pub fn open(uio_id: usize, enable_irq: bool) -> Result<Self, UioDeviceError> {
        consortium_log::trace!(
            "UioDevice::open(uio_id = {}, enable_irq = {})",
            uio_id,
            enable_irq
        );
        let uio = uio::UioDevice::blocking_new(uio_id)?;

        consortium_log::trace!(
            "UioDevice::open: mmap'd descriptor at {:p}, size = {:#x}",
            uio.map_mapping(0)?,
            uio.map_size(0)?
        );

        let services: Services = Arc::new(Mutex::new(Vec::new()));

        let irq_task = if enable_irq {
            consortium_log::trace!("UioDevice::open: spawning IRQ listener task");

            // Open the device fd a second time for async IRQ listening so we do
            // not disturb the mmap held by `uio`. It must be read-write: the UIO
            // re-arm protocol *writes* a u32 to the fd to unmask the interrupt
            // (see `rearm_irq`), and the VFS rejects a write to an O_RDONLY fd
            // with EBADF before it reaches the driver — which tears the listener
            // down after the very first (kernel-armed) interrupt.
            let irq_file = File::options()
                .read(true)
                .write(true)
                .open(format!("/dev/uio{uio_id}"))?;
            let async_fd = AsyncFd::new(irq_file)?;

            consortium_log::trace!("UioDevice::open: starting irq_loop");

            let irq_services = Arc::clone(&services);
            Some(tokio::spawn(async move {
                irq_loop(async_fd, irq_services).await
            }))
        } else {
            consortium_log::trace!("UioDevice::open: IRQ listener disabled");
            None
        };

        consortium_log::trace!("UioDevice::open: returning UioDevice handle");

        Ok(Self {
            inner: Arc::new(Inner { services }),
            uio,
            irq_task,
        })
    }

    /// Map a UIO memory mapping by index and return its virtual base address.
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
    pub fn mmap(&self, index: usize) -> Result<usize, UioDeviceError> {
        Ok(self.uio.map_mapping(index)? as usize)
    }

    /// Map a RAM-backed UIO window by index, zero its entire length, and return
    /// its virtual base address.
    ///
    /// Shared-memory carveouts (the IPC descriptor region and the debug rings)
    /// keep their contents across a coprocessor reset, so a warm reboot can leave
    /// stale descriptor magic, slot pointers, or ring state behind. The AP owns
    /// the first touch of these windows, so it clears them here before the
    /// descriptor handshake or ring bring-up reads them.
    ///
    /// This is only valid for `reserved-memory` (RAM) windows. Never call it on a
    /// register-backed window (a doorbell block): writing zeros to it would clear
    /// live MMIO registers. Use [`mmap`](Self::mmap) for those.
    ///
    /// [`mmap`]: Self::mmap
    pub fn mmap_zeroed(&self, index: usize) -> Result<usize, UioDeviceError> {
        let base = self.uio.map_mapping(index)? as usize;
        let len = self.uio.map_size(index)?;
        // The kernel maps UIO memory noncached (ARM Device-nGnRnE). A bulk
        // `memset`/`write_bytes` is *not* usable here: glibc's AArch64 `memset`
        // uses `DC ZVA` and unaligned SIMD stores, both illegal on Device memory,
        // which fault as SIGBUS. Zero with aligned `write_volatile` word stores,
        // the same access discipline the descriptor/slot protocol uses. UIO
        // windows are page-aligned, so the `usize` steps stay naturally aligned;
        // a trailing sub-word remainder (only possible for a non-word-multiple
        // length) is cleared byte-by-byte.
        let step = core::mem::size_of::<usize>();
        let words = len / step;
        // SAFETY: `base`/`len` describe the freshly mapped UIO RAM window this
        // device owns; before the coprocessor boots the AP is its sole writer, so
        // these in-bounds volatile stores have no aliasing hazard.
        unsafe {
            let word_ptr = base as *mut usize;
            for i in 0..words {
                word_ptr.add(i).write_volatile(0);
            }
            let byte_ptr = base as *mut u8;
            for i in (words * step)..len {
                byte_ptr.add(i).write_volatile(0);
            }
        }
        Ok(base)
    }

    /// Register a doorbell's IRQ [`Service`] so the IRQ task runs it on every
    /// M→A interrupt.
    ///
    /// A masking doorbell binds its own IRQ entry point here — e.g.
    /// [`IpccDoorbell::notify_rx_occupied_interrupt`] or
    /// [`MuDoorbell::notify_gi_interrupt`] — which masks the asserted lines
    /// before waking, so a level-sensitive line does not immediately re-fire the
    /// UIO IRQ. The doorbell is `Clone`, so bind a clone before moving the
    /// original into a transport:
    ///
    /// ```ignore
    /// let mu = unsafe { MuDoorbell::<Primary, 7>::new(dev.mmap(1)?) };
    /// let db = mu.clone();
    /// dev.bind_service(::std::sync::Arc::new(move || db.notify_gi_interrupt()));
    /// let transport = SharedMemoryTransport::<Primary, _, _>::new(mu, desc, slot_pair);
    /// ```
    ///
    /// [`IpccDoorbell::notify_rx_occupied_interrupt`]: https://docs.rs/consortium-ipc-doorbell-ipcc
    /// [`MuDoorbell::notify_gi_interrupt`]: https://docs.rs/consortium-ipc-doorbell-mu
    pub fn bind_service(&self, service: Service) {
        consortium_log::trace!("UioDevice::bind_service: registering IRQ service");
        self.inner
            .services
            .lock()
            .expect("service list poisoned")
            .push(service);
    }

    /// Register a doorbell's [`Notify`] so the IRQ task wakes its `wait()`
    /// futures on every M→A interrupt.
    ///
    /// Convenience wrapper over [`bind_service`](Self::bind_service) for
    /// doorbells with no line masking (the wakeup just calls `notify_waiters()`).
    /// Masking doorbells should bind their own service instead so a
    /// level-sensitive line is masked before the wake.
    pub fn bind_notifier(&self, notifier: Arc<Notify>) {
        consortium_log::trace!("UioDevice::bind_notifier: registering notifier");
        self.bind_service(Arc::new(move || notifier.notify_waiters()));
    }
}

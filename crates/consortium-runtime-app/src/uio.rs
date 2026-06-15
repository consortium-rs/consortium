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

use std::{fs::File, io::Write as _, sync::Arc};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_ipc::{Chan, Doorbell};
use consortium_ipc_transport_memory::{
    ChannelDirection, Descriptor, DescriptorError, SharedMemoryTransport,
    SharedMemoryTransportable, descriptor_ack,
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
}

impl From<UioError> for UioDeviceError {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

// ── UioDevice ─────────────────────────────────────────────────────────────────

struct Inner {
    region_base: *mut u8,
    descriptor: Descriptor,
    /// Fires on every M→A IRQ. All pending `RecvFut`s and `SendFut`s wake
    /// and re-check their slots.
    irq_notify: Arc<Notify>,
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
    /// Kept alive to preserve the mmap lifetime.
    _uio: uio::UioDevice,
}

impl UioDevice {
    /// Open `/dev/uio{uio_num}`, mmap resource 0, parse the descriptor, and
    /// start the IRQ listener task.
    pub fn open(
        uio_num: usize,
        num_channels: usize,
        transfer_size: usize,
    ) -> Result<Self, UioDeviceError> {
        let uio = uio::UioDevice::blocking_new(uio_num)?;

        // map_resource reads the mapping size from sysfs and mmaps resource 0.
        let region_base = uio.map_resource(0)? as *mut u8;

        // SAFETY: region_base is valid for the lifetime of `uio`; descriptor
        // must already be in Ready state - call mmap::wait_for_ready first.
        let descriptor = unsafe { descriptor_ack(region_base, num_channels, transfer_size) }?;

        let irq_notify = Arc::new(Notify::new());

        // Open the device fd a second time for async IRQ listening so we do
        // not disturb the mmap held by `uio`.
        let irq_file = File::open(format!("/dev/uio{uio_num}"))?;
        let async_fd = AsyncFd::new(irq_file)?;

        let notify = Arc::clone(&irq_notify);
        tokio::spawn(async move { irq_loop(async_fd, notify).await });

        Ok(Self {
            inner: Arc::new(Inner {
                region_base,
                descriptor,
                irq_notify,
            }),
            _uio: uio,
        })
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

async fn irq_loop(async_fd: AsyncFd<File>, notify: Arc<Notify>) {
    loop {
        if unmask_irq(async_fd.get_ref()).is_err() {
            break;
        }
        match async_fd.readable().await {
            Err(_) => break,
            Ok(mut guard) => {
                let mut count = [0u8; 4];
                let _ = guard.try_io(|fd| {
                    use std::io::Read;
                    fd.get_ref().try_clone()?.read_exact(&mut count)
                });
                guard.clear_ready();
            }
        }
        notify.notify_waiters();
    }
}

fn unmask_irq(f: &File) -> std::io::Result<()> {
    f.try_clone()?.write_all(&1u32.to_ne_bytes())
}

#![cfg(target_os = "linux")]
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

#[cfg(not(target_os = "linux"))]
compile_error!(
    "`consortium-uio` is only supported on Linux (requires /dev/uioN and compatible UIO driver)"
);

use std::{
    fs::File,
    future::Future,
    io::Write as _,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_ipc::{
    chan::Chan,
    descriptor::{ChannelLayout, Descriptor, DescriptorError},
    slot,
    transport::Transport,
};
use pin_project::pin_project;

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("uio: {0:?}")]
    Uio(UioError),

    #[error("descriptor: {0}")]
    Descriptor(#[from] DescriptorError),

    #[error("channel {0} out of range (device has {1})")]
    ChannelOutOfRange(u8, u8),
}

impl From<UioError> for Error {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

// ── UioDevice ────────────────────────────────────────────────────────────────

struct Inner {
    region_base: *mut u8,
    descriptor: Descriptor,
    /// Fires on every M→A IRQ.  All pending `RecvFut`s wake and re-check.
    irq_notify: Arc<Notify>,
}

// SAFETY: raw pointer into a DMA-coherent mmap that outlives `Inner`.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

/// Entry point for the consortium UIO device (`/dev/uioN`).
///
/// Parses the shared-memory descriptor (must be in `Ready` state) and spawns
/// a background Tokio task that waits for M→A IRQs and broadcasts wakeups to
/// all waiting [`UioChannel`] receivers.
pub struct UioDevice {
    inner: Arc<Inner>,
    /// Kept alive to preserve the mmap lifetime.
    _uio: uio::UioDevice,
}

impl UioDevice {
    pub fn open(uio_num: usize) -> Result<Self, Error> {
        let uio = uio::UioDevice::blocking_new(uio_num)?;

        // map_resource reads the mapping size from sysfs and mmaps it.
        let region_base = uio.map_resource(0)? as *mut u8;

        // SAFETY: region_base is valid for the lifetime of `uio`.
        let descriptor = unsafe { consortium_ipc::descriptor::read_descriptor(region_base) }?;

        let irq_notify = Arc::new(Notify::new());

        // Open the device fd a second time for async IRQ waiting so we
        // do not disturb the mmap held by `uio`.
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

    pub fn effective_mtu(&self) -> usize {
        self.inner.descriptor.effective_mtu
    }

    pub fn num_channels(&self) -> u8 {
        self.inner.descriptor.num_channels
    }

    /// Return a [`UioChannel`] for `chan`.
    ///
    /// `UioChannel` implements [`Transport`] and can be handed directly to
    /// [`Channel`](consortium_core::Channel).
    pub fn channel(&self, chan: Chan) -> Result<UioChannel, Error> {
        let id = chan.id() as usize;
        if id >= self.inner.descriptor.channels.len() {
            return Err(Error::ChannelOutOfRange(
                chan.id(),
                self.inner.descriptor.num_channels,
            ));
        }
        let layout = self.inner.descriptor.channels[id].clone();
        let effective_mtu = self.inner.descriptor.effective_mtu;
        Ok(UioChannel {
            inner: Arc::clone(&self.inner),
            layout,
            effective_mtu,
        })
    }
}

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

// ── UioChannel ───────────────────────────────────────────────────────────────

/// A single logical channel over the consortium UIO device.
///
/// Implements [`Transport`]; obtain via [`UioDevice::channel`].
pub struct UioChannel {
    inner: Arc<Inner>,
    layout: ChannelLayout,
    effective_mtu: usize,
}

// SAFETY: volatile accesses on DMA-coherent memory; no aliasing Rust references.
unsafe impl Send for UioChannel {}

impl Transport for UioChannel {
    type Error = Error;
    // Futures own all state via Arc; PhantomData carries the GAT lifetime
    // without imposing any borrow from `self`.
    type SendFut<'a>
        = SendFut
    where
        Self: 'a;
    type RecvFut<'a>
        = RecvFut
    where
        Self: 'a;

    fn send<'a>(&mut self, _ch: Chan, data: &[u8]) -> Self::SendFut<'a> {
        SendFut {
            inner: Arc::clone(&self.inner),
            tx_offset: self.layout.tx_offset as usize,
            data: data.to_vec(),
            notified: None,
        }
    }

    fn recv<'a>(&mut self, _ch: Chan, buf: &mut [u8]) -> Self::RecvFut<'a> {
        // SAFETY: `buf` is always `&'static mut [u8]` inside `Channel`
        // (the `Channel` struct stores `buf: &'static mut [u8]`).
        // The raw pointer is valid for the lifetime of the caller's borrow.
        RecvFut {
            inner: Arc::clone(&self.inner),
            rx_offset: self.layout.rx_offset as usize,
            buf_ptr: buf.as_mut_ptr(),
            buf_len: buf.len(),
            notified: None,
        }
    }

    fn max_message_size(&self) -> usize {
        self.effective_mtu
    }
}

// ── SendFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`UioChannel::send`].
#[pin_project]
pub struct SendFut {
    inner: Arc<Inner>,
    tx_offset: usize,
    data: Vec<u8>,
    #[pin]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

// SAFETY: raw pointer in Inner is to DMA-coherent memory; Arc is Send.
unsafe impl Send for SendFut {}

impl Future for SendFut {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let base = unsafe { this.inner.region_base.add(*this.tx_offset) };

        if !unsafe { slot::is_consumed(base) } {
            // Initialise the Notified handle exactly once, then poll it via
            // pin-projection so we get Pin<&mut Notified> — not Pin<&mut &mut Notified>.
            if this.notified.is_none() {
                // SAFETY: `inner` holds an Arc to the Notify, so it
                // outlives the future. We can safely erase the borrow lifetime.
                let notified = this.inner.irq_notify.notified();
                let notified = unsafe {
                    std::mem::transmute::<
                        tokio::sync::futures::Notified<'_>,
                        tokio::sync::futures::Notified<'static>,
                    >(notified)
                };
                this.notified.set(Some(notified));
            }
            // as_pin_mut() gives Pin<&mut Notified<'_>>, satisfying Future::poll.
            if let Some(notified) = this.notified.as_pin_mut() {
                let _ = notified.poll(cx);
            }
            return Poll::Pending;
        }

        // SAFETY: tx_offset is valid per the descriptor.
        unsafe { slot::write(base, this.data) };

        // TODO: write channel ID to MU TX data register (UIO map1) to
        // let M-core skip scanning all slots.

        Poll::Ready(Ok(()))
    }
}

// ── RecvFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`UioChannel::recv`].
#[pin_project]
pub struct RecvFut {
    inner: Arc<Inner>,
    rx_offset: usize,
    /// Pointer to the caller's receive buffer.
    ///
    /// # Safety
    /// Must remain valid and exclusively accessible for the lifetime of this
    /// future.  `Channel` satisfies this because `buf` is `&'static mut [u8]`
    /// and `async fn recv(&mut self)` holds `&mut self` across the `.await`.
    buf_ptr: *mut u8,
    buf_len: usize,
    #[pin]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

// SAFETY: documented above; the pointer is exclusively owned by this future.
unsafe impl Send for RecvFut {}

impl Future for RecvFut {
    type Output = Result<usize, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let base = unsafe { this.inner.region_base.add(*this.rx_offset) };
        let buf = unsafe { core::slice::from_raw_parts_mut(*this.buf_ptr, *this.buf_len) };

        if let Some(n) = unsafe { slot::try_read(base, buf) } {
            return Poll::Ready(Ok(n));
        }

        // Initialise the Notified handle exactly once, then poll it via
        // pin-projection so we get Pin<&mut Notified> — not Pin<&mut &mut Notified>.
        if this.notified.is_none() {
            // SAFETY: `inner` holds an Arc to the Notify, so it
            // outlives the future. We can safely erase the borrow lifetime.
            let notified = this.inner.irq_notify.notified();
            let notified = unsafe {
                std::mem::transmute::<
                    tokio::sync::futures::Notified<'_>,
                    tokio::sync::futures::Notified<'static>,
                >(notified)
            };
            this.notified.set(Some(notified));
        }
        // as_pin_mut() gives Pin<&mut Notified<'_>>, satisfying Future::poll.
        if let Some(notified) = this.notified.as_pin_mut() {
            let _ = notified.poll(cx);
        }

        Poll::Pending
    }
}

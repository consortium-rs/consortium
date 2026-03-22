#![cfg(target_os = "linux")]
mod descriptor;
mod slot;

use std::{
    fs::File,
    future::Future,
    io::Write as _,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_core::{chan::Chan, transport::Transport};
pub use descriptor::{DescState, Descriptor, DescriptorError};

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
        let descriptor = unsafe { descriptor::parse(region_base) }?;

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
    layout: descriptor::ChannelLayout,
    effective_mtu: usize,
}

// SAFETY: volatile accesses on DMA-coherent memory; no aliasing Rust references.
unsafe impl Send for UioChannel {}

impl Transport for UioChannel {
    type Error = Error;
    // Futures own all state via Arc; PhantomData carries the GAT lifetime
    // without imposing any borrow from `self`.
    type SendFut<'a>
        = SendFut<'a>
    where
        Self: 'a;
    type RecvFut<'a>
        = RecvFut<'a>
    where
        Self: 'a;

    fn send<'a>(&mut self, _ch: Chan, data: &[u8]) -> SendFut<'a> {
        SendFut {
            inner: Arc::clone(&self.inner),
            tx_offset: self.layout.tx_offset,
            data: data.to_vec(),
            notified: None,
            _ph: PhantomData,
        }
    }

    fn recv<'a>(&mut self, _ch: Chan, buf: &mut [u8]) -> RecvFut<'a> {
        // SAFETY: `buf` is always `&'static mut [u8]` inside `Channel`
        // (the `Channel` struct stores `buf: &'static mut [u8]`).
        // The raw pointer is valid for the lifetime of the caller's borrow.
        RecvFut {
            inner: Arc::clone(&self.inner),
            rx_offset: self.layout.rx_offset,
            buf_ptr: buf.as_mut_ptr(),
            buf_len: buf.len(),
            _ph: PhantomData,
        }
    }

    fn max_message_size(&self) -> usize {
        self.effective_mtu
    }
}

// ── SendFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`UioChannel::send`].
pub struct SendFut<'a> {
    inner: Arc<Inner>,
    tx_offset: usize,
    data: Vec<u8>,
    notified: Option<tokio::sync::futures::Notified<'a>>,
    _ph: PhantomData<&'a ()>,
}

// SAFETY: raw pointer in Inner is to DMA-coherent memory; Arc is Send.
unsafe impl Send for SendFut<'_> {}

impl Future for SendFut<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let base = unsafe { self.inner.region_base.add(self.tx_offset) };

        if !unsafe { slot::is_consumed(base) } {
            // Only create the Notified handle once
            if self.notified.is_none() {
                self.notified = Some(self.inner.irq_notify.notified());
            }
            // Poll the existing handle — no new spawns
            let notified = Pin::new(self.notified.as_mut().unwrap());
            let _ = notified.poll(cx); // registers the waker properly
            return Poll::Pending;
        }

        // SAFETY: tx_offset is valid per the descriptor.
        unsafe { slot::write(base, &self.data) };

        // TODO: write channel ID to MU TX data register (UIO map1) to
        // let M-core skip scanning all slots.

        Poll::Ready(Ok(()))
    }
}

// ── RecvFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`UioChannel::recv`].
pub struct RecvFut<'a> {
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
    notified: Option<tokio::sync::futures::Notified<'a>>,
    _ph: PhantomData<&'a ()>,
}

// SAFETY: documented above; the pointer is exclusively owned by this future.
unsafe impl Send for RecvFut<'_> {}

impl Future for RecvFut<'_> {
    type Output = Result<usize, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let base = unsafe { self.inner.region_base.add(self.rx_offset) };
        let buf = unsafe { core::slice::from_raw_parts_mut(self.buf_ptr, self.buf_len) };

        if let Some(n) = unsafe { slot::try_read(base, buf) } {
            return Poll::Ready(Ok(n));
        }

        // Only create the Notified handle once
        if self.notified.is_none() {
            self.notified = Some(self.inner.irq_notify.notified());
        }
        // Poll the existing handle — no new spawns
        let notified = Pin::new(self.notified.as_mut().unwrap());
        let _ = notified.poll(cx); // registers the waker properly

        Poll::Pending
    }
}

use core::{
    future::Future,
    marker::{PhantomData, PhantomPinned},
};

use consortium_ipc::{Chan, DdrRegion, Doorbell, Region, SramRegion, Transport};

use crate::{
    descriptor::{ChannelLayout, NUM_CHANNELS_OFFSET, RX_UPDATED_ID_OFFSET, read_channel},
    error::Error,
    slot_internal as slot,
    utils::rv_u8,
};

pub struct SharedMemoryTransport<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    /// The base pointer to the 0x00 of the shared memory region, where the descriptor header starts.
    base: *mut u8,

    tx_mtu: u32,
    rx_mtu: u32,
}

impl<D: Doorbell, R: SharedMemoryTransportable> SharedMemoryTransport<D, R> {
    /// Construct a `SharedMemoryTransport` for a single channel.
    ///
    /// # Safety
    ///
    /// - `base` must be valid for reads and writes for at least the size of the descriptor header (0x20 bytes) plus the channel table (32 entries * 0x10 bytes each).
    /// - No other Rust reference to the shared memory region may exist while this transport is in use.
    /// - The memory must be accessible to both this core and the remote core.
    pub unsafe fn new(ch: Chan, doorbell: D, region: R, base: *mut u8) -> Self {
        Self {
            ch,
            doorbell,
            region,
            base,
            tx_mtu: 0,
            rx_mtu: 0,
        }
    }
}

pub struct SendFut<'a, D: Doorbell, R: SharedMemoryTransportable> {
    result: Result<(), Error<D>>,

    _ph: PhantomData<&'a mut SharedMemoryTransport<D, R>>,
}

#[pin_project::pin_project]
pub struct RecvFut<'a, D: Doorbell, R: SharedMemoryTransportable> {
    transport: *mut SharedMemoryTransport<D, R>,
    buf_ptr: *mut u8,
    buf_len: usize,

    #[cfg(feature = "std")]
    #[pin]
    notified: Option<tokio::sync::futures::Notified<'static>>,

    _ph: PhantomData<&'a mut SharedMemoryTransport<D, R>>,
    _pin: PhantomPinned,
}

// SAFETY: the pointer is exclusively owned by this future.
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send for SendFut<'_, D, R> {}
// SAFETY: the pointer is exclusively owned by this future.
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send for RecvFut<'_, D, R> {}

impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Transport
    for SharedMemoryTransport<D, R>
{
    type Error = Error<D>;
    type RecvFut<'a>
        = RecvFut<'a, D, R>
    where
        Self: 'a;
    type SendFut<'a>
        = SendFut<'a, D, R>
    where
        Self: 'a;

    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a> {
        let mut channel_id = 0xFF;

        // Step 1. Find the tx slot for this channel.
        for chan in 0..unsafe { rv_u8(self.base, NUM_CHANNELS_OFFSET) } {
            // Allocate the first free channel slot for transmission.
            let channel = unsafe { read_channel(self.base, chan as usize) };
            if unsafe { slot::is_consumed(channel.slot_addr(self.base)) }
                && channel.direction.is_tx()
            {
                channel_id = chan;
                break;
            }
        }

        if channel_id == 0xFF {
            // No free channel slot was found.
            return SendFut {
                result: Err(Error::NoChannel),
                _ph: PhantomData,
            };
        }

        // Step 2. Visit the slot.
        let channel = unsafe { read_channel(self.base, channel_id as usize) };
        let slot_base = unsafe { channel.slot_addr(self.base) };
        unsafe { slot::write(slot_base, data) };

        self.region.flush();

        let result = self.doorbell.ring(self.ch).map_err(Error::Doorbell);

        // Step 3. Ring the doorbell to notify the remote core.
        SendFut {
            result,
            _ph: PhantomData,
        }
    }

    fn recv<'a>(&mut self, data: &mut [u8]) -> Self::RecvFut<'a> {
        RecvFut {
            transport: self as *mut _,
            buf_ptr: data.as_mut_ptr(),
            buf_len: data.len(),
            #[cfg(feature = "std")]
            notified: None,
            _ph: PhantomData,
            _pin: PhantomPinned,
        }
    }

    fn max_send_size(&self) -> usize {
        self.tx_mtu as usize
    }

    fn max_recv_size(&self) -> usize {
        self.rx_mtu as usize
    }
}

impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Future for RecvFut<'_, D, R> {
    type Output = Result<usize, Error<D>>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // SAFETY: `transport` is exclusively owned by this future, so it's safe to get a mutable reference.
        let this = self.project();
        let transport = unsafe { &mut **this.transport };

        // Step 1. Find the rx slot for this channel.
        let mut channel_id = unsafe { rv_u8(transport.base, RX_UPDATED_ID_OFFSET) };
        let channel_num = unsafe { rv_u8(transport.base, NUM_CHANNELS_OFFSET) };
        let mut channel: Option<ChannelLayout> = if channel_id < channel_num {
            // Fast path: the doorbell indicated a specific channel has new data, so we check that one first.
            let channel_layout = unsafe { read_channel(transport.base, channel_id as usize) };
            if channel_layout.direction.is_rx()
                && !unsafe { slot::is_consumed(channel_layout.slot_addr(transport.base)) }
            {
                Some(channel_layout)
            } else {
                None
            }
        } else {
            None
        };
        // Step 2. Check if the slot is valid and contains a message for this channel.
        if channel.is_none() {
            // FALLBACK: iterate the channels to find the first valid message.
            for chan in 0..channel_num {
                let channel_layout = unsafe { read_channel(transport.base, chan as usize) };
                if channel_layout.direction.is_rx()
                    && !unsafe { slot::is_consumed(channel_layout.slot_addr(transport.base)) }
                {
                    channel_id = chan;
                    channel = Some(channel_layout);
                    break;
                }
            }
        }

        if channel.is_none() {
            // No valid channel with new data was found.
            return core::task::Poll::Pending;
        }

        // Step 3. Visit the slot.
        let channel = unsafe { read_channel(transport.base, channel_id as usize) };
        let slot_base = unsafe { channel.slot_addr(transport.base) };

        // SAFETY: the mutable buffer is exclusively owned by this future, so it's safe to create a mutable slice from the raw pointer and length.
        let mut buf = unsafe { core::slice::from_raw_parts_mut(*this.buf_ptr, *this.buf_len) };

        if let Some(n) = unsafe { slot::try_read(slot_base, &mut buf) } {
            // Successfully read a message from the slot.
            return core::task::Poll::Ready(Ok(n));
        }

        transport.region.invalidate();

        // Step 4. No message in the slot - poll the doorbell to wait for a notification.
        let mut wait_fut = transport.doorbell.wait(transport.ch);

        // SAFETY: `wait_fut` is a local variable and we only pin it for the duration of this poll call, so it will not be moved after pinning.
        match unsafe { core::pin::Pin::new_unchecked(&mut wait_fut) }.poll(cx) {
            core::task::Poll::Ready(_) => {
                // Doorbell already fired before we polled - schedule ourselves for an immediate re-poll to read the slot.
                cx.waker().wake_by_ref();
            }
            core::task::Poll::Pending => {}
        }

        core::task::Poll::Pending
    }
}

impl<D: Doorbell, R: SharedMemoryTransportable> Future for SendFut<'_, D, R> {
    type Output = Result<(), Error<D>>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        core::task::Poll::Ready(match &this.result {
            Ok(()) => Ok(()),
            Err(_) => core::mem::replace(&mut this.result, Ok(())),
        })
    }
}

pub trait SharedMemoryTransportable: Region {}

impl SharedMemoryTransportable for DdrRegion {}
impl SharedMemoryTransportable for SramRegion {}

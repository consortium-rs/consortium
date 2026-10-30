use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};

use atomic_waker::AtomicWaker;
use consortium_ipc::{Chan, Doorbell, MaybeSend, RecvTransport};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, DescState, MAGIC, RX_UPDATED_ID_OFFSET, SharedMemoryTransport,
    VERSION, channel_table_entry_offset, slot, slot_offset, slot_stride,
};
use consortium_shared_memory::UncachedRegion;
use consortium_shared_memory::descriptor::DescriptorHeader;
use futures::{pin_mut, task::noop_waker_ref};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = slot_offset(0, MTU);
const M_TO_A_SLOT: usize = slot_offset(1, MTU);
const REGION_LEN: usize = M_TO_A_SLOT + slot_stride(MTU);
#[cfg(feature = "std")]
const RX_CHANNEL: usize = 1;
#[cfg(not(feature = "std"))]
const RX_CHANNEL: usize = 0;
#[cfg(feature = "std")]
type Side = consortium_ipc::Primary;
#[cfg(not(feature = "std"))]
type Side = consortium_ipc::Secondary;

static APP_RX: AtomicDoorbellState = AtomicDoorbellState::new();
static REMOTE_RX: AtomicDoorbellState = AtomicDoorbellState::new();

struct AtomicDoorbellState {
    pending: AtomicBool,
    waker: AtomicWaker,
}

impl AtomicDoorbellState {
    const fn new() -> Self {
        Self {
            pending: AtomicBool::new(false),
            waker: AtomicWaker::new(),
        }
    }

    fn reset(&self) {
        self.pending.store(false, Ordering::Release);
    }
}

#[derive(Clone, Copy)]
struct AtomicDoorbell {
    tx: &'static AtomicDoorbellState,
    rx: &'static AtomicDoorbellState,
}

impl AtomicDoorbell {
    fn pair() -> (Self, Self) {
        APP_RX.reset();
        REMOTE_RX.reset();

        (
            Self {
                tx: &REMOTE_RX,
                rx: &APP_RX,
            },
            Self {
                tx: &APP_RX,
                rx: &REMOTE_RX,
            },
        )
    }
}

struct AtomicWaitFut {
    state: &'static AtomicDoorbellState,
    ch: Chan,
}

impl Future for AtomicWaitFut {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.pending.swap(false, Ordering::AcqRel) {
            return Poll::Ready(Ok(self.ch));
        }

        self.state.waker.register(cx.waker());

        if self.state.pending.swap(false, Ordering::AcqRel) {
            Poll::Ready(Ok(self.ch))
        } else {
            Poll::Pending
        }
    }
}

impl Doorbell for AtomicDoorbell {
    type Error = Infallible;

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        self.tx.pending.store(true, Ordering::Release);
        self.tx.waker.wake();
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        AtomicWaitFut { state: self.rx, ch }
    }
}

fn channel(offset: usize, direction: ChannelDirection) -> ChannelLayout {
    let mut layout = ChannelLayout::default();
    layout.offset = offset as u32;
    layout.size = SLOT_SIZE as u32;
    layout.direction = direction;
    layout
}

fn init_two_channel_descriptor(base: *mut u8) {
    let a_to_m = channel(A_TO_M_SLOT, ChannelDirection::PrimaryToSecondary);
    let m_to_a = channel(M_TO_A_SLOT, ChannelDirection::SecondaryToPrimary);

    unsafe {
        DescriptorHeader::new(MAGIC, VERSION).write_to_raw_ptr(base);
        base.add(0x06).write_volatile(2);
        (base.add(0x10) as *mut u32).write_volatile(MTU as u32);
        (base.add(0x18) as *mut u32).write_volatile(DescState::Ready as u32);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);

        for (index, channel) in [a_to_m, m_to_a].iter().enumerate() {
            let entry = base.add(channel_table_entry_offset(index));
            (entry as *mut u32).write_volatile(channel.offset);
            (entry.add(4) as *mut u32).write_volatile(channel.size);
            entry.add(8).write_volatile(channel.direction as u8);
        }
    }
}

#[test]
fn recv_future_wakes_from_atomic_waker_doorbell() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
    let (app_doorbell, remote_doorbell) = AtomicDoorbell::pair();
    let ch1 = unsafe { Chan::new_unchecked(0) };
    let ch2 = unsafe { Chan::new_unchecked(2) };
    let mut transport =
        unsafe { SharedMemoryTransport::<Side, _, _>::new(ch1, ch2, app_doorbell, region) };
    let mut rx = [0; MTU];

    // The recv future borrows `rx` until it is dropped; scope it so the
    // buffer can be inspected afterwards.
    let n = {
        let recv = transport.recv(&mut rx);
        pin_mut!(recv);
        let mut cx = Context::from_waker(noop_waker_ref());
        assert!(matches!(recv.as_mut().poll(&mut cx), Poll::Pending));

        unsafe {
            let rx_layout =
                ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(RX_CHANNEL)));
            slot::write(rx_layout.slot_addr(base), b"atomic-waker", false);
            base.add(RX_UPDATED_ID_OFFSET)
                .write_volatile(RX_CHANNEL as u8);
        }
        remote_doorbell
            .ring(ch2)
            .expect("wake app-side atomic waker doorbell");

        match recv.as_mut().poll(&mut cx) {
            Poll::Ready(Ok(n)) => n,
            Poll::Ready(Err(_)) => panic!("recv future should not fail after atomic waker ring"),
            Poll::Pending => panic!("recv future should be ready after atomic waker ring"),
        }
    };
    assert_eq!(&rx[..n], b"atomic-waker");
}

/// One `AtomicWaker`-backed line per doorbell channel.
///
/// [`AtomicDoorbell`] routes by *side* — whatever the app rings lands in one place — which
/// is enough while each side waits on exactly one thing. Acknowledgement gives each side a
/// second line, and collapsing the two would hide the very property the design depends on,
/// so this bus addresses lines by id the way the hardware doorbells do.
static LINES: [AtomicDoorbellState; 4] = [
    AtomicDoorbellState::new(),
    AtomicDoorbellState::new(),
    AtomicDoorbellState::new(),
    AtomicDoorbellState::new(),
];

#[derive(Clone, Copy, Debug)]
struct AtomicLinedDoorbell;

impl AtomicLinedDoorbell {
    fn reset() -> Self {
        for line in &LINES {
            line.reset();
        }
        Self
    }
}

impl Doorbell for AtomicLinedDoorbell {
    type Error = Infallible;

    fn ring(&self, ch: Chan) -> Result<(), Infallible> {
        let line = &LINES[ch.id() as usize];
        line.pending.store(true, Ordering::Release);
        line.waker.wake();
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        AtomicWaitFut {
            state: &LINES[ch.id() as usize],
            ch,
        }
    }
}

/// Descriptor channel this side transmits on — the one `RX_CHANNEL` is not.
const TX_CHANNEL: usize = 1 - RX_CHANNEL;

/// `send_acked` parks on an `AtomicWaker` doorbell and resolves when the peer drains.
///
/// The `std` build of this file runs as [`Primary`] and the `no_std` build as
/// [`Secondary`], so this covers the send-side park on the executor shape firmware
/// actually uses: hand-rolled polling with a `noop` waker, no Tokio anywhere.
///
/// [`Primary`]: consortium_ipc::Primary
/// [`Secondary`]: consortium_ipc::Secondary
#[test]
fn send_acked_future_wakes_from_atomic_waker_doorbell() {
    use consortium_ipc::{AckSendTransport, Connect};
    use consortium_ipc_transport_memory::DescriptorLayout;

    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_ack_capable_descriptor(base);

    let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
    let doorbell = AtomicLinedDoorbell::reset();
    let ch_tx = unsafe { Chan::new_unchecked(0) };
    let ch_rx = unsafe { Chan::new_unchecked(2) };
    let ch_ack_tx = unsafe { Chan::new_unchecked(1) };
    let ch_ack_rx = unsafe { Chan::new_unchecked(3) };

    let mut transport = unsafe {
        SharedMemoryTransport::<Side, _, _>::new_acked(
            ch_tx, ch_rx, ch_ack_tx, ch_ack_rx, doorbell, region,
        )
    };

    let mut cx = Context::from_waker(noop_waker_ref());

    // Complete the handshake, which is what latches the acknowledgement capability.  As
    // `Secondary` the descriptor is already published so this resolves on the first poll;
    // as `Primary` it waits for the peer's ack ring, which the "remote" supplies here.
    {
        let connect = transport.connect(DescriptorLayout::new(2, MTU).with_ack(true));
        pin_mut!(connect);
        if connect.as_mut().poll(&mut cx).is_pending() {
            doorbell.ring(ch_rx).expect("remote acks the descriptor");
            assert!(
                connect.as_mut().poll(&mut cx).is_ready(),
                "handshake should complete once the peer acks"
            );
        }
    }

    let send = transport.send_acked(b"atomic-acked");
    pin_mut!(send);

    // Published and rung, but not yet drained: the future must hold.
    assert!(
        matches!(send.as_mut().poll(&mut cx), Poll::Pending),
        "send_acked must not resolve before the peer drains the slot"
    );

    let tx_slot = unsafe {
        let layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(TX_CHANNEL)));
        layout.slot_addr(base)
    };
    assert!(
        unsafe { slot::ack_requested(tx_slot) },
        "the published frame must carry the per-message acknowledgement request"
    );

    // The remote drains it and rings the acknowledgement line.
    let mut remote_buf = [0; MTU];
    let outcome = unsafe {
        let remote_region = UncachedRegion::new(base, REGION_LEN);
        slot::try_read(tx_slot, &mut remote_buf, &remote_region)
    };
    assert_eq!(outcome.delivered(), Some(b"atomic-acked".len()));
    assert!(outcome.ack_requested());
    doorbell
        .ring(ch_ack_rx)
        .expect("remote acknowledges the drained slot");

    match send.as_mut().poll(&mut cx) {
        Poll::Ready(Ok(())) => {}
        Poll::Ready(Err(error)) => panic!("send_acked should succeed, got {error:?}"),
        Poll::Pending => panic!("send_acked should resolve once the peer acknowledges"),
    }
}

fn init_ack_capable_descriptor(base: *mut u8) {
    init_two_channel_descriptor(base);
    unsafe {
        for index in 0..2 {
            base.add(channel_table_entry_offset(index) + 9)
                .write_volatile(consortium_ipc_transport_memory::FLAG_CHAN_ACK_CAPABLE);
        }
    }
}

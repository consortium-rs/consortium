use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};

use atomic_waker::AtomicWaker;
use consortium_ipc::{Chan, Doorbell, RecvTransport, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport,
    channel_table_entry_offset, descriptor_init, slot,
};
use futures::{pin_mut, task::noop_waker_ref};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = 0x100;
const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;
#[cfg(feature = "std")]
const RX_CHANNEL: usize = 1;
#[cfg(not(feature = "std"))]
const RX_CHANNEL: usize = 0;

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
    type WaitFut<'a> = AtomicWaitFut;

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        self.tx.pending.store(true, Ordering::Release);
        self.tx.waker.wake();
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        AtomicWaitFut { state: self.rx, ch }
    }

    fn pending(&self, _ch: Chan) -> bool {
        self.rx.pending.load(Ordering::Acquire)
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
    let a_to_m = channel(A_TO_M_SLOT, ChannelDirection::AToM);
    let m_to_a = channel(M_TO_A_SLOT, ChannelDirection::MToA);

    unsafe {
        descriptor_init(base, 2, MTU, &[a_to_m, m_to_a]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }
}

#[test]
fn recv_future_wakes_from_atomic_waker_doorbell() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let region = unsafe { SramRegion::new(base, REGION_LEN) };
    let (app_doorbell, remote_doorbell) = AtomicDoorbell::pair();
    let ch = unsafe { Chan::new_unchecked(0) };
    let mut transport = unsafe { SharedMemoryTransport::new(ch, app_doorbell, region, base) };
    let mut rx = [0; MTU];

    let recv = transport.recv(&mut rx);
    pin_mut!(recv);
    let mut cx = Context::from_waker(noop_waker_ref());
    assert!(matches!(recv.as_mut().poll(&mut cx), Poll::Pending));

    unsafe {
        let rx_layout =
            ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(RX_CHANNEL)));
        slot::write(rx_layout.slot_addr(base), b"atomic-waker");
        base.add(RX_UPDATED_ID_OFFSET)
            .write_volatile(RX_CHANNEL as u8);
    }
    remote_doorbell
        .ring(ch)
        .expect("wake app-side atomic waker doorbell");

    match recv.as_mut().poll(&mut cx) {
        Poll::Ready(Ok(n)) => assert_eq!(&rx[..n], b"atomic-waker"),
        Poll::Ready(Err(_)) => panic!("recv future should not fail after atomic waker ring"),
        Poll::Pending => panic!("recv future should be ready after atomic waker ring"),
    }
}

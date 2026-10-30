use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Waker},
};

use consortium_ipc::{Chan, Doorbell, MaybeSend, RecvTransport};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, DescState, MAGIC, Primary, RX_UPDATED_ID_OFFSET,
    SharedMemoryTransport, VERSION, channel_table_entry_offset, slot, slot_offset, slot_stride,
};
use consortium_shared_memory::UncachedRegion;
use consortium_shared_memory::descriptor::DescriptorHeader;
use embassy_executor::Executor;
use embassy_futures::yield_now;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use embassy_time::Duration;

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = slot_offset(0, MTU);
const M_TO_A_SLOT: usize = slot_offset(1, MTU);
const REGION_LEN: usize = M_TO_A_SLOT + slot_stride(MTU);

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

#[derive(Clone, Debug)]
struct EmbassyDoorbell {
    tx: Arc<Mutex<DoorbellState>>,
    rx: Arc<Mutex<DoorbellState>>,
}

#[derive(Debug, Default)]
struct DoorbellState {
    pending: bool,
    waker: Option<Waker>,
}

impl EmbassyDoorbell {
    fn pair() -> (Self, Self) {
        let a = Arc::new(Mutex::new(DoorbellState::default()));
        let b = Arc::new(Mutex::new(DoorbellState::default()));

        (
            Self {
                tx: a.clone(),
                rx: b.clone(),
            },
            Self { tx: b, rx: a },
        )
    }
}

struct EmbassyWaitFut {
    state: Arc<Mutex<DoorbellState>>,
    ch: Chan,
}

impl Future for EmbassyWaitFut {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self
            .state
            .lock()
            .expect("doorbell state lock is not poisoned");
        if state.pending {
            state.pending = false;
            Poll::Ready(Ok(self.ch))
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Doorbell for EmbassyDoorbell {
    type Error = Infallible;

    fn ring(&self, _ch: Chan) -> Result<(), Infallible> {
        let waker = {
            let mut state = self.tx.lock().expect("doorbell state lock is not poisoned");
            state.pending = true;
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        EmbassyWaitFut {
            state: self.rx.clone(),
            ch,
        }
    }
}

#[embassy_executor::task]
async fn receive_task(
    mut transport: SharedMemoryTransport<Primary, EmbassyDoorbell, UncachedRegion>,
    done: &'static AtomicBool,
) {
    let signal = Signal::<NoopRawMutex, ()>::new();
    signal.signal(());
    signal.wait().await;
    assert_eq!(Duration::from_millis(1).as_micros(), 1_000);

    let mut rx = [0; MTU];
    let n = transport
        .recv(&mut rx)
        .await
        .expect("Embassy executor receives shared-memory payload");
    assert_eq!(&rx[..n], b"embassy-pong");
    done.store(true, Ordering::SeqCst);
}

#[embassy_executor::task]
async fn remote_task(base_addr: usize, doorbell: EmbassyDoorbell) {
    yield_now().await;

    let ch = unsafe { Chan::new_unchecked(0) };
    let base = base_addr as *mut u8;
    unsafe {
        let rx_layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx_layout.slot_addr(base), b"embassy-pong", false);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }
    doorbell.ring(ch).expect("wake app-side Embassy doorbell");
}

#[test]
fn recv_future_wakes_on_embassy_platform_std_executor() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    let backing = Box::leak(backing);
    let base = backing.as_mut_ptr();
    let base_addr = base as usize;
    let ch1 = unsafe { Chan::new_unchecked(0) };
    let ch2 = unsafe { Chan::new_unchecked(1) };
    let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
    let (app_doorbell, remote_doorbell) = EmbassyDoorbell::pair();
    let transport = unsafe { SharedMemoryTransport::new(ch1, ch2, app_doorbell, region) };
    let done = Box::leak(Box::new(AtomicBool::new(false)));
    let executor = Box::leak(Box::new(Executor::new()));

    executor.run_until(
        |spawner| {
            spawner.spawn(receive_task(transport, done).expect("create receive task"));
            spawner.spawn(remote_task(base_addr, remote_doorbell).expect("create remote task"));
        },
        || done.load(Ordering::SeqCst),
    );
}

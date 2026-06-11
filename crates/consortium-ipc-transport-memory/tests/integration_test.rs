mod common;

use std::sync::mpsc;

use common::doorbell::HostDoorbell;
use consortium_ipc::{Chan, Doorbell, SramRegion, Transport};
use consortium_ipc_transport_memory::{
    descriptor::{
        ChannelDirection, ChannelLayout, RX_UPDATED_ID_OFFSET, channel_table_entry_offset,
        descriptor_init,
    },
    slot,
    transport::SharedMemoryTransport,
};

const MTU: usize = 64;
const SLOT_SIZE: usize = slot::SLOT_HEADER + MTU;
const A_TO_M_SLOT: usize = 0x100;
const M_TO_A_SLOT: usize = A_TO_M_SLOT + SLOT_SIZE;
const REGION_LEN: usize = M_TO_A_SLOT + SLOT_SIZE;

#[test]
fn shared_memory_transport_round_trips_over_heap_region_between_host_threads() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();

    let mut a_to_m = ChannelLayout::default();
    a_to_m.offset = A_TO_M_SLOT as u32;
    a_to_m.size = SLOT_SIZE as u32;
    a_to_m.direction = ChannelDirection::AToM;

    let mut m_to_a = ChannelLayout::default();
    m_to_a.offset = M_TO_A_SLOT as u32;
    m_to_a.size = SLOT_SIZE as u32;
    m_to_a.direction = ChannelDirection::MToA;

    unsafe {
        descriptor_init(base, 2, MTU, &[a_to_m, m_to_a]);
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(0xFF);
    }

    let backing = Box::leak(backing);
    let base_addr = backing.as_mut_ptr() as usize;
    let (app_doorbell, remote_doorbell) = HostDoorbell::pair();
    let ch = unsafe { Chan::new_unchecked(0) };
    let (reply_written_tx, reply_written_rx) = mpsc::channel();

    let remote_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("create remote host runtime");

        rt.block_on(async move {
            remote_doorbell
                .wait(ch)
                .await
                .expect("remote waits for app doorbell");

            let base = base_addr as *mut u8;
            let app_to_remote =
                unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
            let remote_to_app =
                unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1))) };

            let mut rx = [0; MTU];
            let n = unsafe { slot::try_read(app_to_remote.slot_addr(base), &mut rx) }
                .expect("remote reads app payload");
            assert_eq!(&rx[..n], b"ping");

            unsafe {
                slot::write(remote_to_app.slot_addr(base), b"pong");
                base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
            }

            reply_written_tx
                .send(())
                .expect("signal reply is in shared memory");
            remote_doorbell.ring(ch).expect("ring app doorbell");
        });
    });

    let app_thread = std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("create app host runtime");

        rt.block_on(async move {
            let base = base_addr as *mut u8;
            let region = unsafe { SramRegion::new(base, REGION_LEN) };
            let mut transport =
                unsafe { SharedMemoryTransport::new(ch, app_doorbell, region, base) };

            transport.send(b"ping").await.expect("send app payload");

            reply_written_rx
                .recv()
                .expect("remote writes reply before app receive");

            let mut rx = [0; MTU];
            let n = transport.recv(&mut rx).await.expect("receive reply");
            assert_eq!(&rx[..n], b"pong");
        });
    });

    app_thread.join().expect("app thread completes");
    remote_thread.join().expect("remote thread completes");
}

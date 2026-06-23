use std::sync::mpsc;

use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{MTU, REGION_LEN, init_two_channel_descriptor};
use consortium_ipc::{Chan, Doorbell, RecvTransport, SendTransport, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport, channel_table_entry_offset, slot,
};

#[test]
fn shared_memory_transport_round_trips_over_heap_region_between_host_threads() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();

    init_two_channel_descriptor(base);

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

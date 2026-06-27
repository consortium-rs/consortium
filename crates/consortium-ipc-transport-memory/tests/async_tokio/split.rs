use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{
    HostSramTransportRx, HostSramTransportTx, MTU, REGION_LEN, init_two_channel_descriptor,
};
use consortium_ipc::{Chan, RecvTransport, SendTransport};
use consortium_ipc_transport_memory::{
    ChannelLayout, Primary, RX_UPDATED_ID_OFFSET, Secondary, SharedMemoryTransport,
    channel_table_entry_offset, slot,
};
use consortium_shared_memory::SramRegion;

/// Demonstrate that `split()` produces two independent halves that can coexist
/// in the same scope. With a combined [`SharedMemoryTransport`] the borrow
/// checker would not allow holding a send future and a receive future at the same
/// time; the split halves remove that restriction.
#[test]
fn split_into_halves_allows_tx_and_rx_to_coexist() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    init_two_channel_descriptor(base);

    unsafe {
        let rx_layout = ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(1)));
        slot::write(rx_layout.slot_addr(base), b"from-remote");
        base.add(RX_UPDATED_ID_OFFSET).write_volatile(1);
    }

    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create host runtime");

    rt.block_on(async move {
        let region = unsafe { SramRegion::new(base, REGION_LEN) };
        let (doorbell, _remote) = HostDoorbell::pair();
        let ch_tx = unsafe { Chan::new_unchecked(0) };
        let ch_rx = unsafe { Chan::new_unchecked(1) };
        let transport = unsafe { SharedMemoryTransport::new(ch_tx, ch_rx, doorbell, region, base) };

        let (mut tx, mut rx): (HostSramTransportTx<Primary>, HostSramTransportRx<Primary>) =
            transport.split();

        tx.send(b"to-remote").await.expect("tx half sends");

        let mut buf = [0; MTU];
        let n = rx.recv(&mut buf).await.expect("rx half receives");
        assert_eq!(&buf[..n], b"from-remote");

        let tx_layout =
            unsafe { ChannelLayout::from_raw_ptr(base.add(channel_table_entry_offset(0))) };
        let mut raw = [0; MTU];
        let n = unsafe { slot::try_read(tx_layout.slot_addr(base), &mut raw) }
            .expect("tx slot is readable");
        assert_eq!(&raw[..n], b"to-remote");
    });
}

/// Two-thread multidirectional demo: A and M each split their transport and send
/// and receive at the same time within a single thread.
///
/// Both cores share a *single* descriptor and slot region; each transport selects
/// its [`Side`] so the absolute channel directions resolve to the right TX/RX from
/// each core's perspective.
#[test]
fn split_halves_allow_full_duplex_between_two_threads() {
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    init_two_channel_descriptor(base_addr as *mut u8);

    let (a_doorbell, m_doorbell) = HostDoorbell::pair();

    std::thread::scope(|scope| {
        let a_thread = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create A-side runtime");

            rt.block_on(async move {
                let base = base_addr as *mut u8;
                let ch_tx = unsafe { Chan::new_unchecked(0) };
                let ch_rx = unsafe { Chan::new_unchecked(1) };
                let region = unsafe { SramRegion::new(base, REGION_LEN) };
                let transport = unsafe {
                    SharedMemoryTransport::<Primary, _, _>::new(
                        ch_tx, ch_rx, a_doorbell, region, base,
                    )
                };

                let (mut tx, mut rx): (HostSramTransportTx<Primary>, HostSramTransportRx<Primary>) =
                    transport.split();

                let mut buf = [0u8; MTU];
                let (send_res, recv_res) = futures::join!(tx.send(b"ping"), rx.recv(&mut buf));

                send_res.expect("A sends ping");
                let n = recv_res.expect("A receives pong");
                assert_eq!(&buf[..n], b"pong");
            });
        });

        let m_thread = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create M-side runtime");

            rt.block_on(async move {
                let base = base_addr as *mut u8;
                let ch_rx = unsafe { Chan::new_unchecked(0) };
                let ch_tx = unsafe { Chan::new_unchecked(1) };
                let region = unsafe { SramRegion::new(base, REGION_LEN) };
                let transport = unsafe {
                    SharedMemoryTransport::<Secondary, _, _>::new(
                        ch_tx, ch_rx, m_doorbell, region, base,
                    )
                };

                let (mut tx, mut rx): (
                    HostSramTransportTx<Secondary>,
                    HostSramTransportRx<Secondary>,
                ) = transport.split();

                let mut buf = [0u8; MTU];
                let (recv_res, send_res) = futures::join!(rx.recv(&mut buf), tx.send(b"pong"));

                let n = recv_res.expect("M receives ping");
                assert_eq!(&buf[..n], b"ping");
                send_res.expect("M sends pong");
            });
        });

        a_thread.join().expect("A-side thread completes");
        m_thread.join().expect("M-side thread completes");
    });
}

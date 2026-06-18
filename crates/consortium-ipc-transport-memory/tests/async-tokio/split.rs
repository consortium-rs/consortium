use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{
    A_DESCRIPTOR_OFFSET, HostSramTransportRx, HostSramTransportTx, M_DESCRIPTOR_OFFSET, MTU,
    PAIRED_REGION_LEN, REGION_LEN, init_paired_descriptors, init_two_channel_descriptor,
};
use consortium_ipc::{Chan, RecvTransport, SendTransport, SramRegion};
use consortium_ipc_transport_memory::{
    ChannelLayout, RX_UPDATED_ID_OFFSET, SharedMemoryTransport, channel_table_entry_offset, slot,
};

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
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, base) };

        let (mut tx, mut rx): (HostSramTransportTx, HostSramTransportRx) = transport.split();

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
#[test]
fn split_halves_allow_full_duplex_between_two_threads() {
    let mut backing = vec![0u8; PAIRED_REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    init_paired_descriptors(base_addr as *mut u8);

    let (a_doorbell, m_doorbell) = HostDoorbell::pair();

    std::thread::scope(|scope| {
        let a_thread = scope.spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create A-side runtime");

            rt.block_on(async move {
                let a_base = (base_addr + A_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(a_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, a_doorbell, region, a_base) };

                let (mut tx, mut rx): (HostSramTransportTx, HostSramTransportRx) =
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
                let m_base = (base_addr + M_DESCRIPTOR_OFFSET) as *mut u8;
                let ch = unsafe { Chan::new_unchecked(0) };
                let region = unsafe { SramRegion::new(m_base, PAIRED_REGION_LEN) };
                let transport =
                    unsafe { SharedMemoryTransport::new(ch, m_doorbell, region, m_base) };

                let (mut tx, mut rx): (HostSramTransportTx, HostSramTransportRx) =
                    transport.split();

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

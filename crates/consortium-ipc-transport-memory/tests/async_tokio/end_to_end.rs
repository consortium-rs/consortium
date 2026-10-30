use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{MTU, REGION_LEN, init_two_channel_descriptor};
use consortium_ipc::{Chan, RecvTransport, SendTransport};
use consortium_ipc_transport_memory::{Primary, Secondary, SharedMemoryTransport};
use consortium_shared_memory::UncachedRegion;

/// End-to-end ping/pong over the shared-memory transport.
///
/// Both cores are modelled over a *single* shared descriptor and shared slot
/// region — the same memory the real A-core and M-core would map. On real
/// firmware each core's TX/RX perspective is fixed by the build target; here both
/// threads run under `std`, so each transport selects its [`Side`] explicitly to
/// emulate the per-core direction interpretation. Neither side touches the shared
/// memory directly: both drive a real [`SharedMemoryTransport`].
#[test]
fn shared_memory_transport_round_trips_over_one_shared_descriptor() {
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
                let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
                let mut transport = unsafe {
                    SharedMemoryTransport::<Primary, _, _>::new(ch_tx, ch_rx, a_doorbell, region)
                };

                transport.send(b"ping").await.expect("A sends ping");

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("A receives pong");
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
                let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
                let mut transport = unsafe {
                    SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, m_doorbell, region)
                };

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("M receives ping");
                assert_eq!(&buf[..n], b"ping");

                transport.send(b"pong").await.expect("M sends pong");
            });
        });

        a_thread.join().expect("A-side thread completes");
        m_thread.join().expect("M-side thread completes");
    });
}

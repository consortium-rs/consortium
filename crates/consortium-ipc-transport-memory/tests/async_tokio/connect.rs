use crate::common::doorbell::HostDoorbell;
use consortium_ipc::{Chan, Connect, RecvTransport, SendTransport};
use consortium_ipc_transport_memory::{
    DescriptorLayout, Primary, Secondary, SharedMemoryTransport, get_state,
};
use consortium_shared_memory::UncachedRegion;

const MTU: usize = 64;
const REGION_LEN: usize = 0x800;

/// Bring-up code that knows only the trait.
///
/// This is the point of [`Connect`]: one call site drives either end of the
/// link, with the protocol role chosen by the transport's `Side` rather than by
/// a branch here. Both sides of the tests below go through this function.
async fn bring_up<T: Connect>(transport: &mut T, params: T::Params) -> Result<(), T::Error> {
    transport.connect(params).await
}

/// Primary and secondary handshake over one shared region, then exchange a
/// message across the descriptor the handshake itself built.
///
/// Unlike the other transport tests, nothing here hand-writes a descriptor: the
/// channel table under test is the one `connect` published and validated.
#[test]
fn connect_builds_a_usable_link_from_both_sides() {
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;

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

                // The channel table does not exist yet, so no size is knowable.
                assert_eq!(transport.max_send_size(), 0);
                assert_eq!(transport.max_recv_size(), 0);

                bring_up(&mut transport, DescriptorLayout::new(2, MTU))
                    .await
                    .expect("primary connect");

                assert_eq!(transport.max_send_size(), MTU);
                assert_eq!(transport.max_recv_size(), MTU);

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

                // Same call as the primary above; the `Side` picks the role.
                bring_up(&mut transport, DescriptorLayout::new(2, MTU))
                    .await
                    .expect("secondary connect");

                assert_eq!(transport.max_send_size(), MTU);
                assert_eq!(transport.max_recv_size(), MTU);

                let mut buf = [0u8; MTU];
                let n = transport.recv(&mut buf).await.expect("M receives ping");
                assert_eq!(&buf[..n], b"ping");
                transport.send(b"pong").await.expect("M sends pong");
            });
        });

        a_thread.join().expect("A-side thread");
        m_thread.join().expect("M-side thread");
    });

    assert_eq!(
        unsafe { get_state(backing.as_ptr()) },
        consortium_ipc_transport_memory::DescState::Ready
    );
}

/// A secondary expecting a different channel count refuses the link, and the
/// failure arrives as the transport's own error type.
///
/// The descriptor is pre-published here rather than handshaken, so the
/// secondary's readiness gate passes immediately and no peer thread is needed.
#[test]
fn connect_secondary_rejects_a_mismatched_channel_count() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("create runtime");
    rt.block_on(secondary_rejects_mismatched_channel_count());
}

async fn secondary_rejects_mismatched_channel_count() {
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base = backing.as_mut_ptr();
    crate::transport_support::init_two_channel_descriptor(base);

    let (_a_doorbell, m_doorbell) = HostDoorbell::pair();
    let ch_rx = unsafe { Chan::new_unchecked(0) };
    let ch_tx = unsafe { Chan::new_unchecked(1) };
    let region = unsafe { UncachedRegion::new(base, REGION_LEN) };
    let mut transport =
        unsafe { SharedMemoryTransport::<Secondary, _, _>::new(ch_tx, ch_rx, m_doorbell, region) };

    let err = bring_up(&mut transport, DescriptorLayout::new(4, MTU))
        .await
        .expect_err("secondary should reject a 2-channel descriptor");

    assert!(
        matches!(
            err,
            consortium_ipc_transport_memory::Error::Descriptor(
                consortium_ipc_transport_memory::DescriptorError::InvalidChannelCount {
                    expected: 4,
                    actual: 2,
                }
            )
        ),
        "unexpected error: {err:?}"
    );
    // A refused handshake leaves the transport with no usable MTU.
    assert_eq!(transport.max_send_size(), 0);
}

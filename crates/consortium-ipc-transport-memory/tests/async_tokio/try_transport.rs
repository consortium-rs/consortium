//! The non-waiting pair (`TrySendTransport` / `TryRecvTransport`) and the core
//! `spin_until` helper over it. Plain `#[test]`s: nothing here needs an executor,
//! which is the point.

use crate::common::doorbell::HostDoorbell;
use crate::transport_support::{MTU, REGION_LEN, init_two_channel_descriptor};
use consortium_core::{
    TryError, TryRecvTransport, TrySendTransport, UncachedRegion,
    wait::{BlockingError, Readiness, spin_until},
};
use consortium_ipc::{Chan, Primary, Secondary};
use consortium_ipc_transport_memory::{Error, SharedMemoryTransport};

type Pair = (
    SharedMemoryTransport<Primary, HostDoorbell, UncachedRegion>,
    SharedMemoryTransport<Secondary, HostDoorbell, UncachedRegion>,
);

/// A primary and a secondary over one heap-backed window, each with its own
/// doorbell handle.
fn pair(base: *mut u8) -> Pair {
    init_two_channel_descriptor(base);
    let (primary_bell, secondary_bell) = HostDoorbell::pair();
    let ch0 = unsafe { Chan::new_unchecked(0) };
    let ch1 = unsafe { Chan::new_unchecked(1) };
    let primary = unsafe {
        SharedMemoryTransport::<Primary, _, _>::new(
            ch0,
            ch1,
            primary_bell,
            UncachedRegion::new(base, REGION_LEN),
        )
    };
    let secondary = unsafe {
        SharedMemoryTransport::<Secondary, _, _>::new(
            ch1,
            ch0,
            secondary_bell,
            UncachedRegion::new(base, REGION_LEN),
        )
    };
    (primary, secondary)
}

#[test]
fn try_send_would_block_until_the_peer_takes_the_frame() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let (mut primary, mut secondary) = pair(backing.as_mut_ptr());
    let mut buf = [0; MTU];

    primary
        .try_send(b"first")
        .expect("free slot takes the frame");
    assert!(matches!(
        primary.try_send(b"second"),
        Err(TryError::WouldBlock)
    ));

    let n = secondary.try_recv(&mut buf).expect("frame is readable");
    assert_eq!(&buf[..n], b"first");

    primary
        .try_send(b"second")
        .expect("drained slot takes the next frame");
    let n = secondary.try_recv(&mut buf).expect("frame is readable");
    assert_eq!(&buf[..n], b"second");
}

#[test]
fn try_recv_would_block_when_nothing_is_published() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let (_primary, mut secondary) = pair(backing.as_mut_ptr());
    let mut buf = [0; MTU];

    assert!(matches!(
        secondary.try_recv(&mut buf),
        Err(TryError::WouldBlock)
    ));
}

#[test]
fn oversized_try_send_fails_rather_than_blocking() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let (mut primary, _secondary) = pair(backing.as_mut_ptr());

    let err = primary
        .try_send(&[0; MTU + 1])
        .expect_err("a frame larger than the slot can never fit");
    assert!(matches!(
        err,
        TryError::Fail(Error::PayloadTooLarge { capacity: MTU, .. })
    ));
}

#[test]
fn spin_until_drives_try_recv_without_an_executor() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let (mut primary, mut secondary) = pair(backing.as_mut_ptr());
    let mut buf = [0; MTU];

    primary
        .try_send(b"spun")
        .expect("free slot takes the frame");
    let n = spin_until(
        || Readiness::from_try(secondary.try_recv(&mut buf)),
        || panic!("a published frame is ready on the first check"),
    )
    .expect("frame is readable");
    assert_eq!(&buf[..n], b"spun");
}

#[test]
fn spin_until_reports_the_deadline() {
    let mut backing = vec![0; REGION_LEN].into_boxed_slice();
    let (_primary, mut secondary) = pair(backing.as_mut_ptr());
    let mut buf = [0; MTU];
    let mut budget = 3;

    let err = spin_until(
        || Readiness::from_try(secondary.try_recv(&mut buf)),
        || {
            budget -= 1;
            budget == 0
        },
    )
    .expect_err("nothing is ever published");
    assert!(matches!(err, BlockingError::TimedOut));
    assert_eq!(budget, 0);
}

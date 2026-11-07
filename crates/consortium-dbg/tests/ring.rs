//! The `CDBG` ring through `consortium-core`'s non-waiting transport pair:
//! `RingWriter` (`TrySendTransport`) and `RingReader` (`TryRecvTransport`) over a
//! heap-backed `UncachedRegion`.

#![allow(clippy::multiple_unsafe_ops_per_block)]

use consortium_core::{
    TryError, TryRecvTransport, TrySendTransport, UncachedRegion,
    wait::{BlockingError, Readiness, spin_until},
};
use consortium_dbg::{
    descriptor::{DESCRIPTOR_SIZE, Flags, MAX_FRAME_LEN},
    packet::PACKET_HEADER_SIZE,
    ring::{RingError, RingReader, RingWriter},
};

/// A heap window standing in for debug SRAM, with a writer and a reader over it.
struct Rig {
    backing: Box<[u8]>,
    writer: RingWriter<UncachedRegion>,
    reader: RingReader<UncachedRegion>,
}

fn rig(len: usize) -> Rig {
    // Garbage, as a warm reset would leave it: `init` must not trust any of it.
    let mut backing = vec![0xC0u8; len].into_boxed_slice();
    let base = backing.as_mut_ptr();
    // SAFETY: `backing` outlives both ends; the rig holds the only writer and the
    // only reader.
    let writer = unsafe { RingWriter::init(UncachedRegion::new(base, len)) }.unwrap();
    // SAFETY: as above.
    let reader = unsafe { RingReader::new(UncachedRegion::new(base, len)) }.unwrap();
    Rig {
        backing,
        writer,
        reader,
    }
}

/// Move one frame across any try transport pair, as generic code would.
fn relay<W: TrySendTransport, R: TryRecvTransport>(
    writer: &mut W,
    reader: &mut R,
    frame: &[u8],
    buf: &mut [u8],
) -> usize
where
    W::Error: core::fmt::Debug,
    R::Error: core::fmt::Debug,
{
    writer.try_send(frame).expect("ring has room");
    reader.try_recv(buf).expect("frame is readable")
}

#[test]
fn frames_round_trip_in_order() {
    let mut r = rig(256);
    let mut buf = [0u8; MAX_FRAME_LEN];
    for frame in [&b"alpha"[..], b"beta", b"gamma"] {
        let n = relay(&mut r.writer, &mut r.reader, frame, &mut buf);
        assert_eq!(&buf[..n], frame);
    }
    assert!(matches!(
        r.reader.try_recv(&mut buf),
        Err(TryError::WouldBlock)
    ));
}

#[test]
fn a_full_ring_would_block_until_the_reader_drains() {
    let mut r = rig(64);
    let frame = [0x5Au8; 16];
    let mut sent = 0;
    while r.writer.try_send(&frame).is_ok() {
        sent += 1;
    }
    assert!(sent > 0);
    assert!(matches!(
        r.writer.try_send(&frame),
        Err(TryError::WouldBlock)
    ));
    assert!(r.writer.snapshot().flags.contains(Flags::Overflow));

    let mut buf = [0u8; MAX_FRAME_LEN];
    assert_eq!(r.reader.try_recv(&mut buf).unwrap(), frame.len());
    r.writer
        .try_send(&frame)
        .expect("a drained slot takes the next frame");
    assert!(
        !r.writer.snapshot().flags.contains(Flags::Overflow),
        "a successful write clears the overflow latch"
    );
}

#[test]
fn an_oversized_frame_fails_instead_of_blocking() {
    let mut r = rig(64);
    let max = r.writer.max_send_size();
    assert_eq!(max, 64 - DESCRIPTOR_SIZE - PACKET_HEADER_SIZE);

    let err = r.writer.try_send(&vec![0u8; max + 1]).unwrap_err();
    assert_eq!(
        err,
        TryError::Fail(RingError::FrameTooLarge { len: max + 1, max })
    );
    assert!(r.writer.snapshot().flags.contains(Flags::Overflow));
    r.writer
        .try_send(&vec![0u8; max])
        .expect("the largest frame fits");
}

#[test]
fn no_frame_exceeds_one_packet_however_large_the_ring() {
    let r = rig(4096);
    assert_eq!(r.writer.max_send_size(), MAX_FRAME_LEN);
    assert_eq!(r.reader.max_recv_size(), MAX_FRAME_LEN);
}

#[test]
fn a_short_buffer_leaves_the_frame_unread() {
    let mut r = rig(256);
    r.writer.try_send(b"0123456789").unwrap();

    let err = r.reader.try_recv(&mut [0u8; 4]).unwrap_err();
    assert_eq!(
        err,
        TryError::Fail(RingError::BufferTooSmall { len: 4, needed: 10 })
    );
    let mut buf = [0u8; MAX_FRAME_LEN];
    let n = r
        .reader
        .try_recv(&mut buf)
        .expect("the frame is still there");
    assert_eq!(&buf[..n], b"0123456789");
}

#[test]
fn a_faulted_ring_refuses_frames() {
    let mut r = rig(256);
    let mut desc = r.writer.snapshot();
    desc.flags.insert(Flags::Error);
    // SAFETY: the descriptor lies inside the rig's backing buffer.
    unsafe { desc.write_to_raw_ptr(r.backing.as_mut_ptr()) };
    assert_eq!(
        r.writer.try_send(b"late"),
        Err(TryError::Fail(RingError::Faulted))
    );
}

#[test]
fn mark_panic_keeps_the_other_flags() {
    let mut r = rig(64);
    while r.writer.try_send(&[0u8; 16]).is_ok() {}
    r.writer.mark_panic();
    let flags = r.reader.snapshot().flags;
    assert!(flags.contains(Flags::Panic));
    assert!(flags.contains(Flags::Overflow));
}

#[test]
fn init_rejects_regions_that_cannot_hold_a_frame_or_a_u16_capacity() {
    for len in [DESCRIPTOR_SIZE + PACKET_HEADER_SIZE, 0x1_0000] {
        let mut backing = vec![0u8; len];
        // SAFETY: `backing` outlives the attempt.
        let result = unsafe { RingWriter::init(UncachedRegion::new(backing.as_mut_ptr(), len)) };
        assert!(
            matches!(result, Err(RingError::RegionLength { len: l }) if l == len),
            "{len}"
        );
    }
}

#[test]
fn attach_validates_and_continues_an_existing_ring() {
    let mut backing = vec![0u8; 128].into_boxed_slice();
    let base = backing.as_mut_ptr();
    // SAFETY: `backing` outlives every end built on it; one writer, one reader at a time.
    unsafe {
        let mut first = RingWriter::init(UncachedRegion::new(base, 128)).unwrap();
        first.try_send(b"before").unwrap();
        let mut again = RingWriter::attach(first.into_region()).unwrap();
        again.try_send(b"after").unwrap();

        let mut reader = RingReader::new(UncachedRegion::new(base, 128)).unwrap();
        let mut buf = [0u8; MAX_FRAME_LEN];
        let n = reader.try_recv(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"before");
        let n = reader.try_recv(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"after");
    }

    let mut garbage = vec![0xFFu8; 64];
    // SAFETY: `garbage` outlives the attempt.
    let err = unsafe { RingWriter::attach(UncachedRegion::new(garbage.as_mut_ptr(), 64)) };
    assert!(matches!(err, Err(RingError::MagicMismatch)));
}

#[test]
fn the_reader_refuses_a_capacity_grown_past_its_region() {
    let mut r = rig(128);
    // A misbehaving writer rewrites `capacity` after the reader validated it.
    let mut desc = r.reader.snapshot();
    desc.capacity = 4096;
    // SAFETY: the descriptor lies inside the rig's backing buffer.
    unsafe { desc.write_to_raw_ptr(r.backing.as_mut_ptr()) };
    assert_eq!(
        r.reader.try_recv(&mut [0u8; MAX_FRAME_LEN]),
        Err(TryError::Fail(RingError::CapacityMismatch {
            descriptor: 4096,
            mapped: 128
        }))
    );
}

#[test]
fn spin_until_waits_for_a_frame_from_another_thread() {
    let mut backing = vec![0u8; 256].into_boxed_slice();
    let base = backing.as_mut_ptr();
    // SAFETY: `backing` outlives both threads (joined below); one writer, one reader.
    let mut writer = unsafe { RingWriter::init(UncachedRegion::new(base, 256)) }.unwrap();
    // SAFETY: as above.
    let mut reader = unsafe { RingReader::new(UncachedRegion::new(base, 256)) }.unwrap();

    let producer = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(20));
        writer.try_send(b"late").unwrap();
    });
    let start = std::time::Instant::now();
    let mut buf = [0u8; MAX_FRAME_LEN];
    let n = spin_until(
        || Readiness::from_try(reader.try_recv(&mut buf)),
        || start.elapsed() > std::time::Duration::from_secs(5),
    )
    .expect("the frame arrives before the deadline");
    producer.join().unwrap();
    assert_eq!(&buf[..n], b"late");

    let err = spin_until(|| Readiness::from_try(reader.try_recv(&mut buf)), || true).unwrap_err();
    assert_eq!(err, BlockingError::TimedOut);
}

#[cfg(feature = "logger")]
#[test]
fn the_logger_and_a_ring_reader_agree() {
    use consortium_dbg::logger::ConsortiumLogger;
    let mut backing = vec![0xABu8; 256].into_boxed_slice();
    let base = backing.as_mut_ptr();
    // SAFETY: `backing` covers 256 bytes and outlives every access below.
    unsafe {
        ConsortiumLogger::force_init_shared_memory(base, 256);
        ConsortiumLogger::write_shared_memory(base, b"from the logger");
    }
    // SAFETY: as above; this is the only reader.
    let mut reader = unsafe { RingReader::new(UncachedRegion::new(base, 256)) }.unwrap();
    let mut buf = [0u8; MAX_FRAME_LEN];
    let n = reader.try_recv(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"from the logger");
}

#[test]
fn both_ends_refuse_a_non_coherent_region() {
    use consortium_core::CachedRegion;
    let mut backing = vec![0u8; 256].into_boxed_slice();
    let base = backing.as_mut_ptr();
    // SAFETY: `backing` outlives every attempt below.
    unsafe {
        assert_eq!(
            RingWriter::init(CachedRegion::new(base, 256)).err(),
            Some(RingError::NonCoherentRegion)
        );
        // A valid ring is still refused through a cached mapping of it.
        RingWriter::init(UncachedRegion::new(base, 256)).unwrap();
        assert_eq!(
            RingWriter::attach(CachedRegion::new(base, 256)).err(),
            Some(RingError::NonCoherentRegion)
        );
        assert_eq!(
            RingReader::new(CachedRegion::new(base, 256)).err(),
            Some(RingError::NonCoherentRegion)
        );
    }
}

#[test]
fn drain_stops_at_the_frames_present_when_it_began() {
    let mut r = rig(256);
    r.writer.try_send(b"one").unwrap();
    r.writer.try_send(b"two").unwrap();

    let Rig { writer, reader, .. } = &mut r;
    let mut seen = Vec::new();
    let drained = reader
        .drain(|frame| {
            seen.push(frame.to_vec());
            // Firmware keeps logging while the host drains.
            writer.try_send(b"late").unwrap();
        })
        .unwrap();
    assert_eq!(drained, 2);
    assert_eq!(seen, vec![b"one".to_vec(), b"two".to_vec()]);

    // What arrived mid-drain is still there for the next call.
    let mut later = Vec::new();
    assert_eq!(
        r.reader.drain(|frame| later.push(frame.to_vec())).unwrap(),
        2
    );
    assert_eq!(later, vec![b"late".to_vec(), b"late".to_vec()]);
    assert_eq!(r.reader.drain(|_| panic!("ring is empty")).unwrap(), 0);
}

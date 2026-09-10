use consortium_ipc_transport_memory::slot::{
    self, FLAG_ACK_REQUESTED, FLAG_CONSUMED, FLAG_INTEGRITY_ERROR, FLAG_VALID, ReadOutcome,
    SLOT_HEADER,
};
use consortium_shared_memory::UncachedRegion;

fn flags(slot: &[u8]) -> u32 {
    u32::from_le_bytes(slot[4..8].try_into().expect("slot flag bytes"))
}

#[test]
fn write_then_read_returns_payload_and_consumes_slot() {
    let payload = b"hello-shared-memory";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload, false);
    }

    assert_eq!(flags(&backing), FLAG_VALID);
    assert!(!unsafe { slot::is_consumed(base) });

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    let outcome = unsafe { slot::try_read(base, &mut rx, &region) };

    assert_eq!(
        outcome,
        ReadOutcome::Delivered {
            len: payload.len(),
            ack_requested: false,
        }
    );
    assert_eq!(&rx, payload);
    assert_eq!(flags(&backing), FLAG_CONSUMED);
    assert!(unsafe { slot::is_consumed(base) });
}

#[test]
fn corrupted_payload_sets_integrity_error() {
    let payload = b"trusted-by-crc";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload, false);
    }
    backing[SLOT_HEADER + 3] ^= 0x55;

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(
        unsafe { slot::try_read(base, &mut rx, &region) },
        ReadOutcome::Corrupt {
            ack_requested: false
        }
    );
    assert_eq!(flags(&backing), FLAG_INTEGRITY_ERROR);
    assert!(unsafe { slot::is_consumed(base) });
}

#[test]
fn short_receive_buffer_is_treated_as_integrity_error() {
    let payload = b"longer-than-rx";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload, false);
    }

    let mut rx = [0; 4];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(
        unsafe { slot::try_read(base, &mut rx, &region) },
        ReadOutcome::Corrupt {
            ack_requested: false
        }
    );
    assert_eq!(flags(&backing), FLAG_INTEGRITY_ERROR);
    assert!(unsafe { slot::is_consumed(base) });
}

#[test]
fn ack_request_is_published_with_valid_and_reported_on_read() {
    let payload = b"please-confirm";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload, true);
    }

    // The request must ride in the same store that publishes the frame, so a peer that
    // observes `FLAG_VALID` always observes the request along with it.
    assert_eq!(flags(&backing), FLAG_VALID | FLAG_ACK_REQUESTED);
    assert!(unsafe { slot::ack_requested(base) });
    // ...and it must not make the slot look reusable to the sender.
    assert!(!unsafe { slot::is_consumed(base) });

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(
        unsafe { slot::try_read(base, &mut rx, &region) },
        ReadOutcome::Delivered {
            len: payload.len(),
            ack_requested: true,
        }
    );
    assert_eq!(&rx, payload);
    assert_eq!(flags(&backing), FLAG_CONSUMED);
}

#[test]
fn ack_request_survives_a_failed_integrity_check() {
    // The sender is parked waiting for this slot to drain.  A corrupt payload still frees
    // the slot, so the receiver has to report the request or the sender waits forever for
    // an edge nobody will produce.
    let payload = b"confirm-even-if-corrupt";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload, true);
    }
    backing[SLOT_HEADER + 5] ^= 0xA5;

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(
        unsafe { slot::try_read(base, &mut rx, &region) },
        ReadOutcome::Corrupt {
            ack_requested: true
        }
    );
    assert_eq!(flags(&backing), FLAG_INTEGRITY_ERROR);
}

#[test]
fn empty_slot_reports_nothing() {
    let mut backing = vec![0; SLOT_HEADER + 8];
    let base = backing.as_mut_ptr();

    let mut rx = [0; 8];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(
        unsafe { slot::try_read(base, &mut rx, &region) },
        ReadOutcome::Empty
    );
}

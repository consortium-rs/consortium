use consortium_ipc_transport_memory::slot::{
    self, FLAG_CONSUMED, FLAG_INTEGRITY_ERROR, FLAG_VALID, SLOT_HEADER,
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
        slot::write(base, payload);
    }

    assert_eq!(flags(&backing), FLAG_VALID);
    assert!(!unsafe { slot::is_consumed(base) });

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    let n = unsafe { slot::try_read(base, &mut rx, &region) }.expect("read valid slot");

    assert_eq!(n, payload.len());
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
        slot::write(base, payload);
    }
    backing[SLOT_HEADER + 3] ^= 0x55;

    let mut rx = vec![0; payload.len()];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(unsafe { slot::try_read(base, &mut rx, &region) }, None);
    assert_eq!(flags(&backing), FLAG_INTEGRITY_ERROR);
    assert!(unsafe { slot::is_consumed(base) });
}

#[test]
fn short_receive_buffer_is_treated_as_integrity_error() {
    let payload = b"longer-than-rx";
    let mut backing = vec![0; SLOT_HEADER + payload.len()];
    let base = backing.as_mut_ptr();

    unsafe {
        slot::write(base, payload);
    }

    let mut rx = [0; 4];
    let region = unsafe { UncachedRegion::new(base, backing.len()) };
    assert_eq!(unsafe { slot::try_read(base, &mut rx, &region) }, None);
    assert_eq!(flags(&backing), FLAG_INTEGRITY_ERROR);
    assert!(unsafe { slot::is_consumed(base) });
}

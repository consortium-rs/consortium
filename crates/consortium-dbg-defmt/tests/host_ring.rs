#![cfg(feature = "host")]

use consortium_dbg_defmt::{
    descriptor::{DESCRIPTOR_SIZE, Descriptor, READ_PTR_OFFSET, WRITE_PTR_OFFSET},
    host::{HostError, HostRing},
    logger::ConsortiumLogger,
};

const REGION_LEN: usize = 256;

fn init_region() -> Box<[u8]> {
    let mut region = vec![0u8; REGION_LEN].into_boxed_slice();
    // SAFETY: region is heap-allocated and exclusively owned; pointer valid for REGION_LEN bytes.
    unsafe {
        Descriptor::new(REGION_LEN as u16).write_to_raw_ptr(region.as_mut_ptr());
    }
    region
}

// ── construction validation ───────────────────────────────────────────────────

#[test]
fn host_ring_rejects_region_smaller_than_descriptor() {
    let mut buf = vec![0u8; DESCRIPTOR_SIZE - 1];
    // SAFETY: buf is exclusively owned; pointer valid for buf.len() bytes.
    let result = unsafe { HostRing::from_raw_parts(buf.as_mut_ptr(), buf.len()) };
    assert!(matches!(
        result,
        Err(HostError::CapacityMismatch { descriptor: 0, .. })
    ));
}

#[test]
fn host_ring_rejects_bad_magic() {
    let mut buf = vec![0u8; REGION_LEN]; // zeroed — magic will not match CDBG
    // SAFETY: buf is exclusively owned; pointer valid for REGION_LEN bytes.
    let result = unsafe { HostRing::from_raw_parts(buf.as_mut_ptr(), buf.len()) };
    assert!(matches!(result, Err(HostError::MagicMismatch)));
}

#[test]
fn host_ring_rejects_bad_version() {
    let mut region = init_region();
    region[4] = 99; // version byte is at offset 4 inside DescriptorHeader
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let result = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN) };
    assert!(matches!(result, Err(HostError::VersionMismatch)));
}

#[test]
fn host_ring_rejects_capacity_exceeding_mapped_length() {
    let mut region = init_region(); // descriptor.capacity == REGION_LEN == 256
    // Pass len=128 so capacity(256) > len(128).
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes; we only read the descriptor.
    let result = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), 128) };
    assert!(matches!(
        result,
        Err(HostError::CapacityMismatch {
            descriptor: 256,
            mapped: 128
        })
    ));
}

#[test]
fn host_ring_rejects_write_ptr_out_of_bounds() {
    let mut region = init_region();
    // Overwrite write_ptr to 0, which is below DATA_START.
    let zero = 0u16.to_le_bytes();
    region[WRITE_PTR_OFFSET] = zero[0];
    region[WRITE_PTR_OFFSET + 1] = zero[1];
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let result = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN) };
    assert!(matches!(
        result,
        Err(HostError::PointerOutOfBounds { ptr: 0, .. })
    ));
}

#[test]
fn host_ring_rejects_read_ptr_out_of_bounds() {
    let mut region = init_region();
    // Overwrite read_ptr to capacity (== REGION_LEN), which is at the boundary (invalid).
    let oob = (REGION_LEN as u16).to_le_bytes();
    region[READ_PTR_OFFSET] = oob[0];
    region[READ_PTR_OFFSET + 1] = oob[1];
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let result = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN) };
    assert!(matches!(
        result,
        Err(HostError::PointerOutOfBounds { ptr: 256, .. })
    ));
}

#[test]
fn host_ring_constructs_on_valid_descriptor() {
    let mut region = init_region();
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let result = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN) };
    assert!(result.is_ok());
}

// ── read_packet / drain_packets ───────────────────────────────────────────────

#[test]
fn host_ring_read_packet_returns_none_on_empty_ring() {
    let mut region = init_region();
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN).unwrap() };
    assert!(matches!(ring.read_packet(), Ok(None)));
}

#[test]
fn host_ring_drain_returns_empty_vec_on_empty_ring() {
    let mut region = init_region();
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN).unwrap() };
    assert_eq!(ring.drain_packets().unwrap(), Vec::<Vec<u8>>::new());
}

#[test]
fn host_ring_drains_packets_written_by_logger() {
    let mut region = init_region();
    let base = region.as_mut_ptr();

    // SAFETY: base is valid for REGION_LEN bytes; descriptor was initialized by init_region.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"alpha");
    }
    // SAFETY: same region; independent write.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"beta");
    }
    // SAFETY: same region; independent write.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"gamma");
    }

    // SAFETY: region is exclusively owned; base valid for REGION_LEN bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(base, REGION_LEN).unwrap() };
    let packets = ring.drain_packets().unwrap();

    assert_eq!(
        packets,
        vec![b"alpha".to_vec(), b"beta".to_vec(), b"gamma".to_vec()]
    );
}

#[test]
fn host_ring_read_ptr_advances_after_each_packet() {
    let mut region = init_region();
    let base = region.as_mut_ptr();

    // SAFETY: base is valid for REGION_LEN bytes; descriptor was initialized by init_region.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"first");
    }
    // SAFETY: same region; independent write.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"second");
    }

    // SAFETY: region is exclusively owned; base valid for REGION_LEN bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(base, REGION_LEN).unwrap() };

    let snap_before = ring.snapshot();
    let first = ring.read_packet().unwrap().unwrap();
    let snap_after_first = ring.snapshot();

    assert_eq!(first, b"first");
    assert!(snap_after_first.read_ptr > snap_before.read_ptr);

    let second = ring.read_packet().unwrap().unwrap();
    let snap_after_second = ring.snapshot();

    assert_eq!(second, b"second");
    assert!(snap_after_second.read_ptr > snap_after_first.read_ptr);

    assert!(matches!(ring.read_packet(), Ok(None)));
}

#[test]
fn host_ring_is_empty_after_full_drain() {
    let mut region = init_region();
    let base = region.as_mut_ptr();

    // SAFETY: base is valid for REGION_LEN bytes; descriptor was initialized by init_region.
    unsafe {
        ConsortiumLogger::write_shared_memory(base, b"only");
    }

    // SAFETY: region is exclusively owned; base valid for REGION_LEN bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(base, REGION_LEN).unwrap() };
    let packets = ring.drain_packets().unwrap();
    assert_eq!(packets, vec![b"only".to_vec()]);

    // Ring must be empty now.
    assert!(matches!(ring.read_packet(), Ok(None)));
    assert_eq!(ring.drain_packets().unwrap(), Vec::<Vec<u8>>::new());
}

#[test]
fn host_ring_snapshot_reflects_descriptor_state() {
    let mut region = init_region();
    // SAFETY: region is exclusively owned; pointer valid for REGION_LEN bytes.
    let ring = unsafe { HostRing::from_raw_parts(region.as_mut_ptr(), REGION_LEN).unwrap() };
    let snap = ring.snapshot();

    assert!(snap.check());
    assert_eq!(snap.capacity, REGION_LEN as u16);
    assert_eq!(snap.read_ptr, DESCRIPTOR_SIZE as u16);
    assert_eq!(snap.write_ptr, DESCRIPTOR_SIZE as u16);
}

// ── env-gated decode fixture (ignored by default) ─────────────────────────────

/// Run with:
///
/// ```text
/// CONSORTIUM_DEFMT_ELF=/path/to/firmware.elf \
/// CONSORTIUM_DEFMT_PACKETS_HEX=aabbcc;ddeeff \
/// [CONSORTIUM_DEFMT_EXPECT=substring] \
/// cargo test -p consortium-dbg-defmt --features decode \
///   -- --ignored defmt_decode_from_env_fixture
/// ```
///
/// `CONSORTIUM_DEFMT_PACKETS_HEX` is a semicolon-separated list of hex-encoded
/// packet payloads exactly as produced by the MCU's defmt logger.
#[cfg(feature = "decode")]
#[test]
#[ignore = "requires CONSORTIUM_DEFMT_ELF and CONSORTIUM_DEFMT_PACKETS_HEX env vars"]
fn defmt_decode_from_env_fixture() {
    use consortium_dbg_defmt::host::{DefmtContext, DefmtSession};

    let elf_path = std::env::var("CONSORTIUM_DEFMT_ELF")
        .expect("set CONSORTIUM_DEFMT_ELF to the path of the firmware ELF");
    let packets_hex = std::env::var("CONSORTIUM_DEFMT_PACKETS_HEX")
        .expect("set CONSORTIUM_DEFMT_PACKETS_HEX to semicolon-separated hex packet payloads");
    let expect_substr = std::env::var("CONSORTIUM_DEFMT_EXPECT").ok();

    let elf_bytes = std::fs::read(&elf_path)
        .unwrap_or_else(|e| panic!("failed to read ELF at {elf_path}: {e}"));

    let ctx =
        DefmtContext::from_elf_bytes(&elf_bytes).expect("failed to parse defmt table from ELF");
    let mut session = DefmtSession::new(&ctx);

    // Decode hex → raw bytes for each semicolon-separated entry.
    let packet_payloads: Vec<Vec<u8>> = packets_hex
        .split(';')
        .filter(|s| !s.trim().is_empty())
        .map(|h| {
            let h = h.trim();
            hex::decode(h).unwrap_or_else(|e| panic!("invalid hex {h:?}: {e}"))
        })
        .collect();

    // Write payloads into a heap-backed CDBG ring and drain them back out.
    let region_len = usize::max(
        DESCRIPTOR_SIZE + packet_payloads.iter().map(|p| p.len() + 4).sum::<usize>(),
        512,
    );
    let mut region = vec![0u8; region_len].into_boxed_slice();
    let base = region.as_mut_ptr();

    // SAFETY: region is heap-allocated; exclusively owned; valid for region_len bytes.
    unsafe {
        Descriptor::new(region_len as u16).write_to_raw_ptr(base);
    }
    for payload in &packet_payloads {
        // SAFETY: descriptor was initialized above; base valid for region_len bytes.
        unsafe {
            ConsortiumLogger::write_shared_memory(base, payload);
        }
    }

    // SAFETY: region is exclusively owned; base valid for region_len bytes.
    let mut ring = unsafe { HostRing::from_raw_parts(base, region_len).unwrap() };
    let drained = ring.drain_packets().unwrap();

    let mut decoded_frames: Vec<String> = Vec::new();
    for packet in &drained {
        session
            .push_packet(packet, |frame| {
                decoded_frames.push(frame.display(true).to_string());
            })
            .expect("defmt decoding failed");
    }

    assert!(
        !decoded_frames.is_empty(),
        "no defmt frames decoded from {len} packet(s)",
        len = drained.len()
    );

    if let Some(substr) = &expect_substr {
        let found = decoded_frames.iter().any(|s| s.contains(substr.as_str()));
        assert!(
            found,
            "expected substring {substr:?} not found in decoded frames:\n{frames}",
            frames = decoded_frames.join("\n")
        );
    }
}

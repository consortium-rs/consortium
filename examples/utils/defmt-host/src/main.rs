// Copyright 2026 Ethan Wu
// SPDX-License-Identifier: Apache-2.0

//! One-shot host reader for a consortium CDBG defmt ring exposed through UIO.

use std::{env, error::Error, fs, io, path::PathBuf};

use consortium_dbg::host::{DefmtContext, DefmtSession, HostRing};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let (Some(elf), device, None) = (args.next(), args.next(), args.next()) else {
        return Err(usage(&program).into());
    };
    let device = device.map_or_else(|| "dbg-shm".to_string(), |d| d.to_string_lossy().into_owned());
    let uio_id = resolve(&device)?;

    // The `_original.elf` carries the `.defmt` table the frames index into.
    let elf = fs::read(PathBuf::from(elf))?;
    let context = DefmtContext::from_elf_bytes(&elf)?;

    // Do not listen for UIO interrupts or poll: consume only what is available now.
    let device = uio::UioDevice::blocking_new(uio_id)?;
    let base = device.map_mapping(0)?;
    let len = device.map_size(0)?;

    // SAFETY: `device` owns map 0 for the lifetime of `ring`; this process is
    // the sole host reader and therefore the sole writer of CDBG `read_ptr`.
    let mut ring = unsafe { HostRing::from_raw_parts(base.cast(), len)? };

    // Descriptor state before draining. `flags` is the key signal: `Overflow`
    // (a dropped frame) or a wrap punches a hole in the single rzcobs stream and
    // everything past it decodes as `Malformed` until the decoder resyncs.
    let snap = ring.snapshot();
    let pending = snap.write_ptr.wrapping_sub(snap.read_ptr);
    eprintln!(
        "CDBG uio{uio_id}: mapped_len={:#x} capacity={:#x} read_ptr={:#x} write_ptr={:#x} pending~={:#x} flags={:?}",
        ring.mapped_len(),
        snap.capacity,
        snap.read_ptr,
        snap.write_ptr,
        pending,
        snap.flags,
    );

    let packets = ring.drain_packets()?;
    eprintln!(
        "drained {} CDBG packet(s), {} payload byte(s) total",
        packets.len(),
        packets.iter().map(Vec::len).sum::<usize>(),
    );

    let mut session = DefmtSession::new(&context);
    let mut decoded = 0usize;
    let mut malformed = 0usize;

    for (index, packet) in packets.iter().enumerate() {
        eprintln!("packet {index}: {} byte(s) [{}]", packet.len(), hex(packet));
        if let Err(error) = session.push_packet(packet, |frame| {
            decoded += 1;
            println!("{}", frame.display_message());
        }) {
            malformed += 1;
            eprintln!("  warning: decode error at packet {index}: {error}");
        }
    }

    eprintln!(
        "drained {} CDBG packet(s): decoded {decoded} defmt frame(s), {malformed} decode error(s)",
        packets.len()
    );

    Ok(())
}

/// Renders `bytes` as space-separated lowercase hex for diagnostic dumps.
fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Resolves a `uioN` number from `device`: a bare number is taken as is,
/// otherwise the device whose sysfs name is `device` or `device@<unit-address>`.
fn resolve(device: &str) -> Result<usize, Box<dyn Error>> {
    if let Ok(id) = device.parse() {
        return Ok(id);
    }
    let mut found = Vec::new();
    for entry in fs::read_dir("/sys/class/uio")? {
        let entry = entry?;
        let Some(id) = entry
            .file_name()
            .to_str()
            .and_then(|file| file.strip_prefix("uio")?.parse::<usize>().ok())
        else {
            continue;
        };
        let name = fs::read_to_string(entry.path().join("name"))?;
        let name = name.trim();
        let exact = name == device;
        if exact || name.split_once('@').is_some_and(|(node, _)| node == device) {
            found.push((!exact, id));
        }
    }
    found.sort();
    found
        .first()
        .map(|(_, id)| *id)
        .ok_or_else(|| format!("no UIO device named {device:?} under /sys/class/uio").into())
}

fn usage(program: &std::ffi::OsStr) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "usage: {} <FIRMWARE_ORIGINAL_ELF> [UIO_NAME_OR_ID (default: dbg-shm)]",
            program.to_string_lossy()
        ),
    )
}

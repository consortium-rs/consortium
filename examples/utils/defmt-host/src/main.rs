// Copyright 2026 Ethan Wu
// SPDX-License-Identifier: Apache-2.0

//! One-shot host reader for a consortium CDBG defmt ring exposed through UIO.

use std::{env, error::Error, io};

use consortium_dbg::host::{DefmtContext, DefmtSession, HostRing};

#[repr(align(16))]
struct Aligned<const N: usize>([u8; N]);

static FIRMWARE_ELF: Aligned<
    {
        include_bytes!(
            "../../../melt-pot/stm32mp25/mcu/target/thumbv8m.main-none-eabihf/release/melt-pot-stm32mp25-mcu_original.elf"
        )
        .len()
    },
> = Aligned(*include_bytes!(
    "../../../melt-pot/stm32mp25/mcu/target/thumbv8m.main-none-eabihf/release/melt-pot-stm32mp25-mcu_original.elf"
));

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let uio_id = args
        .next()
        .map(|value| value.to_string_lossy().parse::<usize>())
        .transpose()?
        .unwrap_or(0);
    if args.next().is_some() {
        return Err(usage(&program).into());
    }

    let context = DefmtContext::from_elf_bytes(&FIRMWARE_ELF.0)?;

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

fn usage(program: &std::ffi::OsStr) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("usage: {} [UIO_DEVICE_ID]", program.to_string_lossy()),
    )
}

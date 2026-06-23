// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

#![cfg(target_os = "linux")]

use std::sync::{Arc, Mutex};

use consortium_codec::PostcardCodec;
use consortium_dbg_defmt::config as dbg_config;
use consortium_dbg_defmt::host::HostRing;
use consortium_hmi::{Bridge, KioskApp};
use consortium_ipc::{Chan, Channel, Rx, SramRegion};
use consortium_ipc_doorbell_hsem::HsemDoorbell;
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, SharedMemoryTransport, config as ipc_config, descriptor_init,
    slot::SLOT_HEADER,
};
use melt_pot_shared::SensorReading;

// ── Shared-memory layout (CA35 address space) ────────────────────────────────
//
// The CA35 reaches the *same* shared descriptor + slots the CM33 uses, but the
// address is dynamic: map UIO device `ipc_config::UIO` (the `[ipc]` `uio` index
// from stm32mp25/Consortium.toml) and pass its mmap'd virtual base below. The
// MTU comes from the same `[ipc]` section via `consortium_ipc_transport_memory::config`.
//
// For local testing without hardware, this example substitutes a heap allocation.

const DESC_SIZE: usize = 0x300;
const MTU: usize = ipc_config::MTU as usize;
const SLOT_SIZE: usize = SLOT_HEADER + MTU;

const M_TO_A_SLOT_OFFSET: usize = DESC_SIZE * 2;
const IPC_REGION_LEN: usize = M_TO_A_SLOT_OFFSET + SLOT_SIZE;

// HSEM semaphore 0: CM33 rings → CA35 IRQ.  The virtual address of the HSEM
// peripheral after UIO / device-tree mapping.
// PLATFORM: replace with the mapped virtual address.
const HSEM_VIRT_BASE: usize = 0; // placeholder — fill in after mmap
const IPC_CHAN: u8 = 0;

// ── CDBG debug ring (CM33 defmt logs) ────────────────────────────────────────
//
// The CM33 firmware writes defmt frames into this region; the CA35 reads them
// via HostRing.  Map UIO device `dbg_config::UIO` (the `[dbg]` `uio` index) and
// call HostRing::from_raw_parts(ptr, DBG_SRAM_LEN). The length comes from the
// `[dbg]` section via `consortium_dbg_defmt::config`.
// PLATFORM: replace DBG_SRAM_VIRT_BASE with the virtual address after mmap.
const DBG_SRAM_VIRT_BASE: usize = 0; // placeholder — fill in after mmap
const DBG_SRAM_LEN: usize = dbg_config::LENGTH as usize;

// ── OP-TEE Trusted Application UUID ──────────────────────────────────────────
//
// `consortium-builder` injects the `[tee] uuid` from Consortium.toml as the
// `CONSORTIUM_TEE_UUID` compile-time environment variable (the same value the
// TA's build script consumes), so the CA and TA always agree. A plain
// `cargo build` falls back to the placeholder UUID in Consortium.toml.
const TA_UUID: &str = match option_env!("CONSORTIUM_TEE_UUID") {
    Some(uuid) => uuid,
    None => "12345678-1234-1234-1234-123456789012",
};

// ── Status shared between the IPC receive task and the HMI bridge ────────────

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Status {
    temperature_mdc: i32,
    seq: u32,
    within_limits: bool,
}

#[tokio::main]
async fn main() {
    // ── IPC shared memory ─────────────────────────────────────────────────────
    //
    // On real hardware, map the IPC region via UIO device `ipc_config::UIO`.
    // For demonstration, allocate on the heap so the example compiles and runs
    // on a Linux development host without STM32MP25 hardware.
    let mut backing = vec![0u8; IPC_REGION_LEN];
    let app_desc_base = backing.as_mut_ptr();

    // App descriptor: one RX channel (M→A slot) labeled MToA.
    // The CM33's descriptor labels the same physical slot AToM (its TX).
    let m_to_a_chan = {
        let mut ch = ChannelLayout::default();
        ch.offset = M_TO_A_SLOT_OFFSET as u32;
        ch.size = SLOT_SIZE as u32;
        ch.direction = ChannelDirection::MToA;
        ch
    };

    // SAFETY: app_desc_base covers IPC_REGION_LEN bytes; no aliased Rust reference exists.
    unsafe { descriptor_init(app_desc_base, 1, MTU, &[m_to_a_chan]) };

    // SAFETY: IPC_CHAN is the single descriptor channel initialized above.
    let ch = unsafe { Chan::new_unchecked(IPC_CHAN) };
    // SAFETY: app_desc_base points to the heap-backed IPC region of IPC_REGION_LEN bytes.
    let region = unsafe { SramRegion::new(app_desc_base, IPC_REGION_LEN) };
    // PROC=1: CA35 interrupt controller view (RM0457 §22, processor IDs 1–3).
    let doorbell = HsemDoorbell::<1>::new(HSEM_VIRT_BASE);
    // SAFETY: app_desc_base satisfies the descriptor layout invariant above.
    let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, app_desc_base) };
    let buf: &'static mut [u8] = Box::leak(Box::new([0u8; MTU]));
    let mut ipc_rx = Channel::<Rx, SensorReading, _, PostcardCodec>::new(ch, transport, buf);

    // ── CDBG debug ring ───────────────────────────────────────────────────────
    //
    // On real hardware, map 0x0A02_1000 via UIO and pass the virtual pointer.
    // DBG_SRAM_VIRT_BASE is 0 in the demo, so the ring is disabled.
    let dbg_ring: Option<HostRing> = if DBG_SRAM_VIRT_BASE != 0 {
        let ptr = DBG_SRAM_VIRT_BASE as *mut u8;
        // SAFETY: ptr is a valid mapped region of DBG_SRAM_LEN bytes.
        unsafe { HostRing::from_raw_parts(ptr, DBG_SRAM_LEN).ok() }
    } else {
        None
    };

    // ── Shared state: latest status ───────────────────────────────────────────
    let state: Arc<Mutex<Option<Status>>> = Arc::new(Mutex::new(None));

    // ── IPC receive loop runs on the tokio runtime started by `#[tokio::main]` ──
    let handle = tokio::runtime::Handle::current();

    let state_ipc = Arc::clone(&state);
    tokio::spawn(async move {
        let mut dbg = dbg_ring;
        loop {
            match ipc_rx.recv().await {
                Ok(msg) => {
                    let reading = msg.into_inner();
                    let within_limits = tee_check(reading.temperature_mdc);
                    *state_ipc.lock().unwrap() = Some(Status {
                        temperature_mdc: reading.temperature_mdc,
                        seq: reading.seq,
                        within_limits,
                    });
                }
                Err(e) => eprintln!("ipc recv error: {e}"),
            }
            // Drain any defmt log packets the CM33 wrote since the last receive.
            if let Some(ref mut ring) = dbg
                && let Ok(packets) = ring.drain_packets()
            {
                for pkt in packets {
                    eprintln!("[defmt] {} raw bytes", pkt.len());
                }
            }
        }
    });

    // ── HMI bridge ───────────────────────────────────────────────────────────
    let mut bridge = Bridge::with_runtime(handle);

    let state_hmi = Arc::clone(&state);
    bridge.handle("get_status", move |_payload| {
        match state_hmi.lock().unwrap().clone() {
            Some(s) => serde_json::to_string(&s).unwrap_or_else(|_| "null".into()),
            None => "null".into(),
        }
    });

    let app = KioskApp::new("melt-pot-stm32mp25", None, None).expect("KioskApp init");
    app.attach_bridge(bridge).expect("bridge attach");
    app.load().expect("HMI load");

    // Run the GLib main loop on this thread (required by WebKit/Cog).
    // The tokio worker threads continue running the IPC receive loop concurrently.
    glib::MainLoop::new(None, false).run();
}

/// Software safe range, mirroring the TA's `TEMP_LIMIT_LO..=TEMP_LIMIT_HI`.
/// Used as the fallback verdict when OP-TEE is not reachable.
const SOFT_TEMP_RANGE: std::ops::RangeInclusive<i32> = -20_000..=85_000;

/// Call the TEE to check whether `temperature_mdc` is within limits.
///
/// Attempts the real OP-TEE path first: open a session to the melt-pot TA and
/// invoke its `check_temperature` command (the `ca` feature on the TA crate
/// makes [`call_check_temperature`](melt_pot_stm32mp25_ta::call_check_temperature)
/// available). If OP-TEE is not reachable — no `/dev/tee*`, `tee-supplicant`
/// down, or the TA not installed, as on a plain Linux development host — it
/// falls back to the same software range check so the demo still runs.
fn tee_check(temperature_mdc: i32) -> bool {
    match tee_check_optee(temperature_mdc) {
        Ok(within_limits) => within_limits,
        Err(err) => {
            eprintln!("melt-pot: OP-TEE check unavailable ({err}); using software fallback");
            SOFT_TEMP_RANGE.contains(&temperature_mdc)
        }
    }
}

/// Real OP-TEE invocation, mirroring the Teaclave CA pattern:
/// `Context::new` → `open_session(uuid)` → invoke the command.
fn tee_check_optee(temperature_mdc: i32) -> consortium_tee::TeeResult<bool> {
    use consortium_tee::TeeError;
    use optee_teec::{Context, Uuid};

    let uuid = Uuid::parse_str(TA_UUID).map_err(TeeError::Session)?;
    let mut ctx = Context::new().map_err(TeeError::Context)?;
    let mut session = ctx.open_session(uuid).map_err(TeeError::Session)?;
    melt_pot_stm32mp25_ta::call_check_temperature(&mut session, temperature_mdc)
}

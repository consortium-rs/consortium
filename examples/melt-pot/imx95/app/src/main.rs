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
use consortium_ipc_doorbell_mu::{IMX95_CA55_CM7_RX_CHAN, MuDoorbell};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, SharedMemoryTransport, config as ipc_config, descriptor_init,
    slot::SLOT_HEADER,
};
use melt_pot_shared::SensorReading;

// ── Shared-memory layout (CA55 address space) ────────────────────────────────
//
// The CA55 reaches the *same* shared descriptor + slots the CM7 uses, but the
// address is dynamic: map UIO device `ipc_config::UIO` (the `[ipc]` `uio` index
// from imx95/Consortium.toml) and pass its mmap'd virtual base below. The MTU
// comes from the same `[ipc]` section via `consortium_ipc_transport_memory::config`.
//
// For local testing without hardware, this example substitutes a heap allocation.

const DESC_SIZE: usize = 0x300;
const MTU: usize = ipc_config::MTU as usize;
const SLOT_SIZE: usize = SLOT_HEADER + MTU;

const M_TO_A_SLOT_OFFSET: usize = DESC_SIZE * 2;
const IPC_REGION_LEN: usize = M_TO_A_SLOT_OFFSET + SLOT_SIZE;

// MU base addresses for i.MX95 (8 instances, MU1 through MU8).
// PLATFORM: replace with the virtual addresses after mmap / UIO mapping.
const MU_VIRT_BASES: [usize; 8] = [0usize; 8]; // placeholder — fill after mmap

// ── CDBG debug ring (CM7 defmt logs) ─────────────────────────────────────────
//
// The CM7 firmware writes defmt frames into this region; the CA55 reads them
// via HostRing.  Map UIO device `dbg_config::UIO` (the `[dbg]` `uio` index) and
// call HostRing::from_raw_parts(ptr, DBG_SRAM_LEN). The length comes from the
// `[dbg]` section via `consortium_dbg_defmt::config`.
// PLATFORM: replace DBG_SRAM_VIRT_BASE with the virtual address after mmap.
const DBG_SRAM_VIRT_BASE: usize = 0; // placeholder — fill in after mmap
const DBG_SRAM_LEN: usize = dbg_config::LENGTH as usize;

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
    // On real hardware, map the OCRAM region via UIO device `ipc_config::UIO`.
    // For demonstration, allocate on the heap so the example compiles and runs
    // on a Linux development host without i.MX95 hardware.
    let mut backing = vec![0u8; IPC_REGION_LEN];
    let app_desc_base = backing.as_mut_ptr();

    // App descriptor: one RX channel (M→A slot) labeled MToA.
    // The CM7's descriptor labels the same physical slot AToM (its TX).
    let m_to_a_chan = {
        let mut ch = ChannelLayout::default();
        ch.offset = M_TO_A_SLOT_OFFSET as u32;
        ch.size = SLOT_SIZE as u32;
        ch.direction = ChannelDirection::MToA;
        ch
    };

    // SAFETY: app_desc_base covers IPC_REGION_LEN bytes; no aliased Rust reference exists.
    unsafe { descriptor_init(app_desc_base, 1, MTU, &[m_to_a_chan]) };

    // IMX95_CA55_CM7_RX_CHAN = 25 = MU7 GI1.
    // CA55 waits on this channel; CM7 rings it to wake the CA55.
    // SAFETY: IMX95_CA55_CM7_RX_CHAN is a valid MU channel index.
    let ch = unsafe { Chan::new_unchecked(IMX95_CA55_CM7_RX_CHAN) };
    // SAFETY: app_desc_base points to the heap-backed IPC region of IPC_REGION_LEN bytes.
    let region = unsafe { SramRegion::new(app_desc_base, IPC_REGION_LEN) };
    let doorbell = MuDoorbell::new(MU_VIRT_BASES);
    // SAFETY: app_desc_base satisfies the descriptor layout invariant above.
    let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, app_desc_base) };
    let buf: &'static mut [u8] = Box::leak(Box::new([0u8; MTU]));
    let mut ipc_rx = Channel::<Rx, SensorReading, _, PostcardCodec>::new(ch, transport, buf);

    // ── CDBG debug ring ───────────────────────────────────────────────────────
    //
    // On real hardware, map 0x204C_1000 via UIO and pass the virtual pointer.
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
            // Drain any defmt log packets the CM7 wrote since the last receive.
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

    let app = KioskApp::new("melt-pot-imx95", None, None).expect("KioskApp init");
    app.attach_bridge(bridge).expect("bridge attach");
    app.load().expect("HMI load");

    // Run the GLib main loop on this thread (required by WebKit/Cog).
    // The tokio worker threads continue running the IPC receive loop concurrently.
    glib::MainLoop::new(None, false).run();
}

/// Call the TEE to check whether `temperature_mdc` is within limits.
///
/// On real hardware with OP-TEE, initialise an `optee_teec::Context` and call
/// `melt_pot_imx95_ta::call_check_temperature(&mut ctx, temperature_mdc)`.
/// The `ca` feature on the TA crate makes that function available.
///
/// This stub performs a software-only check so the demo runs without OP-TEE.
fn tee_check(temperature_mdc: i32) -> bool {
    // REPLACE with:
    //   let mut ctx = optee_teec::Context::new().expect("TEE context");
    //   melt_pot_imx95_ta::call_check_temperature(&mut ctx, temperature_mdc)
    //       .unwrap_or(false)
    (-20_000..=85_000).contains(&temperature_mdc)
}

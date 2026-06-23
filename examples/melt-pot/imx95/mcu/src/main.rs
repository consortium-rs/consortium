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

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use panic_halt as _;

use consortium_codec::PostcardCodec;
// Depending on `consortium-dbg-defmt` with the `logger` feature registers its
// `#[global_logger]` (`ConsortiumLogger`) as defmt's sink, so the `defmt::info!`
// calls below land in the CDBG ring described by `[dbg]`. No import is needed —
// defmt resolves the global logger through the linker.
use consortium_ipc::{Chan, Channel, SramRegion, Tx};
use consortium_ipc_doorbell_mu::{IMX95_CA55_CM7_RX_CHAN, MuDoorbell};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, SharedMemoryTransport, config as ipc_config, descriptor_init,
    slot::SLOT_HEADER,
};
use melt_pot_shared::SensorReading;

// ── Shared-memory layout (CM7 address space) ────────────────────────────────
//
// The base/length/MTU come from the `[ipc]` section of imx95/Consortium.toml,
// injected at compile time by consortium-builder as `CONSORTIUM_IPC_*` and read
// here through `consortium_ipc_transport_memory::config`. The CM7 reaches the
// shared descriptor + slots through this raw physical `BASE`; the CA55 maps the
// *same* OCRAM through the region's UIO device.
const IPC_SRAM_BASE: usize = ipc_config::BASE as usize;

const DESC_SIZE: usize = 0x300;
const MTU: usize = ipc_config::MTU as usize;
const SLOT_SIZE: usize = SLOT_HEADER + MTU;

const MCU_DESC_OFFSET: usize = DESC_SIZE;
const M_TO_A_SLOT_PHYS: usize = DESC_SIZE * 2;
const IPC_REGION_LEN: usize = M_TO_A_SLOT_PHYS + SLOT_SIZE;

// M→A slot offset relative to the MCU descriptor base.
const M_TO_A_CHAN_OFFSET: usize = M_TO_A_SLOT_PHYS - MCU_DESC_OFFSET;

// ── MU7 doorbell ─────────────────────────────────────────────────────────────
//
// MU7 GI0 (channel 24) is used for CM7 → CA55 signalling.
// `IMX95_CA55_CM7_RX_CHAN` is channel 24 from the CA55's perspective (its RX),
// which is the channel CM7 rings to wake the CA55.
//
// MU base addresses for i.MX95 (8 instances, MU1 through MU8).
// PLATFORM: replace with the actual mapped bases for your boot configuration.
const MU_BASES: [usize; 8] = [
    0x4442_4000, // MU1
    0x4442_8000, // MU2
    0x4442_C000, // MU3
    0x4443_0000, // MU4
    0x4443_4000, // MU5
    0x4443_8000, // MU6
    0x4443_C000, // MU7  ← CA55 <-> CM7
    0x4444_0000, // MU8
];

// ── CDBG debug ring ───────────────────────────────────────────────────────────
//
// The `ConsortiumLogger` global_logger locates the CDBG ring from the `[dbg]`
// section of Consortium.toml, injected by consortium-builder as `CONSORTIUM_DBG_*`
// and read through `consortium_dbg_defmt::config` — no `#[no_mangle]` statics or
// linker symbols are required. The CA55 maps the same region via the [dbg] UIO.

// Scratch buffer for the postcard encoder — lives in .bss, never on the stack.
static mut TX_BUF: [u8; MTU] = [0u8; MTU];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // SAFETY: IPC_SRAM_BASE is the CM7 address of the DMA-coherent shared OCRAM;
    // no other Rust reference aliases this memory.
    let ipc_base = IPC_SRAM_BASE as *mut u8;
    let mcu_desc_base = unsafe { ipc_base.add(MCU_DESC_OFFSET) };

    // MCU descriptor: one TX channel (M→A slot), labeled AToM so that
    // `send_bytes` finds it via `channel.direction.is_tx()`.
    // The CA55's descriptor labels the same physical slot MToA (RX for app).
    let m_to_a_chan = {
        let mut ch = ChannelLayout::default();
        ch.offset = M_TO_A_CHAN_OFFSET as u32;
        ch.size = SLOT_SIZE as u32;
        ch.direction = ChannelDirection::AToM;
        ch
    };

    // SAFETY: mcu_desc_base covers at least DESC_SIZE bytes of DMA-coherent OCRAM.
    unsafe { descriptor_init(mcu_desc_base, 1, MTU, &[m_to_a_chan]) };

    // IMX95_CA55_CM7_RX_CHAN = 25 = MU7 GI1.  CM7 rings this channel; the CA55
    // is woken via the MU7 GI1 interrupt and reads the data slot.
    // SAFETY: IMX95_CA55_CM7_RX_CHAN is a valid MU channel index (25 < 32).
    let ch = unsafe { Chan::new_unchecked(IMX95_CA55_CM7_RX_CHAN) };
    // SAFETY: ipc_base covers IPC_REGION_LEN bytes; no aliased Rust reference exists.
    let region = unsafe { SramRegion::new(ipc_base, IPC_REGION_LEN) };
    let doorbell = MuDoorbell::new(MU_BASES);
    // SAFETY: mcu_desc_base satisfies the descriptor layout invariant above.
    let transport = unsafe { SharedMemoryTransport::new(ch, doorbell, region, mcu_desc_base) };
    // SAFETY: TX_BUF is only accessed here; this function runs on the single executor thread,
    // so this is the sole live reference to the static.
    let buf: &'static mut [u8] = unsafe { &mut *(&raw mut TX_BUF) };
    let mut channel = Channel::<Tx, SensorReading, _, PostcardCodec>::new(ch, transport, buf);

    let mut seq: u32 = 0;
    loop {
        let reading = SensorReading {
            temperature_mdc: simulated_temperature(seq),
            seq,
        };

        // Emit a defmt frame into the CDBG ring (read by the CA55 via HostRing).
        defmt::info!(
            "sensor seq={=u32} temp_mdc={=i32}",
            reading.seq,
            reading.temperature_mdc
        );

        // Ignore send errors: if the CA55 hasn't consumed the previous slot yet,
        // we skip this sample rather than blocking.
        let _ = channel.send(&reading).await;

        seq = seq.wrapping_add(1);

        // Delay ~1 s at 800 MHz.  In production firmware, replace with an
        // embassy-time `Timer` backed by a properly initialised time driver.
        cortex_m::asm::delay(800_000_000);
    }
}

fn simulated_temperature(seq: u32) -> i32 {
    // Sine-like walk between 20 °C and 30 °C for demo purposes.
    25_000 + ((seq % 20) as i32 - 10) * 500
}

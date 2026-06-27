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
use consortium_ipc_doorbell_hsem::{HsemDoorbell, Secondary};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, SharedMemoryTransport, config as ipc_config, descriptor_init,
    slot::SLOT_HEADER,
};
use melt_pot_shared::SensorReading;

// ── Shared-memory layout (CM33 address space) ───────────────────────────────
//
// The base/length/MTU come from the `[ipc]` section of stm32mp25/Consortium.toml,
// injected at compile time by consortium-builder as `CONSORTIUM_IPC_*` and read
// here through `consortium_ipc_transport_memory::config`. The CM33 reaches the
// shared descriptor + slots through this raw physical `BASE`; the CA35 maps the
// *same* memory through the region's UIO device.
const IPC_SRAM_BASE: usize = ipc_config::BASE as usize;

const DESC_SIZE: usize = 0x300;
const MTU: usize = ipc_config::MTU as usize;
const SLOT_SIZE: usize = SLOT_HEADER + MTU;

const MCU_DESC_OFFSET: usize = DESC_SIZE;
const M_TO_A_SLOT_PHYS: usize = DESC_SIZE * 2;
const IPC_REGION_LEN: usize = M_TO_A_SLOT_PHYS + SLOT_SIZE;

// M→A slot offset relative to the MCU descriptor base.
const M_TO_A_CHAN_OFFSET: usize = M_TO_A_SLOT_PHYS - MCU_DESC_OFFSET;

// ── Doorbell ─────────────────────────────────────────────────────────────────
//
// HSEM semaphore 0: CM33 rings → HSEM interrupt on CA35. On the firmware side the
// HSEM peripheral base is a compile-time constant from the doorbell crate's
// `stm32mp25` feature, so `HsemDoorbell::steal()` takes no address argument.
// Secondary = core C2: the CM33 notification view (RM0457 processor IDs 1–3;
// CA35 = C1/Primary).
const IPC_CHAN: u8 = 0; // doorbell channel = HSEM semaphore index

// ── CDBG debug ring ───────────────────────────────────────────────────────────
//
// The `ConsortiumLogger` global_logger locates the CDBG ring from the `[dbg]`
// section of Consortium.toml, injected by consortium-builder as `CONSORTIUM_DBG_*`
// and read through `consortium_dbg_defmt::config` — no `#[no_mangle]` statics or
// linker symbols are required. The CA35 maps the same region via the [dbg] UIO.

// Scratch buffer for the postcard encoder — lives in .bss, never on the stack.
static mut TX_BUF: [u8; MTU] = [0u8; MTU];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // SAFETY: IPC_SRAM_BASE is the CM33 address of the DMA-coherent shared SRAM;
    // no other Rust reference aliases this memory.
    let ipc_base = IPC_SRAM_BASE as *mut u8;
    let mcu_desc_base = unsafe { ipc_base.add(MCU_DESC_OFFSET) };

    // MCU descriptor: one TX channel (M→A slot), labeled PrimaryToSecondary so that
    // `send_bytes` finds it via `channel.direction.is_tx()`.
    // The A-core's descriptor labels the same physical slot SecondaryToPrimary (RX for app).
    let m_to_a_chan = {
        let mut ch = ChannelLayout::default();
        ch.offset = M_TO_A_CHAN_OFFSET as u32;
        ch.size = SLOT_SIZE as u32;
        ch.direction = ChannelDirection::PrimaryToSecondary;
        ch
    };

    // SAFETY: mcu_desc_base covers at least DESC_SIZE bytes of DMA-coherent SRAM.
    unsafe { descriptor_init(mcu_desc_base, MTU, &[m_to_a_chan]) };

    let ch = unsafe { Chan::new_unchecked(IPC_CHAN) };
    // SAFETY: ipc_base covers IPC_REGION_LEN bytes; no aliased Rust reference exists.
    let region = unsafe { SramRegion::new(ipc_base, IPC_REGION_LEN) };
    // SAFETY: this is the sole handle to the HSEM peripheral for the CM33 (C2).
    let doorbell = unsafe { HsemDoorbell::<Secondary, 1>::steal() };
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

        // Emit a defmt frame into the CDBG ring (read by the CA35 via HostRing).
        defmt::info!(
            "sensor seq={=u32} temp_mdc={=i32}",
            reading.seq,
            reading.temperature_mdc
        );

        // Ignore send errors: if the A-core hasn't consumed the previous slot yet,
        // we skip this sample rather than blocking.
        let _ = channel.send(&reading).await;

        seq = seq.wrapping_add(1);

        // Delay ~1 s at 400 MHz.  In production firmware, replace with an
        // embassy-time `Timer` backed by a properly initialised time driver.
        cortex_m::asm::delay(400_000_000);
    }
}

fn simulated_temperature(seq: u32) -> i32 {
    // Sine-like walk between 20 °C and 30 °C for demo purposes.
    25_000 + ((seq % 20) as i32 - 10) * 500
}

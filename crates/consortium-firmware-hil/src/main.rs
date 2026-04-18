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

use consortium_ipc::{regions::SramRegion, Chan, Transport};
use consortium_ipc_transport_mem::MemTransport;
use consortium_runtime_mcu::{
    descriptor::{self, ChanDesc, DescState},
    hsem::HsemDoorbell,
    slot_pair_ptrs,
};
use cortex_m::asm;
use embassy_executor::Spawner;

// Shared SRAM IPC region — SRAM1 on STM32MP25 (from memory.x SHARED origin)
const SHARED_BASE: usize = 0x20060000;
const MTU: usize = 256;
const SLOT_SIZE: usize = MTU + 8; // 8-byte header

// Descriptor header + 1-channel table = 0x20 (desc header) + 16 bytes = 48 bytes → round to 64
const SLOTS_OFFSET: usize = 64;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let base = SHARED_BASE as *mut u8;

    // 1. Write descriptor to shared SRAM
    // Note: tx_offset/rx_offset in ChanDesc are from A-core perspective,
    // relative to region base (SHARED_BASE). SLOTS_OFFSET skips the descriptor header.
    //   A-core tx (A→M) = M-core rx_ptr = SLOTS_OFFSET + SLOT_SIZE
    //   A-core rx (M→A) = M-core tx_ptr = SLOTS_OFFSET + 0
    let chan_desc = ChanDesc {
        tx_offset: (SLOTS_OFFSET + SLOT_SIZE) as u32, // A→M slot
        tx_size: SLOT_SIZE as u32,
        rx_offset: SLOTS_OFFSET as u32, // M→A slot
        rx_size: SLOT_SIZE as u32,
    };

    unsafe {
        descriptor::write_descriptor(base, 1, MTU as u32, MTU as u32, &[chan_desc]);
        descriptor::set_state(base, DescState::MReady);
    }

    // 2. Wait for kernel to transition descriptor to Ready
    loop {
        let state = unsafe { descriptor::get_state(base) };
        if state == DescState::Ready {
            break;
        }
        asm::nop();
    }

    // 3. Enable HSEM interrupt
    unsafe { consortium_runtime_mcu::irq::enable_hsem_irq() }

    // 4. Build IPC transport
    let (tx_ptr, rx_ptr) = unsafe { slot_pair_ptrs(base.add(SLOTS_OFFSET), 0, SLOT_SIZE) };
    // tx_ptr = M→A (offset SLOTS_OFFSET + 0), rx_ptr = A→M (offset SLOTS_OFFSET + SLOT_SIZE)
    let region = unsafe { SramRegion::new(base.add(SLOTS_OFFSET), 2 * SLOT_SIZE) };
    let doorbell: HsemDoorbell<1> = HsemDoorbell::new();
    let chan = unsafe { Chan::new_unchecked(0) };
    let mut transport = unsafe { MemTransport::new(chan, doorbell, region, tx_ptr, rx_ptr, MTU) };

    // 5. Echo loop: recv u32, send back (val + 1)
    let mut buf = [0u8; MTU];
    loop {
        match transport.recv(chan, &mut buf).await {
            Ok(n) => {
                // Decode message
                let msg: u32 = match postcard::from_bytes(&buf[..n]) {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                // Prepare reply (msg + 1)
                let reply = msg.wrapping_add(1);
                let encoded = match postcard::to_slice(&reply, &mut buf) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                // Send reply
                let _ = transport.send(chan, encoded).await;
            }
            Err(_) => {
                // Recv error, continue
                continue;
            }
        }
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        asm::bkpt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        #[cfg(target_os = "none")]
        asm::bkpt();
    }
}

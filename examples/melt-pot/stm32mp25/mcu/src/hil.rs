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

//! HIL stage narration, CM33 side (STM32MP257F-DK).
//!
//! Every frame starts with `HIL M<n>`. The CA35 decodes this ring and prints
//! these frames beside its own `HIL A<n>` lines, so one console shows both
//! cores' view of each bring-up stage:
//!
//! | Stage | Meaning                                                            |
//! | ----- | ------------------------------------------------------------------ |
//! | M1    | firmware launched; the defmt ring is up (first frame after `init`) |
//! | M2    | interrupt routing: the live vector table reaches the real handlers |
//! | M3    | parked in `Connect` with the doorbell line armed                   |
//! | M4    | the AP published the descriptor (connection requested)             |
//! | M5    | the AP's doorbell was observed asserted (best effort: µs window)    |
//! | M6    | connected: the doorbell IRQ was serviced and the AP was acked      |
//! | M7    | first message sent                                                 |
//! | M8    | the AP read the first message (slot consumed)                      |
//! | M9    | steady-state summary every 10 s                                    |
//!
//! Stalls are reported with the registers that explain them. The addresses
//! mirror `../Consortium.bringup.toml` (IPC window, IPCC1 channels 0/1) and the
//! interrupts come from the chip PAC.

use core::ptr::read_volatile;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use consortium_hal_stm32mp2::Interrupt;
use consortium_hal_stm32mp2::interrupts::__INTERRUPTS;
use consortium_ipc_transport_memory::bell::ipcc::registers::{base, offsets};
use cortex_m::peripheral::NVIC;
use embassy_time::{Duration, Instant, Timer};

/// `[[ipc.channel]] sensor` `config.memory.base`.
const IPC_BASE: usize = 0x2006_4000;
/// Second slot (`SecondaryToPrimary`): `align64(sizeof Descriptor)` + one
/// `align64(12 + mtu)` stride.
const SLOT_TX_FLAGS: usize = IPC_BASE + 0x1c0 + 4;
const CIPC_MAGIC: u32 = u32::from_be_bytes(*b"CIPC");
const FLAG_VALID: u32 = 1 << 0;
const FLAG_CONSUMED: u32 = 1 << 1;

/// IPCC1; the CM33 is processor 2.
const IPCC: usize = base::BASE[0];
/// `party.linux.chan`: the AP rings it, the CM33 waits on it.
const RX_CHAN: u32 = 0;
/// `party.cm33.chan`: the CM33 rings it.
const TX_CHAN: u32 = 1;
const DOORBELL: Interrupt = Interrupt::IPCC1_RX;
const TIMER: Interrupt = Interrupt::TIM2;

const SCB_ICSR: usize = 0xE000_ED04;
const SCB_VTOR: usize = 0xE000_ED08;

static CONNECTED: AtomicBool = AtomicBool::new(false);
static PUBLISHED_SEEN: AtomicBool = AtomicBool::new(false);
static DOORBELL_SEEN: AtomicBool = AtomicBool::new(false);
static LAST_SEQ: AtomicU32 = AtomicU32::new(0);
static SENT: AtomicU32 = AtomicU32::new(0);
static SEND_ERRORS: AtomicU32 = AtomicU32::new(0);

fn rd(addr: usize) -> u32 {
    // SAFETY: every caller passes an always-readable 32-bit location: a System
    // Control Space register, an IPCC1 status/control register, or a word of the
    // uncached IPC window this core's MPU carveout covers. Reads have no side
    // effects on any of them.
    unsafe { read_volatile(addr as *const u32) }
}

/// The vector the core will actually fetch for `irq`, next to the one the
/// linker put in `__INTERRUPTS`.
///
/// VTOR ignores address bits below the table's power-of-two size, so a table
/// that is not aligned to it is fetched from the wrong base and the high IRQs
/// land in `DefaultHandler` (docs/book/src/maintenance/log/jul-11-nvic.md).
fn vector(irq: Interrupt) -> (u32, u32, usize) {
    let n = irq as usize;
    let size = (16 + __INTERRUPTS.len()) * 4;
    let align = size.next_power_of_two();
    let vtor = rd(SCB_VTOR) as usize;
    let fetched = rd((vtor & !(align - 1)) | ((16 + n) * 4));
    let linked = rd(__INTERRUPTS.as_ptr() as usize + n * 4);
    (fetched, linked, align)
}

fn check_vector(irq: Interrupt, what: &str) -> bool {
    let (fetched, linked, align) = vector(irq);
    let enabled = NVIC::is_enabled(irq);
    let ok = fetched == linked && enabled;
    if ok {
        defmt::info!(
            "HIL M2 PASS {=str} IRQ{=u16} vector {=u32:#010x} reaches its handler, NVIC enabled",
            what,
            irq as u16,
            fetched
        );
    } else {
        defmt::error!(
            "HIL M2 FAIL {=str} IRQ{=u16}: fetched vector {=u32:#010x}, linked handler {=u32:#010x}, table align {=usize:#x}, NVIC enabled {=bool}",
            what,
            irq as u16,
            fetched,
            linked,
            align,
            enabled
        );
    }
    ok
}

/// M1 and M2. Call right after `init()`, which brings the ring up.
pub fn launched() {
    let vtor = rd(SCB_VTOR);
    let table = __INTERRUPTS.as_ptr() as u32 - 0x40;
    defmt::info!(
        "HIL M1 PASS firmware launched: defmt ring up, VTOR={=u32:#010x}, vector table at {=u32:#010x} ({=usize} IRQ slots)",
        vtor,
        table,
        __INTERRUPTS.len()
    );
    if vtor != table {
        defmt::warn!("HIL M1 VTOR does not point at this image's vector table");
    }
    let doorbell = check_vector(DOORBELL, "doorbell IPCC1_RX");
    let timer = check_vector(TIMER, "time driver TIM2");
    if doorbell && timer {
        defmt::info!("HIL M2 PASS interrupt routing");
    }
}

/// M3, just before `Connect`.
pub fn connecting() {
    defmt::info!(
        "HIL M3 entering Connect: waiting for the AP's descriptor and its doorbell on IPCC1 ch{=u32}",
        RX_CHAN
    );
}

/// M6, once `Connect` has returned.
///
/// The monitor only runs while `main` is parked, and the doorbell's ISR wakes
/// `main` straight through `Connect`, so M4 and M5 are usually confirmed here
/// rather than sampled.
pub fn connected() {
    if !PUBLISHED_SEEN.swap(true, Ordering::AcqRel) {
        let magic = rd(IPC_BASE);
        if magic == CIPC_MAGIC {
            defmt::info!("HIL M4 PASS connection requested: the AP published the descriptor (confirmed at connect)");
        } else {
            defmt::warn!("HIL M4 descriptor magic is {=u32:#010x} after connect", magic);
        }
    }
    if !DOORBELL_SEEN.swap(true, Ordering::AcqRel) {
        defmt::info!(
            "HIL M5 PASS doorbell interrupt fired: Connect was parked on it and only IPCC1_RX could wake it"
        );
    }
    CONNECTED.store(true, Ordering::Release);
    defmt::info!(
        "HIL M6 PASS connected: doorbell IRQ serviced, descriptor accepted, AP acked on IPCC1 ch{=u32}",
        TX_CHAN
    );
}

/// M7, after each send.
pub fn sent(seq: u32, ok: bool) {
    LAST_SEQ.store(seq, Ordering::Release);
    if ok {
        if SENT.fetch_add(1, Ordering::AcqRel) == 0 {
            defmt::info!(
                "HIL M7 PASS first message sent: seq={=u32}, slot written, doorbell rung on ch{=u32}",
                seq,
                TX_CHAN
            );
        }
    } else if SEND_ERRORS.fetch_add(1, Ordering::AcqRel) == 0 {
        defmt::warn!(
            "HIL M7 send of seq={=u32} failed: the AP has not consumed the previous message",
            seq
        );
        dump_bell();
    }
}

fn dump_bell() {
    defmt::info!(
        "HIL trace IPCC1: C1TOC2SR={=u32:#x} C2TOC1SR={=u32:#x} C2CR={=u32:#x} C2MR={=u32:#x} slot flags={=u32:#x} descriptor magic={=u32:#010x}",
        rd(IPCC + offsets::C1TOC2SR),
        rd(IPCC + offsets::C2TOC1SR),
        rd(IPCC + offsets::C2CR),
        rd(IPCC + offsets::C2MR),
        rd(SLOT_TX_FLAGS),
        rd(IPC_BASE)
    );
}

/// The AP's doorbell has been asserted for a while and nothing consumed it.
fn dump_stuck_doorbell() {
    let (fetched, linked, align) = vector(DOORBELL);
    defmt::error!(
        "HIL M5 FAIL the AP's doorbell is asserted but was not serviced: NVIC enabled={=bool} pending={=bool} active={=bool}, ICSR={=u32:#010x}, PRIMASK={=bool}",
        NVIC::is_enabled(DOORBELL),
        NVIC::is_pending(DOORBELL),
        NVIC::is_active(DOORBELL),
        rd(SCB_ICSR),
        cortex_m::register::primask::read().is_active()
    );
    defmt::error!(
        "HIL M5 trace vector fetched={=u32:#010x} linked={=u32:#010x} align={=usize:#x} VTOR={=u32:#010x}",
        fetched,
        linked,
        align,
        rd(SCB_VTOR)
    );
    dump_bell();
}

/// Watches the doorbell and the IPC window from the CM33's side.
///
/// It runs whenever `main` is parked, which is exactly when the interesting
/// transitions happen.
#[embassy_executor::task]
pub async fn monitor() {
    let mut armed = false;
    let mut asserted_since: Option<Instant> = None;
    let mut stuck_reported = false;
    let mut first_consumed = true;
    let mut last_consumed = u32::MAX;
    let mut consumed = 0u32;
    let mut valid_since: Option<Instant> = None;
    let mut unread_reported = false;
    let mut next_summary = Instant::now() + Duration::from_secs(10);

    loop {
        let now = Instant::now();
        let incoming = (rd(IPCC + offsets::C1TOC2SR) >> RX_CHAN) & 1 == 1;

        if !CONNECTED.load(Ordering::Acquire) {
            let now_armed = rd(IPCC + offsets::C2CR) & 1 == 1
                && (rd(IPCC + offsets::C2MR) >> RX_CHAN) & 1 == 0;
            if now_armed && !armed {
                defmt::info!(
                    "HIL M3 PASS parked in Connect: IPCC1 ch{=u32} armed (C2CR.RXOIE set, unmasked)",
                    RX_CHAN
                );
            }
            armed = now_armed;
            if rd(IPC_BASE) == CIPC_MAGIC && !PUBLISHED_SEEN.swap(true, Ordering::AcqRel) {
                defmt::info!("HIL M4 PASS connection requested: the AP published the descriptor");
            }
            if incoming && !DOORBELL_SEEN.swap(true, Ordering::AcqRel) {
                defmt::info!(
                    "HIL M5 PASS the AP rang IPCC1 ch{=u32}: doorbell interrupt fired",
                    RX_CHAN
                );
            }
        }

        // An asserted doorbell is consumed within microseconds when the IRQ path
        // works; 50 ms of it means the handler is not running.
        if incoming {
            let since = *asserted_since.get_or_insert(now);
            if !stuck_reported && now - since > Duration::from_millis(50) {
                stuck_reported = true;
                dump_stuck_doorbell();
            }
        } else {
            asserted_since = None;
        }

        if CONNECTED.load(Ordering::Acquire) {
            let flags = rd(SLOT_TX_FLAGS);
            if flags & FLAG_VALID != 0 {
                let since = *valid_since.get_or_insert(now);
                if !unread_reported && now - since > Duration::from_secs(3) {
                    unread_reported = true;
                    defmt::warn!(
                        "HIL M8 the AP has not read seq={=u32} for 3 s (is the application running?)",
                        LAST_SEQ.load(Ordering::Acquire)
                    );
                    dump_bell();
                }
            } else if flags & FLAG_CONSUMED != 0 {
                valid_since = None;
                unread_reported = false;
                let seq = LAST_SEQ.load(Ordering::Acquire);
                if seq != last_consumed && SENT.load(Ordering::Acquire) > 0 {
                    last_consumed = seq;
                    consumed += 1;
                    if first_consumed {
                        first_consumed = false;
                        defmt::info!(
                            "HIL M8 PASS the AP read the first message (seq={=u32}): slot consumed",
                            seq
                        );
                    }
                }
            }
        }

        if now >= next_summary {
            next_summary = now + Duration::from_secs(10);
            if CONNECTED.load(Ordering::Acquire) {
                defmt::info!(
                    "HIL M9 steady: sent={=u32} consumed={=u32} send_errors={=u32} last seq={=u32}",
                    SENT.load(Ordering::Acquire),
                    consumed,
                    SEND_ERRORS.load(Ordering::Acquire),
                    LAST_SEQ.load(Ordering::Acquire)
                );
            } else {
                defmt::warn!(
                    "HIL M3 still in Connect: descriptor published={=bool}, doorbell seen={=bool}, armed={=bool}",
                    PUBLISHED_SEEN.load(Ordering::Acquire),
                    DOORBELL_SEEN.load(Ordering::Acquire),
                    armed
                );
            }
        }

        Timer::after_millis(1).await;
    }
}

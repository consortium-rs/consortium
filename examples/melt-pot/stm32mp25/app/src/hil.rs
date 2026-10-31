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

//! HIL stage narration, CA35 side (STM32MP257F-DK).
//!
//! Every line starts with `HIL A<n>`; the CM33's `HIL M<n>` frames arrive on
//! the same console through the debug ring, so one log shows both cores:
//!
//! | Stage | Meaning                                                           |
//! | ----- | ----------------------------------------------------------------- |
//! | A1    | application started                                               |
//! | A2    | firmware loaded: the CM33's remoteproc is running                 |
//! | A3    | UIO windows mapped (IPC, debug ring, IPCC1)                       |
//! | A4    | the CM33 initialized its defmt ring                               |
//! | A5    | connection requested: descriptor published, doorbell rung         |
//! | A6    | the AP's doorbell observed asserted (best effort: µs window)       |
//! | A7    | the CM33 consumed it, i.e. its interrupt fired and was serviced   |
//! | A8    | the AP's doorbell interrupt fired (the CM33's ack)                |
//! | A9    | handshake complete                                                |
//! | A10   | the CM33's first message is in the slot                           |
//! | A11   | the AP's interrupt for that message fired                         |
//! | A12   | the AP read the message                                           |
//! | A13   | steady-state summary every 10 s                                   |
//!
//! A stage that stalls prints the decoded doorbell, descriptor, slot and ring
//! state once, with what it most likely means. The windows are this board's
//! (`../Consortium.bringup.toml`); they are mapped a second time, read-only.

use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{Duration, Instant};

use consortium_ipc_transport_memory::bell::ipcc::registers::offsets;
use consortium_runtime_app::log::{error, info, warn};
use consortium_runtime_app::uio::UioDevice;

const RPROC_CORE: &str = "m33";
const IPC: (&str, u64) = ("ipc-shm", 0x2006_4000);
const DBG: (&str, u64) = ("dbg-shm", 0x2006_2000);
const BELL: (&str, u64) = ("ipcc1", 0x4049_0000);
/// `party.linux.chan`: the AP rings it.
const TX_CHAN: u32 = 0;
/// `party.cm33.chan`: the CM33 rings it.
const RX_CHAN: u32 = 1;

/// Slot offsets: `align64(sizeof Descriptor)` and one `align64(12 + mtu)` later.
const SLOT_TX: usize = 0x100;
const SLOT_RX: usize = 0x1c0;
const CIPC_MAGIC: u32 = u32::from_be_bytes(*b"CIPC");
const CDBG_MAGIC: u32 = u32::from_be_bytes(*b"CDBG");
const FLAG_VALID: u32 = 1 << 0;
/// `consortium_dbg::descriptor::Flags::Overflow`.
const FLAG_RING_OVERFLOW: u32 = 1 << 5;

static REQUESTED: AtomicBool = AtomicBool::new(false);
static CONNECTED: AtomicBool = AtomicBool::new(false);
static CONSUMED_SEEN: AtomicBool = AtomicBool::new(false);
static RECEIVED: AtomicU32 = AtomicU32::new(0);
static LAST_SEQ: AtomicU32 = AtomicU32::new(0);
static GAPS: AtomicU32 = AtomicU32::new(0);
static IRQ_AT_CONNECT: AtomicU32 = AtomicU32::new(0);
static IRQ_AT_REQUEST: AtomicU32 = AtomicU32::new(0);
static IRQ_FIRED: AtomicBool = AtomicBool::new(false);

/// What the AP can see of the doorbell from its own bank.
struct Bell {
    /// Our ring is still asserted: the CM33 has not consumed it.
    out: bool,
    /// The CM33's ring is asserted and we have not consumed it.
    inc: bool,
    /// Our RX line is armed.
    armed: bool,
    raw: String,
}

/// Read-only second mappings of the three windows the transport uses.
pub struct Windows {
    ipc: usize,
    dbg: usize,
    bell: usize,
    _devices: Vec<Arc<UioDevice>>,
}

impl Windows {
    /// A3. `None` (and a FAIL line) if any window cannot be mapped.
    ///
    /// Reuses the devices generated `init()` opened (`Peripherals::uio`): the
    /// `uio` crate holds an exclusive `flock` on each `/dev/uioN`, so opening one
    /// again would block forever. `mmap` returns the mapping each device already
    /// cached, so nothing is mapped or zeroed a second time.
    pub fn open(uio: &[Arc<UioDevice>]) -> Option<Self> {
        // `init()`'s `open_named` order in `consortium.gen.rs`.
        let order = [DBG, IPC, BELL];
        let mut bases = [0usize; 3];
        for (slot, (name, base)) in order.into_iter().enumerate() {
            let Some(dev) = uio.get(slot) else {
                error!("HIL A3 FAIL init() opened {} UIO devices, expected 3", uio.len());
                return None;
            };
            match dev.mmap(0) {
                Ok(addr) => {
                    let at = [IPC.0, DBG.0, BELL.0].iter().position(|n| *n == name).unwrap();
                    bases[at] = addr;
                }
                Err(e) => {
                    error!("HIL A3 FAIL cannot map UIO '{}' at {:#x}: {:?}", name, base, e);
                    return None;
                }
            }
        }
        info!(
            "HIL A3 PASS UIO windows mapped: {} @ {:#x}, {} @ {:#x}, {} @ {:#x}; {} IRQ count {}",
            IPC.0,
            IPC.1,
            DBG.0,
            DBG.1,
            BELL.0,
            BELL.1,
            BELL.0,
            irq_count()
        );
        match irq_line() {
            None => error!(
                "HIL A3 FAIL /proc/interrupts has no '{}' row: the doorbell UIO node has no interrupt ({} expected), so the AP can never be woken",
                BELL.0, BELL_IRQ
            ),
            Some(row) => match row_spi(&row) {
                Some(n) if n != BELL_SPI => error!(
                    "HIL A3 FAIL the doorbell UIO is wired to GIC SPI {} but this side's interrupt is {}: fix `interrupts` in the DTS node ({})",
                    n, BELL_IRQ, row
                ),
                _ => {}
            },
        }
        Some(Self {
            ipc: bases[0],
            dbg: bases[1],
            bell: bases[2],
            _devices: uio.to_vec(),
        })
    }

    fn rd(&self, addr: usize) -> u32 {
        // SAFETY: `addr` is inside one of the three windows mapped in `open`
        // (kept alive by `_devices`) and 4-byte aligned. Every location read is
        // shared memory or an IPCC status/control register; none has read side
        // effects.
        unsafe { std::ptr::read_volatile(addr as *const u32) }
    }

    fn ipc(&self, off: usize) -> u32 {
        self.rd(self.ipc + off)
    }

    fn dbg(&self, off: usize) -> u32 {
        self.rd(self.dbg + off)
    }

    fn bell(&self) -> Bell {
        // The AP is IPCC processor 1.
        let c1cr = self.rd(self.bell + offsets::C1CR);
        let c1mr = self.rd(self.bell + offsets::C1MR);
        let c1to2 = self.rd(self.bell + offsets::C1TOC2SR);
        let c2to1 = self.rd(self.bell + offsets::C2TOC1SR);
        Bell {
            out: (c1to2 >> TX_CHAN) & 1 == 1,
            inc: (c2to1 >> RX_CHAN) & 1 == 1,
            armed: c1cr & 1 == 1 && (c1mr >> RX_CHAN) & 1 == 0,
            raw: format!(
                "C1CR={c1cr:#x} C1MR={c1mr:#x} C1TOC2SR={c1to2:#x} C2TOC1SR={c2to1:#x}"
            ),
        }
    }

    /// Everything that explains a stall, decoded.
    fn dump(&self, why: &str) {
        let b = self.bell();
        error!("HIL trace ({}):", why);
        error!(
            "HIL trace   doorbell {}: AP->M asserted={} M->AP asserted={} AP armed={} | {}",
            BELL.0, b.out, b.inc, b.armed, b.raw
        );
        error!(
            "HIL trace   descriptor magic={:#010x} (CIPC={:#010x}) version/channels={:#010x} state={} rx mtu={}",
            self.ipc(0),
            CIPC_MAGIC,
            self.ipc(4),
            self.ipc(0x18),
            self.ipc(0x10)
        );
        error!(
            "HIL trace   channel table: [{:#x} {:#x} {:#x}] [{:#x} {:#x} {:#x}]",
            self.ipc(0x20),
            self.ipc(0x24),
            self.ipc(0x28),
            self.ipc(0x2c),
            self.ipc(0x30),
            self.ipc(0x34)
        );
        for (name, off) in [("AP->M", SLOT_TX), ("M->AP", SLOT_RX)] {
            error!(
                "HIL trace   slot {} @+{:#x}: len={} flags={:#x} crc={:#010x}",
                name,
                off,
                self.ipc(off),
                self.ipc(off + 4),
                self.ipc(off + 8)
            );
        }
        let (w, r, cap, flags) = self.ring();
        error!(
            "HIL trace   CDBG magic={:#010x} write={} read={} cap={} flags={:#x}",
            self.dbg(0),
            w,
            r,
            cap,
            flags
        );
        error!(
            "HIL trace   /proc/interrupts: {}",
            irq_line().unwrap_or_else(|| format!("no '{}' row", BELL.0))
        );
        error!("HIL trace   {}", rproc_status());
    }

    fn ring(&self) -> (u32, u32, u32, u32) {
        let w4 = self.dbg(4);
        let w8 = self.dbg(8);
        (w4 >> 16, w8 & 0xffff, w8 >> 16, self.dbg(0xc) & 0xffff)
    }
}

/// The doorbell's `/proc/interrupts` row, if the UIO device registered an IRQ.
///
/// `uio_pdrv_genirq` names the IRQ after the node's `linux,uio-name`, or after
/// the node itself (`{name}@{base}`) when a hand-written DTS leaves it out.
fn irq_line() -> Option<String> {
    let node = format!("@{:x}", BELL.1);
    std::fs::read_to_string("/proc/interrupts")
        .ok()?
        .lines()
        .find(|line| {
            line.split_whitespace()
                .last()
                .is_some_and(|n| n == BELL.0 || n.ends_with(&node))
        })
        .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// The SPI the doorbell's A-side interrupt must use (the generated DTS says
/// `GIC_SPI 171`).
const BELL_SPI: u32 = 171;
const BELL_IRQ: &str = "GIC SPI 171";

/// The SPI a `/proc/interrupts` row is wired to: GICv3 numbers SPIs from 32.
fn row_spi(row: &str) -> Option<u32> {
    let mut fields = row.split_whitespace();
    fields.find(|f| f.starts_with("GIC"))?;
    fields.next()?.parse::<u32>().ok()?.checked_sub(32)
}

/// Total count of the doorbell's UIO interrupt across CPUs (0 without a row;
/// [`irq_line`] tells the two apart).
fn irq_count() -> u32 {
    irq_line()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map_while(|f| f.parse::<u32>().ok())
                .sum()
        })
        .unwrap_or(0)
}

/// `remoteprocN (name): state, firmware`, or why there is none.
fn rproc_status() -> String {
    let Ok(dir) = std::fs::read_dir("/sys/class/remoteproc") else {
        return "no /sys/class/remoteproc".into();
    };
    for entry in dir.flatten() {
        let path = entry.path();
        let read = |f: &str| {
            std::fs::read_to_string(path.join(f))
                .map(|s| s.trim().to_owned())
                .unwrap_or_default()
        };
        if read("name") == RPROC_CORE {
            return format!(
                "{} ({}): state={} firmware={}",
                entry.file_name().to_string_lossy(),
                RPROC_CORE,
                read("state"),
                read("firmware")
            );
        }
    }
    format!("no remoteproc named '{RPROC_CORE}'")
}

/// A1 and A2.
pub fn started() {
    info!("HIL A1 PASS application started (pid {})", std::process::id());
    let status = rproc_status();
    if status.contains("state=running") {
        info!("HIL A2 PASS firmware loaded: {}", status);
    } else {
        error!(
            "HIL A2 FAIL the CM33 is not running: {} (load it with remoteproc first)",
            status
        );
    }
}

/// A4: the CM33's `init()` writes the CDBG header before anything else.
pub fn ring_check(w: &Windows) {
    let (wr, rd, cap, flags) = w.ring();
    if w.dbg(0) == CDBG_MAGIC && flags & FLAG_RING_OVERFLOW != 0 {
        warn!(
            "HIL A4 PASS the CM33 initialized its defmt ring, but it overflowed before the AP attached (write={} read={} cap={} flags={:#x}): later CM33 stage frames were dropped",
            wr, rd, cap, flags
        );
    } else if w.dbg(0) == CDBG_MAGIC {
        info!(
            "HIL A4 PASS the CM33 initialized its defmt ring: cap={} write={} read={} flags={:#x}",
            cap, wr, rd, flags
        );
    } else {
        error!(
            "HIL A4 FAIL no CDBG header at {:#x} (magic {:#010x}): the CM33 never reached init(), or it logs to another address",
            DBG.1,
            w.dbg(0)
        );
        w.dump("A4");
    }
}

/// A5, just before `Transceiver::connect`.
pub fn connect_requested() {
    IRQ_AT_REQUEST.store(irq_count(), Ordering::Release);
    REQUESTED.store(true, Ordering::Release);
    info!(
        "HIL A5 PASS connection requested: publishing the descriptor, ringing {} ch{}",
        BELL.0, TX_CHAN
    );
}

/// A9, once `connect` returned.
pub fn connected() {
    if !CONSUMED_SEEN.swap(true, Ordering::AcqRel) {
        info!(
            "HIL A7 PASS (inferred) the CM33 consumed the doorbell between samples; its ack proves its IRQ path"
        );
    }
    let irqs = irq_count();
    let base = IRQ_AT_REQUEST.load(Ordering::Acquire);
    if !IRQ_FIRED.swap(true, Ordering::AcqRel) {
        if irqs > base {
            info!(
                "HIL A8 PASS A-core interrupt fired: {} count {} -> {}",
                BELL.0, base, irqs
            );
        } else {
            warn!(
                "HIL A8 {} count did not move ({}) although the handshake completed",
                BELL.0, irqs
            );
        }
    }
    IRQ_AT_CONNECT.store(irqs, Ordering::Release);
    CONNECTED.store(true, Ordering::Release);
    info!("HIL A9 PASS handshake complete: the CM33 accepted the descriptor and acked");
}

/// `connect` failed.
pub fn connect_failed(e: &dyn Debug) {
    error!("HIL A9 FAIL connect returned {:?}", e);
}

/// A12 for the first message, and sequence tracking for A13.
pub fn received(seq: u32, temperature_mdc: i32) {
    let n = RECEIVED.fetch_add(1, Ordering::AcqRel);
    let last = LAST_SEQ.swap(seq, Ordering::AcqRel);
    if n == 0 {
        info!(
            "HIL A12 PASS message read: seq={} temp_mdc={}",
            seq, temperature_mdc
        );
    } else if seq != last.wrapping_add(1) {
        GAPS.fetch_add(1, Ordering::AcqRel);
        warn!("HIL A13 sequence gap: {} -> {}", last, seq);
    }
}

/// Starts the watcher thread (A6–A8, A10, A11, A13 and stall traces).
pub fn spawn_monitor(w: Windows) {
    std::thread::Builder::new()
        .name("hil-monitor".into())
        .spawn(move || monitor(&w))
        .expect("spawn hil-monitor");
}

fn monitor(w: &Windows) {
    let mut out_seen = false;
    let mut requested_at: Option<Instant> = None;
    let mut connect_dumped = false;
    let mut slot_seen = false;
    let mut msg_irq_seen = false;
    let mut last_rx = (0u32, Instant::now());
    let mut quiet_dumped = false;
    let mut next_summary = Instant::now() + Duration::from_secs(10);
    let mut tick = 0u32;

    loop {
        tick = tick.wrapping_add(1);
        let now = Instant::now();
        // /proc/interrupts is comparatively expensive; read it every 10 ms.
        let irqs = if tick % 10 == 0 { Some(irq_count()) } else { None };

        if REQUESTED.load(Ordering::Acquire) && !CONNECTED.load(Ordering::Acquire) {
            let started = *requested_at.get_or_insert(now);
            let base = IRQ_AT_REQUEST.load(Ordering::Acquire);
            let b = w.bell();
            if b.out && !out_seen {
                out_seen = true;
                info!(
                    "HIL A6 PASS AP->M doorbell asserted ({} ch{})",
                    BELL.0, TX_CHAN
                );
            }
            if out_seen && !b.out && !CONSUMED_SEEN.swap(true, Ordering::AcqRel) {
                info!(
                    "HIL A7 PASS the CM33 consumed the doorbell: its interrupt fired and was serviced"
                );
            }
            if let Some(n) = irqs
                && n > base
                && !IRQ_FIRED.swap(true, Ordering::AcqRel)
            {
                info!(
                    "HIL A8 PASS A-core interrupt fired: {} count {} -> {}",
                    BELL.0, base, n
                );
            }
            if !connect_dumped && now - started > Duration::from_secs(3) {
                connect_dumped = true;
                let why = if w.ipc(0) != CIPC_MAGIC {
                    "descriptor not in shared memory: the AP wrote a different window"
                } else if b.out {
                    "the CM33 never consumed the doorbell: firmware not running, IRQ not routed (see HIL M2), or a stale doorbell"
                } else if !IRQ_FIRED.load(Ordering::Acquire) && b.inc && b.armed {
                    "the CM33 acked: the M->AP line is raised and armed in the doorbell, but Linux never took the interrupt (missing/wrong `interrupts` in the UIO node, or the line is not routed to this core)"
                } else if !IRQ_FIRED.load(Ordering::Acquire) {
                    "the CM33 consumed the doorbell but never acked: check its HIL M5/M6 lines for a Connect error"
                } else {
                    "the interrupt fired but the AP waiter was not woken (UIO service / lost wakeup)"
                };
                error!("HIL A9 FAIL handshake not complete after 3 s: {}", why);
                w.dump("A9");
            }
        }

        if CONNECTED.load(Ordering::Acquire) {
            if !slot_seen && w.ipc(SLOT_RX + 4) & FLAG_VALID != 0 {
                slot_seen = true;
                info!(
                    "HIL A10 PASS the CM33's first message is in the slot ({} bytes)",
                    w.ipc(SLOT_RX)
                );
            }
            if let Some(n) = irqs
                && !msg_irq_seen
                && n > IRQ_AT_CONNECT.load(Ordering::Acquire)
            {
                msg_irq_seen = true;
                info!(
                    "HIL A11 PASS message interrupt fired: {} count {} -> {}",
                    BELL.0,
                    IRQ_AT_CONNECT.load(Ordering::Acquire),
                    n
                );
            }
            let rx = RECEIVED.load(Ordering::Acquire);
            if rx != last_rx.0 {
                last_rx = (rx, now);
                quiet_dumped = false;
            } else if !quiet_dumped && now - last_rx.1 > Duration::from_secs(3) {
                quiet_dumped = true;
                error!(
                    "HIL A13 FAIL no message for 3 s (received {} so far)",
                    rx
                );
                w.dump("A13");
            }
        }

        if now >= next_summary {
            next_summary = now + Duration::from_secs(10);
            if CONNECTED.load(Ordering::Acquire) {
                let (_, _, _, flags) = w.ring();
                info!(
                    "HIL A13 steady: received={} last seq={} gaps={} {} IRQs={} ring flags={:#x}",
                    RECEIVED.load(Ordering::Acquire),
                    LAST_SEQ.load(Ordering::Acquire),
                    GAPS.load(Ordering::Acquire),
                    BELL.0,
                    irq_count(),
                    flags
                );
            }
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}

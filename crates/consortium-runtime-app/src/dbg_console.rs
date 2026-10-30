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

//! Host-side defmt console for coprocessor debug (CDBG) rings.
//!
//! Each coprocessor that declares `[dbg.<core>]` writes defmt frames into a
//! shared-memory ring; the AP maps that ring through UIO and reads it. This
//! module provides the two consumption modes selected by `[dbg] mode`:
//!
//! - [`spawn`] (`mode = "auto"`): one background task per ring that drains it,
//!   decodes the defmt stream against the firmware ELF, and forwards each frame
//!   to [`tracing`] at the mapped level. Turnkey.
//! - [`open`] (`mode = "interface"`): resolves each ring to a [`DbgRingHandle`]
//!   and hands it back so the integrator drives reading and decoding.
//!
//! The dbg ring has no doorbell of its own, so the auto task polls on a fixed
//! interval rather than waiting on an interrupt.
#![cfg(all(feature = "dbg", feature = "ipc-shm"))]

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use consortium_dbg::defmt_decoder::Frame;
use consortium_dbg::defmt_parser::Level;
use consortium_dbg::host::{DefmtContext, DefmtSession, HostError, HostRing};

use crate::uio::{UioDevice, UioDeviceError};

/// `tracing` target attached to every forwarded frame.
const TARGET: &str = "consortium::dbg";

/// Poll interval for draining a ring; also the retry cadence while waiting for
/// the writer to initialize the descriptor.
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Locates one coprocessor's debug ring within the opened UIO devices.
///
/// Emitted by the configuration codegen, one per `[dbg.<core>]`.
#[derive(Clone, Copy, Debug)]
pub struct DbgRingSpec {
    /// Index into the opened UIO owner slice (the device aggregating dbg rings).
    pub uio_id: usize,
    /// mmap window index of this ring within that device.
    pub mmap_id: usize,
    /// Mapped ring length in bytes.
    pub len: usize,
    /// The coprocessor firmware ELF, embedded into the application binary at
    /// compile time (via `include_bytes!` in the configuration codegen) and used
    /// for defmt symbol resolution. Embedding rather than reading a path removes
    /// the on-target file dependency: the build host and the deployed board are
    /// different machines, so a runtime path would be fragile.
    pub elf_bytes: &'static [u8],
    /// Coprocessor label attached to every emitted record (e.g. `cm33`).
    pub label: &'static str,
}

/// Errors raised while bringing up the debug console.
#[derive(Debug, thiserror::Error)]
pub enum DbgConsoleError {
    /// Mapping the ring's UIO window failed.
    #[error(transparent)]
    Uio(#[from] UioDeviceError),
    /// The embedded firmware ELF carried no parseable defmt table.
    #[error(transparent)]
    Host(#[from] HostError),
}

/// A running set of auto-mode console threads.
///
/// Each ring is polled on its own OS thread rather than a Tokio task because the
/// stream decoder ([`DefmtSession`]) is not `Send` and must persist across polls
/// to reassemble frames split over packets. Dropping signals every thread to
/// stop and joins it.
///
/// Each thread owns the [`UioDevice`] its ring is mapped from, because a device
/// unmaps its windows when its last owner drops. The console may therefore be
/// moved out of `Peripherals` and outlive the rest of it.
pub struct DbgConsole {
    stop: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Drop for DbgConsole {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}

/// Spawn one polling thread per ring (auto mode).
///
/// Each ring carries its firmware ELF embedded in the binary, so a thread waits
/// for the writer to publish a valid descriptor, then polls the ring and
/// forwards frames to `tracing`.
pub fn spawn(
    devices: &[Arc<UioDevice>],
    specs: &[DbgRingSpec],
) -> Result<DbgConsole, DbgConsoleError> {
    // Built before the first thread, so if a later ring fails to map, dropping it
    // on the `?` stops and joins the threads already running instead of detaching
    // them.
    let mut console = DbgConsole {
        stop: Arc::new(AtomicBool::new(false)),
        threads: Vec::with_capacity(specs.len()),
    };
    for spec in specs {
        let device = Arc::clone(&devices[spec.uio_id]);
        // The AP only reads the ring; the writing coprocessor initializes the
        // CDBG descriptor (and clears the carveout) on its own boot, so map
        // read-only-style here without zeroing to avoid clobbering that state.
        let base = device.mmap(spec.mmap_id)?;
        let spec = *spec;
        let stop = Arc::clone(&console.stop);
        console.threads.push(std::thread::spawn(move || {
            // Keep the mapping alive for as long as this thread reads it.
            let _mapping = device;
            run_ring(base, spec, &stop)
        }));
    }
    Ok(console)
}

/// Drain `base` until `stop` is set, decoding against the ring's embedded ELF and
/// forwarding to `tracing`.
fn run_ring(base: usize, spec: DbgRingSpec, stop: &AtomicBool) {
    let ctx = match DefmtContext::from_elf_bytes(spec.elf_bytes) {
        Ok(ctx) => ctx,
        Err(err) => {
            consortium_log::error!(target: TARGET, core = spec.label, "defmt table parse failed: {err}");
            return;
        }
    };

    consortium_log::trace!(
        "entering ring loop for {} (base = {:#x}, len = {:#x})",
        spec.label,
        base,
        spec.len
    );

    // The writer initializes the descriptor on its own boot; retry until the
    // magic/pointers validate rather than aborting the thread.
    let mut ring = loop {
        if stop.load(Ordering::Relaxed) {
            return;
        }
        // SAFETY: `base`/`spec.len` describe the UIO-mapped CDBG ring; only this
        // thread reads `read_ptr`, satisfying `HostRing`'s exclusivity invariant.
        match unsafe { HostRing::from_raw_parts(base as *mut u8, spec.len) } {
            Ok(ring) => break ring,
            Err(_) => std::thread::sleep(POLL_INTERVAL),
        }
    };

    let mut session = DefmtSession::new(&ctx);
    while !stop.load(Ordering::Relaxed) {
        if let Ok(packets) = ring.drain_packets() {
            for packet in packets {
                let _ = session.push_packet(&packet, |frame| emit(spec.label, frame));
            }
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Forward one decoded frame to `tracing`, mapping the defmt level to a
/// `tracing` level (an unlevelled frame is treated as `INFO`).
fn emit(label: &str, frame: &Frame<'_>) {
    let msg = frame.display_message().to_string();
    match frame.level() {
        Some(Level::Trace) => tracing::trace!(target: TARGET, core = label, "{msg}"),
        Some(Level::Debug) => tracing::debug!(target: TARGET, core = label, "{msg}"),
        Some(Level::Info) | None => tracing::info!(target: TARGET, core = label, "{msg}"),
        Some(Level::Warn) => tracing::warn!(target: TARGET, core = label, "{msg}"),
        Some(Level::Error) => tracing::error!(target: TARGET, core = label, "{msg}"),
    }
}

/// An integrator-driven handle to one debug ring (interface mode).
///
/// Holds the mapped base, the parsed defmt table, and the ring length; call
/// [`reader`](Self::reader) to obtain a [`DbgReader`] once the writer has
/// initialized the descriptor, and decode frames with a [`DefmtSession`] built
/// from [`context`](Self::context). The handle owns the [`UioDevice`] its ring
/// is mapped from, so it stays valid however the rest of `Peripherals` is
/// moved or dropped.
pub struct DbgRingHandle {
    base: usize,
    len: usize,
    owner: Arc<UioDevice>,
    /// Coprocessor label (e.g. `cm33`).
    pub label: &'static str,
    context: DefmtContext,
}

impl DbgRingHandle {
    /// The parsed defmt table for this ring's firmware.
    pub fn context(&self) -> &DefmtContext {
        &self.context
    }

    /// Try to wrap the ring for reading.
    ///
    /// Returns `Err` (typically a magic mismatch) until the coprocessor has
    /// written a valid descriptor; callers should retry.
    pub fn reader(&self) -> Result<DbgReader, HostError> {
        // SAFETY: `base`/`len` describe the UIO-mapped CDBG ring, which the
        // returned reader keeps mapped by owning its device; the caller is the
        // sole reader of `read_ptr` for the returned handle's lifetime.
        let ring = unsafe { HostRing::from_raw_parts(self.base as *mut u8, self.len) }?;
        Ok(DbgReader {
            ring,
            _owner: Arc::clone(&self.owner),
        })
    }
}

/// A [`HostRing`] that keeps the UIO device it reads mapped.
///
/// Dereferences to the ring, so use it exactly like one.
pub struct DbgReader {
    ring: HostRing,
    _owner: Arc<UioDevice>,
}

impl Deref for DbgReader {
    type Target = HostRing;

    fn deref(&self) -> &HostRing {
        &self.ring
    }
}

impl DerefMut for DbgReader {
    fn deref_mut(&mut self) -> &mut HostRing {
        &mut self.ring
    }
}

/// Resolve every ring to a [`DbgRingHandle`] without spawning tasks (interface
/// mode). Maps each window and parses its firmware ELF; does not require the
/// descriptor to be initialized yet.
pub fn open(
    devices: &[Arc<UioDevice>],
    specs: &[DbgRingSpec],
) -> Result<Vec<DbgRingHandle>, DbgConsoleError> {
    let mut handles = Vec::with_capacity(specs.len());
    for spec in specs {
        // The AP only reads the ring; the writing coprocessor initializes it. See
        // `spawn`.
        let owner = Arc::clone(&devices[spec.uio_id]);
        let base = owner.mmap(spec.mmap_id)?;
        let context = DefmtContext::from_elf_bytes(spec.elf_bytes)?;
        handles.push(DbgRingHandle {
            base,
            len: spec.len,
            owner,
            label: spec.label,
            context,
        });
    }
    Ok(handles)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `DbgRingSpec` carries its firmware ELF as embedded bytes, as the
    /// configuration codegen emits via `include_bytes!`. The `elf_bytes` field is
    /// `&'static [u8]`, so a `const`/`static` byte slice (what `include_bytes!`
    /// expands to) is assignable and the spec stays `Copy`.
    #[test]
    fn spec_holds_embedded_elf_bytes() {
        const ELF: &[u8] = b"\x7fELF fake firmware image";
        let spec = DbgRingSpec {
            uio_id: 3,
            mmap_id: 0,
            len: 0x2000,
            elf_bytes: ELF,
            label: "cm33",
        };
        let copy = spec; // `Copy` is required so the spawn loop can move it per thread.
        assert_eq!(copy.elf_bytes, ELF);
        assert_eq!(copy.label, "cm33");
    }

    /// The embedded bytes flow straight into `DefmtContext::from_elf_bytes` with
    /// no intervening file read, so a non-defmt image surfaces as a decode-time
    /// `Host` error rather than the removed file-read variant.
    #[test]
    fn non_defmt_bytes_surface_as_host_error() {
        let err = match DefmtContext::from_elf_bytes(b"not an elf") {
            Ok(_) => panic!("garbage bytes carry no defmt table"),
            Err(err) => err,
        };
        // The wiring now only fails via `HostError`; the old `Elf` file-read
        // variant no longer exists.
        let _console: DbgConsoleError = err.into();
    }
}

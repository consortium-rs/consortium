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
    /// Index into the `&[UioDevice]` slice (the device aggregating dbg rings).
    pub uio_id: usize,
    /// mmap window index of this ring within that device.
    pub mmap_id: usize,
    /// Mapped ring length in bytes.
    pub len: usize,
    /// Path to the coprocessor firmware artifact used for defmt symbol
    /// resolution (the `[endpoints]` value for this core).
    pub elf_path: &'static str,
    /// Coprocessor label attached to every emitted record (e.g. `cm33`).
    pub label: &'static str,
}

/// Errors raised while bringing up the debug console.
#[derive(Debug, thiserror::Error)]
pub enum DbgConsoleError {
    /// Mapping the ring's UIO window failed.
    #[error(transparent)]
    Uio(#[from] UioDeviceError),
    /// The firmware ELF could not be read from `elf_path`.
    #[error("reading firmware ELF '{path}': {source}")]
    Elf {
        /// The path that failed to read.
        path: String,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The firmware ELF carried no parseable defmt table.
    #[error(transparent)]
    Host(#[from] HostError),
}

/// A running set of auto-mode console threads.
///
/// Each ring is polled on its own OS thread rather than a Tokio task because the
/// stream decoder ([`DefmtSession`]) is not `Send` and must persist across polls
/// to reassemble frames split over packets. Dropping signals every thread to
/// stop and joins it, so a `DbgConsole` must be dropped *before* the
/// [`UioDevice`]s whose mappings the threads read.
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
/// Each thread reads its firmware ELF up front, waits for the writer to publish a
/// valid descriptor, then polls the ring and forwards frames to `tracing`.
pub fn spawn(devices: &[UioDevice], specs: &[DbgRingSpec]) -> Result<DbgConsole, DbgConsoleError> {
    let stop = Arc::new(AtomicBool::new(false));
    let mut threads = Vec::with_capacity(specs.len());
    for spec in specs {
        let base = devices[spec.uio_id].mmap(spec.mmap_id)?;
        let elf = std::fs::read(spec.elf_path).map_err(|source| DbgConsoleError::Elf {
            path: spec.elf_path.to_string(),
            source,
        })?;
        let spec = *spec;
        let stop = Arc::clone(&stop);
        threads.push(std::thread::spawn(move || run_ring(base, elf, spec, &stop)));
    }
    Ok(DbgConsole { stop, threads })
}

/// Drain `base` until `stop` is set, decoding against `elf` and forwarding to
/// `tracing`.
fn run_ring(base: usize, elf: Vec<u8>, spec: DbgRingSpec, stop: &AtomicBool) {
    let ctx = match DefmtContext::from_elf_bytes(&elf) {
        Ok(ctx) => ctx,
        Err(err) => {
            tracing::error!(target: TARGET, core = spec.label, "defmt table parse failed: {err}");
            return;
        }
    };

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
/// [`reader`](Self::reader) to obtain a [`HostRing`] once the writer has
/// initialized the descriptor, and decode frames with a [`DefmtSession`] built
/// from [`context`](Self::context).
pub struct DbgRingHandle {
    base: usize,
    len: usize,
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
    pub fn reader(&self) -> Result<HostRing, HostError> {
        // SAFETY: `base`/`len` describe the UIO-mapped CDBG ring; the caller is
        // the sole reader of `read_ptr` for the returned handle's lifetime.
        unsafe { HostRing::from_raw_parts(self.base as *mut u8, self.len) }
    }
}

/// Resolve every ring to a [`DbgRingHandle`] without spawning tasks (interface
/// mode). Maps each window and parses its firmware ELF; does not require the
/// descriptor to be initialized yet.
pub fn open(
    devices: &[UioDevice],
    specs: &[DbgRingSpec],
) -> Result<Vec<DbgRingHandle>, DbgConsoleError> {
    let mut handles = Vec::with_capacity(specs.len());
    for spec in specs {
        let base = devices[spec.uio_id].mmap(spec.mmap_id)?;
        let elf = std::fs::read(spec.elf_path).map_err(|source| DbgConsoleError::Elf {
            path: spec.elf_path.to_string(),
            source,
        })?;
        let context = DefmtContext::from_elf_bytes(&elf)?;
        handles.push(DbgRingHandle {
            base,
            len: spec.len,
            label: spec.label,
            context,
        });
    }
    Ok(handles)
}

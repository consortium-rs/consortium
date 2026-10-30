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

//! UIO device management for the A-core Linux runtime.
//!
//! [`UioDevice::open`] opens `/dev/uioN`, maps its windows, and — when asked to
//! enable the IRQ — spawns a background Tokio task running the
//! [`irq_loop`](crate::irq) that fans each M→A interrupt out to every bound doorbell.
//!
//! Windows are mapped by kind: [`mmap`](UioDevice::mmap) for register and doorbell
//! blocks, [`mmap_zeroed`](UioDevice::mmap_zeroed) for data windows. The
//! zeroing variant exists because the AP owns first touch of a shared carveout — it
//! is `no-map` reserved memory that survives a warm reboot, so stale descriptor and
//! slot bytes from a previous run must not read as valid.
//!
//! Doorbells are bound before any receive: [`bind_notifier`](UioDevice::bind_notifier)
//! for a plain doorbell, [`bind_service`](UioDevice::bind_service) for a masking one
//! that must mask its asserted lines before the wake.
//!
//! Generated bring-up code drives all of this; the addresses and UIO ids come from
//! `consortium-cfg`.

use std::{
    fs::{self, File},
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::{io::unix::AsyncFd, sync::Notify};
use uio::UioError;

use consortium_ipc_transport_memory::DescriptorError;

use crate::irq::{Service, Services, irq_loop};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum UioDeviceError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("uio: {0:?}")]
    Uio(UioError),

    #[error("descriptor: {0}")]
    Descriptor(#[from] DescriptorError),

    #[error("channel {0} out of range (descriptor has {1} channels)")]
    ChannelOutOfRange(u8, u8),

    #[error("channel pair for chan {0} not found in descriptor")]
    ChannelPairNotFound(u8),

    #[error("no UIO map named {0:?}")]
    MapNotFound(String),

    #[error("no UIO device named {name:?} (nor one mapping {base:#x}) under {root}")]
    DeviceNotFound {
        name: String,
        base: u64,
        root: String,
    },
}

impl From<UioError> for UioDeviceError {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

// ── UioDevice ─────────────────────────────────────────────────────────────────

struct Inner {
    /// Doorbell IRQ services run on every M→A IRQ. Each masks its asserted
    /// lines (if applicable) and wakes the doorbell's pending `RecvFut`s /
    /// `SendFut`s. Populated through [`UioDevice::bind_service`] /
    /// [`UioDevice::bind_notifier`].
    services: Services,
}

// SAFETY: region_base points into a DMA-coherent mmap that outlives `Inner`.
// Concurrent access is synchronised by the slot protocol and irq_notify.
unsafe impl Send for Inner {}
unsafe impl Sync for Inner {}

/// Entry point for the consortium UIO device (`/dev/uioN`).
///
/// Opened with [`open`](Self::open), which maps the device's windows and
/// optionally starts the IRQ listener task. Callers then map individual windows
/// with [`mmap`](Self::mmap) or [`mmap_zeroed`](Self::mmap_zeroed) and bind
/// doorbells with [`bind_notifier`](Self::bind_notifier) or
/// [`bind_service`](Self::bind_service).
pub struct UioDevice {
    inner: Arc<Inner>,
    /// Owns the mmap'd resources; also used to map the
    /// doorbell register block.
    ///
    /// [`map_peripheral`]: Self::map_peripheral
    uio: uio::UioDevice,
    /// Background IRQ listener. Aborted on drop so the task does not outlive
    /// the device (and the mmap it wakes waiters against).
    irq_task: Option<tokio::task::JoinHandle<()>>,
}

impl Drop for UioDevice {
    fn drop(&mut self) {
        if let Some(irq_task) = &self.irq_task {
            irq_task.abort();
        }
    }
}

impl UioDevice {
    /// Open the configured UIO device, mmap `map0`, parse the descriptor, and
    /// optionally start the IRQ listener task.
    ///
    /// The device index comes from the `CONSORTIUM_IPC_UIO`
    /// value injected from the `[ipc]` section of `Consortium.toml` — so it is
    /// not passed by the caller. Building this crate with `ipc-shm` but without
    /// that UIO value is a compile error (see the guard in `lib.rs`), so by the
    /// time `open` runs the index is always real. The mapped address comes from
    /// mmap, not from `CONSORTIUM_IPC_BASE`.
    ///
    /// Set `enable_irq` only for UIO devices backed by an interrupt source.
    /// Memory-only UIO devices may not support the UIO IRQ read/write protocol.
    pub fn open(uio_id: usize, enable_irq: bool) -> Result<Self, UioDeviceError> {
        consortium_log::trace!(
            "UioDevice::open(uio_id = {}, enable_irq = {})",
            uio_id,
            enable_irq
        );
        let uio = uio::UioDevice::blocking_new(uio_id)?;

        consortium_log::trace!(
            "UioDevice::open: mmap'd descriptor at {:p}, size = {:#x}",
            uio.map_mapping(0)?,
            uio.map_size(0)?
        );

        let services: Services = Arc::new(Mutex::new(Vec::new()));

        let irq_task = if enable_irq {
            consortium_log::trace!("UioDevice::open: spawning IRQ listener task");

            // Open the device fd a second time for async IRQ listening so we do
            // not disturb the mmap held by `uio`. It must be read-write: the UIO
            // re-arm protocol *writes* a u32 to the fd to unmask the interrupt
            // (see `rearm_irq`), and the VFS rejects a write to an O_RDONLY fd
            // with EBADF before it reaches the driver — which tears the listener
            // down after the very first (kernel-armed) interrupt.
            let irq_file = File::options()
                .read(true)
                .write(true)
                .open(format!("/dev/uio{uio_id}"))?;
            let async_fd = AsyncFd::new(irq_file)?;

            consortium_log::trace!("UioDevice::open: starting irq_loop");

            let irq_services = Arc::clone(&services);
            Some(tokio::spawn(async move {
                irq_loop(async_fd, irq_services).await
            }))
        } else {
            consortium_log::trace!("UioDevice::open: IRQ listener disabled");
            None
        };

        consortium_log::trace!("UioDevice::open: returning UioDevice handle");

        Ok(Self {
            inner: Arc::new(Inner { services }),
            uio,
            irq_task,
        })
    }

    /// Open the UIO device named `name`, then proceed as [`open`](Self::open).
    ///
    /// `uioN` numbers follow the kernel's probe order, which is not stable
    /// across device trees, so generated code resolves each device by name
    /// instead; see [`find`] for the matching rules. `base` is the physical
    /// address of the device's first window, used only as a fallback.
    pub fn open_named(name: &str, base: u64, enable_irq: bool) -> Result<Self, UioDeviceError> {
        let uio_id = find(Path::new(SYSFS_UIO), name, base)?;
        consortium_log::trace!("UioDevice::open_named: {} resolved to uio{}", name, uio_id);
        Self::open(uio_id, enable_irq)
    }

    /// Map a UIO memory mapping by index and return its virtual base address.
    ///
    /// Map ordering is fixed by `consortium-cfg`: a doorbell's register window is
    /// always `mmap_id` 0, and a shared region's index is its position in
    /// channel-declaration order.
    ///
    /// # Safety (caller obligation)
    ///
    /// The returned address is only valid while this `UioDevice` is alive (it
    /// owns the mapping). The caller must treat it as MMIO: only the doorbell's
    /// typed volatile register accesses may dereference it.
    ///
    /// [`open`]: Self::open
    pub fn mmap(&self, index: usize) -> Result<usize, UioDeviceError> {
        Ok(self.uio.map_mapping(index)? as usize)
    }

    /// Map a RAM-backed UIO window by index, zero its entire length, and return
    /// its virtual base address.
    ///
    /// Shared-memory carveouts (the IPC descriptor region and the debug rings)
    /// keep their contents across a coprocessor reset, so a warm reboot can leave
    /// stale descriptor magic, slot pointers, or ring state behind. The AP owns
    /// the first touch of these windows, so it clears them here before the
    /// descriptor handshake or ring bring-up reads them.
    ///
    /// This is only valid for `reserved-memory` (RAM) windows. Never call it on a
    /// register-backed window (a doorbell block): writing zeros to it would clear
    /// live MMIO registers. Use [`mmap`](Self::mmap) for those.
    ///
    /// [`mmap`]: Self::mmap
    pub fn mmap_zeroed(&self, index: usize) -> Result<usize, UioDeviceError> {
        let base = self.uio.map_mapping(index)? as usize;
        let len = self.uio.map_size(index)?;
        // The kernel maps UIO memory noncached (ARM Device-nGnRnE). A bulk
        // `memset`/`write_bytes` is *not* usable here: glibc's AArch64 `memset`
        // uses `DC ZVA` and unaligned SIMD stores, both illegal on Device memory,
        // which fault as SIGBUS. Zero with aligned `write_volatile` word stores,
        // the same access discipline the descriptor/slot protocol uses. UIO
        // windows are page-aligned, so the `usize` steps stay naturally aligned;
        // a trailing sub-word remainder (only possible for a non-word-multiple
        // length) is cleared byte-by-byte.
        let step = core::mem::size_of::<usize>();
        let words = len / step;
        // SAFETY: `base`/`len` describe the freshly mapped UIO RAM window this
        // device owns; before the coprocessor boots the AP is its sole writer, so
        // these in-bounds volatile stores have no aliasing hazard.
        unsafe {
            let word_ptr = base as *mut usize;
            for i in 0..words {
                word_ptr.add(i).write_volatile(0);
            }
            let byte_ptr = base as *mut u8;
            for i in (words * step)..len {
                byte_ptr.add(i).write_volatile(0);
            }
        }
        Ok(base)
    }

    /// Register a doorbell's IRQ service so the IRQ task runs it on every
    /// M→A interrupt.
    ///
    /// A masking doorbell binds its own IRQ entry point here — e.g.
    /// `BellA::service_interrupt` (IPCC) or
    /// [`MuDoorbell::notify_gi_interrupt`] — which masks the asserted lines
    /// before waking, so a level-sensitive line does not immediately re-fire the
    /// UIO IRQ. The doorbell is `Clone`, so bind a clone before moving the
    /// original into a transport:
    ///
    /// ```ignore
    /// let mu = unsafe { MuDoorbell::<Primary, 7>::new(dev.mmap(1)?) };
    /// let db = mu.clone();
    /// dev.bind_service(::std::sync::Arc::new(move || db.notify_gi_interrupt()));
    /// let transport = SharedMemoryTransport::<Primary, _, _>::new(mu, desc, slot_pair);
    /// ```
    ///
    /// [`MuDoorbell::notify_gi_interrupt`]: https://docs.rs/consortium-ipc-doorbell-mu
    pub fn bind_service(&self, service: Service) {
        consortium_log::trace!("UioDevice::bind_service: registering IRQ service");
        self.inner
            .services
            .lock()
            .expect("service list poisoned")
            .push(service);
    }

    /// Register a doorbell's [`Notify`] so the IRQ task wakes its `wait()`
    /// futures on every M→A interrupt.
    ///
    /// Convenience wrapper over [`bind_service`](Self::bind_service) for
    /// doorbells with no line masking (the wakeup just calls `notify_waiters()`).
    /// Masking doorbells should bind their own service instead so a
    /// level-sensitive line is masked before the wake.
    pub fn bind_notifier(&self, notifier: Arc<Notify>) {
        consortium_log::trace!("UioDevice::bind_notifier: registering notifier");
        self.bind_service(Arc::new(move || notifier.notify_waiters()));
    }
}

// ── Lookup ────────────────────────────────────────────────────────────────────

/// Where the kernel lists UIO devices.
const SYSFS_UIO: &str = "/sys/class/uio";

/// Finds the `uioN` number of the device named `name` under `root` (normally
/// [`SYSFS_UIO`]).
///
/// A device's sysfs `name` is its node's `linux,uio-name` property when set —
/// `consortium-cfg` sets it to exactly `name` — and otherwise the full node
/// name, `name@<unit-address>`. Rules, first hit wins:
///
/// 1. `name` exactly.
/// 2. `name@…`, for a device tree rendered without `linux,uio-name`.
/// 3. No name matched: the device whose map 0 starts at `base`, for a
///    hand-written node under another name. A physical window has exactly one
///    owner, so the address is unambiguous even where the name is not.
fn find(root: &Path, name: &str, base: u64) -> Result<usize, UioDeviceError> {
    let not_found = || UioDeviceError::DeviceNotFound {
        name: name.to_string(),
        base,
        root: root.display().to_string(),
    };
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(not_found()),
        Err(e) => return Err(e.into()),
    };

    let mut devices = Vec::new();
    for entry in entries {
        let entry = entry?;
        let Some(id) = entry
            .file_name()
            .to_str()
            .and_then(|file| file.strip_prefix("uio"))
            .and_then(|id| id.parse::<usize>().ok())
        else {
            continue;
        };
        let path = entry.path();
        let device_name = fs::read_to_string(path.join("name"))?.trim().to_string();
        let map0 = fs::read_to_string(path.join("maps/map0/addr"))
            .ok()
            .and_then(|addr| u64::from_str_radix(addr.trim().trim_start_matches("0x"), 16).ok());
        devices.push((id, device_name, map0));
    }
    devices.sort_by_key(|(id, _, _)| *id);

    let by_unit = |device: &str| device.split_once('@').is_some_and(|(node, _)| node == name);
    devices
        .iter()
        .find(|(_, device, _)| device == name)
        .or_else(|| devices.iter().find(|(_, device, _)| by_unit(device)))
        .or_else(|| devices.iter().find(|(_, _, map0)| *map0 == Some(base)))
        .map(|(id, _, _)| *id)
        .ok_or_else(not_found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway `/sys/class/uio` with one `uioN/{name,maps/map0/addr}` per
    /// entry, removed on drop.
    struct FakeSysfs(PathBuf);

    impl FakeSysfs {
        fn new(test: &str, devices: &[(usize, &str, u64)]) -> Self {
            let root =
                std::env::temp_dir().join(format!("consortium-uio-{test}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            for (id, name, addr) in devices {
                let dir = root.join(format!("uio{id}"));
                fs::create_dir_all(dir.join("maps/map0")).unwrap();
                fs::write(dir.join("name"), format!("{name}\n")).unwrap();
                fs::write(dir.join("maps/map0/addr"), format!("{addr:#018x}\n")).unwrap();
            }
            Self(root)
        }
    }

    impl Drop for FakeSysfs {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn exact_name_wins_regardless_of_probe_order() {
        let sysfs = FakeSysfs::new(
            "exact",
            &[
                (0, "mu1", 0x4422_0000),
                (1, "ipc-shm", 0x2048_0000),
                (2, "dbg-shm", 0x2048_4000),
            ],
        );
        assert_eq!(find(&sysfs.0, "ipc-shm", 0x2048_0000).unwrap(), 1);
        assert_eq!(find(&sysfs.0, "dbg-shm", 0x2048_4000).unwrap(), 2);
        assert_eq!(find(&sysfs.0, "mu1", 0x4422_0000).unwrap(), 0);
    }

    #[test]
    fn node_name_with_unit_address_matches() {
        let sysfs = FakeSysfs::new(
            "unit",
            &[
                (0, "mu1@44220000", 0x4422_0000),
                (1, "mu10@44000000", 0x4400_0000),
            ],
        );
        assert_eq!(find(&sysfs.0, "mu1", 0x4422_0000).unwrap(), 0);
    }

    #[test]
    fn falls_back_to_the_window_address() {
        let sysfs = FakeSysfs::new(
            "addr",
            &[
                (0, "mu1@44220000", 0x4422_0000),
                (1, "ipc-shm-uio@20480000", 0x2048_0000),
                (2, "dbg-shm-uio@20484000", 0x2048_4000),
            ],
        );
        assert_eq!(find(&sysfs.0, "ipc-shm", 0x2048_0000).unwrap(), 1);
        assert_eq!(find(&sysfs.0, "dbg-shm", 0x2048_4000).unwrap(), 2);
    }

    #[test]
    fn a_name_match_beats_an_address_match() {
        let sysfs = FakeSysfs::new(
            "precedence",
            &[(0, "other", 0x2048_0000), (1, "ipc-shm", 0x2050_0000)],
        );
        assert_eq!(find(&sysfs.0, "ipc-shm", 0x2048_0000).unwrap(), 1);
    }

    #[test]
    fn missing_device_is_an_error() {
        let sysfs = FakeSysfs::new("missing", &[(0, "mu1", 0x4422_0000)]);
        assert!(matches!(
            find(&sysfs.0, "ipc-shm", 0x2048_0000),
            Err(UioDeviceError::DeviceNotFound { .. })
        ));
    }
}

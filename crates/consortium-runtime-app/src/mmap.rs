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

//! A-core descriptor polling.
//!
//! The A-core waits for the descriptor state to reach `Ready(3)` before opening the UIO device.
//! Descriptor types and constants are re-exported from `consortium-ipc-transport-memory`.

use consortium_ipc_transport_memory::{DescState, config};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum WaitError {
    Timeout,
    Io(std::io::ErrorKind),
}

impl std::fmt::Display for WaitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "descriptor wait timed out"),
            Self::Io(kind) => write!(f, "descriptor wait failed: {kind:?}"),
        }
    }
}

impl std::error::Error for WaitError {}

impl From<std::io::Error> for WaitError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.kind())
    }
}

/// Guard that holds the mmap'd descriptor region.
///
/// Keeps the mmap alive and provides access to the descriptor state field.
struct DescriptorMmap {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: The mmap'd region is DMA-coherent and thread-safe (read-only volatile accesses).
unsafe impl Send for DescriptorMmap {}
unsafe impl Sync for DescriptorMmap {}

impl Drop for DescriptorMmap {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                libc::munmap(self.ptr as *mut libc::c_void, self.len);
            }
        }
    }
}

impl DescriptorMmap {
    /// Open `/dev/uioN` resource 0 and mmap it.
    fn open(uio_num: usize) -> Result<Arc<Self>, WaitError> {
        use std::os::fd::AsRawFd;

        // Open the UIO device
        let path = format!("/dev/uio{}", uio_num);
        let file = std::fs::File::open(&path)?;
        let fd = file.as_raw_fd();

        // Query the resource size from sysfs
        let sysfs_path = format!("/sys/class/uio/uio{}/maps/map0/size", uio_num);
        let size_str = std::fs::read_to_string(&sysfs_path)?;
        let size: usize = size_str
            .trim()
            .parse()
            .map_err(|_| WaitError::Io(std::io::ErrorKind::InvalidData))?;

        // Mmap the resource
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ,
                libc::MAP_SHARED,
                fd,
                0,
            ) as *mut u8
        };

        if ptr.is_null() {
            return Err(WaitError::Io(std::io::ErrorKind::Other));
        }

        // Ensure the fd is closed (file goes out of scope) but mmap stays alive
        std::mem::forget(file);

        Ok(Arc::new(Self { ptr, len: size }))
    }

    /// Read the descriptor state at offset 0x14.
    fn get_state(&self) -> DescState {
        unsafe {
            let ptr = self.ptr.add(0x14) as *const u32;
            let val = ptr.read_volatile();
            match val {
                0 => DescState::KernelInit,
                1 => DescState::Proposed,
                2 => DescState::MReady,
                3 => DescState::Ready,
                4 => DescState::Suspended,
                _ => DescState::Error,
            }
        }
    }
}

/// Poll the descriptor state until it reaches `Ready(3)`.
///
/// Opens the configured UIO device's `map0` (the DMA-coherent region), mmaps
/// it, and polls the state field at offset 0x14 at regular intervals until
/// `Ready(3)` is seen, a timeout occurs, or an error state is encountered.
///
/// The device index comes from [`config::UIO`] (the `CONSORTIUM_IPC_UIO` value
/// injected from `Consortium.toml`), matching [`UioDevice::open`].
///
/// # Arguments
///
/// * `timeout` - maximum time to wait; uses `Duration::from_secs(30)` as a reasonable default
///
/// # Returns
///
/// - `Ok(())` if state reached `Ready(3)`
/// - `Err(WaitError::Timeout)` if timeout elapsed
/// - `Err(WaitError::Io(kind))` if the UIO device or sysfs cannot be accessed
///
/// # Example
///
/// ```ignore
/// wait_for_ready(Duration::from_secs(10)).await?;
/// let dev = UioDevice::open(num_channels, transfer_size)?;
/// ```
///
/// [`UioDevice::open`]: crate::uio::UioDevice::open
pub async fn wait_for_ready(timeout: Duration) -> Result<(), WaitError> {
    let mmap = DescriptorMmap::open(config::UIO as usize)?;
    let start = Instant::now();
    let poll_interval = Duration::from_millis(50);

    loop {
        let state = mmap.get_state();
        match state {
            DescState::Ready => return Ok(()),
            DescState::Error | DescState::Suspended => {
                // M-core signaled an error; stop waiting and let caller handle it
                return Err(WaitError::Io(std::io::ErrorKind::Other));
            }
            _ => {
                if start.elapsed() > timeout {
                    return Err(WaitError::Timeout);
                }
                tokio::time::sleep(poll_interval).await;
            }
        }
    }
}

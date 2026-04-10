//! A-core descriptor polling.
//!
//! The A-core waits for the descriptor state to reach `Ready(3)` before opening the UIO device.
//! Descriptor types and constants are re-exported from `consortium-ipc-transport-uio`.

use crate::ipc_uio;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub use ipc_uio::{DescState, Descriptor, DescriptorError};

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
/// Opens `/dev/uioN` resource 0 (the DMA-coherent region), mmaps it,
/// and polls the state field at offset 0x14 at regular intervals until
/// `Ready(3)` is seen, a timeout occurs, or an error state is encountered.
///
/// # Arguments
///
/// * `uio_num` — the UIO device number (N in `/dev/uioN`)
/// * `timeout` — maximum time to wait; uses `Duration::from_secs(30)` as a reasonable default
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
/// wait_for_ready(0, Duration::from_secs(10)).await?;
/// let dev = UioDevice::open(0)?;
/// ```
pub async fn wait_for_ready(uio_num: usize, timeout: Duration) -> Result<(), WaitError> {
    let mmap = DescriptorMmap::open(uio_num)?;
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

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

//! Booting and controlling a coprocessor through Linux remoteproc.
//!
//! Linux exposes each coprocessor as `/sys/class/remoteproc/remoteprocN`, and this
//! module is a thin, typed wrapper over that sysfs interface: write the firmware
//! filename, write `start`, read back the state. There is no ioctl and no library —
//! remoteproc's control surface really is a handful of files.
//!
//! ```rust,ignore
//! use consortium_runtime_app::remoteproc::RemoteProc;
//!
//! let rproc = RemoteProc::new(0);
//! rproc.set_firmware("consortium-mcu.elf")?;
//! rproc.start()?;
//! ```
//!
//! # Where this sits in bring-up
//!
//! Starting the coprocessor is what makes the *other* side of every Consortium link
//! exist, so it comes before IPC bring-up. It also does not have to be raced: the
//! shared-memory transport's descriptor magic is a readiness gate, so a secondary that
//! boots late finds the descriptor already published rather than missing a doorbell.
//!
//! The firmware the coprocessor loads is the one `csti build` post-processed —
//! remoteproc expects the vector table in a section named `.isr_vectors`, so the
//! builder renames `.vector_table` for it.
//!
//! # Considerations
//!
//! - **Requires privilege.** These are root-owned sysfs files; a non-root process gets
//!   a permission error from `set_firmware` and `start`.
//! - **`set_firmware` takes a filename, not a path.** The kernel resolves it relative
//!   to its own firmware search path (typically `/lib/firmware`).
//! - **State is read, not watched.** [`RemoteProc::state`] is a point-in-time read;
//!   there is no notification when a coprocessor crashes. Pair it with the debug ring's
//!   panic flag (`consortium_dbg`'s `Flags::Panic`) to distinguish a crash from a quiet
//!   coprocessor.
//! - **The `From` conversions panic on unrecognized input.** They are written against
//!   the kernel's fixed vocabulary; a state string this module does not know is treated
//!   as a bug rather than a recoverable error.

use std::fmt::Display;
use std::io::Result;
use std::path::PathBuf;
use std::time::Duration;

/// A handle to one `/sys/class/remoteproc/remoteprocN` directory.
///
/// Construct with [`RemoteProc::new`], which takes the zero-based index.
#[derive(Debug)]
pub struct RemoteProc {
    /// The sysfs directory this handle reads and writes.
    pub path: PathBuf,
}

/// The coprocessor's lifecycle state, as reported by remoteproc's `state` file.
///
/// The discriminants match the kernel's ordering, and `Display`/`From<String>` use the
/// exact strings sysfs reads and writes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteProcState {
    /// Not running; firmware may be set and `start` written.
    Offline = 0,
    /// Suspended by runtime power management.
    Suspended = 1,
    /// Firmware loaded and executing.
    Running = 2,
    /// The coprocessor faulted. Recovery behaviour depends on
    /// [`set_recovery`](RemoteProc::set_recovery).
    Crashed = 3,
    /// The remoteproc instance has been removed.
    Deleted = 4,
    /// Linux attached to a coprocessor that was already running (booted by a
    /// bootloader or by the secure world rather than by Linux).
    Attached = 5,
    /// Detached: still running, but no longer under Linux's control.
    Detached = 6,
}

impl From<u8> for RemoteProcState {
    fn from(value: u8) -> Self {
        match value {
            0 => RemoteProcState::Offline,
            1 => RemoteProcState::Suspended,
            2 => RemoteProcState::Running,
            3 => RemoteProcState::Crashed,
            4 => RemoteProcState::Deleted,
            5 => RemoteProcState::Attached,
            6 => RemoteProcState::Detached,
            _ => panic!("Invalid value for RemoteProcState: {}", value),
        }
    }
}

impl From<String> for RemoteProcState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "offline" => RemoteProcState::Offline,
            "suspended" => RemoteProcState::Suspended,
            "running" => RemoteProcState::Running,
            "crashed" => RemoteProcState::Crashed,
            "deleted" => RemoteProcState::Deleted,
            "attached" => RemoteProcState::Attached,
            "detached" => RemoteProcState::Detached,
            _ => panic!("Invalid value for RemoteProcState: {}", value),
        }
    }
}

impl Display for RemoteProcState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteProcState::Offline => write!(f, "offline"),
            RemoteProcState::Suspended => write!(f, "suspended"),
            RemoteProcState::Running => write!(f, "running"),
            RemoteProcState::Crashed => write!(f, "crashed"),
            RemoteProcState::Deleted => write!(f, "deleted"),
            RemoteProcState::Attached => write!(f, "attached"),
            RemoteProcState::Detached => write!(f, "detached"),
        }
    }
}

/// What remoteproc does with coprocessor memory on a crash.
///
/// Set through [`RemoteProc::set_coredump`]; matches the `coredump` sysfs vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteProcCoredump {
    /// Collect a coredump on crash, copying memory into a userspace-readable buffer.
    Enabled,
    /// Collect nothing.
    Disabled,
    /// Collect in place, without copying — memory is read directly from the
    /// coprocessor's regions, so recovery waits for the dump to be consumed.
    Inline,
}

impl From<String> for RemoteProcCoredump {
    fn from(value: String) -> Self {
        match value.as_str() {
            "enabled" => RemoteProcCoredump::Enabled,
            "disabled" => RemoteProcCoredump::Disabled,
            "inline" => RemoteProcCoredump::Inline,
            _ => panic!("Invalid value for RemoteProcCoredump: {}", value),
        }
    }
}

impl Display for RemoteProcCoredump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoteProcCoredump::Disabled => write!(f, "disabled"),
            RemoteProcCoredump::Enabled => write!(f, "enabled"),
            RemoteProcCoredump::Inline => write!(f, "inline"),
        }
    }
}

impl RemoteProc {
    /// Constructs a `RemoteProc` handle for the given zero-based remoteproc index.
    pub fn new(index: usize) -> Self {
        let path = PathBuf::from(format!("/sys/class/remoteproc/remoteproc{}", index));
        Self { path }
    }

    /// Reads the current state of the remote processor from sysfs.
    pub fn state(&self) -> Result<RemoteProcState> {
        let state_path = self.path.join("state");
        let state_str = std::fs::read_to_string(state_path)?.trim().to_string();
        Ok(RemoteProcState::from(state_str))
    }

    /// Starts the remote processor by writing `"start"` to the sysfs state file.
    pub fn start(&self) -> Result<()> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "start")?;
        Ok(())
    }

    /// Stops the remote processor by writing `"stop"` to the sysfs state file.
    pub fn stop(&self) -> Result<()> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "stop")?;
        Ok(())
    }

    /// Sets the firmware image name for this remote processor via sysfs.
    pub fn set_firmware(&self, firmware: &str) -> Result<()> {
        let firmware_path = self.path.join("firmware");
        std::fs::write(firmware_path, firmware)?;
        Ok(())
    }

    /// Sets the coredump mode for this remote processor via sysfs.
    pub fn set_coredump(&self, coredump: RemoteProcCoredump) -> Result<()> {
        let coredump_path = self.path.join("coredump");
        std::fs::write(coredump_path, coredump.to_string())?;
        Ok(())
    }

    /// Enables or disables automatic recovery for this remote processor via sysfs.
    pub fn set_recovery(&self, recovery: bool) -> Result<()> {
        let recovery_path = self.path.join("recovery");
        std::fs::write(recovery_path, if recovery { "enabled" } else { "disabled" })?;
        Ok(())
    }

    /// Wait for the remoteproc to reach a specific state, with timeout.
    ///
    /// Polls the state file at regular intervals until the target state is reached or timeout occurs.
    ///
    /// # Arguments
    ///
    /// * `target` - the desired state
    /// * `timeout` - maximum time to wait
    ///
    /// # Errors
    ///
    /// Returns an I/O error if reading the state file fails, or a timeout error if the state
    /// is not reached within the timeout duration.
    pub async fn wait_state(&self, target: RemoteProcState, timeout: Duration) -> Result<()> {
        let start = std::time::Instant::now();
        let interval = Duration::from_millis(100);

        loop {
            match self.state() {
                Ok(state) if state == target => return Ok(()),
                Ok(_) => {
                    if start.elapsed() > timeout {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            format!(
                                "remoteproc did not reach state {target} within {:?}",
                                timeout
                            ),
                        ));
                    }
                    tokio::time::sleep(interval).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

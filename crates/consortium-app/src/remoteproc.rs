use std::fmt;
use std::io;
use std::path::PathBuf;

/// Errors produced by `RemoteProc` operations.
#[derive(Debug)]
pub enum RemoteProcError {
    /// The sysfs file could not be read or written.
    Io(io::Error),
    /// The state string read from sysfs was not recognised.
    UnknownState(String),
    /// The coredump string read from sysfs was not recognised.
    UnknownCoredump(String),
}

impl fmt::Display for RemoteProcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::UnknownState(s) => write!(f, "unknown remoteproc state: {s:?}"),
            Self::UnknownCoredump(s) => write!(f, "unknown coredump mode: {s:?}"),
        }
    }
}

impl std::error::Error for RemoteProcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for RemoteProcError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub struct RemoteProc {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteProcState {
    Offline = 0,
    Suspended = 1,
    Running = 2,
    Crashed = 3,
    Deleted = 4,
    Attached = 5,
    Detached = 6,
}

impl TryFrom<u8> for RemoteProcState {
    type Error = RemoteProcError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RemoteProcState::Offline),
            1 => Ok(RemoteProcState::Suspended),
            2 => Ok(RemoteProcState::Running),
            3 => Ok(RemoteProcState::Crashed),
            4 => Ok(RemoteProcState::Deleted),
            5 => Ok(RemoteProcState::Attached),
            6 => Ok(RemoteProcState::Detached),
            _ => Err(RemoteProcError::UnknownState(value.to_string())),
        }
    }
}

impl TryFrom<String> for RemoteProcState {
    type Error = RemoteProcError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "offline" => Ok(RemoteProcState::Offline),
            "suspended" => Ok(RemoteProcState::Suspended),
            "running" => Ok(RemoteProcState::Running),
            "crashed" => Ok(RemoteProcState::Crashed),
            "deleted" => Ok(RemoteProcState::Deleted),
            "attached" => Ok(RemoteProcState::Attached),
            "detached" => Ok(RemoteProcState::Detached),
            _ => Err(RemoteProcError::UnknownState(value)),
        }
    }
}

impl fmt::Display for RemoteProcState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RemoteProcState {
    /// Returns the sysfs string representation of the state.
    pub fn as_str(&self) -> &'static str {
        match self {
            RemoteProcState::Offline => "offline",
            RemoteProcState::Suspended => "suspended",
            RemoteProcState::Running => "running",
            RemoteProcState::Crashed => "crashed",
            RemoteProcState::Deleted => "deleted",
            RemoteProcState::Attached => "attached",
            RemoteProcState::Detached => "detached",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteProcCoredump {
    Enabled,
    Disabled,
    Inline,
}

impl TryFrom<String> for RemoteProcCoredump {
    type Error = RemoteProcError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "enabled" => Ok(RemoteProcCoredump::Enabled),
            "disabled" => Ok(RemoteProcCoredump::Disabled),
            "inline" => Ok(RemoteProcCoredump::Inline),
            _ => Err(RemoteProcError::UnknownCoredump(value)),
        }
    }
}

impl fmt::Display for RemoteProcCoredump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RemoteProcCoredump {
    /// Returns the sysfs string representation of the coredump mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            RemoteProcCoredump::Enabled => "enabled",
            RemoteProcCoredump::Disabled => "disabled",
            RemoteProcCoredump::Inline => "inline",
        }
    }
}

impl RemoteProc {
    pub fn new(index: usize) -> Self {
        let path = PathBuf::from(format!("/sys/class/remoteproc/remoteproc{}", index));
        Self { path }
    }

    pub fn state(&self) -> Result<RemoteProcState, RemoteProcError> {
        let state_path = self.path.join("state");
        let state_str = std::fs::read_to_string(state_path)?.trim().to_string();
        RemoteProcState::try_from(state_str)
    }

    pub fn start(&self) -> Result<(), RemoteProcError> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "start")?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), RemoteProcError> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "stop")?;
        Ok(())
    }

    pub fn set_firmware(&self, firmware: &str) -> Result<(), RemoteProcError> {
        let firmware_path = self.path.join("firmware");
        std::fs::write(firmware_path, firmware)?;
        Ok(())
    }

    pub fn set_coredump(&self, coredump: RemoteProcCoredump) -> Result<(), RemoteProcError> {
        let coredump_path = self.path.join("coredump");
        std::fs::write(coredump_path, coredump.as_str())?;
        Ok(())
    }

    pub fn set_recovery(&self, recovery: bool) -> Result<(), RemoteProcError> {
        let recovery_path = self.path.join("recovery");
        std::fs::write(recovery_path, if recovery { "enabled" } else { "disabled" })?;
        Ok(())
    }
}

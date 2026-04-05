use std::io::Result;
use std::path::PathBuf;

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

impl ToString for RemoteProcState {
    fn to_string(&self) -> String {
        match self {
            RemoteProcState::Offline => "offline".to_string(),
            RemoteProcState::Suspended => "suspended".to_string(),
            RemoteProcState::Running => "running".to_string(),
            RemoteProcState::Crashed => "crashed".to_string(),
            RemoteProcState::Deleted => "deleted".to_string(),
            RemoteProcState::Attached => "attached".to_string(),
            RemoteProcState::Detached => "detached".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoteProcCoredump {
    Enabled,
    Disabled,
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

impl ToString for RemoteProcCoredump {
    fn to_string(&self) -> String {
        match self {
            RemoteProcCoredump::Enabled => "enabled".to_string(),
            RemoteProcCoredump::Disabled => "disabled".to_string(),
            RemoteProcCoredump::Inline => "inline".to_string(),
        }
    }
}

impl RemoteProc {
    pub fn new(index: usize) -> Self {
        let path = PathBuf::from(format!("/sys/class/remoteproc/remoteproc{}", index));
        Self { path }
    }

    pub fn state(&self) -> Result<RemoteProcState> {
        let state_path = self.path.join("state");
        let state_str = std::fs::read_to_string(state_path)?.trim().to_string();
        Ok(RemoteProcState::from(state_str))
    }

    pub fn start(&self) -> Result<()> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "start")?;
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        let state_path = self.path.join("state");
        std::fs::write(state_path, "stop")?;
        Ok(())
    }

    pub fn set_firmware(&self, firmware: &str) -> Result<()> {
        let firmware_path = self.path.join("firmware");
        std::fs::write(firmware_path, firmware)?;
        Ok(())
    }

    pub fn set_coredump(&self, coredump: RemoteProcCoredump) -> Result<()> {
        let coredump_path = self.path.join("coredump");
        std::fs::write(coredump_path, coredump.to_string())?;
        Ok(())
    }

    pub fn set_recovery(&self, recovery: bool) -> Result<()> {
        let recovery_path = self.path.join("recovery");
        std::fs::write(recovery_path, if recovery { "enabled" } else { "disabled" })?;
        Ok(())
    }
}

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

//! Renders a systemd service unit for the Linux application daemon.
//!
//! Remoteproc bring-up happens from inside the application itself (via sysfs
//! writes), not a separate unit, so there is no `Requires=`/`After=` remoteproc
//! convention to honour: the default ordering is just `After=network.target`.
//! Callers that need richer ordering can extend the rendered text; one unit for
//! the app daemon is sufficient for this pass.

use std::path::{Path, PathBuf};

/// How systemd should restart the daemon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemdRestartPolicy {
    /// Never restart automatically.
    No,
    /// Restart only on non-clean exit (the default).
    #[default]
    OnFailure,
    /// Always restart.
    Always,
}

impl SystemdRestartPolicy {
    /// The `Restart=` value for this policy.
    fn as_str(self) -> &'static str {
        match self {
            Self::No => "no",
            Self::OnFailure => "on-failure",
            Self::Always => "always",
        }
    }
}

/// Parameters for the application's systemd service unit.
#[derive(Debug, Clone)]
pub struct SystemdUnitConfig {
    /// Unit name without the `.service` suffix (e.g. `consortium-app`).
    pub unit_name: String,
    /// `ExecStart=` command (typically `{deploy_root}/app/<binary>`).
    pub exec_start: String,
    /// `WorkingDirectory=` (typically `{deploy_root}/app`).
    pub working_directory: String,
    /// Human-readable `Description=`; a default is used when absent.
    pub description: Option<String>,
    /// Restart policy.
    pub restart: SystemdRestartPolicy,
}

impl Default for SystemdUnitConfig {
    fn default() -> Self {
        Self {
            unit_name: "consortium-app".to_string(),
            exec_start: String::new(),
            working_directory: String::new(),
            description: None,
            restart: SystemdRestartPolicy::default(),
        }
    }
}

impl SystemdUnitConfig {
    /// The file name this unit is written as (`<unit_name>.service`).
    pub fn file_name(&self) -> String {
        format!("{}.service", self.unit_name)
    }
}

/// Renders the `.service` unit text for `config`.
pub fn render_unit(config: &SystemdUnitConfig) -> String {
    let description = config
        .description
        .clone()
        .unwrap_or_else(|| "Consortium application daemon".to_string());

    format!(
        "[Unit]\n\
         Description={description}\n\
         After=network.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         ExecStart={exec_start}\n\
         WorkingDirectory={working_directory}\n\
         Restart={restart}\n\
         \n\
         [Install]\n\
         WantedBy=multi-user.target\n",
        exec_start = config.exec_start,
        working_directory = config.working_directory,
        restart = config.restart.as_str(),
    )
}

/// Renders `config` and writes it to `dir/<unit_name>.service`, returning the
/// written path.
pub async fn write_unit(dir: &Path, config: &SystemdUnitConfig) -> std::io::Result<PathBuf> {
    tokio::fs::create_dir_all(dir).await?;
    let path = dir.join(config.file_name());
    tokio::fs::write(&path, render_unit(config)).await?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> SystemdUnitConfig {
        SystemdUnitConfig {
            unit_name: "consortium-app".to_string(),
            exec_start: "/opt/consortium/app/melt-pot-app".to_string(),
            working_directory: "/opt/consortium/app".to_string(),
            description: None,
            restart: SystemdRestartPolicy::OnFailure,
        }
    }

    #[test]
    fn renders_exact_unit_text() {
        let expected = "\
[Unit]
Description=Consortium application daemon
After=network.target

[Service]
Type=simple
ExecStart=/opt/consortium/app/melt-pot-app
WorkingDirectory=/opt/consortium/app
Restart=on-failure

[Install]
WantedBy=multi-user.target
";
        assert_eq!(render_unit(&config()), expected);
    }

    #[test]
    fn honours_description_and_restart_overrides() {
        let mut cfg = config();
        cfg.description = Some("Melt-pot HMI".to_string());
        cfg.restart = SystemdRestartPolicy::Always;
        let out = render_unit(&cfg);
        assert!(out.contains("Description=Melt-pot HMI\n"));
        assert!(out.contains("Restart=always\n"));
    }

    #[tokio::test]
    async fn write_unit_writes_named_file() {
        let dir = std::env::temp_dir().join(format!("consortium-systemd-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let path = write_unit(&dir, &config()).await.expect("write unit");
        assert_eq!(path, dir.join("consortium-app.service"));
        let text = std::fs::read_to_string(&path).expect("read unit");
        assert_eq!(text, render_unit(&config()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}

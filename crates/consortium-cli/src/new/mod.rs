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

//! The `new` subcommand: scaffold a Consortium project.
//!
//! Writes a `Consortium.toml`, a root Cargo workspace, and the endpoint crates —
//! the Linux application, the CM33 firmware, and the message types they share —
//! wired together over one shared-memory channel and a defmt ring, plus an OP-TEE
//! trusted application and a Cog/WebKit HMI when asked for. Every choice can come
//! from flags; `--interactive` prompts for whatever the flags leave open.

mod template;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Args;
use dialoguer::{Confirm, Input, Select};

use self::template::{Chip, Project, Source};

#[derive(Args)]
pub struct NewArgs {
    /// Directory to create the project in. It must not exist yet, or be empty.
    #[arg(required_unless_present = "interactive")]
    path: Option<PathBuf>,

    /// Package-name prefix for the generated crates. Defaults to the directory name.
    #[arg(long)]
    name: Option<String>,

    /// Target chip.
    #[arg(long, value_enum, required_unless_present = "interactive")]
    chip: Option<Chip>,

    /// Add an OP-TEE trusted application, called from the Linux application.
    #[arg(long)]
    tee: bool,

    /// Add a Cog/WebKit HMI with a Vite frontend.
    #[arg(long)]
    hmi: bool,

    /// Prompt for every choice the other flags leave open.
    #[arg(short, long)]
    interactive: bool,

    /// Depend on a local Consortium checkout by path instead of on crates.io.
    #[arg(long, value_name = "DIR")]
    consortium_path: Option<PathBuf>,
}

/// Scaffolds a project as described by `args`, prompting first if asked to.
pub fn run(args: NewArgs) -> Result<(), String> {
    let (root, project) = if args.interactive {
        prompt(args)?
    } else {
        decide(args)?
    };
    check_empty(&root)?;

    for (relative, contents) in template::render(&project) {
        let path = root.join(&relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        std::fs::write(&path, contents)
            .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    }

    println!(
        "Created `{}` for the {} in {}",
        project.name,
        project.chip.label(),
        root.display()
    );
    println!("  app:      app/ (Linux application)");
    println!("  firmware: mcu/ (Cortex-M33)");
    println!("  shared:   shared/ (IPC message types)");
    if project.tee {
        println!("  tee:      ta/ (OP-TEE trusted application)");
    }
    if project.hmi {
        println!("  hmi:      hmi/ (Vite frontend for Cog/WebKit)");
    }
    println!();
    println!("Next:");
    println!("  cd {}", root.display());
    println!("  csti fetch");
    println!("  csti check");
    println!("  csti build --linux <KERNEL_DIR>");
    Ok(())
}

/// Resolves a project from flags alone.
fn decide(args: NewArgs) -> Result<(PathBuf, Project), String> {
    let root = args.path.ok_or("a project directory is required")?;
    let chip = args.chip.ok_or("--chip is required")?;
    let name = match args.name {
        Some(name) => name,
        None => name_from_path(&root)?,
    };
    template::validate_name(&name)?;
    let project = Project {
        name,
        chip,
        tee: args.tee,
        hmi: args.hmi,
        toolchain: toolchain(args.consortium_path.as_deref()),
        source: source(args.consortium_path.as_deref())?,
        tee_uuid: random_uuid(),
    };
    Ok((root, project))
}

/// Resolves a project from flags, prompting for everything they leave open.
fn prompt(args: NewArgs) -> Result<(PathBuf, Project), String> {
    let source = source(args.consortium_path.as_deref())?;
    let root = match args.path {
        Some(path) => path,
        None => Input::<String>::new()
            .with_prompt("Project directory")
            .interact_text()
            .map(PathBuf::from)
            .map_err(prompt_error)?,
    };
    let name = match args.name {
        Some(name) => name,
        None => {
            let mut input = Input::<String>::new()
                .with_prompt("Project name")
                .validate_with(|name: &String| template::validate_name(name));
            if let Ok(default) = name_from_path(&root) {
                input = input.default(default);
            }
            input.interact_text().map_err(prompt_error)?
        }
    };
    template::validate_name(&name)?;
    let chip = match args.chip {
        Some(chip) => chip,
        None => {
            let labels: Vec<_> = Chip::ALL.iter().map(|c| c.label()).collect();
            let index = Select::new()
                .with_prompt("Chip")
                .items(&labels)
                .default(0)
                .interact()
                .map_err(prompt_error)?;
            Chip::ALL[index]
        }
    };
    let tee = args.tee || confirm("Add an OP-TEE trusted application?")?;
    let hmi = args.hmi || confirm("Add a Cog/WebKit HMI?")?;
    let project = Project {
        name,
        chip,
        tee,
        hmi,
        toolchain: toolchain(args.consortium_path.as_deref()),
        source,
        tee_uuid: random_uuid(),
    };
    Ok((root, project))
}

fn confirm(question: &str) -> Result<bool, String> {
    Confirm::new()
        .with_prompt(question)
        .default(false)
        .interact()
        .map_err(prompt_error)
}

fn prompt_error(e: dialoguer::Error) -> String {
    format!("prompt failed: {e}")
}

/// The project name implied by its directory, which must itself be a valid one.
fn name_from_path(root: &Path) -> Result<String, String> {
    let name = root
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("cannot derive a name from {}; pass --name", root.display()))?;
    template::validate_name(name).map_err(|e| format!("{e}; pass --name"))?;
    Ok(name.to_string())
}

/// crates.io at this `csti`'s own version, or a checkout that has the crates.
fn source(checkout: Option<&Path>) -> Result<Source, String> {
    let Some(checkout) = checkout else {
        return Ok(Source::Registry(env!("CARGO_PKG_VERSION").to_string()));
    };
    let root = checkout
        .canonicalize()
        .map_err(|e| format!("cannot open {}: {e}", checkout.display()))?;
    if !root.join("crates/consortium-ipc/Cargo.toml").is_file() {
        return Err(format!(
            "{} is not a Consortium checkout (no crates/consortium-ipc)",
            root.display()
        ));
    }
    Ok(Source::Path(root))
}

/// The checkout's pinned channel, or `stable` without one. Cargo compiles a path
/// dependency from its own directory, where the checkout's `rust-toolchain.toml`
/// applies, so the project must pin the same channel or mix two rustc versions.
fn toolchain(checkout: Option<&Path>) -> String {
    checkout
        .and_then(|root| std::fs::read_to_string(root.join("rust-toolchain.toml")).ok())
        .and_then(|pin| {
            pin.lines().find_map(|line| {
                let value = line
                    .trim()
                    .strip_prefix("channel")?
                    .trim()
                    .strip_prefix('=')?;
                Some(value.trim().trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "stable".to_string())
}

/// Refuses to scaffold over anything already there.
fn check_empty(root: &Path) -> Result<(), String> {
    match std::fs::read_dir(root) {
        Ok(mut entries) => match entries.next() {
            Some(_) => Err(format!(
                "{} already exists and is not empty",
                root.display()
            )),
            None => Ok(()),
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("cannot open {}: {e}", root.display())),
    }
}

/// A random (version 4) UUID for `[tee] uuid`. Uniqueness, not secrecy, is what a
/// TA UUID needs, so the standard library's randomly keyed hasher is enough.
fn random_uuid() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let mut bytes = [0u8; 16];
    for (i, chunk) in bytes.chunks_mut(8).enumerate() {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_usize(i);
        hasher.write_u128(nanos);
        chunk.copy_from_slice(&hasher.finish().to_le_bytes());
    }
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_builder::pipeline::{
        self,
        check::{self, CheckConfig},
    };

    /// A fresh scratch directory per test case.
    fn scratch(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "csti-new-{label}-{}-{}",
            std::process::id(),
            random_uuid()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write(root: &Path, project: &Project) {
        for (relative, contents) in template::render(project) {
            let path = root.join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, contents).unwrap();
        }
    }

    fn project(chip: Chip, tee: bool, hmi: bool) -> Project {
        Project {
            name: "demo-sys".to_string(),
            chip,
            tee,
            hmi,
            source: Source::Registry("0.4.0".to_string()),
            toolchain: "stable".to_string(),
            tee_uuid: random_uuid(),
        }
    }

    #[test]
    fn uuid_is_version_4() {
        let uuid = random_uuid();
        assert_eq!(uuid.len(), 36);
        assert_eq!(&uuid[14..15], "4");
        assert!(matches!(&uuid[19..20], "8" | "9" | "a" | "b"), "{uuid}");
        assert_ne!(uuid, random_uuid());
    }

    #[test]
    fn names_must_be_crate_names() {
        assert!(template::validate_name("demo").is_ok());
        assert!(template::validate_name("demo-sys_2").is_ok());
        assert!(template::validate_name("2demo").is_err());
        assert!(template::validate_name("my demo").is_err());
        assert!(template::validate_name("").is_err());
    }

    #[test]
    fn refuses_a_non_empty_directory() {
        let root = scratch("occupied");
        assert!(check_empty(&root).is_ok());
        assert!(check_empty(&root.join("missing")).is_ok());
        std::fs::write(root.join("keep"), "").unwrap();
        assert!(check_empty(&root).is_err());
    }

    #[test]
    fn path_source_points_into_the_checkout() {
        let checkout = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let Source::Path(root) = source(Some(&checkout)).unwrap() else {
            panic!("expected a path source");
        };
        let mut project = project(Chip::Mimx9352, true, true);
        project.source = Source::Path(root.clone());
        let files = template::render(&project);
        let app = &files
            .iter()
            .find(|(p, _)| p == Path::new("app/Cargo.toml"))
            .unwrap()
            .1;
        let expected = root.join("crates/consortium-hmi-webkit");
        assert!(app.contains(&expected.display().to_string()), "{app}");
        assert!(source(Some(&checkout.join("crates"))).is_err());

        let pinned = toolchain(Some(&checkout));
        assert_ne!(pinned, "stable");
        assert!(!pinned.contains('"'), "{pinned}");
        assert_eq!(toolchain(None), "stable");
    }

    #[test]
    fn every_toml_file_parses() {
        for chip in Chip::ALL {
            for (path, contents) in template::render(&project(chip, true, true)) {
                if path.extension().is_some_and(|e| e == "toml") {
                    toml::from_str::<toml::Table>(&contents)
                        .unwrap_or_else(|e| panic!("{}: {e}\n{contents}", path.display()));
                }
            }
        }
    }

    #[test]
    fn options_add_only_their_own_files() {
        let paths = |tee, hmi| -> Vec<PathBuf> {
            template::render(&project(Chip::Stm32mp257f, tee, hmi))
                .into_iter()
                .map(|(p, _)| p)
                .collect()
        };
        let bare = paths(false, false);
        assert!(
            !bare
                .iter()
                .any(|p| p.starts_with("ta") || p.starts_with("hmi"))
        );
        assert!(paths(true, false).contains(&PathBuf::from("ta/src/lib.rs")));
        assert!(paths(false, true).contains(&PathBuf::from("hmi/package.json")));
    }

    /// Every combination must load as a session and plan a check, which is
    /// everything `csti check` does short of running cargo.
    #[tokio::test]
    async fn every_combination_plans_a_check() {
        for chip in Chip::ALL {
            for (tee, hmi) in [(false, false), (true, false), (false, true), (true, true)] {
                let root = scratch("plan");
                write(&root, &project(chip, tee, hmi));
                let manifest = root.join("Consortium.toml");
                let what = format!("{chip:?} tee={tee} hmi={hmi}");

                let session = pipeline::load_session(&manifest)
                    .await
                    .unwrap_or_else(|e| panic!("{what}: {e}"));
                assert_eq!(session.config.tee.is_some(), tee, "{what}");
                assert_eq!(session.config.hmi.is_some(), hmi, "{what}");

                let config = CheckConfig {
                    manifest_path: manifest,
                    linux: None,
                    release: false,
                    offline: false,
                    tee_toolchain: None,
                    strict_remoteproc: true,
                };
                let plan = check::plan_check(&session, &config)
                    .await
                    .unwrap_or_else(|e| panic!("{what}: {e}"));
                assert_eq!(plan.firmware.len(), 1, "{what}");
                assert_eq!(plan.firmware[0].config.target, "thumbv8m.main-none-eabihf");
                assert_eq!(plan.ta_skipped.is_some(), tee, "{what}");
            }
        }
    }
}

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

//! The files `csti new` writes, rendered from a [`Project`] description.
//!
//! Rendering is pure: [`render`] returns relative paths and contents, and the caller
//! decides where (and whether) to write them. Every project is a `unified` Cargo
//! workspace — one root `Cargo.toml` whose members are the endpoint crates — with one
//! `link` shared-memory channel between the Linux application and the CM33 firmware
//! and a defmt ring for the firmware. TEE and HMI add a crate or a frontend each.
//!
//! Templates use `{{key}}` placeholders rather than `format!`, so the Rust and
//! JavaScript sources below keep their own braces unescaped.

use std::path::{Path, PathBuf};

use clap::ValueEnum;

/// A chip `csti new` can scaffold for.
///
/// Each one pins a single validated part and its development kit: the addresses,
/// doorbell, and channel numbers below are the ones the melt-pot examples run on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Chip {
    /// STMicroelectronics STM32MP257F (Cortex-A35 + Cortex-M33), STM32MP257F-DK.
    #[value(name = "stm32mp257f")]
    Stm32mp257f,
    /// NXP i.MX 93 MIMX9352 (Cortex-A55 + Cortex-M33), FRDM-i.MX93.
    #[value(name = "mimx9352")]
    Mimx9352,
}

impl Chip {
    /// Every supported chip, in prompt order.
    pub const ALL: [Self; 2] = [Self::Stm32mp257f, Self::Mimx9352];

    /// A one-line description for prompts and summaries.
    pub fn label(self) -> &'static str {
        match self {
            Self::Stm32mp257f => "STM32MP257F (Cortex-A35 + Cortex-M33, STM32MP257F-DK)",
            Self::Mimx9352 => "MIMX9352 (i.MX 93: Cortex-A55 + Cortex-M33, FRDM-i.MX93)",
        }
    }

    fn profile(self) -> ChipProfile {
        match self {
            Self::Stm32mp257f => ChipProfile {
                manifest_chip: "stm32mp257fak3",
                part: "STM32MP257F",
                ap_core: "Cortex-A35",
                devkit: "stm32mp257f-dk",
                kernel_recipe: "linux-stm32mp",
                feature: "stm32mp25x",
                doorbell: "ipcc1",
                doorbell_note: "IPCC1",
                ap_chan: 0,
                mcu_chan: 1,
                memory_note: "SRAM2 (base 0x00060000 in the stm32mp25 chip database)",
                ipc_base: 0x0006_4000,
                dbg_base: 0x0006_2000,
                hal_crate: "consortium-hal-stm32mp2",
                hal_features: &["defmt", "rt", "stm32mp25x", "time-driver-gp-tim"],
                tick_hz: "tick-hz-1_000_000",
                tcm_constrained: false,
            },
            Self::Mimx9352 => ChipProfile {
                manifest_chip: "mimx9352",
                part: "MIMX9352",
                ap_core: "Cortex-A55",
                devkit: "imx93-11x11-frdm",
                kernel_recipe: "linux-imx",
                feature: "mimx93",
                doorbell: "mu1",
                doorbell_note: "MU1 (Linux drives side A, the CM33 side B)",
                ap_chan: 6,
                mcu_chan: 7,
                memory_note: "OCRAM (base 0x00480000 in the mimx93 chip database)",
                ipc_base: 0x0048_0000,
                dbg_base: 0x0048_4000,
                hal_crate: "consortium-hal-imx9",
                hal_features: &["defmt", "mimx93", "rt", "time-driver-lpit"],
                tick_hz: "tick-hz-24_000_000",
                tcm_constrained: true,
            },
        }
    }
}

/// Per-chip constants the templates splice in.
struct ChipProfile {
    /// `[profile] chip`, resolved by prefix against the embedded chip databases.
    manifest_chip: &'static str,
    part: &'static str,
    ap_core: &'static str,
    /// Board DTS file stem in the kernel checkout.
    devkit: &'static str,
    kernel_recipe: &'static str,
    /// The chip-selection feature shared by the transport, runtime, and HAL crates.
    feature: &'static str,
    doorbell: &'static str,
    doorbell_note: &'static str,
    ap_chan: u32,
    mcu_chan: u32,
    memory_note: &'static str,
    ipc_base: u32,
    dbg_base: u32,
    hal_crate: &'static str,
    hal_features: &'static [&'static str],
    /// The `embassy-time` tick rate matching the HAL's time driver.
    tick_hz: &'static str,
    /// The CM33 runs from TCM, where an unoptimized firmware does not fit.
    tcm_constrained: bool,
}

/// IPC window length; one descriptor plus a 128-byte-MTU slot per direction fits.
const IPC_LENGTH: u32 = 0x4000;
/// defmt ring length for the CM33.
const DBG_LENGTH: u32 = 0x2000;

/// Where generated crates get the Consortium crates from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// crates.io, at this exact version.
    Registry(String),
    /// A local Consortium checkout; dependencies point at `<root>/crates/<name>`.
    Path(PathBuf),
}

impl Source {
    /// Renders one Consortium dependency line, `extra` being any further inline-table
    /// keys (`default-features = false, features = [...]`).
    fn dep(&self, name: &str, extra: &str) -> String {
        let location = match self {
            Self::Registry(version) => format!("version = \"{version}\""),
            Self::Path(root) => {
                let path = root.join("crates").join(name);
                format!(
                    "path = \"{}\"",
                    path.display().to_string().replace('\\', "/")
                )
            }
        };
        if extra.is_empty() {
            format!("{name} = {{ {location} }}")
        } else {
            format!("{name} = {{ {extra}, {location} }}")
        }
    }
}

/// Everything `csti new` decided, interactively or from flags.
#[derive(Clone, Debug)]
pub struct Project {
    /// Package-name prefix: crates are `<name>-app`, `<name>-mcu`, and so on.
    pub name: String,
    pub chip: Chip,
    /// Add an OP-TEE trusted application and call it from the application.
    pub tee: bool,
    /// Add a Cog/WebKit HMI and its Vite frontend.
    pub hmi: bool,
    pub source: Source,
    /// `rust-toolchain.toml` channel: the checkout's own pin for a [`Source::Path`],
    /// so path dependencies and registry crates build with one rustc.
    pub toolchain: String,
    /// `[tee] uuid`, also baked into the application as its fallback.
    pub tee_uuid: String,
}

/// Rejects names that are not valid Cargo package names once suffixed.
pub fn validate_name(name: &str) -> Result<(), String> {
    let mut chars = name.chars();
    let first_ok = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    let rest_ok = chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if first_ok && rest_ok {
        Ok(())
    } else {
        Err(format!(
            "`{name}` is not a valid project name: start with an ASCII letter and use only \
             letters, digits, `-`, and `_`"
        ))
    }
}

/// Renders every file of the project as `(relative path, contents)`.
pub fn render(project: &Project) -> Vec<(PathBuf, String)> {
    let chip = project.chip.profile();
    let vars = Vars::new(project, &chip);
    let mut files = vec![
        file("Consortium.toml", consortium_toml(project, &chip, &vars)),
        file("Cargo.toml", workspace_toml(project, &chip)),
        file(".gitignore", gitignore(project)),
        file("rust-toolchain.toml", rust_toolchain(project)),
        file(
            "shared/Cargo.toml",
            vars.fill(SHARED_CARGO).replace(
                "{{ipc_dep}}",
                &project
                    .source
                    .dep("consortium-ipc", "default-features = false"),
            ),
        ),
        file("shared/src/lib.rs", vars.fill(SHARED_LIB)),
        file("app/Cargo.toml", app_cargo(project, &chip, &vars)),
        file("app/src/main.rs", app_main(project, &vars)),
        file("mcu/Cargo.toml", mcu_cargo(project, &chip, &vars)),
        file("mcu/build.rs", MCU_BUILD.to_string()),
        file("mcu/.cargo/config.toml", vars.fill(MCU_CARGO_CONFIG)),
        file("mcu/src/main.rs", vars.fill(MCU_MAIN)),
    ];
    if project.tee {
        files.push(file("ta/Cargo.toml", ta_cargo(project, &vars)));
        files.push(file("ta/build.rs", TA_BUILD.to_string()));
        files.push(file("ta/src/lib.rs", TA_LIB.to_string()));
        files.push(file("ta/src/main.rs", vars.fill(TA_MAIN)));
    }
    if project.hmi {
        files.push(file("hmi/package.json", vars.fill(HMI_PACKAGE)));
        files.push(file("hmi/index.html", vars.fill(HMI_INDEX)));
        files.push(file("hmi/src/main.js", HMI_MAIN.to_string()));
    }
    files
}

fn file(path: &str, contents: String) -> (PathBuf, String) {
    (Path::new(path).to_path_buf(), contents)
}

/// The `{{key}}` substitutions shared by every template.
struct Vars(Vec<(&'static str, String)>);

impl Vars {
    fn new(project: &Project, chip: &ChipProfile) -> Self {
        Self(vec![
            ("name", project.name.clone()),
            ("snake", project.name.replace('-', "_")),
            ("part", chip.part.to_string()),
            ("ap_core", chip.ap_core.to_string()),
            ("tee_uuid", project.tee_uuid.clone()),
        ])
    }

    fn fill(&self, template: &str) -> String {
        self.0
            .iter()
            .fold(template.to_string(), |acc, (key, value)| {
                acc.replace(&format!("{{{{{key}}}}}"), value)
            })
    }
}

fn consortium_toml(project: &Project, chip: &ChipProfile, vars: &Vars) -> String {
    let mut out = vars.fill(
        "# {{name}}: {{ap_core}} Linux application + Cortex-M33 real-time firmware on the
# {{part}}. Generated by `csti new`.
#
#   csti fetch                       # download every endpoint's dependencies
#   csti check                       # validate this manifest and type-check the endpoints
#   csti build --linux <KERNEL_DIR>  # build and stage everything into dist/

[profile]
",
    );
    out.push_str(&format!(
        "# Resolved against the embedded chip database, which also supplies the Rust
# targets for the firmware and the application.
chip = \"{chip}\"
# Board device tree, matched by full file stem in the kernel checkout given to
# `csti build --linux <DIR>`. Remove it to build on the bare SoC `.dtsi`.
devkit        = \"{devkit}\"
shared        = \"./shared\"
deploy_root   = \"/opt/{name}\"
kernel_recipe = \"{recipe}\"
# The Linux target sysroot, e.g. a Yocto SDK's. `/` is the build host's own root,
# which is right when building natively on an aarch64 Linux host or container.
# sysroot = \"/\"
# Every endpoint crate is a member of the Cargo workspace in `Cargo.toml` beside
# this file, and builds into its `target/`.
workspace = \"unified\"

[endpoints]
# Runtime endpoint name -> crate root.
linux = \"./app\"
cm33  = \"./mcu\"
",
        chip = chip.manifest_chip,
        devkit = chip.devkit,
        name = project.name,
        recipe = chip.kernel_recipe,
    ));
    if project.tee {
        out.push_str("optee = \"./ta\"\n");
    }
    out.push_str(&format!(
        "
[dbg]
# How the application consumes the firmware's defmt ring: \"auto\" forwards
# decoded frames to `tracing`; \"interface\" hands back reader handles.
mode = \"auto\"

# defmt ring for the CM33, just above the IPC window below.
[dbg.cm33]
memory = {{ base = {dbg_base:#010x}, length = {dbg_len:#x} }}
mtu    = 64

# One shared-memory channel between the application and the firmware, signalled
# over {doorbell_note}. The window lives in {memory_note}; the CM33 reaches it
# at its physical base and Linux maps it through a UIO device. Each party sends
# the type listed against it, from the `shared` crate.
[[ipc.channel]]
name          = \"link\"
doorbell      = \"{doorbell}\"
pattern       = \"memory\"
primary       = \"linux\"
secondary     = \"cm33\"
party.linux   = {{ chan = {ap_chan}, type = \"Command\" }}
party.cm33    = {{ chan = {mcu_chan}, type = \"Telemetry\" }}
config.memory = {{ base = {ipc_base:#010x}, length = {ipc_len:#x} }}
mtu           = 128
",
        dbg_base = chip.dbg_base,
        dbg_len = DBG_LENGTH,
        doorbell_note = chip.doorbell_note,
        memory_note = chip.memory_note,
        doorbell = chip.doorbell,
        ap_chan = chip.ap_chan,
        mcu_chan = chip.mcu_chan,
        ipc_base = chip.ipc_base,
        ipc_len = IPC_LENGTH,
    ));
    if project.tee {
        out.push_str(&format!(
            "
[tee]
# The trusted application's UUID. The build injects it into both the TA and the
# application, so the two always agree.
uuid = \"{}\"
",
            project.tee_uuid
        ));
    }
    if project.hmi {
        out.push_str(
            "
[hmi]
engine  = \"cog\"
backend = \"wayland\"
# Debug builds load the Vite dev server (`npm run dev` in hmi/) instead.
debug  = \"http://127.0.0.1:5173/\"
source = \"hmi\"
build  = \"hmi/dist\"
",
        );
    }
    out
}

fn workspace_toml(project: &Project, chip: &ChipProfile) -> String {
    let mut members = vec!["app", "mcu", "shared"];
    if project.tee {
        members.push("ta");
    }
    let members = members
        .iter()
        .map(|m| format!("\"{m}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let mut out = format!(
        "# Each endpoint compiles for its own target, with code generated from
# `Consortium.toml`, so build them through `csti check`/`csti build` rather than
# `cargo build` at this root.
[workspace]
members  = [{members}]
resolver = \"3\"

[workspace.package]
edition = \"2024\"
version = \"0.1.0\"
"
    );
    if chip.tcm_constrained {
        out.push_str(
            "
# The CM33 runs from 128 KiB of TCM, where an unoptimized firmware does not fit.
[profile.dev]
opt-level = \"s\"
",
        );
    }
    out.push_str(
        "
[profile.release]
lto = true
",
    );
    out
}

fn rust_toolchain(project: &Project) -> String {
    format!(
        "[toolchain]
channel = \"{}\"
# The firmware and the Linux application targets.
targets = [\"aarch64-unknown-linux-gnu\", \"thumbv8m.main-none-eabihf\"]
",
        project.toolchain
    )
}

fn gitignore(project: &Project) -> String {
    let mut out = String::from("/target\n/dist\n");
    if project.hmi {
        out.push_str("/hmi/node_modules\n/hmi/dist\n");
    }
    out
}

fn app_cargo(project: &Project, chip: &ChipProfile, vars: &Vars) -> String {
    let src = &project.source;
    let feature = chip.feature;
    let mut deps = vec![
        vars.fill("{{name}}-shared = { path = \"../shared\" }"),
        String::new(),
        "# Named directly by the generated `consortium.gen.rs`.".to_string(),
        src.dep("consortium-codec", "features = [\"postcard\"]"),
        src.dep("consortium-ipc", "features = [\"std\"]"),
        src.dep(
            "consortium-ipc-transport-memory",
            &format!("default-features = false, features = [\"std\", \"{feature}\"]"),
        ),
        src.dep(
            "consortium-runtime-app",
            &format!("features = [\"{feature}\"]"),
        ),
        src.dep("consortium-shared-memory", ""),
    ];
    if project.tee {
        deps.push(String::new());
        deps.push("# The trusted application's client-side callers.".to_string());
        deps.push(vars.fill(
            "{{name}}-ta = { default-features = false, features = [\"ca\"], path = \"../ta\" }",
        ));
        deps.push(src.dep("consortium-tee", "features = [\"ca\"]"));
        deps.push("optee-teec = \"0.10.0\"".to_string());
    }
    if project.hmi {
        deps.push(String::new());
        deps.push("# The Cog/WebKit kiosk hosting the HMI.".to_string());
        deps.push(src.dep("consortium-hmi-webkit", ""));
        deps.push("glib       = \"0.22\"".to_string());
        deps.push("serde_json = \"1\"".to_string());
    }
    deps.push(String::new());
    deps.push(
        "tokio   = { features = [\"macros\", \"rt-multi-thread\"], version = \"1\" }".to_string(),
    );
    deps.push("tracing = \"0.1\"".to_string());

    let mut out = vars.fill(
        "# Built by `csti build`, which first writes the `src/consortium.gen.rs` this crate
# `include!`s (UIO bring-up, IPC channels, defmt console) from `../Consortium.toml`.
[package]
description       = \"{{name}} Linux application ({{ap_core}})\"
edition.workspace = true
name              = \"{{name}}-app\"
publish           = false
version.workspace = true

# Linux-only, so a host-side check of the workspace skips it.
[target.'cfg(target_os = \"linux\")'.dependencies]
",
    );
    out.push_str(&deps.join("\n"));
    out.push('\n');
    out
}

fn app_main(project: &Project, vars: &Vars) -> String {
    let mut out = vars.fill(APP_HEADER);
    if project.tee {
        out.push_str(&vars.fill(APP_TEE_UUID));
    }
    if project.hmi {
        out.push_str(&vars.fill(APP_MAIN_HMI));
    } else {
        out.push_str(APP_MAIN_PLAIN);
    }
    if project.tee {
        out.push_str(&vars.fill(APP_TEE_CALL));
    }
    // `tee_check()` is spliced in only when the TA exists.
    let tee_call = if project.tee {
        "\n    tee_check();"
    } else {
        ""
    };
    out.replace("{{tee_call}}", tee_call)
}

fn mcu_cargo(project: &Project, chip: &ChipProfile, vars: &Vars) -> String {
    let src = &project.source;
    let feature = chip.feature;
    let hal_features = chip
        .hal_features
        .iter()
        .map(|f| format!("\"{f}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let deps = [
        vars.fill("{{name}}-shared = { path = \"../shared\" }"),
        String::new(),
        "# Named directly by the generated `consortium.gen.rs`.".to_string(),
        src.dep(
            "consortium-codec",
            "default-features = false, features = [\"postcard\"]",
        ),
        src.dep("consortium-ipc", "default-features = false"),
        src.dep(
            "consortium-ipc-transport-memory",
            &format!("default-features = false, features = [\"defmt\", \"{feature}\"]"),
        ),
        src.dep(
            "consortium-runtime-mcu",
            "default-features = false, features = [\"defmt\", \"macros\"]",
        ),
        src.dep("consortium-shared-memory", ""),
        "# The chip is named here rather than left to arrive transitively, so no".to_string(),
        "# dependency's default chip can sneak in beside it.".to_string(),
        src.dep(
            chip.hal_crate,
            &format!("default-features = false, features = [{hal_features}]"),
        ),
        "# `logger` is the defmt sink behind `[dbg.cm33]`; `panic-handler` reports a panic"
            .to_string(),
        "# through the same ring before halting.".to_string(),
        src.dep(
            "consortium-dbg",
            "default-features = false, features = [\"arch-cortex-m\", \"logger\", \"panic-handler\"]",
        ),
        String::new(),
        "cortex-m         = { features = [\"critical-section-single-core\"], version = \"0.7\" }"
            .to_string(),
        "cortex-m-rt      = \"0.7\"".to_string(),
        "defmt            = \"1\"".to_string(),
        "embassy-executor = { default-features = false, features = [\"executor-thread\", \"platform-cortex-m\"], version = \"0.10\" }".to_string(),
        format!(
            "embassy-time     = {{ default-features = false, features = [\"{}\"], version = \"0.5\" }}",
            chip.tick_hz
        ),
    ];

    let mut out = vars.fill(
        "# Built by `csti build`, which first writes the `src/consortium.gen.rs` this crate
# `include!`s (IPC channels, MPU/NVIC and time-driver bring-up) and `memory.x`
# from `../Consortium.toml`, then post-processes the ELF for Linux remoteproc.
[package]
description       = \"{{name}} Cortex-M33 firmware ({{part}})\"
edition.workspace = true
name              = \"{{name}}-mcu\"
publish           = false
version.workspace = true

[[bin]]
name = \"{{name}}-mcu\"
path = \"src/main.rs\"
test = false

[dependencies]
",
    );
    out.push_str(&deps.join("\n"));
    out.push('\n');
    out
}

fn ta_cargo(project: &Project, vars: &Vars) -> String {
    let src = &project.source;
    let mut out = vars.fill(
        "# Two builds of one crate: the `ta` feature is the OP-TEE trusted application ELF
# `csti build` produces for the Secure World, and the `ca` feature is the
# client-side callers the Linux application links.
[package]
description       = \"{{name}} OP-TEE trusted application\"
edition.workspace = true
name              = \"{{name}}-ta\"
publish           = false
version.workspace = true

[features]
ca      = [\"consortium-tee/ca\", \"dep:optee-teec\"]
default = []
ta      = [\"consortium-tee/ta\", \"dep:optee-utee\", \"dep:optee-utee-sys\", \"optee-utee/unwind_stubs\"]

[lib]
crate-type = [\"rlib\", \"staticlib\"]
name       = \"{{snake}}_ta\"
path       = \"src/lib.rs\"

[[bin]]
name              = \"{{snake}}_ta\"
path              = \"src/main.rs\"
required-features = [\"ta\"]

[target.'cfg(target_os = \"linux\")'.dependencies]
",
    );
    out.push_str(&src.dep("consortium-tee", "default-features = false"));
    out.push_str(
        "
optee-teec     = { optional = true, version = \"0.10.0\" }
optee-utee     = { default-features = false, optional = true, version = \"0.10.0\" }
optee-utee-sys = { default-features = false, optional = true, version = \"0.10.0\" }

[target.'cfg(target_os = \"linux\")'.build-dependencies]
",
    );
    out.push_str(&src.dep("consortium-cfg-common", ""));
    out.push_str("\noptee-utee-build = \"0.10.0\"\n");
    out
}

const SHARED_CARGO: &str = "[package]
description       = \"Message types shared by every {{name}} endpoint\"
edition.workspace = true
name              = \"{{name}}-shared\"
publish           = false
version.workspace = true

[dependencies]
{{ipc_dep}}
serde = { default-features = false, features = [\"derive\"], version = \"1\" }
";

const SHARED_LIB: &str = "//! Message types that cross the core boundary.
//!
//! `Consortium.toml` names these by type: on the `link` channel each party sends
//! the type listed against it (`party.<core>.type`), and the generated
//! `consortium.gen.rs` of every endpoint refers to them by path. The channel uses
//! the default postcard codec, hence the serde derives; `IpcSafe` rejects anything
//! that means nothing in another address space, such as pointers and `usize`.

#![no_std]

use consortium_ipc::IpcSafe;
use serde::{Deserialize, Serialize};

/// Sent once by the Linux application after the link comes up.
#[derive(Clone, Copy, Debug, IpcSafe, Deserialize, Serialize)]
pub struct Command {
    /// How often the firmware should report, in milliseconds.
    pub period_ms: u32,
}

/// Sent periodically by the CM33 firmware.
#[derive(Clone, Copy, Debug, IpcSafe, Deserialize, Serialize)]
pub struct Telemetry {
    /// Report counter; wraps at `u32::MAX`.
    pub seq: u32,
    /// Firmware uptime in milliseconds when the report was taken.
    pub uptime_ms: u32,
}
";

const APP_HEADER: &str = "//! {{name}} Linux application on the {{part}}'s {{ap_core}}.
//!
//! `consortium.gen.rs`, generated by `csti build` from `../Consortium.toml`, maps
//! the IPC window and doorbell through UIO, starts the defmt console that forwards
//! the firmware's log ring to `tracing`, and returns the `link` channel token.

#![cfg(target_os = \"linux\")]

use {{snake}}_shared::{Command, Telemetry};

include!(\"consortium.gen.rs\");

/// Reporting period requested from the firmware.
const PERIOD_MS: u32 = 1_000;
";

const APP_TEE_UUID: &str = "
// `csti build` injects `[tee] uuid` as `CONSORTIUM_TEE_UUID`, the same value the
// TA is built with; a plain `cargo` build falls back to the scaffolded one.
const TA_UUID: &str = match option_env!(\"CONSORTIUM_TEE_UUID\") {
    Some(uuid) => uuid,
    None => \"{{tee_uuid}}\",
};
";

const APP_MAIN_PLAIN: &str = "
#[consortium_runtime_app::main]
async fn main() -> Result<(), ConsortiumError> {
    let (_p, c) = init(Config::default())?;{{tee_call}}

    // Waits for the firmware to publish its side of the channel.
    let mut link = ::consortium_ipc_transport_memory::Transceiver::connect(c.LINK).await?;
    tracing::info!(\"link up\");

    if let Err(e) = link.send(&Command { period_ms: PERIOD_MS }).await {
        tracing::warn!(\"could not send the reporting period: {e:?}\");
    }

    loop {
        match link.recv().await {
            Ok(msg) => {
                let telemetry: Telemetry = msg.into_inner();
                tracing::info!(
                    \"telemetry #{}: firmware up {} ms\",
                    telemetry.seq,
                    telemetry.uptime_ms
                );
            }
            Err(e) => {
                tracing::error!(\"link receive failed: {e:?}\");
                return Ok(());
            }
        }
    }
}
";

const APP_MAIN_HMI: &str = "
use std::sync::{Arc, Mutex};

use consortium_hmi_webkit::{Bridge, KioskApp};

#[consortium_runtime_app::main]
async fn main() -> Result<(), ConsortiumError> {
    let (_p, c) = init(Config::default())?;{{tee_call}}

    // Waits for the firmware to publish its side of the channel.
    let mut link = ::consortium_ipc_transport_memory::Transceiver::connect(c.LINK).await?;
    tracing::info!(\"link up\");

    if let Err(e) = link.send(&Command { period_ms: PERIOD_MS }).await {
        tracing::warn!(\"could not send the reporting period: {e:?}\");
    }

    // The receive loop publishes the latest report; the HMI polls it.
    let latest: Arc<Mutex<Option<Telemetry>>> = Arc::default();
    let writer = Arc::clone(&latest);
    tokio::spawn(async move {
        loop {
            match link.recv().await {
                Ok(msg) => {
                    let telemetry: Telemetry = msg.into_inner();
                    *writer.lock().unwrap() = Some(telemetry);
                }
                Err(e) => {
                    tracing::error!(\"link receive failed: {e:?}\");
                    return;
                }
            }
        }
    });

    // `window.__BRIDGE__.invoke(\"telemetry\")` in hmi/src/main.js lands here.
    let mut bridge = Bridge::new();
    bridge.handle(\"telemetry\", move |_payload| {
        serde_json::to_string(&*latest.lock().unwrap()).unwrap_or_else(|_| \"null\".into())
    });

    let app = KioskApp::new(\"{{name}}\", None, None).expect(\"Cog failed to initialise\");
    app.attach_bridge(bridge).expect(\"HMI bridge failed to attach\");
    app.load().expect(\"HMI failed to load\");
    glib::MainLoop::new(None, false).run();
    Ok(())
}
";

const APP_TEE_CALL: &str = "
/// Asks the trusted application to tag a value, logging the outcome. A missing
/// OP-TEE (no `/dev/tee0`, TA not installed) is reported, not fatal.
fn tee_check() {
    match tee_tag(PERIOD_MS) {
        Ok(tag) => tracing::info!(\"trusted application tagged {PERIOD_MS} as {tag:#010x}\"),
        Err(e) => tracing::warn!(\"trusted application unavailable: {e}\"),
    }
}

/// `Context::new` -> `open_session(uuid)` -> the `#[tee_command]` caller.
fn tee_tag(value: u32) -> consortium_tee::TeeResult<u32> {
    use consortium_tee::TeeError;
    use optee_teec::{Context, Uuid};

    let uuid = Uuid::parse_str(TA_UUID).map_err(TeeError::Session)?;
    let mut ctx = Context::new().map_err(TeeError::Context)?;
    let mut session = ctx.open_session(uuid).map_err(TeeError::Session)?;
    {{snake}}_ta::call_tag(&mut session, value)
}
";

const MCU_BUILD: &str = "fn main() {
    // `csti build` writes `memory.x` here from the lowered remoteproc layout.
    // Cargo links workspace members from the workspace root, so point the linker
    // at this directory explicitly for `link.x`'s `INCLUDE memory.x`.
    let dir = std::env::var(\"CARGO_MANIFEST_DIR\").unwrap();
    println!(\"cargo:rustc-link-search={dir}\");
    println!(\"cargo:rerun-if-changed=memory.x\");

    // `--nmagic` keeps the image compact (no page-alignment padding), `link.x` is
    // cortex-m-rt's linker script, and `defmt.x` lays out defmt's string table.
    println!(\"cargo:rustc-link-arg-bins=--nmagic\");
    println!(\"cargo:rustc-link-arg-bins=-Tlink.x\");
    println!(\"cargo:rustc-link-arg-bins=-Tdefmt.x\");
}
";

const MCU_CARGO_CONFIG: &str = "[build]
target = \"thumbv8m.main-none-eabihf\"

[env]
# defmt drops everything below `error` when DEFMT_LOG is unset. The HAL stays at
# `info`: its time driver traces every timer interrupt, which would flood the
# ring before the application starts draining it.
DEFMT_LOG = \"info,{{snake}}_mcu=trace\"
";

const MCU_MAIN: &str = "//! {{name}} firmware on the {{part}}'s Cortex-M33.
//!
//! `consortium.gen.rs`, generated by `csti build` from `../Consortium.toml`, sets up
//! the defmt ring, MPU, NVIC, doorbell, and Embassy time driver in `init` and
//! returns the `link` channel token. The ring is decoded on the Linux side.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use {{snake}}_shared::{Command, Telemetry};

include!(\"consortium.gen.rs\");

/// Reporting period used until the application asks for another.
const DEFAULT_PERIOD_MS: u32 = 1_000;

#[consortium_runtime_mcu::main]
async fn main(_spawner: Spawner) -> Result<(), ConsortiumError> {
    let (_p, c) = init(Config::default());

    // Waits for the application to validate and acknowledge the channel.
    let mut link = ::consortium_ipc_transport_memory::Transceiver::connect(c.LINK).await?;
    defmt::info!(\"link up\");

    let period_ms = match link.recv().await {
        Ok(msg) => {
            let command: Command = msg.into_inner();
            command.period_ms.max(10)
        }
        Err(_) => {
            defmt::warn!(\"unreadable command; keeping the default period\");
            DEFAULT_PERIOD_MS
        }
    };
    defmt::info!(\"reporting every {=u32} ms\", period_ms);

    let mut seq: u32 = 0;
    loop {
        let telemetry = Telemetry {
            seq,
            uptime_ms: Instant::now().as_millis() as u32,
        };
        if link.send(&telemetry).await.is_err() {
            defmt::warn!(\"telemetry #{=u32} not sent\", seq);
        }
        seq = seq.wrapping_add(1);
        Timer::after_millis(u64::from(period_ms)).await;
    }
}
";

const TA_BUILD: &str = "#[cfg(all(target_os = \"linux\", feature = \"ta\"))]
fn main() -> Result<(), optee_utee_build::Error> {
    use consortium_cfg_common::config::TEE_UUID_ENV;
    use optee_utee_build::TaConfig;

    println!(\"cargo:rerun-if-env-changed={TEE_UUID_ENV}\");

    // `csti build` passes `[tee] uuid` from Consortium.toml.
    let ta_uuid = std::env::var(TEE_UUID_ENV)
        .unwrap_or_else(|_| panic!(\"missing {TEE_UUID_ENV}; build this TA with `csti build`\"));
    let ta_config = TaConfig::new_default_with_cargo_env(&ta_uuid)?;
    optee_utee_build::build(ta_config)
}

#[cfg(not(all(target_os = \"linux\", feature = \"ta\")))]
fn main() {}
";

const TA_LIB: &str = "//! Commands the trusted application serves.
//!
//! Each `#[tee_command]` function gets a TA-side dispatcher (feature `ta`) and a
//! `call_<name>` client function for the Linux application (feature `ca`).

#![cfg_attr(feature = \"ta\", no_std)]
#![cfg(target_os = \"linux\")]

#[cfg(feature = \"ta\")]
extern crate alloc;

use consortium_tee::{tee_command, tee_service};

/// Stands in for a key provisioned into the Secure World. Replace it, and
/// `tag` with real cryptography, before relying on either.
const KEY: u32 = 0x5EC0_4E7E;

/// Tags `value` with a key the Normal World never sees.
#[tee_command]
pub fn tag(value: u32) -> u32 {
    (value ^ KEY).rotate_left(13).wrapping_mul(0x9E37_79B9)
}

tee_service! {
    commands { tag }
}
";

const TA_MAIN: &str = "#![no_std]
#![cfg(target_os = \"linux\")]
#![no_main]

extern crate optee_utee;

use optee_utee::Result;
use optee_utee::prelude::*;
use {{snake}}_ta::invoke_command;

#[ta_create]
fn create() -> Result<()> {
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut ParametersNone) -> Result<()> {
    Ok(())
}

#[ta_close_session]
fn close_session() {}

#[ta_destroy]
fn destroy() {}

#[ta_invoke_command]
fn invoker(command_id: u32, params: &mut ParametersAny<'_>) -> Result<()> {
    invoke_command(command_id, params)
}

include!(concat!(env!(\"OUT_DIR\"), \"/user_ta_header.rs\"));
";

const HMI_PACKAGE: &str = "{
  \"name\": \"{{name}}-hmi\",
  \"private\": true,
  \"version\": \"0.1.0\",
  \"type\": \"module\",
  \"scripts\": {
    \"dev\": \"vite\",
    \"build\": \"vite build\",
    \"preview\": \"vite preview\"
  },
  \"devDependencies\": {
    \"vite\": \"^8.0.0\"
  }
}
";

const HMI_INDEX: &str = "<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{{name}}</title>
  </head>
  <body>
    <h1>{{name}}</h1>
    <p id=\"telemetry\">Waiting for the firmware…</p>
    <script type=\"module\" src=\"/src/main.js\"></script>
  </body>
</html>
";

const HMI_MAIN: &str =
    "// `window.__BRIDGE__` is injected by consortium-hmi-webkit inside the kiosk; a
// plain browser on the Vite dev server has no bridge.
const out = document.querySelector('#telemetry')

async function poll() {
  const bridge = window.__BRIDGE__
  if (!bridge) {
    out.textContent = 'No bridge: open this page in the kiosk.'
    return
  }
  const telemetry = await bridge.invoke('telemetry')
  out.textContent = telemetry
    ? `Report #${telemetry.seq}: firmware up ${telemetry.uptime_ms} ms`
    : 'Waiting for the firmware…'
}

poll()
setInterval(poll, 1000)
";

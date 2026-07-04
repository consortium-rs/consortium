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

mod compile;
pub mod pipeline;
pub mod progress;

// Trait
pub use compile::Compiler;
pub use progress::{BuildReporter, BuildStep};

// Deployment artifact rendering
pub use compile::deploy::systemd::{
    SystemdRestartPolicy, SystemdUnitConfig, render_unit as render_systemd_unit,
    write_unit as write_systemd_unit,
};

// End-to-end pipeline
pub use pipeline::{PipelineConfig, PipelineError, PipelinePlan, run as run_pipeline};

// Compile-time environment-injection carriers
pub use compile::env::{CortexMcuCore, DbgConfig, HmiConfig, SharedMemConfig};

// Backward-compatible builders (String-error API)
pub use compile::application::ApplicationBuilder;
pub use compile::firmware::FirmwareBuilder;
pub use compile::optee::{OpteeBuilder, OpteeEnvironment};

// Typed compiler API
pub use compile::application::{
    ApplicationCompileError, ApplicationCompiler, ApplicationConfiguration,
};
pub use compile::firmware::{FirmwareCompileError, FirmwareCompiler, FirmwareConfiguration};
pub use compile::optee::{OPTEE_TARGET, OpteeCompileError, OpteeCompiler, OpteeConfiguration};
pub use compile::web::{HmiBuildCommand, HmiCompileError, HmiCompiler, HmiConfiguration};

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

// Trait
pub use compile::Compiler;

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

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

//! Runs a built PocketJS `<app>.js` + `<app>.pak` in a native window.
//!
//! Build a mounted app first (its `render()`/`mount()` installs the frame
//! contract), then point this at the outputs:
//!
//! ```console
//! $ cd submodules/pocketjs && bun scripts/build.ts hero-main
//! $ cargo run -p consortium-hmi-pocket --features shell --example kiosk -- \
//!     submodules/pocketjs/dist/hero-main.js submodules/pocketjs/dist/hero-main.pak
//! ```
//!
//! Keys: arrows/WASD = d-pad, X = Circle, Z = Cross, C = Square, V = Triangle,
//! Q/E = triggers, Enter = Start, RShift = Select, Esc = quit.

use std::process::ExitCode;

use consortium_hmi::Bridge;
use consortium_hmi_pocket::{PocketApp, PocketConfig, ShellConfig, run};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(bundle), Some(pak)) = (args.next(), args.next()) else {
        eprintln!("usage: kiosk <bundle.js> <bundle.pak>");
        return ExitCode::FAILURE;
    };

    // A single-threaded runtime is enough: bridge handlers here are trivial.
    let runtime = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            eprintln!("kiosk: building Tokio runtime: {error}");
            return ExitCode::FAILURE;
        }
    };

    let app = match PocketApp::from_files(
        &bundle,
        &pak,
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    ) {
        Ok(app) => app,
        Err(error) => {
            eprintln!("kiosk: loading PocketJS app: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = run(app, ShellConfig::default()) {
        eprintln!("kiosk: {error}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

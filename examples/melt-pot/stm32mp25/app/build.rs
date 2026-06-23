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

//! Emits a non-fatal hint when the shared-memory layout from `Consortium.toml`
//! was not injected, i.e. this crate is being built with a plain `cargo build`
//! instead of through `consortium-builder`.

fn main() {
    println!("cargo:rerun-if-env-changed=CONSORTIUM_IPC_BASE");
    println!("cargo:rerun-if-env-changed=CONSORTIUM_DBG_BASE");

    let ipc = std::env::var_os("CONSORTIUM_IPC_BASE").is_some();
    let dbg = std::env::var_os("CONSORTIUM_DBG_BASE").is_some();

    if !ipc || !dbg {
        println!(
            "cargo:warning=melt-pot: IPC/DBG shared-memory layout not configured \
             (CONSORTIUM_IPC_*/CONSORTIUM_DBG_* unset). Build through \
             `consortium-builder`, which injects the [ipc]/[dbg] sections of \
             Consortium.toml, rather than a plain `cargo build`."
        );
    }
}

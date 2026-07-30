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

//! Implementation of the Consortium runtime entry macros.
//!
//! These operate on [`proc_macro2::TokenStream`] so the thin `proc-macro = true`
//! facade in `consortium-runtime-macros` stays trivial and the logic here is
//! host-testable with `insta` snapshots.
//!
//! Two macro families are provided, one per runtime target:
//!
//! - `mcu_main` / `app_main` wrap the user's `async fn main`, run the generated
//!   `init()` from `consortium.gen.rs`, and hand the body the aggregate
//!   `Context`. The MCU flavor drives `embassy_executor`; the app flavor drives
//!   `tokio`.
//! - `mcu_fail` / `app_fail` mark an optional init-failure handler. See
//!   the `fail` module for the glob-shadowing dispatch mechanism.

mod entry;
mod fail;

pub use entry::{app_main, mcu_main};
pub use fail::{app_fail, mcu_fail};

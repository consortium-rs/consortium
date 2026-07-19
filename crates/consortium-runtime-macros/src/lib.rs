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

//! Proc-macro facade for the Consortium runtime entry points.
//!
//! The runtime crates re-export these under target-neutral names, so firmware
//! writes `#[consortium_runtime_mcu::main]` and applications write
//! `#[consortium_runtime_app::main]`. All logic lives in
//! `consortium-runtime-macros-impl`.

use proc_macro::TokenStream;

/// Entry-point macro for `no_std` firmware: wraps `embassy_executor::main`, runs
/// the generated `init()`, and binds its `Context` for the body.
#[proc_macro_attribute]
pub fn mcu_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_runtime_macros_impl::mcu_main(attr.into(), item.into()).into()
}

/// Entry-point macro for Linux applications: wraps `tokio::main`, runs the
/// generated `init()`, and binds its `Context` for the body.
#[proc_macro_attribute]
pub fn app_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_runtime_macros_impl::app_main(attr.into(), item.into()).into()
}

/// Optional init-failure handler for firmware; see the impl crate for the
/// glob-shadowing dispatch.
#[proc_macro_attribute]
pub fn mcu_fail(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_runtime_macros_impl::mcu_fail(attr.into(), item.into()).into()
}

/// Optional init-failure handler for applications; see the impl crate for the
/// glob-shadowing dispatch.
#[proc_macro_attribute]
pub fn app_fail(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_runtime_macros_impl::app_fail(attr.into(), item.into()).into()
}

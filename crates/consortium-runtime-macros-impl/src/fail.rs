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

//! The `#[consortium_runtime_{mcu,app}::fail]` init-failure handler macros.
//!
//! The companion `#[main]` glue calls `__consortium_init_fail(err)` when the
//! generated `init()` returns `Err`. Both runtime crates export a fallback
//! `__rt::__consortium_init_fail` that `#[main]` glob-imports. Applying `#[fail]`
//! emits a module-scope `fn __consortium_init_fail` that **shadows** that
//! glob-imported fallback (a stable Rust guarantee: a local item shadows a
//! glob import), so the user's handler runs instead. If `#[fail]` is absent the
//! fallback is used unchanged.
//!
//! The user handler receives the concrete generated init-error type. It may
//! return `()` (then the diverging tail runs) or `!` (then it never returns).

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ItemFn};

/// Which runtime a `fail` macro targets — only the hard-fail tail differs.
#[derive(Clone, Copy)]
enum Runtime {
    Mcu,
    App,
}

impl Runtime {
    /// The diverging tail run after the user handler returns (unreachable when
    /// the handler itself is `-> !`).
    fn hard_fail(self) -> TokenStream2 {
        match self {
            // Firmware cannot proceed without a `Context`; park the core.
            Runtime::Mcu => quote! {
                loop {
                    ::cortex_m::asm::wfi();
                }
            },
            // The application exits non-zero so a supervisor can restart it.
            Runtime::App => quote! {
                ::std::process::exit(1);
            },
        }
    }
}

/// `#[consortium_runtime_mcu::fail]`.
pub fn mcu_fail(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    fail_impl(Runtime::Mcu, attr, item)
}

/// `#[consortium_runtime_app::fail]`.
pub fn app_fail(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    fail_impl(Runtime::App, attr, item)
}

fn fail_impl(runtime: Runtime, attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    if !attr.is_empty() {
        return syn::Error::new_spanned(&attr, "`fail` takes no arguments").into_compile_error();
    }

    let func = match syn::parse2::<ItemFn>(item) {
        Ok(func) => func,
        Err(err) => return err.into_compile_error(),
    };

    if let Err(err) = validate(&func) {
        return err.into_compile_error();
    }

    let handler_name = &func.sig.ident;
    // The single parameter's type is the concrete init-error type. Rebind it to
    // a fixed name so the dispatcher's signature matches what `init()` returns
    // and the `#[main]` call site type-checks regardless of the handler's own
    // parameter name.
    let err_ty = match func.sig.inputs.first() {
        Some(FnArg::Typed(pt)) => &pt.ty,
        _ => unreachable!("validated to be a single typed parameter"),
    };
    let hard_fail = runtime.hard_fail();

    quote! {
        #func

        /// Consortium init-failure dispatcher. Shadows the runtime crate's
        /// glob-imported fallback so `#[main]` routes `init()` errors here.
        #[allow(unreachable_code)]
        fn __consortium_init_fail(__consortium_err: #err_ty) -> ! {
            #handler_name(__consortium_err);
            #hard_fail
        }
    }
}

fn validate(func: &ItemFn) -> syn::Result<()> {
    match func.sig.inputs.first() {
        Some(FnArg::Typed(_)) if func.sig.inputs.len() == 1 => Ok(()),
        Some(FnArg::Receiver(recv)) => Err(syn::Error::new_spanned(
            recv,
            "`#[fail]` does not take a `self` parameter",
        )),
        _ => Err(syn::Error::new_spanned(
            &func.sig,
            "`#[fail]` requires exactly one parameter: the init-error value",
        )),
    }
}

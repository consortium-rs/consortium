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

//! The `#[consortium_runtime_{mcu,app}::main]` entry-point macros.

use consortium_macros_helpers::get_output_type;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ItemFn, spanned::Spanned};

/// Which runtime a `main` macro targets. The two flavors differ only in the
/// executor attribute they stack, the runtime crate whose fallback fail handler
/// they glob-import, and how many parameters the user's `main` takes.
#[derive(Clone, Copy)]
enum Runtime {
    /// `no_std` firmware driven by `embassy_executor`. `main` takes
    /// `(context, spawner)`.
    Mcu,
    /// Linux `std` application driven by `tokio`. `main` takes `(context)`.
    App,
}

impl Runtime {
    /// The runtime crate path whose `__rt` module supplies the fallback
    /// `__consortium_init_fail`, glob-imported so a user `#[fail]` handler
    /// shadows it.
    fn rt_crate(self) -> TokenStream2 {
        match self {
            Runtime::Mcu => quote! { ::consortium_runtime_mcu },
            Runtime::App => quote! { ::consortium_runtime_app },
        }
    }

    /// The executor attribute stacked below ours; it expands after this macro.
    fn executor_attr(self) -> TokenStream2 {
        match self {
            Runtime::Mcu => quote! { #[::embassy_executor::main] },
            Runtime::App => quote! { #[::tokio::main] },
        }
    }

    /// Code run once, *before* the generated `init()`, so bring-up logging is
    /// captured. The app installs its `tracing` subscriber here — `init()` opens
    /// the UIO devices and runs the descriptor handshakes, and a failure there is
    /// reported through the `#[fail]` handler, all before any user `main` body
    /// runs; without this, those diagnostics (and `RUST_LOG`) would be dropped
    /// because no subscriber is installed yet. The installer is idempotent, so a
    /// user `main` may still set up its own subscriber. The MCU flavor logs over
    /// `defmt` and needs no host subscriber, so this is empty for it.
    fn pre_init(self) -> TokenStream2 {
        match self {
            Runtime::Mcu => quote! {},
            Runtime::App => quote! {
                ::consortium_runtime_app::__rt::init_tracing();
            },
        }
    }

    /// Number of parameters the user's `main` must declare.
    fn arity(self) -> usize {
        match self {
            Runtime::Mcu => 2,
            Runtime::App => 1,
        }
    }
}

/// `#[consortium_runtime_mcu::main]`.
pub fn mcu_main(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    main_impl(Runtime::Mcu, attr, item)
}

/// `#[consortium_runtime_app::main]`.
pub fn app_main(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    main_impl(Runtime::App, attr, item)
}

fn main_impl(runtime: Runtime, attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    if !attr.is_empty() {
        return syn::Error::new_spanned(&attr, "`main` takes no arguments").into_compile_error();
    }

    let func = match syn::parse2::<ItemFn>(item) {
        Ok(func) => func,
        Err(err) => return err.into_compile_error(),
    };

    if let Err(err) = validate(runtime, &func) {
        return err.into_compile_error();
    }

    // The user's parameters, in order: `context` (MCU + app) then `spawner`
    // (MCU only). We bind the `init()` result to the context pattern and pass
    // the spawner parameter straight through to the executor entry fn.
    let params: Vec<&FnArg> = func.sig.inputs.iter().collect();
    let context_pat = match &params[0] {
        FnArg::Typed(pt) => &pt.pat,
        FnArg::Receiver(_) => unreachable!("validated to be a typed parameter"),
    };

    let entry_name = &func.sig.ident;
    let body = &func.block;
    let rt_crate = runtime.rt_crate();
    let executor_attr = runtime.executor_attr();
    let pre_init = runtime.pre_init();

    // The `init()` failure hop resolves `__consortium_init_fail` through this
    // glob import unless the user defined a `#[fail]` handler, whose local item
    // shadows the glob (a stable language guarantee — only a *glob* import is
    // shadowed silently; a named import would instead collide). The `#[fail]`
    // case leaves the glob unused, hence `#[allow(unused_imports)]`.
    let fail_import = quote! {
        #[allow(unused_imports)]
        use #rt_crate::__rt::*;
    };
    let dispatch = quote! {
        #pre_init
        let #context_pat = match init().await {
            ::core::result::Result::Ok(__consortium_ctx) => __consortium_ctx,
            ::core::result::Result::Err(__consortium_err) => {
                __consortium_init_fail(__consortium_err)
            }
        };
    };

    match runtime {
        Runtime::Mcu => {
            // Reuse the user's `spawner` parameter (pattern + `Spawner` type)
            // verbatim as the embassy entry fn's parameter.
            let spawner = &params[1];
            quote! {
                #fail_import
                #executor_attr
                async fn #entry_name(#spawner) {
                    #dispatch
                    #body
                }
            }
        }
        Runtime::App => quote! {
            #fail_import
            #executor_attr
            async fn #entry_name() {
                #dispatch
                #body
            }
        },
    }
}

fn validate(runtime: Runtime, func: &ItemFn) -> syn::Result<()> {
    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &func.sig,
            "`#[main]` requires an `async fn`",
        ));
    }

    let arity = runtime.arity();
    if func.sig.inputs.len() != arity {
        let expected = match runtime {
            Runtime::Mcu => "`async fn main(context: Context, spawner: Spawner)`",
            Runtime::App => "`async fn main(context: Context)`",
        };
        return Err(syn::Error::new(
            func.sig.inputs.span(),
            format!("`#[main]` expects {expected}"),
        ));
    }

    for arg in &func.sig.inputs {
        if let FnArg::Receiver(recv) = arg {
            return Err(syn::Error::new_spanned(
                recv,
                "`#[main]` does not take a `self` parameter",
            ));
        }
    }

    // `-> ()` and no return type are fine; anything else is rejected because the
    // aggregate flow owns the process/executor lifetime.
    if get_output_type(&func.sig.output).is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.output,
            "`#[main]` requires a `()` return type",
        ));
    }

    Ok(())
}

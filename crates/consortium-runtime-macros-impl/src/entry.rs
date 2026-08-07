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

use consortium_macros_helpers::{extract_result_inner, is_unit_type};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, spanned::Spanned};

#[derive(Clone, Copy)]
enum Runtime {
    Mcu,
    App,
}

impl Runtime {
    fn rt_crate(self) -> TokenStream2 {
        match self {
            Self::Mcu => quote! { ::consortium_runtime_mcu },
            Self::App => quote! { ::consortium_runtime_app },
        }
    }

    fn executor_attr(self) -> TokenStream2 {
        match self {
            Self::Mcu => quote! { #[::embassy_executor::main] },
            Self::App => quote! { #[::tokio::main] },
        }
    }

    fn arity(self) -> usize {
        match self {
            Self::Mcu => 1,
            Self::App => 0,
        }
    }
}

#[derive(Clone, Copy)]
enum OutputKind {
    Unit,
    ResultUnit,
}

pub fn mcu_main(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    main_impl(Runtime::Mcu, attr, item)
}

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

    let attrs = &func.attrs;
    let vis = &func.vis;
    let name = &func.sig.ident;
    let output = &func.sig.output;
    let body = &func.block;
    let executor = runtime.executor_attr();
    let rt = runtime.rt_crate();

    match runtime {
        Runtime::Mcu => {
            let input = func.sig.inputs.first().expect("validated MCU arity");
            let FnArg::Typed(input_typed) = input else {
                unreachable!("receivers are rejected")
            };
            let spawner_ty = &input_typed.ty;
            quote! {
                #(#attrs)*
                #executor
                #vis async fn #name(__consortium_spawner: #spawner_ty) {
                    async fn __consortium_main(#input) #output #body
                    #rt::__rt::MainOutcome::__consortium_report(
                        __consortium_main(__consortium_spawner).await,
                    );
                }
            }
        }
        Runtime::App => quote! {
            #(#attrs)*
            #executor
            #vis async fn #name() {
                async fn __consortium_main() #output #body
                #rt::__rt::MainOutcome::__consortium_report(
                    __consortium_main().await,
                );
            }
        },
    }
}

fn classify_output(output: &ReturnType) -> syn::Result<OutputKind> {
    let ReturnType::Type(_, ty) = output else {
        return Ok(OutputKind::Unit);
    };
    if is_unit_type(ty) {
        return Ok(OutputKind::Unit);
    }
    if let Some(inner) = extract_result_inner(ty) {
        if is_unit_type(inner) {
            return Ok(OutputKind::ResultUnit);
        }
        return Err(syn::Error::new_spanned(
            inner,
            "`#[main]` only accepts `Result<(), E>`",
        ));
    }
    Err(syn::Error::new_spanned(
        ty,
        "`#[main]` requires `()` or `Result<(), E>`",
    ))
}

fn validate(runtime: Runtime, func: &ItemFn) -> syn::Result<OutputKind> {
    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &func.sig,
            "`#[main]` requires an `async fn`",
        ));
    }
    if func.sig.inputs.len() != runtime.arity() {
        let expected = match runtime {
            Runtime::Mcu => "`async fn main(spawner: Spawner)`",
            Runtime::App => "`async fn main()`",
        };
        return Err(syn::Error::new(
            func.sig.inputs.span(),
            format!("`#[main]` expects {expected}"),
        ));
    }
    for arg in &func.sig.inputs {
        if let FnArg::Receiver(receiver) = arg {
            return Err(syn::Error::new_spanned(
                receiver,
                "`#[main]` does not take a `self` parameter",
            ));
        }
    }
    classify_output(&func.sig.output)
}

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

//! Snapshot coverage for the runtime entry macros. Each test renders the impl
//! output with `prettyplease` so intentional changes surface as reviewable
//! diffs. Error paths assert the emitted `compile_error!` token.

use consortium_runtime_macros_impl::{app_fail, app_main, mcu_fail, mcu_main};
use proc_macro2::TokenStream;
use quote::quote;

fn render(tokens: TokenStream) -> String {
    let file = syn::parse2::<syn::File>(tokens).expect("macro output should be a parseable file");
    prettyplease::unparse(&file)
}

#[test]
fn mcu_main_wraps_embassy_and_binds_context() {
    let output = mcu_main(
        quote! {},
        quote! {
            async fn main(context: Context, spawner: Spawner) {
                context.ipc_shm.sensor.send(&reading).await.ok();
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn app_main_wraps_tokio_and_binds_context() {
    let output = app_main(
        quote! {},
        quote! {
            async fn main(context: Context) {
                loop {
                    context.ipc_shm.sensor.recv().await.ok();
                }
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn mcu_fail_emits_shadowing_dispatcher() {
    let output = mcu_fail(
        quote! {},
        quote! {
            fn on_fail(err: ConsortiumInitError) {
                defmt::error!("init failed: {}", err);
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn app_fail_emits_shadowing_dispatcher() {
    let output = app_fail(
        quote! {},
        quote! {
            fn on_fail(err: ConsortiumInitError) -> ! {
                panic!("init failed: {err:?}");
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn main_rejects_non_async() {
    let output = mcu_main(
        quote! {},
        quote! {
            fn main(context: Context, spawner: Spawner) {}
        },
    );
    assert!(output.to_string().contains("async fn"));
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn mcu_main_rejects_wrong_arity() {
    let output = mcu_main(
        quote! {},
        quote! {
            async fn main(context: Context) {}
        },
    );
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn app_main_rejects_wrong_arity() {
    let output = app_main(
        quote! {},
        quote! {
            async fn main(context: Context, spawner: Spawner) {}
        },
    );
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn main_rejects_non_unit_return() {
    let output = app_main(
        quote! {},
        quote! {
            async fn main(context: Context) -> u32 { 0 }
        },
    );
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn main_rejects_attribute_args() {
    let output = app_main(
        quote! { foo = bar },
        quote! {
            async fn main(context: Context) {}
        },
    );
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn fail_rejects_wrong_arity() {
    let output = mcu_fail(
        quote! {},
        quote! {
            fn on_fail() {}
        },
    );
    assert!(output.to_string().contains("compile_error"));
}

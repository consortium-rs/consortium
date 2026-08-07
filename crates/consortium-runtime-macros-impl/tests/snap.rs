// Copyright 2026 Ethan Wu
// SPDX-License-Identifier: Apache-2.0

use consortium_runtime_macros_impl::{app_main, mcu_main};
use proc_macro2::TokenStream;
use quote::quote;

fn render(tokens: TokenStream) -> String {
    let file = syn::parse2::<syn::File>(tokens).expect("macro output should be parseable");
    prettyplease::unparse(&file)
}

#[test]
fn mcu_main_wraps_embassy_and_preserves_spawner_pattern() {
    let output = mcu_main(
        quote! {},
        quote! {
            async fn main(_spawner: Spawner) {
                let (_p, _c) = init(Config::default());
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn app_main_wraps_tokio_without_parameters() {
    let output = app_main(
        quote! {},
        quote! {
            async fn main() {
                let (_p, _c) = init(Config::default()).unwrap();
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn mcu_main_accepts_result_unit() {
    let output = mcu_main(
        quote! {},
        quote! {
            async fn main(spawner: Spawner) -> Result<(), ConsortiumError> {
                let _ = spawner;
                Ok(())
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn app_main_accepts_result_unit() {
    let output = app_main(
        quote! {},
        quote! {
            async fn main() -> Result<(), ConsortiumError> {
                Ok(())
            }
        },
    );
    insta::assert_snapshot!(render(output));
}

#[test]
fn main_rejects_non_async() {
    let output = mcu_main(quote! {}, quote! { fn main(spawner: Spawner) {} });
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn mcu_main_rejects_wrong_arity() {
    let output = mcu_main(quote! {}, quote! { async fn main() {} });
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn app_main_rejects_wrong_arity() {
    let output = app_main(quote! {}, quote! { async fn main(spawner: Spawner) {} });
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn main_rejects_non_unit_return() {
    let output = app_main(quote! {}, quote! { async fn main() -> u32 { 0 } });
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn main_rejects_result_non_unit() {
    let output = app_main(
        quote! {},
        quote! { async fn main() -> Result<u32, Error> { Ok(0) } },
    );
    assert!(output.to_string().contains("compile_error"));
}

#[test]
fn main_rejects_attribute_args() {
    let output = app_main(quote! { foo = bar }, quote! { async fn main() {} });
    assert!(output.to_string().contains("compile_error"));
}

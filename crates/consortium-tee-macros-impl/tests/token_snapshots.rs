use proc_macro2::TokenStream;
use quote::quote;

fn pretty_snapshot(tokens: TokenStream) -> String {
    let file = syn::parse_file(&tokens.to_string()).expect("macro output should parse as Rust");
    prettyplease::unparse(&file)
}

#[test]
fn tee_service_expansion_is_snapshot_without_optee_deps() {
    let expanded = consortium_tee_macros_impl::tee_service(quote! {
        context ServiceContext
        commands { ping, echo_bytes }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_command_expansion_is_snapshot_without_optee_deps() {
    let expanded = consortium_tee_macros_impl::tee_command(
        quote! { codec = TestCodec },
        quote! {
            fn echo(
                ctx: &mut ServiceContext,
                input: &[u8],
                counter: &mut u64,
            ) -> ::consortium_tee::TeeResult<Vec<u8>> {
                *counter += 1;
                Ok(input.to_vec())
            }
        },
    );

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_derive_expansion_is_snapshot_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        struct CommandPayload<T> {
            opcode: u32,
            body: T,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_max_size_override_is_snapshot_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        #[tee(max_size = 4096)]
        struct LargePayload {
            body: u32,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_empty_attribute_defaults_max_size_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        #[tee]
        struct DefaultPayload {
            body: u32,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_invalid_attribute_error_is_snapshot_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        #[tee(buffer = 4096)]
        struct BadAttribute {
            body: Vec<u8>,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_widening_warnings_are_snapshot_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        enum WarningPayload {
            Small {
                a: u8,
                b: u16,
                c: i8,
                d: i16,
                e: char,
            },
            Nested(Option<u8>),
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_param_rejection_errors_are_snapshot_without_codec_deps() {
    let expanded = consortium_tee_macros_impl::derive_tee_param(quote! {
        struct RejectedPayload<'a> {
            raw: *const u8,
            borrowed: &'a u32,
            callback: fn(u32) -> u32,
            platform_width: Option<usize>,
            float_value: f32,
            too_wide: u128,
            erased: dyn Send,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_command_too_many_parameters_error_is_snapshot_without_optee_deps() {
    let expanded = consortium_tee_macros_impl::tee_command(
        quote! {},
        quote! {
            fn crowded(a: u32, b: u32, c: u32, d: u32, e: u32) {}
        },
    );

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn tee_command_unsupported_type_error_is_snapshot_without_optee_deps() {
    let expanded = consortium_tee_macros_impl::tee_command(
        quote! {},
        quote! {
            fn unsupported(input: (u32, u64)) -> (u32, u64) {
                input
            }
        },
    );

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

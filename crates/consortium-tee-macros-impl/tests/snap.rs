use proc_macro2::TokenStream;
use quote::quote;

fn fmt(tokens: &TokenStream) -> String {
    match syn::parse_file(&tokens.to_string()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => format!("{tokens}\n"),
    }
}

fn snapshot_derive(input: TokenStream) -> String {
    let output = consortium_tee_macros_impl::derive_tee_param(input.clone());
    format!(
        "// Input:\n#[derive(consortium_tee::TeeParam)]\n{}\n// Expands to:\n{}",
        fmt(&input),
        fmt(&output)
    )
}

fn snapshot_service(input: TokenStream) -> String {
    let output = consortium_tee_macros_impl::tee_service(input.clone());
    let input_code = quote! {
        consortium_tee::tee_service! {
            #input
        }
    };
    format!(
        "// Input:\n{}\n// Expands to:\n{}",
        fmt(&input_code),
        fmt(&output)
    )
}

fn snapshot_command(attr: TokenStream, item: TokenStream) -> String {
    let output = consortium_tee_macros_impl::tee_command(attr.clone(), item.clone());
    let attr_line = if attr.is_empty() {
        "#[consortium_tee::tee_command]\n".to_string()
    } else {
        format!("#[consortium_tee::tee_command({attr})]\n")
    };
    format!(
        "// Input:\n{}{}\n// Expands to:\n{}",
        attr_line,
        fmt(&item),
        fmt(&output)
    )
}

#[test]
fn tee_service_expansion() {
    insta::assert_snapshot!(snapshot_service(quote! {
        context ServiceContext
        commands { ping, echo_bytes }
    }));
}

#[test]
fn tee_service_expansion_no_ctx() {
    insta::assert_snapshot!(snapshot_service(quote! {
        commands { ping, echo_bytes, hello }
    }));
}

#[test]
fn tee_service_fail_unknown_conflict() {
    insta::assert_snapshot!(snapshot_service(quote! {
        commands { hello, world, unknown, conflict }
    }));
}

#[test]
fn tee_command_expansion() {
    insta::assert_snapshot!(snapshot_command(
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
    ));
}

#[test]
fn tee_param_derive_expansion() {
    insta::assert_snapshot!(snapshot_derive(quote! {
        struct CommandPayload<T> {
            opcode: u32,
            body: T,
        }
    }));
}

#[test]
fn tee_param_max_size_override() {
    insta::assert_snapshot!(snapshot_derive(quote! {
        #[tee(max_size = 4096)]
        struct LargePayload {
            body: u32,
        }
    }));
}

#[test]
fn tee_param_empty_attribute_defaults_max_size() {
    insta::assert_snapshot!(snapshot_derive(quote! {
        #[tee]
        struct DefaultPayload {
            body: u32,
        }
    }));
}

#[test]
fn tee_param_invalid_attribute_error() {
    insta::assert_snapshot!(snapshot_derive(quote! {
        #[tee(buffer = 4096)]
        struct BadAttribute {
            body: Vec<u8>,
        }
    }));
}

#[test]
fn tee_param_widening_warnings() {
    insta::assert_snapshot!(snapshot_derive(quote! {
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
    }));
}

#[test]
fn tee_param_rejection_errors() {
    insta::assert_snapshot!(snapshot_derive(quote! {
        struct RejectedPayload<'a> {
            raw: *const u8,
            borrowed: &'a u32,
            callback: fn(u32) -> u32,
            platform_width: Option<usize>,
            float_value: f32,
            too_wide: u128,
            erased: dyn Send,
        }
    }));
}

#[test]
fn tee_command_too_many_parameters_error() {
    insta::assert_snapshot!(snapshot_command(
        quote! {},
        quote! {
            fn crowded(a: u32, b: u32, c: u32, d: u32, e: u32) {}
        },
    ));
}

#[test]
fn tee_command_unsupported_type_error() {
    insta::assert_snapshot!(snapshot_command(
        quote! {},
        quote! {
            fn unsupported(input: (u32, u64)) -> (u32, u64) {
                input
            }
        },
    ));
}

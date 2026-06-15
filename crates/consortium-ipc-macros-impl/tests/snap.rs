use proc_macro2::TokenStream;
use quote::quote;

fn snapshot(input: TokenStream) -> String {
    let output = consortium_ipc_macros_impl::derive_ipc_safe(input.clone());

    let input_file = syn::parse_file(&input.to_string()).expect("input should parse as Rust");
    let output_file =
        syn::parse_file(&output.to_string()).expect("macro output should parse as Rust");

    format!(
        "// Input:\n#[derive(consortium_ipc::IpcSafe)]\n{}\n// Expands to:\n{}",
        prettyplease::unparse(&input_file),
        prettyplease::unparse(&output_file),
    )
}

// ── success cases ─────────────────────────────────────────────────────────────

#[test]
fn struct_plain() {
    insta::assert_snapshot!(snapshot(quote! {
        struct SensorReading {
            sensor_id: u32,
            value: f32,
            flags: u8,
        }
    }));
}

#[test]
fn struct_generic() {
    insta::assert_snapshot!(snapshot(quote! {
        struct Wrapper<T> {
            value: T,
            tag: u32,
        }
    }));
}

#[test]
fn enum_plain() {
    insta::assert_snapshot!(snapshot(quote! {
        enum Status {
            Idle,
            Running,
            Error(u32),
        }
    }));
}

#[test]
fn struct_no_fields() {
    insta::assert_snapshot!(snapshot(quote! {
        struct Empty;
    }));
}

#[test]
fn struct_array_field() {
    insta::assert_snapshot!(snapshot(quote! {
        struct Buffer {
            data: [u32; 16],
        }
    }));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn raw_ptr_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct HasPtr {
            ptr: *const u8,
        }
    }));
}

#[test]
fn reference_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct HasRef<'a> {
            r: &'a u32,
        }
    }));
}

#[test]
fn fn_ptr_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct HasFnPtr {
            cb: fn(u32) -> u32,
        }
    }));
}

#[test]
fn usize_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct HasUsize {
            len: usize,
        }
    }));
}

#[test]
fn nested_usize_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct HasNestedUsize {
            data: [usize; 4],
        }
    }));
}

#[test]
fn multiple_errors() {
    insta::assert_snapshot!(snapshot(quote! {
        struct MultipleProblems {
            ptr: *const u8,
            len: usize,
        }
    }));
}

use proc_macro2::TokenStream;
use quote::quote;

fn snapshot(input: TokenStream) -> String {
    let output = consortium_codec_macros_impl::derive_trusted_archive(input.clone());

    let input_file = syn::parse_file(&input.to_string()).expect("input should parse as Rust");
    let output_file =
        syn::parse_file(&output.to_string()).expect("macro output should parse as Rust");

    format!(
        "// Input:\n#[derive(consortium_codec::TrustedArchive)]\n{}\n// Expands to:\n{}",
        prettyplease::unparse(&input_file),
        prettyplease::unparse(&output_file),
    )
}

// ── success cases ─────────────────────────────────────────────────────────────

#[test]
fn struct_repr_c() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(C)]
        struct Point {
            x: u32,
            y: u32,
        }
    }));
}

#[test]
fn struct_repr_align() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(align(16))]
        struct Aligned {
            data: [u8; 16],
        }
    }));
}

#[test]
fn struct_repr_c_align_combined() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(C, align(16))]
        struct AlignedC {
            a: u32,
            b: u64,
        }
    }));
}

#[test]
fn struct_generic() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(C)]
        struct Wrapper<T> {
            value: T,
            count: u32,
        }
    }));
}

#[test]
fn struct_no_fields() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(C)]
        struct Empty;
    }));
}

#[test]
fn enum_repr_u8() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(u8)]
        enum Status {
            Idle,
            Running,
            Error(u32),
        }
    }));
}

#[test]
fn enum_repr_c() {
    insta::assert_snapshot!(snapshot(quote! {
        #[repr(C)]
        enum Direction {
            North,
            South,
            East,
            West,
        }
    }));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn struct_missing_repr_error() {
    insta::assert_snapshot!(snapshot(quote! {
        struct NakedStruct {
            x: u32,
        }
    }));
}

#[test]
fn enum_missing_repr_error() {
    insta::assert_snapshot!(snapshot(quote! {
        enum NakedEnum {
            A,
            B,
        }
    }));
}

use proc_macro2::TokenStream;
use quote::quote;

fn pretty_snapshot(tokens: TokenStream) -> String {
    let file = syn::parse_file(&tokens.to_string()).expect("macro output should parse as Rust");
    prettyplease::unparse(&file)
}

// ── success cases ─────────────────────────────────────────────────────────────

#[test]
fn struct_repr_c() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(C)]
        struct Point {
            x: u32,
            y: u32,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn struct_repr_align() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(align(16))]
        struct Aligned {
            data: [u8; 16],
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn struct_repr_c_align_combined() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(C, align(16))]
        struct AlignedC {
            a: u32,
            b: u64,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn struct_generic() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(C)]
        struct Wrapper<T> {
            value: T,
            count: u32,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn struct_no_fields() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(C)]
        struct Empty;
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn enum_repr_u8() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(u8)]
        enum Status {
            Idle,
            Running,
            Error(u32),
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn enum_repr_c() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        #[repr(C)]
        enum Direction {
            North,
            South,
            East,
            West,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn struct_missing_repr_error() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        struct NakedStruct {
            x: u32,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

#[test]
fn enum_missing_repr_error() {
    let expanded = consortium_codec_macros_impl::derive_trusted_archive(quote! {
        enum NakedEnum {
            A,
            B,
        }
    });

    insta::assert_snapshot!(pretty_snapshot(expanded));
}

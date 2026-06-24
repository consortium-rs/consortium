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

use consortium_macros_helpers::collect_field_types;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, GenericParam, parse_quote};

/// Derive macro for `consortium_codec::rkyv::TrustedArchive`.
///
/// Two invariants are checked at compile time:
///
/// 1. **Stable repr** — the type must declare a layout that makes its rkyv archived form safe
///    to access without validation (`access_unchecked`):
///    - Structs and unions: `#[repr(C)]` or `#[repr(align(N))]`
///    - Enums: `#[repr(u8)]`, `#[repr(u16)]`, `#[repr(u32)]`, `#[repr(u64)]`,
///      `#[repr(i8)]`, `#[repr(i16)]`, `#[repr(i32)]`, `#[repr(i64)]`, or `#[repr(C)]`
///
/// 2. **Field bounds** — every field type must itself satisfy `TrustedArchive`, enforced via
///    generated `where`-clause predicates. Generic type parameters receive the same bound.
///
/// The generated `unsafe impl` includes a short invariant note for auditors.
pub fn derive_trusted_archive(input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2::<DeriveInput>(input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    derive_trusted_archive_impl(input).unwrap_or_else(|e| e.into_compile_error())
}

struct ReprInfo {
    has_c: bool,
    has_align: bool,
    has_integer: bool,
}

fn parse_repr(attrs: &[Attribute]) -> syn::Result<ReprInfo> {
    let mut info = ReprInfo {
        has_c: false,
        has_align: false,
        has_integer: false,
    };

    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("C") {
                info.has_c = true;
            } else if meta.path.is_ident("align") || meta.path.is_ident("packed") {
                // Consume the parenthesized argument: `align(16)` or `packed(2)`.
                if meta.input.peek(syn::token::Paren) {
                    let content;
                    syn::parenthesized!(content in meta.input);
                    let _: proc_macro2::TokenStream = content.parse()?;
                }
                if meta.path.is_ident("align") {
                    info.has_align = true;
                }
            } else if let Some(ident) = meta.path.get_ident()
                && matches!(
                    ident.to_string().as_str(),
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64"
                )
            {
                info.has_integer = true;
            }
            Ok(())
        })?;
    }

    Ok(info)
}

fn check_repr(attrs: &[Attribute], span: Span, is_enum: bool) -> syn::Result<()> {
    let repr = parse_repr(attrs)?;

    if is_enum {
        if !repr.has_integer && !repr.has_c {
            return Err(syn::Error::new(
                span,
                "TrustedArchive enums must carry an integer repr (#[repr(u8)], #[repr(u16)], \
                 etc.) or #[repr(C)] to guarantee a stable discriminant in the archived form",
            ));
        }
    } else if !repr.has_c && !repr.has_align {
        return Err(syn::Error::new(
            span,
            "TrustedArchive structs must carry #[repr(C)] or #[repr(align(N))] to guarantee \
             stable memory layout for unchecked archive access",
        ));
    }

    Ok(())
}

fn derive_trusted_archive_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let is_enum = matches!(input.data, Data::Enum(_));

    check_repr(&input.attrs, name.span(), is_enum)?;

    let field_types = collect_field_types(&input.data);

    // Propagate the bound to existing generic type parameters so that
    // `Wrapper<T>` only compiles when `T: TrustedArchive`.
    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(tp) = param {
            tp.bounds
                .push(parse_quote!(::consortium_codec::TrustedArchive));
        }
    }

    // Emit an explicit where-clause predicate for every distinct field type.
    // Concrete types (e.g. `u32`) are trivially satisfied; generic types
    // (e.g. `T` or `[T; 4]`) become constraints visible to callers.
    let where_clause = generics.make_where_clause();
    for ty in &field_types {
        where_clause
            .predicates
            .push(parse_quote!(#ty: ::consortium_codec::TrustedArchive));
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        // SAFETY: the derive macro verified that this type carries a stable repr
        // (#[repr(C)] or #[repr(align(N))] for structs/unions; integer or #[repr(C)]
        // for enums) and that every field type satisfies TrustedArchive, so the rkyv
        // archived form is safe to access without validation via `access_unchecked`.
        unsafe impl #impl_generics ::consortium_codec::TrustedArchive
            for #name #ty_generics
            #where_clause
        {}
    })
}

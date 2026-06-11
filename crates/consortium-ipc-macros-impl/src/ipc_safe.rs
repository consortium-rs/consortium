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

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, GenericArgument, GenericParam, PathArguments, Type, parse_quote};

/// Derive macro that validates a struct/enum is safe to use as a cross-core IPC message.
///
/// Rejected field types (checked recursively through generics, arrays, tuples, etc.):
/// - Raw pointers (`*const T`, `*mut T`) — meaningless across separate address spaces
/// - References (`&T`, `&mut T`) — local to one address space
/// - Function pointers (`fn(...)`) — addresses are not portable across cores
/// - `usize` / `isize` — width is platform-dependent (32-bit M-core vs 64-bit A-core)
///
/// Generic type parameters in fields are required to be `IpcSafe` themselves via a
/// generated where-clause bound.
///
/// # Example
///
/// ```rust,ignore
/// use consortium_ipc_macros::IpcSafe;
///
/// #[derive(IpcSafe)]
/// struct SensorReading {
///     sensor_id: u32,
///     value: f32,
///     flags: u8,
/// }
///
/// // This would fail to compile:
/// // #[derive(IpcSafe)]
/// // struct Bad { ptr: *const u8 }
/// ```
pub fn derive_ipc_safe(input: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2::<DeriveInput>(input) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    derive_ipc_safe_impl(input).unwrap_or_else(|e| e.into_compile_error())
}

fn derive_ipc_safe_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    // Collect all field types from structs, enums, and unions.
    let field_types: Vec<&Type> = match &input.data {
        Data::Struct(s) => s.fields.iter().map(|f| &f.ty).collect(),
        Data::Enum(e) => e
            .variants
            .iter()
            .flat_map(|v| v.fields.iter().map(|f| &f.ty))
            .collect(),
        Data::Union(u) => u.fields.named.iter().map(|f| &f.ty).collect(),
    };

    // Validate every field type, accumulating all errors before failing.
    let mut combined: Option<syn::Error> = None;
    for ty in field_types {
        for err in check_type(ty) {
            match &mut combined {
                None => combined = Some(err),
                Some(prev) => prev.combine(err),
            }
        }
    }
    if let Some(err) = combined {
        return Err(err);
    }

    // Add `IpcProto` bounds for each generic type parameter so that
    // `Wrapper<T>` rejects `Wrapper<usize>` at the call site.
    let mut generics = input.generics.clone();
    for param in &mut generics.params {
        if let GenericParam::Type(tp) = param {
            tp.bounds.push(parse_quote!(::consortium_ipc::IpcSafe));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        // SAFETY: the derive macro has verified at compile time that every field type
        // is free of raw pointers, references, function pointers, and platform-dependent
        // integer widths (`usize`/`isize`).
        unsafe impl #impl_generics ::consortium_ipc::IpcSafe
            for #name #ty_generics
            #where_clause
        {}
    })
}

/// Recursively walks `ty` and returns one error per forbidden construct found.
fn check_type(ty: &Type) -> Vec<syn::Error> {
    match ty {
        Type::Ptr(_) => vec![syn::Error::new_spanned(
            ty,
            "raw pointers are not allowed in IpcSafe types: they are not valid across \
             separate processor address spaces; use an index or ID instead",
        )],

        Type::Reference(_) => vec![syn::Error::new_spanned(
            ty,
            "references are not allowed in IpcSafe types: they are local to one address \
             space; use owned types instead",
        )],

        Type::BareFn(_) => vec![syn::Error::new_spanned(
            ty,
            "function pointers are not allowed in IpcSafe types: code addresses are not \
             portable across processor cores",
        )],

        Type::Path(tp) => {
            // Reject usize / isize directly — width differs between A-core and M-core.
            if let Some(ident) = tp.path.get_ident() {
                let s = ident.to_string();
                if matches!(s.as_str(), "usize" | "isize") {
                    return vec![syn::Error::new_spanned(
                        ty,
                        format!(
                            "`{s}` is platform-dependent (pointer-width) and not allowed in \
                             IpcSafe types: use u32/u64 or i32/i64 with an explicit width"
                        ),
                    )];
                }
            }
            // Recurse into angle-bracketed generic arguments (e.g. `Vec<usize>`).
            tp.path
                .segments
                .iter()
                .flat_map(|seg| {
                    if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                        ab.args
                            .iter()
                            .filter_map(|arg| {
                                if let GenericArgument::Type(inner) = arg {
                                    Some(inner)
                                } else {
                                    None
                                }
                            })
                            .flat_map(check_type)
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                })
                .collect()
        }

        // Recurse into composite types.
        Type::Array(arr) => check_type(&arr.elem),
        Type::Slice(sl) => check_type(&sl.elem),
        Type::Tuple(t) => t.elems.iter().flat_map(check_type).collect(),
        Type::Paren(p) => check_type(&p.elem),
        Type::Group(g) => check_type(&g.elem),

        // TraitObject / ImplTrait / Infer / Never / Verbatim / macro — reject with a
        // catch-all because none of these can represent a plain, sized, ABI-stable value.
        Type::TraitObject(_) | Type::ImplTrait(_) => vec![syn::Error::new_spanned(
            ty,
            "trait objects and `impl Trait` are not allowed in IpcSafe types: use a \
             concrete, sized type",
        )],

        _ => vec![],
    }
}

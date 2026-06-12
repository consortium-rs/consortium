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

use proc_macro_warning::Warning;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens as _, quote};
use syn::spanned::Spanned as _;
use syn::{
    Attribute, Data, DeriveInput, GenericArgument, GenericParam, Ident, LitInt, Meta,
    PathArguments, Type, parse_quote,
};

const DEFAULT_MAX_SIZE: &str = "1024";

/// Derive macro for `consortium_tee::TeeParam`.
///
/// Validates that the type's fields are safe for TEE parameter serialization — the same
/// structural rules as `IpcSafe`:
/// - No raw pointers (`*const T`, `*mut T`)
/// - No bare references as fields (`&T`, `&mut T`)
/// - No function pointers (`fn(…)`)
/// - No `usize` / `isize` (pointer-width differs between 32-bit TA and 64-bit CA)
///
/// Generates `impl<C> TeeParam<C> for T` where `C` is constrained by
/// `CodecFor<T, Decoded<'_> = T>`. The codec is NOT fixed at derive time — it is chosen
/// at the `#[tee_command(codec = …)]` call site, keeping `TeeParam` codec-agnostic.
///
/// Override `MAX_SIZE` with `#[tee(max_size = N)]` if the type's worst-case serialized
/// size differs from the default of 1024 bytes.
pub fn derive_tee_param(item: TokenStream2) -> TokenStream2 {
    let input = match syn::parse2::<DeriveInput>(item) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    derive_tee_param_impl(input).unwrap_or_else(|e| e.into_compile_error())
}

fn derive_tee_param_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let max_size = parse_tee_max_size(&input.attrs)?;

    // Collect all field types for validation.
    let field_types: Vec<&Type> = match &input.data {
        Data::Struct(s) => s.fields.iter().map(|f| &f.ty).collect(),
        Data::Enum(e) => e
            .variants
            .iter()
            .flat_map(|v| v.fields.iter().map(|f| &f.ty))
            .collect(),
        Data::Union(u) => u.fields.named.iter().map(|f| &f.ty).collect(),
    };

    // Validate all field types, accumulating errors and warnings before failing.
    let mut combined: Option<syn::Error> = None;
    let mut all_warnings: Vec<TokenStream2> = vec![];
    for ty in field_types {
        let (errs, warns) = check_tee_type(ty);
        for err in errs {
            match &mut combined {
                None => combined = Some(err),
                Some(prev) => prev.combine(err),
            }
        }
        all_warnings.extend(warns);
    }
    if let Some(err) = combined {
        return Err(err);
    }
    all_warnings.sort_by_key(|warning| warning.to_string());
    all_warnings.dedup_by_key(|warning| warning.to_string());

    // Keep the original ty_generics (without the injected codec param) for the
    // `for #name #orig_ty_generics` part of the impl.
    let (_, orig_ty_generics, _) = input.generics.split_for_impl();

    // Build the impl generics by cloning the original and injecting the codec param.
    let codec_param = Ident::new("__TeeCodec", Span::call_site());
    let mut generics = input.generics.clone();

    // Add `TeeParam<__TeeCodec>` bounds to each existing type parameter so that
    // `Wrapper<T>` only compiles when `T: TeeParam<C>`.
    for param in &mut generics.params {
        if let GenericParam::Type(tp) = param {
            tp.bounds
                .push(parse_quote!(::consortium_tee::TeeParam<#codec_param>));
        }
    }

    // Inject the codec type parameter after the existing params.
    generics.params.push(parse_quote!(#codec_param));

    // Add where-clause bounds: CodecFor (for serialization) and 'static (so
    // Decoded<'_> = Self can be resolved for owned types).
    let where_clause = generics.make_where_clause();
    where_clause.predicates.push(parse_quote!(
        #codec_param: for<'__buf> ::consortium_codec::CodecFor<
            #name #orig_ty_generics,
            Decoded<'__buf> = #name #orig_ty_generics,
        >
    ));
    where_clause
        .predicates
        .push(parse_quote!(#name #orig_ty_generics: 'static));

    // impl_generics: <T, __TeeCodec>  (and the where clause)
    // ty_generics from the modified set would include __TeeCodec, which is wrong for
    // the `for Type` part, so we use orig_ty_generics there.
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #(#all_warnings)*
        impl #impl_generics ::consortium_tee::TeeParam<#codec_param>
            for #name #orig_ty_generics
            #where_clause
        {
            const MAX_SIZE: usize = #max_size;

            fn into_tee_param(
                &self,
            ) -> ::core::result::Result<::consortium_tee::TeeParamRepr, ::consortium_tee::TeeParamError>
            {
                let mut __buf = ::alloc::vec![0u8; <Self as ::consortium_tee::TeeParam<#codec_param>>::MAX_SIZE];
                <#codec_param as ::consortium_codec::CodecFor<Self>>::encode(self, &mut __buf)
                    .map(|__len| {
                        __buf.truncate(__len);
                        ::consortium_tee::TeeParamSlot::Serialized(__buf)
                    })
                    .map_err(|_| ::consortium_tee::TeeParamError::EncodeFailed)
            }

            fn from_tee_param(
                repr: ::consortium_tee::TeeParamReprRef<'_>,
            ) -> ::core::result::Result<Self, ::consortium_tee::TeeParamError>
            {
                match repr {
                    ::consortium_tee::TeeParamSlot::Serialized(__bytes) => {
                        <#codec_param as ::consortium_codec::CodecFor<Self>>::decode(__bytes)
                            .map_err(|_| ::consortium_tee::TeeParamError::DecodeFailed)
                    }
                    ::consortium_tee::TeeParamSlot::Primitive { .. } => {
                        Err(::consortium_tee::TeeParamError::DecodeFailed)
                    }
                }
            }
        }
    })
}

fn parse_tee_max_size(attrs: &[Attribute]) -> syn::Result<LitInt> {
    let mut max_size = None;

    for attr in attrs {
        if !attr.path().is_ident("tee") {
            continue;
        }
        if matches!(attr.meta, Meta::Path(_)) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("max_size") {
                return Err(meta.error("expected `max_size`"));
            }
            if max_size.is_some() {
                return Err(meta.error("duplicate `max_size`"));
            }

            let value = meta.value()?;
            let lit: LitInt = value.parse()?;
            lit.base10_parse::<usize>()?;
            max_size = Some(lit);
            Ok(())
        })?;
    }

    Ok(max_size.unwrap_or_else(|| LitInt::new(DEFAULT_MAX_SIZE, Span::call_site())))
}

/// Recursively checks `ty` for TEE-unsafe or lossy constructs.
///
/// Returns `(errors, warnings)` where warnings are emitted via `proc_macro_warning` and
/// included in the generated output.
///
/// | Category         | Types                          | Action  |
/// |-----------------|-------------------------------|---------|
/// | Auto-widen      | `u8`, `u16` → `u32`           | Warning |
/// | Auto-widen      | `i8`, `i16` → `i32` → `u32`  | Warning |
/// | Auto-widen      | `char` → `u32` scalar         | Warning |
/// | Native          | `u32`, `i32`, `u64`, `i64`, `bool`, `(u32, u32)` | OK |
/// | Ambiguous float | `f32`, `f64`                  | Error   |
/// | Too wide        | `u128`, `i128`                | Error   |
/// | Platform-dep    | `usize`, `isize`              | Error   |
/// | Unsendable      | raw ptrs, refs, fn ptrs       | Error   |
/// | Not concrete    | trait objects, `impl Trait`   | Error   |
fn check_tee_type(ty: &Type) -> (Vec<syn::Error>, Vec<TokenStream2>) {
    let mut errors: Vec<syn::Error> = vec![];
    let mut warnings: Vec<TokenStream2> = vec![];

    match ty {
        Type::Ptr(_) => errors.push(syn::Error::new_spanned(
            ty,
            "raw pointers are not allowed in TeeParam types: they cannot be serialized \
             across the TEE secure-world boundary",
        )),

        Type::Reference(_) => errors.push(syn::Error::new_spanned(
            ty,
            "bare references are not allowed as TeeParam fields: use owned types instead",
        )),

        Type::BareFn(_) => errors.push(syn::Error::new_spanned(
            ty,
            "function pointers are not allowed in TeeParam types: they are not serializable",
        )),

        Type::Path(tp) => {
            if let Some(ident) = tp.path.get_ident() {
                match ident.to_string().as_str() {
                    "usize" | "isize" => {
                        errors.push(syn::Error::new_spanned(
                            ty,
                            format!(
                                "`{}` is pointer-width dependent and not allowed in TeeParam \
                                 types: the TA may be 32-bit while the CA is 64-bit; \
                                 use `u32`/`u64` or `i32`/`i64`",
                                ident
                            ),
                        ));
                        return (errors, warnings);
                    }
                    "f32" | "f64" => {
                        errors.push(syn::Error::new_spanned(
                            ty,
                            format!(
                                "TEE value slots are `uint32_t` pairs; `{}` encoding is \
                                 ambiguous across TEE boundaries; use `u32` directly or \
                                 implement `TeeParam` with explicit encoding",
                                ident
                            ),
                        ));
                        return (errors, warnings);
                    }
                    "u128" | "i128" => {
                        errors.push(syn::Error::new_spanned(
                            ty,
                            format!(
                                "`{}` does not fit in two `u32` TEE Value fields; \
                                 wrap it in a newtype and implement `TeeParam` using a \
                                 Memref (serialized) slot",
                                ident
                            ),
                        ));
                        return (errors, warnings);
                    }
                    s @ ("u8" | "u16") => {
                        let old = format!("`{s}` as a TeeParam field");
                        let new = format!(
                            "`{s}` is widened to `u32` in the TEE Value slot (lossless); \
                             use `u32` directly to silence this warning"
                        );
                        let warning_name = format!("tee_param_implicit_widen_{s}");
                        warnings.push(
                            Warning::new_deprecated(&warning_name)
                                .old(&old)
                                .new(&new)
                                .span(ty.span())
                                .build()
                                .to_token_stream(),
                        );
                    }
                    s @ ("i8" | "i16") => {
                        let old = format!("`{s}` as a TeeParam field");
                        let new = format!(
                            "`{s}` is sign-extended to `i32` then bit-cast to `u32` \
                             in the TEE Value slot (lossless); use `i32` directly \
                             to silence this warning"
                        );
                        let warning_name = format!("tee_param_implicit_widen_{s}");
                        warnings.push(
                            Warning::new_deprecated(&warning_name)
                                .old(&old)
                                .new(&new)
                                .span(ty.span())
                                .build()
                                .to_token_stream(),
                        );
                    }
                    "char" => {
                        warnings.push(
                            Warning::new_deprecated("tee_param_implicit_widen_char")
                                .old("`char` as a TeeParam field")
                                .new(
                                    "`char` is mapped to its Unicode scalar value (`u32`) in the \
                                     TEE Value slot; use `u32` directly to silence this warning",
                                )
                                .span(ty.span())
                                .build()
                                .to_token_stream(),
                        );
                    }
                    _ => {}
                }
            }
            // Recurse into angle-bracketed generic arguments (e.g. `Option<u8>`).
            for seg in &tp.path.segments {
                if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                    for arg in &ab.args {
                        if let GenericArgument::Type(inner) = arg {
                            let (e, w) = check_tee_type(inner);
                            errors.extend(e);
                            warnings.extend(w);
                        }
                    }
                }
            }
        }

        Type::Array(arr) => {
            let (e, w) = check_tee_type(&arr.elem);
            errors.extend(e);
            warnings.extend(w);
        }
        Type::Slice(sl) => {
            let (e, w) = check_tee_type(&sl.elem);
            errors.extend(e);
            warnings.extend(w);
        }
        Type::Tuple(t) => {
            for elem in &t.elems {
                let (e, w) = check_tee_type(elem);
                errors.extend(e);
                warnings.extend(w);
            }
        }
        Type::Paren(p) => {
            let (e, w) = check_tee_type(&p.elem);
            errors.extend(e);
            warnings.extend(w);
        }
        Type::Group(g) => {
            let (e, w) = check_tee_type(&g.elem);
            errors.extend(e);
            warnings.extend(w);
        }

        Type::TraitObject(_) | Type::ImplTrait(_) => errors.push(syn::Error::new_spanned(
            ty,
            "trait objects and `impl Trait` are not allowed in TeeParam types: use a \
             concrete, sized type",
        )),

        _ => {}
    }

    (errors, warnings)
}

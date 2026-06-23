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

use heck::ToPascalCase;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Ident, ItemFn, LitInt, PatType, Token};

// Parses `#[tee_command(codec = MyCodec, ctx = context)]` attributes.
struct TeeArgs {
    pub codec: Option<syn::Path>,
    pub ctx: Option<CtxArg>,
}

enum CtxArg {
    First(Ident),
    Ident(Ident),
    Index(LitInt),
}

impl syn::parse::Parse for TeeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut codec = None;
        let mut ctx = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "codec" {
                if codec.is_some() {
                    return Err(syn::Error::new(ident.span(), "duplicate `codec` argument"));
                }
                let _eq: Token![=] = input.parse()?;
                codec = Some(input.parse()?);
            } else if ident == "ctx" {
                if ctx.is_some() {
                    return Err(syn::Error::new(ident.span(), "duplicate `ctx` argument"));
                }
                ctx = if input.peek(Token![=]) {
                    let _eq: Token![=] = input.parse()?;
                    if input.peek(LitInt) {
                        Some(CtxArg::Index(input.parse()?))
                    } else {
                        Some(CtxArg::Ident(input.parse()?))
                    }
                } else {
                    Some(CtxArg::First(ident))
                };
            } else {
                return Err(syn::Error::new(ident.span(), "expected `codec` or `ctx`"));
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        Ok(TeeArgs { codec, ctx })
    }
}

/// Generates two companion functions for a TA command handler.
///
/// **`function_name_dispatched`** (TA side) — unpacks `::optee_utee::Parameters` via the
/// [`TeeParam`] trait, calls the original function, and flushes any output or inout values
/// back into their slots.
///
/// **`call_function_name`** (CA side, `#[cfg(feature = "ca")]`) — packs arguments via
/// [`TeeParam`], calls `invoke_command`, then reads output slots back.
///
/// The original function body is emitted unchanged.
///
/// # Codec attribute
///
/// When any parameter or return type implements `TeeParam<C>` for a serialized codec, you
/// must specify the codec:
///
/// ```no_run,ignore
/// #[tee_command(codec = PostcardCodec, ctx)]
/// fn my_handler(ctx: &mut Context, config: MyConfig) -> Result<(), TeeError> { /* … */ }
/// ```
///
/// The codec `C` is passed as the type parameter to all `TeeParam<C>` calls. For
/// primitive types (integers, booleans) the codec is ignored — they always use the
/// `TeeParamSlot::Primitive` variant. Omitting `codec` uses `::consortium_tee::NoCodec`
/// as the default, which causes a compile error if any serialized `TeeParam` types are
/// present (since `NoCodec` does not implement `CodecFor<T>`).
///
/// # Parameter type support
///
/// | Rust type            | TEE slot       | Notes                                          |
/// |----------------------|----------------|------------------------------------------------|
/// | `&[u8]`              | `MemrefInput`  |                                                |
/// | `&mut [u8]`          | `MemrefInout`  |                                                |
/// | `&mut u32`           | `ValueInout`   | TA reads initial, writes result back           |
/// | `&mut i32`           | `ValueInout`   | bit-cast through u32                           |
/// | `&mut u64`           | `ValueInout`   | split across a(low)/b(high)                    |
/// | `T: TeeParam<C>`     | Value/Memref   | dispatched via `T::into_tee_param` / `from`    |
/// | `&mut T: TeeParam<C>`| `MemrefInout`  | inout codec round-trip                         |
///
/// `ctx` marks the first parameter as TA context. The parameter must be a mutable
/// reference to a named path type and is skipped when assigning TEE slots. `ctx = name`
/// and `ctx = 0` are accepted for compatibility and emit a deprecation warning.
///
/// Return values use the same `TeeParam<C>` dispatch.
pub fn tee_command(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let TeeArgs { codec, ctx } = match syn::parse2::<TeeArgs>(attr) {
        Ok(args) => args,
        Err(err) => return err.into_compile_error(),
    };

    // Resolved codec type token — NoCodec when omitted.
    let codec_ty: TokenStream2 = match &codec {
        Some(c) => quote! { #c },
        None => quote! { ::consortium_tee::NoCodec },
    };

    let input = match syn::parse2::<ItemFn>(item) {
        Ok(input) => input,
        Err(err) => return err.into_compile_error(),
    };
    let fn_name = &input.sig.ident;
    let dispatched_name = Ident::new(&format!("{}_dispatched", fn_name), fn_name.span());
    let call_name = Ident::new(&format!("call_{}", fn_name), fn_name.span());

    let parameters: Vec<&PatType> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pt) = arg {
                Some(pt)
            } else {
                None
            }
        })
        .collect();

    let (ctx_index, ctx_warning) = match resolve_context_index(ctx.as_ref(), &parameters) {
        Ok(resolved) => resolved,
        Err(err) => return err.into_compile_error(),
    };
    let ctx_param = ctx_index.map(|index| parameters[index]);
    let tee_params: Vec<&PatType> = parameters
        .iter()
        .enumerate()
        .filter_map(|(index, param)| (Some(index) != ctx_index).then_some(*param))
        .collect();

    // Return-type analysis.
    let output_ty: Option<&syn::Type> = get_output_type(&input.sig.output);
    let has_output = output_ty.is_some();
    let ret_is_result = is_result_returning(&input.sig.output);

    // TA side: unpack stmts
    let mut slot_index = 0usize;
    let mut unpack_stmts = vec![];
    let mut post_call_stmts = vec![];
    let mut dispatch_call_args = vec![];

    for (param_index, param) in parameters.iter().enumerate() {
        let pat = &param.pat;
        let ty = &param.ty;
        if Some(param_index) == ctx_index {
            dispatch_call_args.push(quote! { #pat });
            continue;
        }
        let (unpack, post, next) = gen_unpack(pat, ty, slot_index, &codec_ty);
        unpack_stmts.push(unpack);
        if let Some(p) = post {
            post_call_stmts.push(p);
        }
        dispatch_call_args.push(quote! { #pat });
        slot_index = next;
    }

    // Output slot index (immediately after the last input slot).
    let output_slot = slot_index;

    // TA side: call + optional flush
    let call_and_flush = if let Some(out_ty) = output_ty {
        let flush = gen_flush_ta(output_slot, out_ty, &codec_ty);
        if ret_is_result {
            quote! {
                let __ret_val = #fn_name(#(#dispatch_call_args),*)?;
                #(#post_call_stmts)*
                #flush
                Ok(())
            }
        } else {
            quote! {
                let __ret_val = #fn_name(#(#dispatch_call_args),*);
                #(#post_call_stmts)*
                #flush
                Ok(())
            }
        }
    } else if !post_call_stmts.is_empty() {
        if ret_is_result {
            quote! {
                #fn_name(#(#dispatch_call_args),*)?;
                #(#post_call_stmts)*
                Ok(())
            }
        } else {
            quote! {
                #fn_name(#(#dispatch_call_args),*);
                #(#post_call_stmts)*
                Ok(())
            }
        }
    } else {
        quote! { #fn_name(#(#dispatch_call_args),*) }
    };

    let param_binding = if !tee_params.is_empty() || has_output {
        quote! { params }
    } else {
        quote! { _params }
    };

    // TA dispatched fn — only compiled when the "ta" feature is active.
    // Without this gate a CA-only consumer would pull in ::optee_utee even though
    // it only needs the generated `call_*` functions on the CA side.
    let dispatched_fn = if let Some(ctx) = ctx_param {
        let ctx_pat = &ctx.pat;
        let ctx_ty = &ctx.ty;
        quote! {
            #[cfg(feature = "ta")]
            fn #dispatched_name(#ctx_pat: #ctx_ty, #param_binding: &mut ::optee_utee::Parameters) -> ::optee_utee::Result<()> {
                #(#unpack_stmts)*
                #call_and_flush
            }
        }
    } else {
        quote! {
            #[cfg(feature = "ta")]
            fn #dispatched_name(#param_binding: &mut ::optee_utee::Parameters) -> ::optee_utee::Result<()> {
                #(#unpack_stmts)*
                #call_and_flush
            }
        }
    };

    // CA side: pack stmts
    let mut pack_stmts = vec![];
    let mut post_invoke_stmts = vec![];
    let mut ca_sig_args = vec![];
    let mut new_params_args = vec![];

    for (i, param) in tee_params.iter().enumerate() {
        let pat = &param.pat;
        let ty = &param.ty;
        let slot_ident = Ident::new(&format!("_p{}", i), Span::call_site());
        ca_sig_args.push(quote! { #pat: #ty });
        let (pack, post) = gen_pack_ca(&slot_ident, pat, ty, i, &codec_ty);
        pack_stmts.push(pack);
        if let Some(p) = post {
            post_invoke_stmts.push(p);
        }
        new_params_args.push(quote! { #slot_ident });
    }

    if has_output {
        let out_ty = output_ty.unwrap();
        let out_slot = tee_params.len();
        let slot_ident = Ident::new(&format!("_p{}", out_slot), Span::call_site());
        pack_stmts.push(gen_output_pack_ca(&slot_ident, out_ty, out_slot, &codec_ty));
        new_params_args.push(quote! { #slot_ident });
    }

    while new_params_args.len() < 4 {
        new_params_args.push(quote! { ::optee_teec::ParamNone });
    }

    let command_variant = Ident::new(&fn_name.to_string().to_pascal_case(), fn_name.span());

    // CA fn
    let ca_fn = if has_output {
        let out_ty = output_ty.unwrap();
        let read_back = gen_output_read_ca(tee_params.len(), out_ty, &codec_ty);
        quote! {
            #[cfg(feature = "ca")]
            fn #call_name(session: &mut ::optee_teec::Session, #(#ca_sig_args),*) -> ::consortium_tee::TeeResult<#out_ty> {
                #(#pack_stmts)*
                let mut _operation = ::optee_teec::Operation::new(0, #(#new_params_args),*);
                session.invoke_command(Command::#command_variant as u32, &mut _operation)?;
                #(#post_invoke_stmts)*
                Ok(#read_back)
            }
        }
    } else if !post_invoke_stmts.is_empty() {
        quote! {
            #[cfg(feature = "ca")]
            fn #call_name(session: &mut ::optee_teec::Session, #(#ca_sig_args),*) -> ::consortium_tee::TeeResult<()> {
                #(#pack_stmts)*
                let mut _operation = ::optee_teec::Operation::new(0, #(#new_params_args),*);
                session.invoke_command(Command::#command_variant as u32, &mut _operation)?;
                #(#post_invoke_stmts)*
                Ok(())
            }
        }
    } else {
        quote! {
            #[cfg(feature = "ca")]
            fn #call_name(session: &mut ::optee_teec::Session, #(#ca_sig_args),*) -> ::consortium_tee::TeeResult<()> {
                #(#pack_stmts)*
                let mut _operation = ::optee_teec::Operation::new(0, #(#new_params_args),*);
                session.invoke_command(Command::#command_variant as u32, &mut _operation)?;
                Ok(())
            }
        }
    };

    quote! {
        #input
        #ctx_warning
        #dispatched_fn
        #ca_fn
    }
}

// ── Type helpers ─────────────────────────────────────────────────────────────

fn get_output_type(ret: &syn::ReturnType) -> Option<&syn::Type> {
    let ty = match ret {
        syn::ReturnType::Default => return None,
        syn::ReturnType::Type(_, ty) => ty.as_ref(),
    };
    if let Some(inner) = extract_result_inner(ty) {
        if !is_unit_type(inner) {
            return Some(inner);
        }
        return None;
    }
    if !is_unit_type(ty) { Some(ty) } else { None }
}

fn is_result_returning(ret: &syn::ReturnType) -> bool {
    match ret {
        syn::ReturnType::Type(_, ty) => extract_result_inner(ty.as_ref()).is_some(),
        _ => false,
    }
}

fn extract_result_inner(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if seg.ident != "Result" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments
            && let Some(syn::GenericArgument::Type(inner)) = ab.args.first()
        {
            return Some(inner);
        }
    }
    None
}

fn is_unit_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Tuple(t) if t.elems.is_empty())
}

fn is_primitive(ty: &syn::Type, name: &str) -> bool {
    if let syn::Type::Path(tp) = ty {
        tp.qself.is_none() && tp.path.is_ident(name)
    } else {
        false
    }
}

fn is_vec_u8(ty: &syn::Type) -> bool {
    if let syn::Type::Path(tp) = ty
        && tp.qself.is_none()
        && let Some(seg) = tp.path.segments.last()
        && seg.ident == "Vec"
        && let syn::PathArguments::AngleBracketed(ab) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = ab.args.first()
    {
        return is_primitive(inner, "u8");
    }
    false
}

fn is_slice_ref(ty: &syn::Type, mutable: bool) -> bool {
    if let syn::Type::Reference(r) = ty {
        r.mutability.is_some() == mutable && matches!(*r.elem, syn::Type::Slice(_))
    } else {
        false
    }
}

/// `&mut u32` / `&mut i32` / `&mut u64` — returns the inner primitive name if matched.
fn mut_scalar_inner(ty: &syn::Type) -> Option<&'static str> {
    if let syn::Type::Reference(r) = ty
        && r.mutability.is_some()
        && let syn::Type::Path(tp) = r.elem.as_ref()
        && tp.qself.is_none()
    {
        let ident = tp.path.get_ident()?.to_string();
        return match ident.as_str() {
            "u32" => Some("u32"),
            "i32" => Some("i32"),
            "u64" => Some("u64"),
            _ => None,
        };
    }
    None
}

/// `(u32, u32)` tuple.
fn is_tuple_u32_u32(ty: &syn::Type) -> bool {
    if let syn::Type::Tuple(t) = ty
        && t.elems.len() == 2
    {
        return is_primitive(&t.elems[0], "u32") && is_primitive(&t.elems[1], "u32");
    }
    false
}

fn resolve_context_index(
    ctx: Option<&CtxArg>,
    parameters: &[&PatType],
) -> syn::Result<(Option<usize>, TokenStream2)> {
    let Some(ctx) = ctx else {
        return Ok((None, quote! {}));
    };

    let (index, warning) = match ctx {
        CtxArg::First(ident) => {
            if parameters.is_empty() {
                return Err(syn::Error::new(
                    ident.span(),
                    "`ctx` requires a first parameter to use as context",
                ));
            }
            (0, quote! {})
        }
        CtxArg::Ident(ident) => {
            let index = parameters
                .iter()
                .position(|param| {
                    if let syn::Pat::Ident(pat_ident) = param.pat.as_ref() {
                        pat_ident.ident == *ident
                    } else {
                        false
                    }
                })
                .ok_or_else(|| {
                    syn::Error::new(
                        ident.span(),
                        format!("could not find context parameter `{ident}`"),
                    )
                })?;
            (index, gen_deprecated_ctx_value_warning(index))
        }
        CtxArg::Index(lit) => {
            let index = lit.base10_parse::<usize>()?;
            if index >= parameters.len() {
                return Err(syn::Error::new(
                    lit.span(),
                    format!("context parameter index {index} is out of range"),
                ));
            }
            (index, gen_deprecated_ctx_value_warning(index))
        }
    };

    let param = parameters[index];
    if !is_context_ref(&param.ty) {
        return Err(syn::Error::new_spanned(
            &param.ty,
            "`ctx` parameter must have type `&mut SomePath`",
        ));
    }

    Ok((Some(index), warning))
}

fn gen_deprecated_ctx_value_warning(index: usize) -> TokenStream2 {
    if index == 0 {
        quote! {
            const _: () = {
                #[deprecated(
                    note = "`#[tee_command(ctx = ...)]` is accepted for compatibility; prefer bare `ctx`"
                )]
                const __TEE_COMMAND_CTX_VALUE_WARNING: () = ();
                __TEE_COMMAND_CTX_VALUE_WARNING
            };
        }
    } else {
        quote! {
            const _: () = {
                #[deprecated(
                    note = "`#[tee_command(ctx = ...)]` is accepted for compatibility; move the context to the first parameter and use bare `ctx`"
                )]
                const __TEE_COMMAND_CTX_VALUE_WARNING: () = ();
                __TEE_COMMAND_CTX_VALUE_WARNING
            };
        }
    }
}

/// `&mut SomeIdent` (mutable reference to a named path type, not a primitive scalar, not a
/// slice).
fn is_context_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(r) = ty
        && r.mutability.is_some()
    {
        if let syn::Type::Path(tp) = r.elem.as_ref() {
            if let Some(ident) = tp.path.get_ident() {
                if matches!(ident.to_string().as_str(), "u32" | "i32" | "u64") {
                    return false;
                }
            }
            return true;
        }
    }
    false
}

/// Path type that is not a known primitive or `Vec<u8>` — routed through `TeeParam<C>`.
fn is_tee_param_path(ty: &syn::Type) -> bool {
    if let syn::Type::Path(tp) = ty {
        if tp.qself.is_some() {
            return false;
        }
        if let Some(ident) = tp.path.get_ident() {
            if matches!(
                ident.to_string().as_str(),
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "bool"
            ) {
                return false;
            }
        }
        if is_vec_u8(ty) {
            return false;
        }
        return true;
    }
    false
}

/// `&mut T` where T is a non-primitive path type — routed through `TeeParam<C>` MemrefInout.
fn is_mut_tee_param_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(r) = ty
        && r.mutability.is_some()
    {
        return is_tee_param_path(r.elem.as_ref());
    }
    false
}

// ── TA-side unpack / flush code generation ───────────────────────────────────

/// Returns `(unpack_stmt, optional_post_call_flush, next_slot_index)`.
fn gen_unpack(
    pat: &syn::Pat,
    ty: &syn::Type,
    slot: usize,
    codec: &TokenStream2,
) -> (TokenStream2, Option<TokenStream2>, usize) {
    if slot >= 4 {
        return (
            quote! { compile_error!("Too many parameters: TEE only supports up to 4"); },
            None,
            slot,
        );
    }
    let idx = syn::Index::from(slot);
    let tmp = Ident::new(&format!("__p_{}", slot), Span::call_site());

    // Priority 1: &[u8]
    if is_slice_ref(ty, false) {
        let stmt = quote! {
            // SAFETY: slot #idx is MemrefInput - type match guaranteed by #[tee_command]
            let mut #tmp = unsafe { params.#idx.as_memref()? };
            let #pat: &[u8] = #tmp.buffer();
        };
        return (stmt, None, slot + 1);
    }

    // Priority 2: &mut [u8]
    if is_slice_ref(ty, true) {
        let stmt = quote! {
            // SAFETY: slot #idx is MemrefInout - type match guaranteed by #[tee_command]
            let mut #tmp = unsafe { params.#idx.as_memref()? };
            let #pat: &mut [u8] = #tmp.buffer();
        };
        return (stmt, None, slot + 1);
    }

    // Priority 3: &mut u32 / &mut i32 / &mut u64 (ValueInout)
    if let Some(scalar) = mut_scalar_inner(ty) {
        let val_tmp = Ident::new(&format!("__val_{}", slot), Span::call_site());
        let (unpack_stmt, flush_stmt) = match scalar {
            "u32" => {
                let u = quote! {
                    // SAFETY: slot #idx is ValueInout - type match guaranteed by #[tee_command]
                    let mut #tmp = unsafe { params.#idx.as_value()? };
                    let mut #val_tmp: u32 = #tmp.a();
                    let #pat: &mut u32 = &mut #val_tmp;
                };
                let f = quote! {
                    // Flush &mut u32 back into ValueInout slot.
                    #tmp.set_a(*#pat);
                };
                (u, f)
            }
            "i32" => {
                let u = quote! {
                    // SAFETY: slot #idx is ValueInout - type match guaranteed by #[tee_command]
                    let mut #tmp = unsafe { params.#idx.as_value()? };
                    let mut #val_tmp: i32 = #tmp.a() as i32;
                    let #pat: &mut i32 = &mut #val_tmp;
                };
                let f = quote! {
                    // Flush &mut i32 back into ValueInout slot (bit-cast).
                    #tmp.set_a(*#pat as u32);
                };
                (u, f)
            }
            "u64" => {
                let u = quote! {
                    // SAFETY: slot #idx is ValueInout - type match guaranteed by #[tee_command]
                    let mut #tmp = unsafe { params.#idx.as_value()? };
                    let mut #val_tmp: u64 = (#tmp.b() as u64) << 32 | #tmp.a() as u64;
                    let #pat: &mut u64 = &mut #val_tmp;
                };
                let f = quote! {
                    // Flush &mut u64 back into ValueInout slot (split across a/b).
                    #tmp.set_a(*#pat as u32);
                    #tmp.set_b((*#pat >> 32) as u32);
                };
                (u, f)
            }
            _ => unreachable!(),
        };
        return (unpack_stmt, Some(flush_stmt), slot + 1);
    }

    // Priority 4: &mut T: TeeParam  (MemrefInout, round-trip via TeeParam<C>)
    if is_mut_tee_param_ref(ty) {
        let inner_ty = if let syn::Type::Reference(r) = ty {
            r.elem.as_ref()
        } else {
            unreachable!()
        };
        let val_tmp = Ident::new(&format!("__val_{}", slot), Span::call_site());
        let repr_flush = Ident::new(&format!("__repr_flush_{}", slot), Span::call_site());
        let flush_bytes = Ident::new(&format!("__flush_bytes_{}", slot), Span::call_site());
        let unpack = quote! {
            // SAFETY: slot #idx is MemrefInout - type match guaranteed by #[tee_command]
            let mut #tmp = unsafe { params.#idx.as_memref()? };
            let mut #val_tmp: #inner_ty =
                <#inner_ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Serialized(#tmp.buffer())
                ).map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
            let #pat: &mut #inner_ty = &mut #val_tmp;
        };
        let flush = quote! {
            // Re-serialize modified &mut T back into MemrefInout slot.
            let #repr_flush =
                <#inner_ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&#val_tmp)
                    .map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
            let ::consortium_tee::TeeParamSlot::Serialized(ref #flush_bytes) = #repr_flush
                else { unreachable!() };
            #tmp.buffer()[..#flush_bytes.len()].copy_from_slice(#flush_bytes.as_slice());
        };
        return (unpack, Some(flush), slot + 1);
    }

    // Priority 5: any remaining owned type (primitives, (u32,u32), Vec<u8>, T: TeeParam).
    // The slot kind (Value vs Memref) is determined by whether the TeeParam impl returns
    // Primitive or Serialized; we use the type name to pick the right `as_value` / `as_memref`.
    let is_value_slot = is_primitive(ty, "u8")
        || is_primitive(ty, "u16")
        || is_primitive(ty, "u32")
        || is_primitive(ty, "u64")
        || is_primitive(ty, "i8")
        || is_primitive(ty, "i16")
        || is_primitive(ty, "i32")
        || is_primitive(ty, "i64")
        || is_primitive(ty, "bool")
        || is_tuple_u32_u32(ty);

    if is_value_slot {
        let stmt = quote! {
            // SAFETY: slot #idx is ValueInput - type match guaranteed by #[tee_command]
            let mut #tmp = unsafe { params.#idx.as_value()? };
            let #pat = <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                ::consortium_tee::TeeParamSlot::Primitive { a: #tmp.a(), b: #tmp.b() }
            ).map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
        };
        return (stmt, None, slot + 1);
    }

    // Vec<u8> and T: TeeParam — Memref slot.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        let stmt = quote! {
            // SAFETY: slot #idx is MemrefInput - type match guaranteed by #[tee_command]
            let mut #tmp = unsafe { params.#idx.as_memref()? };
            let #pat = <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                ::consortium_tee::TeeParamSlot::Serialized(#tmp.buffer())
            ).map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
        };
        return (stmt, None, slot + 1);
    }

    (
        quote! { compile_error!("Unsupported TEE parameter type"); },
        None,
        slot,
    )
}

/// Emits statements that write `__ret_val` into the output parameter slot at `slot`.
fn gen_flush_ta(slot: usize, ty: &syn::Type, codec: &TokenStream2) -> TokenStream2 {
    if slot >= 4 {
        return quote! { compile_error!("Too many parameters: no room for the output slot (TEE max 4)"); };
    }
    let idx = syn::Index::from(slot);

    let is_value_slot = is_primitive(ty, "u8")
        || is_primitive(ty, "u16")
        || is_primitive(ty, "u32")
        || is_primitive(ty, "u64")
        || is_primitive(ty, "i8")
        || is_primitive(ty, "i16")
        || is_primitive(ty, "i32")
        || is_primitive(ty, "i64")
        || is_primitive(ty, "bool")
        || is_tuple_u32_u32(ty);

    if is_value_slot {
        return quote! {
            // SAFETY: slot #idx is ValueOutput - type match guaranteed by #[tee_command]
            let mut __p_out = unsafe { params.#idx.as_value()? };
            let __repr_out = <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&__ret_val)
                .map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
            let ::consortium_tee::TeeParamSlot::Primitive { a: __pa_out, b: __pb_out } = __repr_out
                else { unreachable!() };
            __p_out.set_a(__pa_out);
            __p_out.set_b(__pb_out);
        };
    }

    // Vec<u8> and T: TeeParam — Memref slot.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        return quote! {
            // SAFETY: slot #idx is MemrefOutput - type match guaranteed by #[tee_command]
            let mut __p_out = unsafe { params.#idx.as_memref()? };
            let __repr_out = <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&__ret_val)
                .map_err(|_| ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters))?;
            let ::consortium_tee::TeeParamSlot::Serialized(ref __out_bytes) = __repr_out
                else { unreachable!() };
            __p_out.buffer()[..__out_bytes.len()].copy_from_slice(__out_bytes.as_slice());
        };
    }

    quote! { compile_error!("Unsupported TEE return type"); }
}

// ── CA-side pack / read-back code generation ─────────────────────────────────

/// Returns `(pack_stmt, optional_post_invoke_readback)`.
fn gen_pack_ca(
    slot_ident: &Ident,
    pat: &syn::Pat,
    ty: &syn::Type,
    slot: usize,
    codec: &TokenStream2,
) -> (TokenStream2, Option<TokenStream2>) {
    // Priority 1: &[u8]
    if is_slice_ref(ty, false) {
        return (
            quote! { let #slot_ident = ::optee_teec::ParamTmpRef::new_input(#pat); },
            None,
        );
    }

    // Priority 2: &mut [u8]
    if is_slice_ref(ty, true) {
        return (
            quote! { let #slot_ident = ::optee_teec::ParamTmpRef::new_inout(#pat); },
            None,
        );
    }

    // Priority 3: &mut u32 / &mut i32 / &mut u64 (ValueInout)
    if let Some(scalar) = mut_scalar_inner(ty) {
        let slot_idx = syn::Index::from(slot);
        let (pack, read_back) = match scalar {
            "u32" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat, 0, ::optee_teec::ParamType::ValueInout); },
                quote! { *#pat = _operation.parameters().#slot_idx.a(); },
            ),
            "i32" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat as u32, 0, ::optee_teec::ParamType::ValueInout); },
                quote! { *#pat = _operation.parameters().#slot_idx.a() as i32; },
            ),
            "u64" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat as u32, (*#pat >> 32) as u32, ::optee_teec::ParamType::ValueInout); },
                quote! {
                    *#pat = (_operation.parameters().#slot_idx.b() as u64) << 32
                        | _operation.parameters().#slot_idx.a() as u64;
                },
            ),
            _ => unreachable!(),
        };
        return (pack, Some(read_back));
    }

    // Priority 4: &mut T: TeeParam  (MemrefInout)
    if is_mut_tee_param_ref(ty) {
        let inner_ty = if let syn::Type::Reference(r) = ty {
            r.elem.as_ref()
        } else {
            unreachable!()
        };
        let buf_ident = Ident::new(&format!("__inout_buf_{}", slot), Span::call_site());
        let repr_ident = Ident::new(&format!("__repr_inout_{}", slot), Span::call_site());
        let src_ident = Ident::new(&format!("__src_inout_{}", slot), Span::call_site());
        let slot_idx = syn::Index::from(slot);
        let pack = quote! {
            let mut #buf_ident = vec![0u8; <#inner_ty as ::consortium_tee::TeeParam<#codec>>::MAX_SIZE];
            let #repr_ident = <#inner_ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(#pat)
                .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?;
            let ::consortium_tee::TeeParamSlot::Serialized(ref #src_ident) = #repr_ident
                else { unreachable!() };
            #buf_ident[..#src_ident.len()].copy_from_slice(#src_ident.as_slice());
            let #slot_ident = ::optee_teec::ParamTmpRef::new_inout(&mut #buf_ident);
        };
        let read_back = quote! {
            *#pat = {
                let __updated_size = _operation.parameters().#slot_idx.updated_size();
                <#inner_ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Serialized(&#buf_ident[..__updated_size])
                ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            };
        };
        return (pack, Some(read_back));
    }

    // Priority 5: Value-slot owned types.
    let is_value_slot = is_primitive(ty, "u8")
        || is_primitive(ty, "u16")
        || is_primitive(ty, "u32")
        || is_primitive(ty, "u64")
        || is_primitive(ty, "i8")
        || is_primitive(ty, "i16")
        || is_primitive(ty, "i32")
        || is_primitive(ty, "i64")
        || is_primitive(ty, "bool")
        || is_tuple_u32_u32(ty);

    if is_value_slot {
        let repr_ident = Ident::new(&format!("__repr_{}", slot), Span::call_site());
        let pa_ident = Ident::new(&format!("__pa_{}", slot), Span::call_site());
        let pb_ident = Ident::new(&format!("__pb_{}", slot), Span::call_site());
        let pack = quote! {
            let #repr_ident = <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&#pat)
                .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?;
            let ::consortium_tee::TeeParamSlot::Primitive { a: #pa_ident, b: #pb_ident } = #repr_ident
                else { unreachable!() };
            let #slot_ident = ::optee_teec::ParamValue::new(#pa_ident, #pb_ident, ::optee_teec::ParamType::ValueInput);
        };
        return (pack, None);
    }

    // Priority 6: Vec<u8> and T: TeeParam — Memref slot.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        let repr_ident = Ident::new(&format!("__repr_{}", slot), Span::call_site());
        let bytes_ident = Ident::new(&format!("__bytes_{}", slot), Span::call_site());
        let pack = quote! {
            let #repr_ident = <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&#pat)
                .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?;
            let ::consortium_tee::TeeParamSlot::Serialized(ref #bytes_ident) = #repr_ident
                else { unreachable!() };
            let #slot_ident = ::optee_teec::ParamTmpRef::new_input(#bytes_ident.as_slice());
        };
        return (pack, None);
    }

    (
        quote! {
            let #slot_ident = ::optee_teec::ParamNone;
            compile_error!("Unsupported TEE parameter type");
        },
        None,
    )
}

/// Emits the declaration(s) that reserve a slot for the return value on the CA side.
fn gen_output_pack_ca(
    slot_ident: &Ident,
    ty: &syn::Type,
    slot: usize,
    codec: &TokenStream2,
) -> TokenStream2 {
    let is_value_slot = is_primitive(ty, "u8")
        || is_primitive(ty, "u16")
        || is_primitive(ty, "u32")
        || is_primitive(ty, "u64")
        || is_primitive(ty, "i8")
        || is_primitive(ty, "i16")
        || is_primitive(ty, "i32")
        || is_primitive(ty, "i64")
        || is_primitive(ty, "bool")
        || is_tuple_u32_u32(ty);

    if is_value_slot {
        return quote! { let #slot_ident = ::optee_teec::ParamValue::new(0, 0, ::optee_teec::ParamType::ValueOutput); };
    }

    if is_vec_u8(ty) || is_tee_param_path(ty) {
        let buf_ident = Ident::new(&format!("__out_buf_{}", slot), Span::call_site());
        return quote! {
            let mut #buf_ident = vec![0u8; <#ty as ::consortium_tee::TeeParam<#codec>>::MAX_SIZE];
            let #slot_ident = ::optee_teec::ParamTmpRef::new_output(&mut #buf_ident);
        };
    }

    quote! { compile_error!("Unsupported TEE return type for output parameter"); }
}

/// Emits the expression that reads the return value back from `_operation` on the CA side.
fn gen_output_read_ca(slot: usize, ty: &syn::Type, codec: &TokenStream2) -> TokenStream2 {
    let slot_idx = syn::Index::from(slot);
    let buf_ident = Ident::new(&format!("__out_buf_{}", slot), Span::call_site());

    let is_value_slot = is_primitive(ty, "u8")
        || is_primitive(ty, "u16")
        || is_primitive(ty, "u32")
        || is_primitive(ty, "u64")
        || is_primitive(ty, "i8")
        || is_primitive(ty, "i16")
        || is_primitive(ty, "i32")
        || is_primitive(ty, "i64")
        || is_primitive(ty, "bool")
        || is_tuple_u32_u32(ty);

    if is_value_slot {
        return quote! {
            {
                let __v = _operation.parameters().#slot_idx;
                <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Primitive { a: __v.a(), b: __v.b() }
                ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            }
        };
    }

    if is_vec_u8(ty) || is_tee_param_path(ty) {
        // `__out_buf_N` was declared by gen_output_pack_ca. After invoke_command the
        // TA has written into it; the reconstructed parameter's updated_size() reflects
        // how many bytes were actually written.
        return quote! {
            {
                let __updated_size = _operation.parameters().#slot_idx.updated_size();
                <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Serialized(&#buf_ident[..__updated_size])
                ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            }
        };
    }

    quote! { compile_error!("Unsupported TEE return type") }
}

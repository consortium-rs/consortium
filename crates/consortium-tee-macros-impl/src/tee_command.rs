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

use consortium_macros_helpers::{
    get_output_type, is_context_ref, is_mut_tee_param_ref, is_primitive, is_result_returning,
    is_slice_ref, is_tee_param_path, is_tuple_u32_u32, is_vec_u8, mut_scalar_inner,
};
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
/// **`function_name_dispatched`** (TA side, `#[cfg(feature = "ta")]`): takes the TA's
/// `optee_utee::ParametersAny`, checks every slot's runtime type with the typed accessors
/// (`as_value_input`, `as_memref_output`, ...; unused slots must be `None`), unpacks the
/// arguments via the `TeeParam` trait, calls the original function, and writes any output
/// or inout values back. All checks run before the call, so a mismatched layout returns
/// `BadParameters` without running the handler. There is no `unsafe` in the expansion.
///
/// **`call_function_name`** (CA side, `#[cfg(feature = "ca")]`): packs arguments via
/// `TeeParam`, calls `invoke_command`, then reads output slots back.
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
/// | `&[u8]`              | `MemrefInput`  | TA receives a TA-owned copy                    |
/// | `&mut [u8]`          | `MemrefInout`  | copied in, written back whole                  |
/// | `&mut u32`           | `ValueInout`   | TA reads initial, writes result back           |
/// | `&mut i32`           | `ValueInout`   | bit-cast through u32                           |
/// | `&mut u64`           | `ValueInout`   | split across a(low)/b(high)                    |
/// | `T: TeeParam<C>`     | Value/Memref   | dispatched via `T::into_tee_param` / `from`    |
/// | `&mut T: TeeParam<C>`| `MemrefInout`  | length-framed codec round-trip                 |
///
/// Memref contents are copied, never borrowed from shared memory, so the Normal World
/// cannot change them after the TA has looked. A serialized input larger than its
/// `TeeParam::MAX_SIZE` is rejected before it is copied. A `&mut T` slot carries a
/// 4-byte length header ahead of the payload (see `consortium_tee::inout`), because an
/// in/out buffer is sized for the worst case and its size field cannot say how much
/// of it the CA's encoding filled.
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

    // Resolved codec type token, and use NoCodec when omitted.
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

    // TA side. Every slot is checked before the handler runs, so a CA that sends a
    // different layout gets `BadParameters` without any handler side effect.
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

    // Output slot (immediately after the last input slot). Its accessor is taken up
    // front with the inputs; only the write happens after the call.
    let output_write = if let Some(out_ty) = output_ty {
        let (acquire, write) = gen_output_ta(slot_index, out_ty, &codec_ty);
        unpack_stmts.push(acquire);
        slot_index += 1;
        write
    } else {
        quote! {}
    };

    // Slots the command does not use must arrive as `None`.
    for slot in slot_index..4 {
        let p = slot_ident(slot);
        unpack_stmts.push(quote! { #p.as_none()?; });
    }

    let call = if ret_is_result {
        quote! { #fn_name(#(#dispatch_call_args),*)? }
    } else {
        quote! { #fn_name(#(#dispatch_call_args),*) }
    };
    let call_stmt = if has_output {
        quote! { let __ret_val = #call; }
    } else {
        quote! { #call; }
    };

    // TA dispatched fn (only compiled when the "ta" feature is active).
    // Without this gate a CA-only consumer would pull in ::optee_utee even though
    // it only needs the generated `call_*` functions on the CA side.
    let ctx_arg = ctx_param.map(|ctx| {
        let ctx_pat = &ctx.pat;
        let ctx_ty = &ctx.ty;
        quote! { #ctx_pat: #ctx_ty, }
    });
    let dispatched_fn = quote! {
        #[cfg(feature = "ta")]
        fn #dispatched_name(#ctx_arg params: &mut ::optee_utee::ParametersAny<'_>) -> ::optee_utee::Result<()> {
            let (__p_0, __p_1, __p_2, __p_3) = params;
            #(#unpack_stmts)*
            #call_stmt
            #(#post_call_stmts)*
            #output_write
            Ok(())
        }
    };

    // CA side. Read-back happens in two phases: `capture` copies sizes and values out
    // of the finished operation, which ends its borrow of the slot buffers, and only
    // then does `decode` read those buffers.
    let mut pack_stmts = vec![];
    let mut capture_stmts = vec![];
    let mut decode_stmts = vec![];
    let mut ca_sig_args = vec![];
    let mut new_params_args = vec![];

    for (i, param) in tee_params.iter().enumerate() {
        let pat = &param.pat;
        let ty = &param.ty;
        let slot_ident = Ident::new(&format!("_p{}", i), Span::call_site());
        ca_sig_args.push(quote! { #pat: #ty });
        let (pack, capture, decode) = gen_pack_ca(&slot_ident, pat, ty, i, &codec_ty);
        pack_stmts.push(pack);
        capture_stmts.push(capture);
        decode_stmts.push(decode);
        new_params_args.push(quote! { #slot_ident });
    }

    let (ret_ty, ret_value) = if let Some(out_ty) = output_ty {
        let out_slot = tee_params.len();
        let slot_ident = Ident::new(&format!("_p{}", out_slot), Span::call_site());
        pack_stmts.push(gen_output_pack_ca(&slot_ident, out_ty, out_slot, &codec_ty));
        new_params_args.push(quote! { #slot_ident });
        let (capture, value) = gen_output_read_ca(out_slot, out_ty, &codec_ty);
        capture_stmts.push(capture);
        decode_stmts.push(quote! { let __ret_val = #value; });
        (quote! { #out_ty }, quote! { __ret_val })
    } else {
        (quote! { () }, quote! { () })
    };

    while new_params_args.len() < 4 {
        new_params_args.push(quote! { ::optee_teec::ParamNone });
    }

    let after = if capture_stmts.iter().any(|c| !c.is_empty()) {
        quote! { let __after = _operation.parameters(); }
    } else {
        quote! {}
    };

    let command_variant = Ident::new(&fn_name.to_string().to_pascal_case(), fn_name.span());

    // CA side function (only compiled when the "ca" feature is active).
    let ca_fn = quote! {
        #[cfg(feature = "ca")]
        pub fn #call_name(session: &mut ::optee_teec::Session, #(#ca_sig_args),*) -> ::consortium_tee::TeeResult<#ret_ty> {
            #(#pack_stmts)*
            let mut _operation = ::optee_teec::Operation::new(0, #(#new_params_args),*);
            session.invoke_command(Command::#command_variant as u32, &mut _operation)?;
            #after
            #(#capture_stmts)*
            #(#decode_stmts)*
            Ok(#ret_value)
        }
    };

    quote! {
        #input
        #ctx_warning
        #dispatched_fn
        #ca_fn
    }
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

// ── Shared helpers ───────────────────────────────────────────────────────────

/// The TA-side binding for slot `slot`, a `&mut optee_utee::ParameterAny`.
fn slot_ident(slot: usize) -> Ident {
    Ident::new(&format!("__p_{}", slot), Span::call_site())
}

/// Types that travel in a Value slot rather than a Memref.
fn is_value_slot(ty: &syn::Type) -> bool {
    ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "bool"]
        .iter()
        .any(|name| is_primitive(ty, name))
        || is_tuple_u32_u32(ty)
}

fn bad_parameters() -> TokenStream2 {
    quote! { ::optee_utee::Error::new(::optee_utee::ErrorKind::BadParameters) }
}

// ── TA-side unpack / flush code generation ───────────────────────────────────
//
// Every accessor below is a safe, type-checked call on `optee_utee::ParameterAny`
// (`as_value_input`, `as_memref_output`, ...) that returns `BadParameters` when the
// slot's runtime type differs. Memref contents are only ever *copied* in and out
// (`read_to_vec`, `set_output`, `write_at`), never borrowed, so the Normal World cannot
// change bytes between the TA's validation and its use of them. Trait methods are
// called by path so the TA crate does not have to import `optee_utee::prelude`.

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
    let p = slot_ident(slot);
    let bad = bad_parameters();
    let handle = Ident::new(&format!("__h_{}", slot), Span::call_site());
    let buf = Ident::new(&format!("__buf_{}", slot), Span::call_site());
    let val = Ident::new(&format!("__val_{}", slot), Span::call_site());

    // Priority 1: &[u8] (MemrefInput), handed to the handler as a TA-owned copy.
    if is_slice_ref(ty, false) {
        let stmt = quote! {
            let #buf = ::optee_utee::ParameterMemrefRead::read_to_vec(#p.as_memref_input()?);
            let #pat: &[u8] = &#buf;
        };
        return (stmt, None, slot + 1);
    }

    // Priority 2: &mut [u8] (MemrefInout), copied in, then written back whole.
    if is_slice_ref(ty, true) {
        let stmt = quote! {
            let #handle = #p.as_memref_inout()?;
            let mut #buf = ::optee_utee::ParameterMemrefRead::read_to_vec(&*#handle);
            let #pat: &mut [u8] = &mut #buf;
        };
        let flush = quote! {
            ::optee_utee::ParameterMemrefWrite::write_at(&mut *#handle, 0, &#buf)?;
        };
        return (stmt, Some(flush), slot + 1);
    }

    // Priority 3: &mut u32 / &mut i32 / &mut u64 (ValueInout)
    if let Some(scalar) = mut_scalar_inner(ty) {
        let read_a = quote! { ::optee_utee::ParameterValueRead::get_a(&*#handle) };
        let read_b = quote! { ::optee_utee::ParameterValueRead::get_b(&*#handle) };
        let (init, flush) = match scalar {
            "u32" => (
                quote! { let mut #val: u32 = #read_a; },
                quote! {
                    // Flush &mut u32 back into the ValueInout slot.
                    ::optee_utee::ParameterValueWrite::set_a(&mut *#handle, #val);
                },
            ),
            "i32" => (
                quote! { let mut #val: i32 = #read_a as i32; },
                quote! {
                    // Flush &mut i32 back into the ValueInout slot (bit-cast).
                    ::optee_utee::ParameterValueWrite::set_a(&mut *#handle, #val as u32);
                },
            ),
            "u64" => (
                quote! { let mut #val: u64 = (#read_b as u64) << 32 | #read_a as u64; },
                quote! {
                    // Flush &mut u64 back into the ValueInout slot (split across a/b).
                    ::optee_utee::ParameterValueWrite::set_a(&mut *#handle, #val as u32);
                    ::optee_utee::ParameterValueWrite::set_b(&mut *#handle, (#val >> 32) as u32);
                },
            ),
            _ => unreachable!(),
        };
        let unpack = quote! {
            let #handle = #p.as_value_inout()?;
            #init
            let #pat = &mut #val;
        };
        return (unpack, Some(flush), slot + 1);
    }

    // Priority 4: &mut T: TeeParam (MemrefInout, length-framed round-trip via TeeParam<C>)
    if is_mut_tee_param_ref(ty) {
        let inner_ty = if let syn::Type::Reference(r) = ty {
            r.elem.as_ref()
        } else {
            unreachable!()
        };
        let param = quote! { <#inner_ty as ::consortium_tee::TeeParam<#codec>> };
        let bytes = Ident::new(&format!("__bytes_{}", slot), Span::call_site());
        let frame = Ident::new(&format!("__frame_{}", slot), Span::call_site());
        let unpack = quote! {
            let #handle = #p.as_memref_inout()?;
            if ::optee_utee::ParameterMemref::buffer_len(&*#handle)
                > ::consortium_tee::inout::capacity(#param::MAX_SIZE)
            {
                return Err(#bad);
            }
            let #buf = ::optee_utee::ParameterMemrefRead::read_to_vec(&*#handle);
            let mut #val: #inner_ty = #param::from_tee_param(
                ::consortium_tee::TeeParamSlot::Serialized(
                    ::consortium_tee::inout::read(&#buf, #param::MAX_SIZE).map_err(|_| #bad)?
                )
            ).map_err(|_| #bad)?;
            let #pat: &mut #inner_ty = &mut #val;
        };
        let flush = quote! {
            // Re-serialize the modified &mut T and write it back framed.
            let ::consortium_tee::TeeParamSlot::Serialized(#bytes) =
                #param::into_tee_param(&#val).map_err(|_| #bad)?
            else {
                return Err(#bad);
            };
            let #frame = ::consortium_tee::inout::to_vec(&#bytes).map_err(|_| #bad)?;
            ::optee_utee::ParameterMemrefWrite::set_output(&mut *#handle, &#frame)?;
        };
        return (unpack, Some(flush), slot + 1);
    }

    // Priority 5: owned Value-slot types (primitives, (u32, u32)).
    if is_value_slot(ty) {
        let stmt = quote! {
            let #handle = #p.as_value_input()?;
            let #pat = <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                ::consortium_tee::TeeParamSlot::Primitive {
                    a: ::optee_utee::ParameterValueRead::get_a(#handle),
                    b: ::optee_utee::ParameterValueRead::get_b(#handle),
                }
            ).map_err(|_| #bad)?;
        };
        return (stmt, None, slot + 1);
    }

    // Priority 6: Vec<u8> and T: TeeParam (MemrefInput). `MAX_SIZE` bounds the copy, so
    // a CA cannot make the TA allocate more than the type admits.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        let stmt = quote! {
            let #handle = #p.as_memref_input()?;
            if ::optee_utee::ParameterMemref::buffer_len(#handle)
                > <#ty as ::consortium_tee::TeeParam<#codec>>::MAX_SIZE
            {
                return Err(#bad);
            }
            let #buf = ::optee_utee::ParameterMemrefRead::read_to_vec(#handle);
            let #pat = <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                ::consortium_tee::TeeParamSlot::Serialized(&#buf)
            ).map_err(|_| #bad)?;
        };
        return (stmt, None, slot + 1);
    }

    (
        quote! { compile_error!("Unsupported TEE parameter type"); },
        None,
        slot,
    )
}

/// Returns `(acquire, write)` for the return value's slot: `acquire` takes the typed
/// output accessor before the handler runs, `write` stores `__ret_val` into it after.
fn gen_output_ta(
    slot: usize,
    ty: &syn::Type,
    codec: &TokenStream2,
) -> (TokenStream2, TokenStream2) {
    if slot >= 4 {
        return (
            quote! { compile_error!("Too many parameters: no room for the output slot (TEE max 4)"); },
            quote! {},
        );
    }
    let p = slot_ident(slot);
    let bad = bad_parameters();

    if is_value_slot(ty) {
        return (
            quote! { let __out = #p.as_value_output()?; },
            quote! {
                let ::consortium_tee::TeeParamSlot::Primitive { a: __out_a, b: __out_b } =
                    <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&__ret_val)
                        .map_err(|_| #bad)?
                else {
                    return Err(#bad);
                };
                ::optee_utee::ParameterValueWrite::set_a(&mut *__out, __out_a);
                ::optee_utee::ParameterValueWrite::set_b(&mut *__out, __out_b);
            },
        );
    }

    // Vec<u8> and T: TeeParam — Memref slot. `set_output` copies the bytes, sets the size
    // the CA reads back, and fails with `ShortBuffer` instead of overrunning.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        return (
            quote! { let __out = #p.as_memref_output()?; },
            quote! {
                let ::consortium_tee::TeeParamSlot::Serialized(__out_bytes) =
                    <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&__ret_val)
                        .map_err(|_| #bad)?
                else {
                    return Err(#bad);
                };
                ::optee_utee::ParameterMemrefWrite::set_output(&mut *__out, &__out_bytes)?;
            },
        );
    }

    (
        quote! { compile_error!("Unsupported TEE return type"); },
        quote! {},
    )
}

// ── CA-side pack / read-back code generation ─────────────────────────────────

/// Returns `(pack_stmt, capture_stmt, decode_stmt)`; the last two may be empty.
///
/// `capture` runs while `__after` (the operation's parameters) is live and copies out
/// plain values; `decode` runs after it, once the slot buffers are free to borrow.
fn gen_pack_ca(
    slot_ident: &Ident,
    pat: &syn::Pat,
    ty: &syn::Type,
    slot: usize,
    codec: &TokenStream2,
) -> (TokenStream2, TokenStream2, TokenStream2) {
    let slot_idx = syn::Index::from(slot);

    // Priority 1: &[u8]
    if is_slice_ref(ty, false) {
        return (
            quote! { let #slot_ident = ::optee_teec::ParamTmpRef::new_input(#pat); },
            quote! {},
            quote! {},
        );
    }

    // Priority 2: &mut [u8]
    if is_slice_ref(ty, true) {
        return (
            quote! { let #slot_ident = ::optee_teec::ParamTmpRef::new_inout(#pat); },
            quote! {},
            quote! {},
        );
    }

    // Priority 3: &mut u32 / &mut i32 / &mut u64 (ValueInout)
    if let Some(scalar) = mut_scalar_inner(ty) {
        let (pack, capture) = match scalar {
            "u32" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat, 0, ::optee_teec::ParamType::ValueInout); },
                quote! { *#pat = __after.#slot_idx.a(); },
            ),
            "i32" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat as u32, 0, ::optee_teec::ParamType::ValueInout); },
                quote! { *#pat = __after.#slot_idx.a() as i32; },
            ),
            "u64" => (
                quote! { let #slot_ident = ::optee_teec::ParamValue::new(*#pat as u32, (*#pat >> 32) as u32, ::optee_teec::ParamType::ValueInout); },
                quote! {
                    *#pat = (__after.#slot_idx.b() as u64) << 32 | __after.#slot_idx.a() as u64;
                },
            ),
            _ => unreachable!(),
        };
        return (pack, capture, quote! {});
    }

    // Priority 4: &mut T: TeeParam (MemrefInout, length-framed; see `consortium_tee::inout`)
    if is_mut_tee_param_ref(ty) {
        let inner_ty = if let syn::Type::Reference(r) = ty {
            r.elem.as_ref()
        } else {
            unreachable!()
        };
        let param = quote! { <#inner_ty as ::consortium_tee::TeeParam<#codec>> };
        let buf_ident = Ident::new(&format!("__inout_buf_{}", slot), Span::call_site());
        let src_ident = Ident::new(&format!("__src_inout_{}", slot), Span::call_site());
        let size_ident = Ident::new(&format!("__inout_size_{}", slot), Span::call_site());
        let pack = quote! {
            let mut #buf_ident = vec![0u8; ::consortium_tee::inout::capacity(#param::MAX_SIZE)];
            let ::consortium_tee::TeeParamSlot::Serialized(#src_ident) =
                #param::into_tee_param(#pat).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            else {
                return Err(::consortium_tee::TeeError::ParamMismatch);
            };
            ::consortium_tee::inout::write(&mut #buf_ident, &#src_ident)
                .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?;
            let #slot_ident = ::optee_teec::ParamTmpRef::new_inout(&mut #buf_ident);
        };
        let capture = quote! { let #size_ident = __after.#slot_idx.updated_size(); };
        let decode = quote! {
            *#pat = #param::from_tee_param(
                ::consortium_tee::TeeParamSlot::Serialized(
                    ::consortium_tee::inout::read(
                        &#buf_ident[..#size_ident.min(#buf_ident.len())],
                        #param::MAX_SIZE,
                    ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
                )
            ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?;
        };
        return (pack, capture, decode);
    }

    // Priority 5: Value-slot owned types.
    if is_value_slot(ty) {
        let pa_ident = Ident::new(&format!("__pa_{}", slot), Span::call_site());
        let pb_ident = Ident::new(&format!("__pb_{}", slot), Span::call_site());
        let pack = quote! {
            let ::consortium_tee::TeeParamSlot::Primitive { a: #pa_ident, b: #pb_ident } =
                <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&#pat)
                    .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            else {
                return Err(::consortium_tee::TeeError::ParamMismatch);
            };
            let #slot_ident = ::optee_teec::ParamValue::new(#pa_ident, #pb_ident, ::optee_teec::ParamType::ValueInput);
        };
        return (pack, quote! {}, quote! {});
    }

    // Priority 6: Vec<u8> and T: TeeParam — Memref slot.
    if is_vec_u8(ty) || is_tee_param_path(ty) {
        let bytes_ident = Ident::new(&format!("__bytes_{}", slot), Span::call_site());
        let pack = quote! {
            let ::consortium_tee::TeeParamSlot::Serialized(#bytes_ident) =
                <#ty as ::consortium_tee::TeeParam<#codec>>::into_tee_param(&#pat)
                    .map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            else {
                return Err(::consortium_tee::TeeError::ParamMismatch);
            };
            let #slot_ident = ::optee_teec::ParamTmpRef::new_input(#bytes_ident.as_slice());
        };
        return (pack, quote! {}, quote! {});
    }

    (
        quote! {
            let #slot_ident = ::optee_teec::ParamNone;
            compile_error!("Unsupported TEE parameter type");
        },
        quote! {},
        quote! {},
    )
}

/// Emits the declaration(s) that reserve a slot for the return value on the CA side.
fn gen_output_pack_ca(
    slot_ident: &Ident,
    ty: &syn::Type,
    slot: usize,
    codec: &TokenStream2,
) -> TokenStream2 {
    if is_value_slot(ty) {
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

/// Returns `(capture_stmt, value_expr)` reading the return value back on the CA side.
fn gen_output_read_ca(
    slot: usize,
    ty: &syn::Type,
    codec: &TokenStream2,
) -> (TokenStream2, TokenStream2) {
    let slot_idx = syn::Index::from(slot);
    let buf_ident = Ident::new(&format!("__out_buf_{}", slot), Span::call_site());

    if is_value_slot(ty) {
        return (
            quote! {
                let __out_a = __after.#slot_idx.a();
                let __out_b = __after.#slot_idx.b();
            },
            quote! {
                <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Primitive { a: __out_a, b: __out_b }
                ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            },
        );
    }

    if is_vec_u8(ty) || is_tee_param_path(ty) {
        // `__out_buf_N` was declared by gen_output_pack_ca. The TA's `set_output` reported
        // how many bytes it wrote; clamp in case a TA reports more than the buffer holds.
        return (
            quote! { let __out_size = __after.#slot_idx.updated_size(); },
            quote! {
                <#ty as ::consortium_tee::TeeParam<#codec>>::from_tee_param(
                    ::consortium_tee::TeeParamSlot::Serialized(&#buf_ident[..__out_size.min(#buf_ident.len())])
                ).map_err(|_| ::consortium_tee::TeeError::ParamMismatch)?
            },
        );
    }

    (
        quote! {},
        quote! { compile_error!("Unsupported TEE return type") },
    )
}

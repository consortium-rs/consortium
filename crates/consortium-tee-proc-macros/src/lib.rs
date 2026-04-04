use heck::ToPascalCase;
use proc_macro::{TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::{ItemFn, PatType, parse_macro_input, Ident};

use crate::service::TeeServiceInput;

mod service;

#[proc_macro]
/// Input:
/// 
/// ```no-run
/// tee_service! {
///   (optional) context ContextStruct
///   commands { function_a, function_b }
/// }
/// ```
/// 
/// generates:
/// 
/// 1. `Command` enum with variants `FunctionA`, `FunctionB`. This exports the command identifiers that the client can invoke. The macro will convert the function names to CamelCase for the enum variants.
/// e.g.,
/// ```no-run
/// pub enum Command {
///     FunctionA = 0,
///     FunctionB = 1,
/// }
/// ```
/// 2. `invoke_command` function that matches on the command and dispatches to the corresponding function.
/// e.g.,
/// ```no-run
/// #[invoke_command]
/// pub(crate) fn invoke_command(cmd: Command, params: &mut Parameters) -> Result<()> {
///     match cmd {
///         Command::FunctionA => function_a_dispatched(params),
///         Command::FunctionB => function_b_dispatched(params),
///     }
/// }
/// ```
/// 
/// And if it has a context specified, it will generate the function as follows:
/// ```no-run
/// #[invoke_command]
/// pub(crate) fn invoke_command(ctx: &mut ContextStruct, cmd: Command, params: &mut Parameters) -> Result<()> {
///     match cmd {
///         Command::FunctionA => function_a_dispatched(ctx, params),
///         Command::FunctionB => function_b_dispatched(ctx, params),
///     }
/// }
pub fn tee_service(input: TokenStream) -> TokenStream {
    let TeeServiceInput { commands, context } =
        parse_macro_input!(input as TeeServiceInput);

    let variant_idents: Vec<Ident> = commands.iter().map(|cmd| {
        let pascal = cmd.to_string().to_pascal_case();
        Ident::new(&pascal, cmd.span())
    }).collect();

    let discriminant_literals: Vec<_> = (0u64..)
        .take(commands.len())
        .map(|n| syn::LitInt::new(&n.to_string(), Span::call_site()))
        .collect();

    // `Unknown` gets the next discriminant after all real commands.
    let unknown_discriminant =
        syn::LitInt::new(&commands.len().to_string(), Span::call_site());


    let command_enum = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        #[repr(u64)]
        pub enum Command {
            #(#variant_idents = #discriminant_literals,)*
            #[default]
            Unknown = #unknown_discriminant,
        }
    };

    let dispatch_functions: Vec<Ident> = commands.iter().map(|cmd| {
        let dispatched_name = format!("{}_dispatched", cmd);
        Ident::new(&dispatched_name, cmd.span())
    }).collect();

    let invoke_function = if let Some(ctx_ident) = context {
        quote! {
            #[invoke_command]
            pub(crate) fn invoke_command(ctx: &mut #ctx_ident, cmd: Command, params: &mut optee_utee::Parameters) -> Result<()> {
                match cmd {
                    #(Command::#variant_idents => #dispatch_functions(ctx, params),)*
                    Command::Unknown => Err(Error::UnknownCommand),
                }
            }
        }
    } else {
        quote! {
            #[invoke_command]
            pub(crate) fn invoke_command(cmd: Command, params: &mut optee_utee::Parameters) -> Result<()> {
                match cmd {
                    #(Command::#variant_idents => #dispatch_functions(params),)*
                    Command::Unknown => Err(Error::UnknownCommand),
                }
            }
        }
    };

    let expanded = quote! {
        #command_enum
        #invoke_function
    };

    expanded.into()
}

/// This macro generates a dispatch wrapper for a function that can be invoked via `invoke_command`.
/// It has two outputs: `_dispatched` for TA side, and `call_` for CA side (via feature flag).
/// The original function is left untouched for internal calls.
/// Example:
/// ```no-run
/// #[fee_function]
/// fn function_a(ctx: &mut ContextStruct, param1: u32, param2: &mut [u8]) -> Result<()> {
///     // function body
/// }
/// ```
/// generates:
/// ```no-run
/// fn function_a(ctx: &mut ContextStruct, param1: u32, param2: &mut [u8]) -> Result<()> {
///     // function body
/// }
/// 
/// fn function_a_dispatched(ctx: &mut ContextStruct, params: &mut Parameters) -> Result<()> {
///     let param_0: u32 = params.param_0().as_value().a();
///     let param_1: &mut [u8] = params.param_1().as_memref_mut().buffer();
///     function_a(ctx, param_0, param_1)
/// }
/// 
/// #[cfg(feature = "ca")]
/// fn call_function_a(param1: u32, param2: &mut [u8]) -> Result<()> {
///     let p0 = ParamValue::new(param1, 0, optee_utee::ParamType::ValueInput);
///     let p1 = ParamMemref::new(param2, optee_utee::ParamType::MemrefInout);
///     invoke_command(Command::FunctionA, &mut optee_utee::Parameters::new(p0, p1, optee_utee::ParamNone, optee_utee::ParamNone))
/// }
/// ```
#[proc_macro_attribute]
pub fn fee_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let dispatched_name = Ident::new(&format!("{}_dispatched", fn_name), fn_name.span());
    let call_name = Ident::new(&format!("call_{}", fn_name), fn_name.span());

    let parameters: Vec<&PatType> = input.sig.inputs.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            Some(pat_type)
        } else {
            None
        }
    }).collect();

    // Detect if the first parameter is a context (`&mut SomeIdent`, not a slice).
    // If so, it gets forwarded directly — not packed/unpacked from `params`.
    let (ctx_param, tee_params) = match parameters.first() {
        Some(p) if is_context_ref(&p.ty) => (Some(*p), &parameters[1..]),
        _ => (None, &parameters[..]),
    };

    // ── TA side: unpack stmts + call args ────────────────────────────────────
    let mut slot_index = 0usize;
    let mut unpack_stmts = vec![];
    let mut dispatch_call_args = vec![];

    // Forward ctx first if present
    if let Some(ctx) = ctx_param {
        let pat = &ctx.pat;
        dispatch_call_args.push(quote! { #pat });
    }

    for param in tee_params {
        let pat = &param.pat;
        let ty = &param.ty;
        let (stmt, next_slot) = gen_unpack_resolver(pat, ty, slot_index);
        unpack_stmts.push(stmt);
        dispatch_call_args.push(quote! { #pat });
        slot_index = next_slot;
    }

    // ── CA side: pack exprs + Parameters::new args ───────────────────────────
    let mut pack_stmts = vec![];
    let mut ca_param_args = vec![];  // params for the `call_` fn signature
    let mut new_params_args = vec![]; // up to 4 slots for Parameters::new

    for (i, param) in tee_params.iter().enumerate() {
        let pat = &param.pat;
        let ty = &param.ty;
        let slot_ident = Ident::new(&format!("_p{}", i), Span::call_site());

        ca_param_args.push(quote! { #pat: #ty });

        let pack = gen_pack_expr(&slot_ident, pat, ty);
        pack_stmts.push(pack);
        new_params_args.push(quote! { #slot_ident });
    }

    // Parameters::new always takes exactly 4 args; pad with `None`
    while new_params_args.len() < 4 {
        new_params_args.push(quote! { optee_utee::ParamNone });
    }

    // ── Dispatched fn signature (with or without ctx) ────────────────────────
    let param_binding = if tee_params.is_empty() {
        quote! { _params }
    } else {
        quote! { params }
    };

    let dispatched_fn = if let Some(ctx) = ctx_param {
        let ctx_pat = &ctx.pat;
        let ctx_ty = &ctx.ty;
        quote! {
            fn #dispatched_name(#ctx_pat: #ctx_ty, #param_binding: &mut optee_utee::Parameters) -> Result<()> {
                #(#unpack_stmts)*
                #fn_name(#(#dispatch_call_args),*)
            }
        }
    } else {
        quote! {
            fn #dispatched_name(#param_binding: &mut optee_utee::Parameters) -> Result<()> {
                #(#unpack_stmts)*
                #fn_name(#(#dispatch_call_args),*)
            }
        }
    };

    // ── Command variant name (PascalCase of fn name) ─────────────────────────
    let command_variant = Ident::new(&fn_name.to_string().to_pascal_case(), fn_name.span());

    let ca_fn = quote! {
        #[cfg(feature = "ca")]
        fn #call_name(#(#ca_param_args),*) -> Result<()> {
            #(#pack_stmts)*
            invoke_command(Command::#command_variant, &mut optee_utee::Parameters::new(#(#new_params_args),*))
        }
    };

    let expanded = quote! {
        #input
        #dispatched_fn
        #ca_fn
    };

    expanded.into()
}

/// Pack a TEE parameter into its wire representation on the CA side.
fn gen_pack_expr(
    slot_ident: &Ident,
    pat: &syn::Pat,
    ty: &syn::Type,
) -> proc_macro2::TokenStream {
    if is_primitive(ty, "bool") {
        quote! { let #slot_ident = ParamValue::new(#pat as u32, 0, ParamType::ValueInput); }
    } else if is_primitive(ty, "u32") {
        quote! { let #slot_ident = ParamValue::new(#pat, 0, ParamType::ValueInput); }
    } else if is_slice_ref(ty, false) {
        quote! { let #slot_ident = ParamMemref::new(#pat, ParamType::MemrefInput); }
    } else if is_slice_ref(ty, true) {
        quote! { let #slot_ident = ParamMemref::new(#pat, ParamType::MemrefInout); }
    } else {
        quote! { compile_error!("Unsupported TEE parameter type"); }
    }
}

/// Returns true if `ty` looks like `&mut SomeIdent` (a context reference),
/// i.e. a mutable reference to a named type — NOT a slice.
fn is_context_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(r) = ty {
        r.mutability.is_some() && matches!(*r.elem, syn::Type::Path(_))
    } else {
        false
    }
}

fn gen_unpack_resolver(
    pat: &syn::Pat,
    ty: &syn::Type,
    slot: usize,
) -> (proc_macro2::TokenStream, usize) {
    let slot_ident = quote::format_ident!("param_{}", slot);

    if slot >= 4 {
        return (quote! { compile_error!("Too many parameters: TEE only supports up to 4"); }, slot);
    }

    if is_primitive(ty, "bool") {
        (quote! { let #pat: bool = params.#slot_ident().as_value().a() != 0; }, slot + 1)
    } else if is_primitive(ty, "u32") {
        (quote! { let #pat: u32 = params.#slot_ident().as_value().a(); }, slot + 1)
    } else if is_slice_ref(ty, false) {
        (quote! { let #pat: &[u8] = params.#slot_ident().as_memref().buffer(); }, slot + 1)
    } else if is_slice_ref(ty, true) {
        (quote! { let #pat: &mut [u8] = params.#slot_ident().as_memref_mut().buffer(); }, slot + 1)
    } else {
        (quote! { compile_error!("Unsupported TEE parameter type"); }, slot + 1)
    }
}

fn is_primitive(ty: &syn::Type, name: &str) -> bool {
    if let syn::Type::Path(tp) = ty {
        tp.qself.is_none() && tp.path.is_ident(name)
    } else {
        false
    }
}

fn is_slice_ref(ty: &syn::Type, mutable: bool) -> bool {
    if let syn::Type::Reference(r) = ty {
        let mutability_matches = r.mutability.is_some() == mutable;
        mutability_matches && matches!(*r.elem, syn::Type::Slice(_))
    } else {
        false
    }
}

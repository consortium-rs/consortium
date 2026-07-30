//! Reusable `syn` predicates shared by Consortium's macro implementation crates.
//!
//! Three macro families — `consortium-ipc-macros-impl`, `consortium-codec-macros-impl`,
//! and `consortium-tee-macros-impl` — keep asking the same questions of an AST. Is this
//! type a `usize`? Is it a `&mut [u8]`? What are the type arguments inside `Vec<T>`?
//! What does this function actually return, ignoring `Result` and `()`?
//!
//! Those questions have fiddly answers and no natural home in any one derive, so they
//! live here. The rule the workspace follows: **reusable AST helpers go in this crate;
//! domain-specific policy stays in the owning `*-macros-impl` crate.** `IpcSafe`
//! rejecting `usize` is policy; recognizing that a `Type` *is* a `usize` is a helper.
//!
//! # What "syntactic" means here, and why it matters
//!
//! Every predicate is **purely syntactic**. A proc-macro runs before type resolution, so
//! it sees tokens, not types. [`is_vec_u8`] matches the *path* `Vec<u8>`; it cannot know
//! that a `type MyBuf = Vec<u8>;` alias is the same thing, and it will happily match a
//! user's own unrelated `Vec`.
//!
//! That is a real limitation rather than an oversight — no proc-macro can do better — and
//! it shapes how the derives are written. Where a syntactic check is not enough, the
//! derives push the question to the compiler instead by emitting a `where`-clause bound,
//! which is why `#[derive(IpcSafe)]` adds `T: IpcSafe` to generic parameters rather than
//! inspecting them.
//!
//! # Contents
//!
//! | Group               | Helpers                                                   |
//! | ------------------- | --------------------------------------------------------- |
//! | Field/type walking  | [`collect_field_types`], [`path_type_arguments`]            |
//! | Return types        | [`get_output_type`], [`is_result_returning`], [`extract_result_inner`] |
//! | Primitive shapes    | [`is_unit_type`], [`is_primitive`], [`is_vec_u8`], [`is_tuple_u32_u32`] |
//! | Reference shapes    | [`is_slice_ref`], [`mut_scalar_inner`], [`is_context_ref`]   |
//! | TEE parameter kinds | [`is_tee_param_path`], [`is_mut_tee_param_ref`]              |
//!
//! The last group is TEE-flavoured by name but generic in substance: it distinguishes
//! "a path type routed through a codec" from "a primitive that fits in a value slot",
//! which is the classification `tee_command` needs when assigning parameter slots.

use syn::{Data, GenericArgument, PathArguments, ReturnType, Type};

/// Collect every field type from a struct, enum, or union.
pub fn collect_field_types(data: &Data) -> Vec<&Type> {
    match data {
        Data::Struct(data) => data.fields.iter().map(|field| &field.ty).collect(),
        Data::Enum(data) => data
            .variants
            .iter()
            .flat_map(|variant| variant.fields.iter().map(|field| &field.ty))
            .collect(),
        Data::Union(data) => data.fields.named.iter().map(|field| &field.ty).collect(),
    }
}

/// Collect type arguments nested inside path segments, such as the `T` in `Vec<T>`.
pub fn path_type_arguments(ty: &Type) -> Vec<&Type> {
    let Type::Path(ty_path) = ty else {
        return Vec::new();
    };

    ty_path
        .path
        .segments
        .iter()
        .flat_map(|segment| {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                args.args
                    .iter()
                    .filter_map(|arg| {
                        if let GenericArgument::Type(inner) = arg {
                            Some(inner)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        })
        .collect()
}

/// Return the meaningful output type, unwrapping `Result<T, E>` and ignoring unit.
pub fn get_output_type(ret: &ReturnType) -> Option<&Type> {
    let ty = match ret {
        ReturnType::Default => return None,
        ReturnType::Type(_, ty) => ty.as_ref(),
    };
    if let Some(inner) = extract_result_inner(ty) {
        if !is_unit_type(inner) {
            return Some(inner);
        }
        return None;
    }
    if !is_unit_type(ty) { Some(ty) } else { None }
}

/// Return true if a return type is syntactically `Result<_, _>`.
pub fn is_result_returning(ret: &ReturnType) -> bool {
    match ret {
        ReturnType::Type(_, ty) => extract_result_inner(ty.as_ref()).is_some(),
        ReturnType::Default => false,
    }
}

/// Extract the `T` from a syntactic `Result<T, E>` path.
pub fn extract_result_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(ty_path) = ty {
        let segment = ty_path.path.segments.last()?;
        if segment.ident != "Result" {
            return None;
        }
        if let PathArguments::AngleBracketed(args) = &segment.arguments
            && let Some(GenericArgument::Type(inner)) = args.args.first()
        {
            return Some(inner);
        }
    }
    None
}

/// Return true for the unit type `()`.
pub fn is_unit_type(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

/// Return true for a simple path matching `name`, such as `u32`.
pub fn is_primitive(ty: &Type, name: &str) -> bool {
    if let Type::Path(ty_path) = ty {
        ty_path.qself.is_none() && ty_path.path.is_ident(name)
    } else {
        false
    }
}

/// Return true for a syntactic `Vec<u8>` path.
pub fn is_vec_u8(ty: &Type) -> bool {
    if let Type::Path(ty_path) = ty
        && ty_path.qself.is_none()
        && let Some(segment) = ty_path.path.segments.last()
        && segment.ident == "Vec"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(inner)) = args.args.first()
    {
        return is_primitive(inner, "u8");
    }
    false
}

/// Return true for a reference to any slice, matching mutability exactly.
pub fn is_slice_ref(ty: &Type, mutable: bool) -> bool {
    if let Type::Reference(reference) = ty {
        reference.mutability.is_some() == mutable
            && matches!(reference.elem.as_ref(), Type::Slice(_))
    } else {
        false
    }
}

/// Match `&mut u32`, `&mut i32`, or `&mut u64`, returning the inner primitive name.
pub fn mut_scalar_inner(ty: &Type) -> Option<&'static str> {
    if let Type::Reference(reference) = ty
        && reference.mutability.is_some()
        && let Type::Path(ty_path) = reference.elem.as_ref()
        && ty_path.qself.is_none()
    {
        let ident = ty_path.path.get_ident()?.to_string();
        return match ident.as_str() {
            "u32" => Some("u32"),
            "i32" => Some("i32"),
            "u64" => Some("u64"),
            _ => None,
        };
    }
    None
}

/// Return true for `(u32, u32)`.
pub fn is_tuple_u32_u32(ty: &Type) -> bool {
    if let Type::Tuple(tuple) = ty
        && tuple.elems.len() == 2
    {
        return is_primitive(&tuple.elems[0], "u32") && is_primitive(&tuple.elems[1], "u32");
    }
    false
}

/// Return true for `&mut SomePath`, excluding primitive scalar inout types.
pub fn is_context_ref(ty: &Type) -> bool {
    if let Type::Reference(reference) = ty
        && reference.mutability.is_some()
        && let Type::Path(ty_path) = reference.elem.as_ref()
    {
        if let Some(ident) = ty_path.path.get_ident()
            && matches!(ident.to_string().as_str(), "u32" | "i32" | "u64")
        {
            return false;
        }
        return true;
    }
    false
}

/// Return true for a non-primitive path type routed through `TeeParam<C>`.
pub fn is_tee_param_path(ty: &Type) -> bool {
    if let Type::Path(ty_path) = ty {
        if ty_path.qself.is_some() {
            return false;
        }
        if let Some(ident) = ty_path.path.get_ident()
            && matches!(
                ident.to_string().as_str(),
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "bool"
            )
        {
            return false;
        }
        if is_vec_u8(ty) {
            return false;
        }
        return true;
    }
    false
}

/// Return true for `&mut T` where `T` is a non-primitive path type.
pub fn is_mut_tee_param_ref(ty: &Type) -> bool {
    if let Type::Reference(reference) = ty
        && reference.mutability.is_some()
    {
        return is_tee_param_path(reference.elem.as_ref());
    }
    false
}

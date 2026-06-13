use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
    ptr,
};

use libcogcore_sys as sys;

use crate::error::GlibError;

pub(crate) fn optional_cstring(value: Option<&str>) -> crate::Result<Option<CString>> {
    value.map(CString::new).transpose().map_err(Into::into)
}

pub(crate) fn cstring(value: &str) -> crate::Result<CString> {
    CString::new(value).map_err(Into::into)
}

pub(crate) fn optional_ptr(value: Option<&CString>) -> *const c_char {
    value.map_or(ptr::null(), |value| value.as_ptr())
}

pub(crate) fn bool_to_gboolean(value: bool) -> sys::gboolean {
    if value { 1 } else { 0 }
}

pub(crate) fn gboolean_to_bool(value: sys::gboolean) -> bool {
    value != 0
}

pub(crate) fn string_from_const(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    // SAFETY: The caller passes a non-null pointer returned by Cog/GLib for a
    // NUL-terminated string that remains valid for the duration of this copy.
    let value = unsafe { CStr::from_ptr(ptr) };
    Some(value.to_string_lossy().into_owned())
}

pub(crate) fn string_from_owned(ptr: *mut c_char, context: &'static str) -> crate::Result<String> {
    if ptr.is_null() {
        return Err(crate::Error::Null(context));
    }

    // SAFETY: The pointer is a non-null transfer-full gchar* returned by GLib.
    let value = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    let mem = ptr.cast::<c_void>();
    // SAFETY: The pointer came from GLib/Cog as transfer-full memory and is
    // freed exactly once after copying it into an owned Rust String.
    unsafe { sys::g_free(mem) };

    Ok(value)
}

pub(crate) fn take_gerror(ptr: *mut sys::GError) -> GlibError {
    if ptr.is_null() {
        return GlibError {
            domain: 0,
            code: 0,
            message: "unknown GLib error".to_string(),
        };
    }

    // SAFETY: The pointer is a non-null GError** output from a GLib API. Its
    // fields are copied before freeing the GError below.
    let error = unsafe { &*ptr };
    let message = string_from_const(error.message).unwrap_or_else(|| "unknown GLib error".into());
    let glib_error = GlibError {
        domain: error.domain,
        code: error.code,
        message,
    };
    // SAFETY: Ownership of this GError was transferred through a GError**
    // output parameter, so the wrapper must release it after copying fields.
    unsafe { sys::g_error_free(ptr) };

    glib_error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_embedded_nul() {
        assert!(cstring("bad\0input").is_err());
    }

    #[test]
    fn converts_gboolean() {
        assert_eq!(bool_to_gboolean(false), 0);
        assert_eq!(bool_to_gboolean(true), 1);
        assert!(!gboolean_to_bool(0));
        assert!(gboolean_to_bool(27));
    }
}

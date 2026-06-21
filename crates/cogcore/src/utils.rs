use std::ptr;

use cogcore_sys as sys;

use crate::{Error, Result, raw};

/// Converts an application ID into a D-Bus object path.
pub fn appid_to_dbus_object_path(appid: &str) -> Result<String> {
    let appid = raw::cstring(appid)?;
    // SAFETY: `appid` is a valid NUL-terminated string for this call. Cog
    // returns a transfer-full gchar*.
    let path = unsafe { sys::cog_appid_to_dbus_object_path(appid.as_ptr()) };
    raw::string_from_owned(path, "cog_appid_to_dbus_object_path")
}

/// Guesses a URI from user input.
pub fn uri_guess_from_user_input(uri_like: &str, is_cli_arg: bool) -> Result<String> {
    let uri_like = raw::cstring(uri_like)?;
    let mut error = ptr::null_mut();
    // SAFETY: `uri_like` is a valid NUL-terminated string for this call, and
    // `error` is a valid GError** out parameter.
    let uri = unsafe {
        sys::cog_uri_guess_from_user_input(
            uri_like.as_ptr(),
            raw::bool_to_gboolean(is_cli_arg),
            &mut error,
        )
    };
    if uri.is_null() {
        if error.is_null() {
            Err(Error::Null("cog_uri_guess_from_user_input"))
        } else {
            Err(Error::glib(error))
        }
    } else {
        raw::string_from_owned(uri, "cog_uri_guess_from_user_input")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_embedded_nul() {
        assert!(appid_to_dbus_object_path("bad\0appid").is_err());
        assert!(uri_guess_from_user_input("bad\0uri", false).is_err());
    }
}

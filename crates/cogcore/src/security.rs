use cogcore_sys as sys;

use crate::{object::ObjectPtr, raw};

/// Owned wrapper around WebKit's scheme security manager.
///
/// Obtained via [`Shell::security_manager`]. Register custom URI schemes
/// here before the first page load so WebKit applies the policies to all
/// navigations.
#[derive(Clone)]
pub struct SecurityManager {
    ptr: ObjectPtr<sys::WebKitSecurityManager>,
}

impl SecurityManager {
    pub(crate) fn from_borrowed(
        ptr: *mut sys::WebKitSecurityManager,
        context: &'static str,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptr: ObjectPtr::from_borrowed(ptr, context)?,
        })
    }

    /// Registers `scheme` as a secure origin (`window.isSecureContext === true`).
    ///
    /// Must be called before the first navigation to `scheme://` URIs; WebKit
    /// evaluates security policies at load time.
    pub fn register_secure(&self, scheme: &str) -> crate::Result<()> {
        let scheme = raw::cstring(scheme)?;
        // SAFETY: The security manager is a live borrowed GObject ref-counted
        // through ObjectPtr. `scheme` is a valid NUL-terminated string for the
        // duration of the call.
        unsafe {
            sys::webkit_security_manager_register_uri_scheme_as_secure(
                self.as_ptr(),
                scheme.as_ptr(),
            )
        };
        Ok(())
    }

    /// Registers `scheme` so pages served from it may make cross-origin requests.
    pub fn register_cors_enabled(&self, scheme: &str) -> crate::Result<()> {
        let scheme = raw::cstring(scheme)?;
        // SAFETY: Same invariants as `register_secure`.
        unsafe {
            sys::webkit_security_manager_register_uri_scheme_as_cors_enabled(
                self.as_ptr(),
                scheme.as_ptr(),
            )
        };
        Ok(())
    }

    fn as_ptr(&self) -> *mut sys::WebKitSecurityManager {
        self.ptr.as_ptr()
    }
}

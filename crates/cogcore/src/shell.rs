use cogcore_sys as sys;

use crate::{handlers::RequestHandler, object::ObjectPtr, raw, security::SecurityManager};

/// Owned Cog shell.
#[derive(Clone)]
pub struct Shell {
    ptr: ObjectPtr<sys::CogShell>,
}

impl Shell {
    /// Creates a new shell.
    pub fn new(name: &str, automated: bool) -> crate::Result<Self> {
        let name = raw::cstring(name)?;
        // SAFETY: `name` is a valid NUL-terminated string for the duration of
        // the call, and Cog returns a transfer-full GObject pointer.
        let ptr = unsafe { sys::cog_shell_new(name.as_ptr(), raw::bool_to_gboolean(automated)) };
        Ok(Self {
            ptr: ObjectPtr::from_owned(ptr, "cog_shell_new")?,
        })
    }

    /// Returns the shell name.
    pub fn name(&self) -> Option<String> {
        // SAFETY: `self.ptr` owns a live CogShell and Cog returns a borrowed
        // string pointer copied immediately by the wrapper.
        let name = unsafe { sys::cog_shell_get_name(self.as_ptr()) };
        raw::string_from_const(name)
    }

    /// Returns the configured device scale factor.
    pub fn device_scale_factor(&self) -> f64 {
        // SAFETY: `self.ptr` owns a live CogShell.
        unsafe { sys::cog_shell_get_device_scale_factor(self.as_ptr()) }
    }

    /// Returns whether this shell was created in automated mode.
    pub fn is_automated(&self) -> bool {
        // SAFETY: `self.ptr` owns a live CogShell.
        let automated = unsafe { sys::cog_shell_is_automated(self.as_ptr()) };
        raw::gboolean_to_bool(automated)
    }

    /// Installs a request handler for a URI scheme.
    pub fn set_request_handler(&self, scheme: &str, handler: &RequestHandler) -> crate::Result<()> {
        let scheme = raw::cstring(scheme)?;
        // SAFETY: The shell and handler are live GObjects, and `scheme` is a
        // valid NUL-terminated string for the duration of the call.
        unsafe {
            sys::cog_shell_set_request_handler(self.as_ptr(), scheme.as_ptr(), handler.as_ptr())
        };
        Ok(())
    }

    /// Returns the WebKit security manager for this shell's web context.
    ///
    /// Use this to register custom URI schemes as secure or CORS-enabled
    /// **before** the first page load.
    pub fn security_manager(&self) -> crate::Result<SecurityManager> {
        // SAFETY: `self.ptr` owns a live CogShell. The returned security
        // manager is borrowed from the web context and must not be unreffed
        // by the caller; `SecurityManager::from_borrowed` takes a ref.
        let mgr = unsafe {
            sys::webkit_web_context_get_security_manager(self.web_context_ptr())
        };
        SecurityManager::from_borrowed(mgr, "webkit_web_context_get_security_manager")
    }

    /// Returns the borrowed WebKit context pointer for advanced integrations.
    pub fn web_context_ptr(&self) -> *mut sys::WebKitWebContext {
        // SAFETY: `self.ptr` owns a live CogShell. The returned pointer is
        // borrowed from Cog and must not be unreffed by the caller.
        unsafe { sys::cog_shell_get_web_context(self.as_ptr()) }
    }

    /// Returns the borrowed WebKit settings pointer for advanced integrations.
    pub fn web_settings_ptr(&self) -> *mut sys::WebKitSettings {
        // SAFETY: `self.ptr` owns a live CogShell. The returned pointer is
        // borrowed from Cog and must not be unreffed by the caller.
        unsafe { sys::cog_shell_get_web_settings(self.as_ptr()) }
    }

    /// Returns the borrowed GLib key file pointer for advanced integrations.
    pub fn config_file_ptr(&self) -> *mut sys::GKeyFile {
        // SAFETY: `self.ptr` owns a live CogShell. The returned pointer is
        // borrowed from Cog and must not be freed by the caller.
        unsafe { sys::cog_shell_get_config_file(self.as_ptr()) }
    }

    /// Returns the underlying Cog pointer.
    pub fn as_ptr(&self) -> *mut sys::CogShell {
        self.ptr.as_ptr()
    }
}

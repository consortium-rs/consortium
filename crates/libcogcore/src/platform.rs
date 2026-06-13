use std::{marker::PhantomData, ptr};

use libcogcore_sys as sys;

use crate::{Error, Result, Shell, raw};

/// Borrowed global Cog platform.
#[derive(Copy, Clone)]
pub struct Platform {
    ptr: *mut sys::CogPlatform,
    _not_send_sync: PhantomData<*mut sys::CogPlatform>,
}

impl Platform {
    /// Returns the current global Cog platform.
    pub fn get() -> Result<Self> {
        // SAFETY: Cog owns the global platform pointer. This wrapper borrows it
        // and never unrefs it.
        let ptr = unsafe { sys::cog_platform_get() };
        if ptr.is_null() {
            return Err(Error::Null("cog_platform_get"));
        }
        Ok(Self {
            ptr,
            _not_send_sync: PhantomData,
        })
    }

    /// Sets up the platform for the given shell.
    pub fn setup(&self, shell: &Shell, params: Option<&str>) -> Result<()> {
        let params = raw::optional_cstring(params)?;
        let mut error = ptr::null_mut();
        // SAFETY: Platform and shell are live Cog objects, `params` is null or a
        // valid NUL-terminated string, and `error` is a valid GError** out param.
        let ok = unsafe {
            sys::cog_platform_setup(
                self.ptr,
                shell.as_ptr(),
                raw::optional_ptr(params.as_ref()),
                &mut error,
            )
        };
        if raw::gboolean_to_bool(ok) {
            Ok(())
        } else {
            Err(Error::glib(error))
        }
    }

    /// Returns a borrowed WebKit view backend for advanced integrations.
    pub fn view_backend_ptr(
        &self,
        related_view: Option<*mut sys::WebKitWebView>,
    ) -> Result<*mut sys::WebKitWebViewBackend> {
        let mut error = ptr::null_mut();
        // SAFETY: Platform is a live Cog object. `related_view` is either null
        // or a caller-provided WebKit pointer, and `error` is a valid out param.
        let backend = unsafe {
            sys::cog_platform_get_view_backend(
                self.ptr,
                related_view.unwrap_or(ptr::null_mut()),
                &mut error,
            )
        };
        if backend.is_null() {
            if error.is_null() {
                Err(Error::Null("cog_platform_get_view_backend"))
            } else {
                Err(Error::glib(error))
            }
        } else {
            Ok(backend)
        }
    }

    /// Runs Cog platform web view initialization on a raw WebKit view.
    ///
    /// # Safety
    ///
    /// `view` must be a valid `WebKitWebView*` compatible with this platform.
    pub unsafe fn init_web_view_ptr(&self, view: *mut sys::WebKitWebView) {
        // SAFETY: The caller guarantees that `view` is a valid WebKitWebView.
        unsafe { sys::cog_platform_init_web_view(self.ptr, view) };
    }

    /// Returns the underlying borrowed Cog pointer.
    pub fn as_ptr(&self) -> *mut sys::CogPlatform {
        self.ptr
    }
}

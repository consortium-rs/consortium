use libcogcore_sys as sys;

use crate::{Viewport, object::ObjectPtr, raw};

/// Owned Cog web view.
#[derive(Clone)]
pub struct View {
    ptr: ObjectPtr<sys::CogView>,
}

impl View {
    /// Creates a new view with default properties.
    pub fn new() -> crate::Result<Self> {
        // SAFETY: Passing a null first property name terminates the variadic
        // property list, matching GObject construction conventions.
        let ptr = unsafe { sys::cog_view_new(std::ptr::null()) };
        Ok(Self {
            ptr: ObjectPtr::from_owned(ptr, "cog_view_new")?,
        })
    }

    pub(crate) fn from_borrowed(
        ptr: *mut sys::CogView,
        context: &'static str,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptr: ObjectPtr::from_borrowed(ptr, context)?,
        })
    }

    /// Enables or disables Cog key bindings for this view.
    pub fn set_use_key_bindings(&self, enable: bool) {
        // SAFETY: `self.ptr` owns a live CogView.
        unsafe { sys::cog_view_set_use_key_bindings(self.as_ptr(), raw::bool_to_gboolean(enable)) };
    }

    /// Returns whether Cog key bindings are enabled for this view.
    pub fn use_key_bindings(&self) -> bool {
        // SAFETY: `self.ptr` owns a live CogView.
        let enabled = unsafe { sys::cog_view_get_use_key_bindings(self.as_ptr()) };
        raw::gboolean_to_bool(enabled)
    }

    /// Returns whether this view is visible.
    pub fn is_visible(&self) -> bool {
        // SAFETY: `self.ptr` owns a live CogView.
        let visible = unsafe { sys::cog_view_is_visible(self.as_ptr()) };
        raw::gboolean_to_bool(visible)
    }

    /// Makes this view visible, returning whether Cog accepted the change.
    pub fn set_visible(&self) -> bool {
        // SAFETY: `self.ptr` owns a live CogView.
        let visible = unsafe { sys::cog_view_set_visible(self.as_ptr()) };
        raw::gboolean_to_bool(visible)
    }

    /// Enables or disables fullscreen, returning whether Cog accepted the change.
    pub fn set_fullscreen(&self, enable: bool) -> bool {
        // SAFETY: `self.ptr` owns a live CogView.
        let fullscreen =
            unsafe { sys::cog_view_set_fullscreen(self.as_ptr(), raw::bool_to_gboolean(enable)) };
        raw::gboolean_to_bool(fullscreen)
    }

    /// Returns whether this view is fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        // SAFETY: `self.ptr` owns a live CogView.
        let fullscreen = unsafe { sys::cog_view_is_fullscreen(self.as_ptr()) };
        raw::gboolean_to_bool(fullscreen)
    }

    /// Returns the borrowed WPE backend pointer for advanced integrations.
    pub fn backend_ptr(&self) -> *mut sys::wpe_view_backend {
        // SAFETY: `self.ptr` owns a live CogView. The returned pointer is
        // borrowed from Cog/WPE.
        unsafe { sys::cog_view_get_backend(self.as_ptr()) }
    }

    /// Returns the viewport containing this view, if it has one.
    pub fn viewport(&self) -> Option<Viewport> {
        // SAFETY: `self.ptr` owns a live CogView. Cog returns a borrowed
        // CogViewport or null when the view has not been added to one.
        let viewport = unsafe { sys::cog_view_get_viewport(self.as_ptr()) };
        Viewport::from_borrowed(viewport, "cog_view_get_viewport").ok()
    }

    /// Returns the underlying Cog pointer.
    pub fn as_ptr(&self) -> *mut sys::CogView {
        self.ptr.as_ptr()
    }

    /// Returns the content manager attached to this view.
    pub fn user_content_manager(&self) -> Option<crate::UserContentManager> {
        // SAFETY: self.ptr owns a live CogView, which is also a WebKitWebView.
        let ucm = unsafe { sys::webkit_web_view_get_user_content_manager(self.as_webkit_ptr()) };
        crate::UserContentManager::from_borrowed(ucm, "webkit_web_view_get_user_content_manager")
            .ok()
    }

    /// Navigates the view to the given URI.
    pub fn load_uri(&self, uri: &str) -> crate::Result<()> {
        let uri = raw::cstring(uri)?;
        // SAFETY: self.ptr owns a live WebKitWebView and uri is a valid NUL-terminated string.
        unsafe { sys::webkit_web_view_load_uri(self.as_webkit_ptr(), uri.as_ptr()) };
        Ok(())
    }

    /// Evaluates `script` in the view, ignoring any result or error.
    pub fn run_js(&self, script: &str) -> crate::Result<()> {
        let script = raw::cstring(script)?;
        // SAFETY: self.ptr owns a live WebKitWebView; null cancellable and callback
        // make this a fire-and-forget evaluation.
        unsafe {
            sys::webkit_web_view_evaluate_javascript(
                self.as_webkit_ptr(),
                script.as_ptr(),
                -1,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                None,
                std::ptr::null_mut(),
            )
        };
        Ok(())
    }

    /// Returns this view as an opaque WebKit pointer.
    pub fn as_webkit_ptr(&self) -> *mut sys::WebKitWebView {
        self.as_ptr().cast::<sys::WebKitWebView>()
    }
}

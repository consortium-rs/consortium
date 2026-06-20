use libcogcore_sys as sys;

use crate::raw;

/// An owned user script injected into WebKit pages.
pub struct UserScript {
    ptr: *mut sys::WebKitUserScript,
}

impl UserScript {
    /// Creates a script injected at document start into all frames.
    pub fn new_at_document_start(source: &str) -> crate::Result<Self> {
        let source = raw::cstring(source)?;
        // SAFETY: source is a valid NUL-terminated string; null allow/block lists
        // apply the script to all origins; returns a transfer-full script reference.
        let ptr = unsafe {
            sys::webkit_user_script_new(
                source.as_ptr(),
                sys::WebKitUserContentInjectedFrames_WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES,
                sys::WebKitUserScriptInjectionTime_WEBKIT_USER_SCRIPT_INJECT_AT_DOCUMENT_START,
                std::ptr::null(),
                std::ptr::null(),
            )
        };
        if ptr.is_null() {
            return Err(crate::Error::Null("webkit_user_script_new"));
        }
        Ok(Self { ptr })
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::WebKitUserScript {
        self.ptr
    }
}

impl Drop for UserScript {
    fn drop(&mut self) {
        // SAFETY: self.ptr is a transfer-full WebKitUserScript reference acquired
        // in new_at_document_start and released exactly once here.
        unsafe { sys::webkit_user_script_unref(self.ptr) };
    }
}

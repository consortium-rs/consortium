use std::os::raw::c_void;

use libcogcore_sys as sys;

use crate::{object::ObjectPtr, raw};

/// Owned wrapper around a WebKitUserContentManager.
#[derive(Clone)]
pub struct UserContentManager {
    ptr: ObjectPtr<sys::WebKitUserContentManager>,
}

impl UserContentManager {
    pub(crate) fn from_borrowed(
        ptr: *mut sys::WebKitUserContentManager,
        context: &'static str,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptr: ObjectPtr::from_borrowed(ptr, context)?,
        })
    }

    /// Adds a user script to be injected into every page loaded by the owning view.
    pub fn add_script(&self, script: &crate::UserScript) {
        // SAFETY: manager and script are live GObject references for the duration of this call.
        unsafe { sys::webkit_user_content_manager_add_script(self.as_ptr(), script.as_ptr()) };
    }

    /// Registers a named JS→Rust message channel.
    ///
    /// After this call JS can call:
    /// `window.webkit.messageHandlers.<name>.postMessage(data)`
    pub fn register_script_message_handler(&self, name: &str) -> crate::Result<()> {
        let name = raw::cstring(name)?;
        // SAFETY: manager is a live GObject and name is a valid NUL-terminated string.
        unsafe {
            sys::webkit_user_content_manager_register_script_message_handler(
                self.as_ptr(),
                name.as_ptr(),
            )
        };
        Ok(())
    }

    /// Connects `callback` to receive JS messages posted to the channel named `name`.
    ///
    /// `name` must match a prior call to [`register_script_message_handler`]. The
    /// closure receives the raw string passed to `postMessage` from JS.
    pub fn connect_script_message(
        &self,
        name: &str,
        callback: impl Fn(String) + 'static,
    ) -> crate::Result<()> {
        let signal = format!("script-message-received::{name}");
        let signal_cstr = raw::cstring(&signal)?;

        let boxed: Box<dyn Fn(String) + 'static> = Box::new(callback);
        let data = Box::into_raw(Box::new(boxed)).cast::<c_void>();

        // SAFETY: GCallback is a type-erased C function pointer; the actual ABI
        // matches the script-message-received signal: (manager, result, user_data).
        let cb: unsafe extern "C" fn() = unsafe {
            std::mem::transmute::<
                unsafe extern "C" fn(
                    *mut sys::WebKitUserContentManager,
                    *mut sys::WebKitJavascriptResult,
                    sys::gpointer,
                ),
                unsafe extern "C" fn(),
            >(script_message_trampoline)
        };

        // SAFETY: manager is a live GObject; data is a leaked Box valid until
        // destroy_closure_data is called by GLib when the signal is disconnected.
        unsafe {
            sys::g_signal_connect_data(
                self.as_ptr().cast::<c_void>(),
                signal_cstr.as_ptr(),
                Some(cb),
                data,
                Some(destroy_closure_data),
                0,
            )
        };

        Ok(())
    }

    /// Returns the underlying pointer.
    pub fn as_ptr(&self) -> *mut sys::WebKitUserContentManager {
        self.ptr.as_ptr()
    }
}

unsafe extern "C" fn script_message_trampoline(
    _manager: *mut sys::WebKitUserContentManager,
    result: *mut sys::WebKitJavascriptResult,
    user_data: sys::gpointer,
) {
    // SAFETY: user_data is a leaked Box<Box<dyn Fn(String)>> from connect_script_message,
    // kept alive by g_signal_connect_data until destroy_closure_data is called.
    let callback = unsafe { &*(user_data as *const Box<dyn Fn(String) + 'static>) };

    // SAFETY: result is a live WebKitJavascriptResult delivered by WebKit's signal dispatch.
    let js_value = unsafe { sys::webkit_javascript_result_get_js_value(result) };
    if js_value.is_null() {
        return;
    }

    // SAFETY: js_value is a live JSCValue obtained from the WebKitJavascriptResult above.
    let str_ptr = unsafe { sys::jsc_value_to_string(js_value) };

    let message = raw::string_from_owned(str_ptr, "jsc_value_to_string").unwrap_or_default();
    callback(message);
}

unsafe extern "C" fn destroy_closure_data(data: sys::gpointer, _closure: *mut sys::GClosure) {
    // SAFETY: data is a leaked Box<Box<dyn Fn(String)>> from connect_script_message.
    // GLib calls this exactly once when the signal connection is destroyed.
    let boxed = unsafe { Box::from_raw(data as *mut Box<dyn Fn(String) + 'static>) };
    drop(boxed);
}

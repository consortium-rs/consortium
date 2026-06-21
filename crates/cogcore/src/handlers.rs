use glib::translate::ToGlibPtr;
use cogcore_sys as sys;

use crate::{Error, Result, object::ObjectPtr, raw};

/// Owned Cog request handler interface object.
#[derive(Clone)]
pub struct RequestHandler {
    ptr: ObjectPtr<sys::CogRequestHandler>,
}

impl RequestHandler {
    pub(crate) fn from_owned(
        ptr: *mut sys::CogRequestHandler,
        context: &'static str,
    ) -> Result<Self> {
        Ok(Self {
            ptr: ObjectPtr::from_owned(ptr, context)?,
        })
    }

    /// Returns the underlying Cog request handler pointer.
    pub fn as_ptr(&self) -> *mut sys::CogRequestHandler {
        self.ptr.as_ptr()
    }
}

/// Directory-backed request handler.
#[derive(Clone)]
pub struct DirectoryFilesHandler {
    handler: RequestHandler,
}

impl DirectoryFilesHandler {
    /// Creates a handler rooted at a `gio::File` directory.
    pub fn new(base_path: &gio::File) -> Result<Self> {
        let file: *mut gio::ffi::GFile = base_path.to_glib_none().0;
        let file = file.cast::<sys::GFile>();
        // SAFETY: `file` is a borrowed GFile pointer from `base_path` and is
        // valid for the duration of the call. Cog returns a transfer-full
        // request handler object.
        let ptr = unsafe { sys::cog_directory_files_handler_new(file) };
        Ok(Self {
            handler: RequestHandler::from_owned(ptr, "cog_directory_files_handler_new")?,
        })
    }

    /// Validates whether a `gio::File` can be used as a directory handler root.
    pub fn is_suitable_path(file: &gio::File) -> Result<bool> {
        let file: *mut gio::ffi::GFile = file.to_glib_none().0;
        let file = file.cast::<sys::GFile>();
        let mut error = std::ptr::null_mut();
        // SAFETY: `file` is a borrowed GFile pointer valid for this call, and
        // `error` is a valid GError** out parameter.
        let ok = unsafe { sys::cog_directory_files_handler_is_suitable_path(file, &mut error) };
        if raw::gboolean_to_bool(ok) {
            Ok(true)
        } else if error.is_null() {
            Ok(false)
        } else {
            Err(Error::glib(error))
        }
    }

    /// Returns whether host names are included in generated file routes.
    pub fn use_host(&self) -> bool {
        // SAFETY: The wrapped handler was constructed as a
        // CogDirectoryFilesHandler and remains live.
        let use_host =
            unsafe { sys::cog_directory_files_handler_get_use_host(self.directory_ptr()) };
        raw::gboolean_to_bool(use_host)
    }

    /// Sets whether host names are included in generated file routes.
    pub fn set_use_host(&self, use_host: bool) {
        // SAFETY: The wrapped handler was constructed as a
        // CogDirectoryFilesHandler and remains live.
        unsafe {
            sys::cog_directory_files_handler_set_use_host(
                self.directory_ptr(),
                raw::bool_to_gboolean(use_host),
            )
        };
    }

    /// Returns how many URI path components are stripped.
    pub fn strip_components(&self) -> u32 {
        // SAFETY: The wrapped handler was constructed as a
        // CogDirectoryFilesHandler and remains live.
        unsafe { sys::cog_directory_files_handler_get_strip_components(self.directory_ptr()) }
    }

    /// Sets how many URI path components are stripped.
    pub fn set_strip_components(&self, count: u32) {
        // SAFETY: The wrapped handler was constructed as a
        // CogDirectoryFilesHandler and remains live.
        unsafe {
            sys::cog_directory_files_handler_set_strip_components(self.directory_ptr(), count)
        };
    }

    /// Returns this value as its request handler interface.
    pub fn as_request_handler(&self) -> &RequestHandler {
        &self.handler
    }

    /// Returns the underlying request handler pointer.
    pub fn as_ptr(&self) -> *mut sys::CogRequestHandler {
        self.handler.as_ptr()
    }

    fn directory_ptr(&self) -> *mut sys::CogDirectoryFilesHandler {
        self.handler
            .as_ptr()
            .cast::<sys::CogDirectoryFilesHandler>()
    }
}

/// Host-based request route handler.
#[derive(Clone)]
pub struct HostRoutesHandler {
    handler: RequestHandler,
}

impl HostRoutesHandler {
    /// Creates a host route handler with an optional fallback handler.
    pub fn new(fallback_handler: Option<&RequestHandler>) -> Result<Self> {
        let fallback_handler =
            fallback_handler.map_or(std::ptr::null_mut(), RequestHandler::as_ptr);
        // SAFETY: The fallback pointer is null or a live CogRequestHandler. Cog
        // returns a transfer-full request handler object.
        let ptr = unsafe { sys::cog_host_routes_handler_new(fallback_handler) };
        Ok(Self {
            handler: RequestHandler::from_owned(ptr, "cog_host_routes_handler_new")?,
        })
    }

    /// Returns whether a host route exists.
    pub fn contains(&self, host: &str) -> Result<bool> {
        let host = raw::cstring(host)?;
        // SAFETY: The wrapped handler is a live CogHostRoutesHandler and `host`
        // is a valid NUL-terminated string for this call.
        let contains =
            unsafe { sys::cog_host_routes_handler_contains(self.host_ptr(), host.as_ptr()) };
        Ok(raw::gboolean_to_bool(contains))
    }

    /// Adds or replaces a host route.
    pub fn add(&self, host: &str, handler: &RequestHandler) -> Result<bool> {
        let host = raw::cstring(host)?;
        // SAFETY: The route handler and target handler are live Cog objects, and
        // `host` is a valid NUL-terminated string.
        let added = unsafe {
            sys::cog_host_routes_handler_add(self.host_ptr(), host.as_ptr(), handler.as_ptr())
        };
        Ok(raw::gboolean_to_bool(added))
    }

    /// Removes a host route.
    pub fn remove(&self, host: &str) -> Result<bool> {
        let host = raw::cstring(host)?;
        // SAFETY: The wrapped handler is live and `host` is a valid
        // NUL-terminated string.
        let removed =
            unsafe { sys::cog_host_routes_handler_remove(self.host_ptr(), host.as_ptr()) };
        Ok(raw::gboolean_to_bool(removed))
    }

    /// Adds a filesystem path route for `host`.
    pub fn add_path(&self, host: &str, base_path: &str) -> Result<bool> {
        let host = raw::cstring(host)?;
        let base_path = raw::cstring(base_path)?;
        // SAFETY: The wrapped handler is live and both strings are valid
        // NUL-terminated strings for this call.
        let added = unsafe {
            sys::cog_host_routes_handler_add_path(
                self.host_ptr(),
                host.as_ptr(),
                base_path.as_ptr(),
            )
        };
        Ok(raw::gboolean_to_bool(added))
    }

    /// Returns this value as its request handler interface.
    pub fn as_request_handler(&self) -> &RequestHandler {
        &self.handler
    }

    /// Returns the underlying request handler pointer.
    pub fn as_ptr(&self) -> *mut sys::CogRequestHandler {
        self.handler.as_ptr()
    }

    fn host_ptr(&self) -> *mut sys::CogHostRoutesHandler {
        self.handler.as_ptr().cast::<sys::CogHostRoutesHandler>()
    }
}

/// Prefix-based request route handler.
#[derive(Clone)]
pub struct PrefixRoutesHandler {
    handler: RequestHandler,
}

impl PrefixRoutesHandler {
    /// Creates a prefix route handler with an optional fallback handler.
    pub fn new(fallback_handler: Option<&RequestHandler>) -> Result<Self> {
        let fallback_handler =
            fallback_handler.map_or(std::ptr::null_mut(), RequestHandler::as_ptr);
        // SAFETY: The fallback pointer is null or a live CogRequestHandler. Cog
        // returns a transfer-full request handler object.
        let ptr = unsafe { sys::cog_prefix_routes_handler_new(fallback_handler) };
        Ok(Self {
            handler: RequestHandler::from_owned(ptr, "cog_prefix_routes_handler_new")?,
        })
    }

    /// Mounts a request handler at a path prefix.
    pub fn mount(&self, path_prefix: &str, handler: &RequestHandler) -> Result<bool> {
        let path_prefix = raw::cstring(path_prefix)?;
        // SAFETY: The prefix handler and target handler are live Cog objects,
        // and `path_prefix` is a valid NUL-terminated string.
        let mounted = unsafe {
            sys::cog_prefix_routes_handler_mount(
                self.prefix_ptr(),
                path_prefix.as_ptr(),
                handler.as_ptr(),
            )
        };
        Ok(raw::gboolean_to_bool(mounted))
    }

    /// Unmounts a path prefix.
    pub fn unmount(&self, path_prefix: &str) -> Result<bool> {
        let path_prefix = raw::cstring(path_prefix)?;
        // SAFETY: The wrapped handler is live and `path_prefix` is a valid
        // NUL-terminated string.
        let unmounted = unsafe {
            sys::cog_prefix_routes_handler_unmount(self.prefix_ptr(), path_prefix.as_ptr())
        };
        Ok(raw::gboolean_to_bool(unmounted))
    }

    /// Mounts a filesystem path at a path prefix.
    pub fn mount_path(&self, path_prefix: &str, base_path: &str) -> Result<bool> {
        let path_prefix = raw::cstring(path_prefix)?;
        let base_path = raw::cstring(base_path)?;
        // SAFETY: The wrapped handler is live and both strings are valid
        // NUL-terminated strings for this call.
        let mounted = unsafe {
            sys::cog_prefix_routes_handler_mount_path(
                self.prefix_ptr(),
                path_prefix.as_ptr(),
                base_path.as_ptr(),
            )
        };
        Ok(raw::gboolean_to_bool(mounted))
    }

    /// Returns this value as its request handler interface.
    pub fn as_request_handler(&self) -> &RequestHandler {
        &self.handler
    }

    /// Returns the underlying request handler pointer.
    pub fn as_ptr(&self) -> *mut sys::CogRequestHandler {
        self.handler.as_ptr()
    }

    fn prefix_ptr(&self) -> *mut sys::CogPrefixRoutesHandler {
        self.handler.as_ptr().cast::<sys::CogPrefixRoutesHandler>()
    }
}

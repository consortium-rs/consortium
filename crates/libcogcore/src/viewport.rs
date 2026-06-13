use libcogcore_sys as sys;

use crate::{View, object::ObjectPtr, raw};

/// Owned Cog viewport.
#[derive(Clone)]
pub struct Viewport {
    ptr: ObjectPtr<sys::CogViewport>,
}

impl Viewport {
    /// Creates a new viewport.
    pub fn new() -> crate::Result<Self> {
        // SAFETY: Cog returns a transfer-full GObject pointer.
        let ptr = unsafe { sys::cog_viewport_new() };
        Ok(Self {
            ptr: ObjectPtr::from_owned(ptr, "cog_viewport_new")?,
        })
    }

    pub(crate) fn from_borrowed(
        ptr: *mut sys::CogViewport,
        context: &'static str,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptr: ObjectPtr::from_borrowed(ptr, context)?,
        })
    }

    /// Adds a view to this viewport.
    pub fn add(&self, view: &View) {
        // SAFETY: Both pointers are live Cog GObjects.
        unsafe { sys::cog_viewport_add(self.as_ptr(), view.as_ptr()) };
    }

    /// Removes a view from this viewport.
    pub fn remove(&self, view: &View) {
        // SAFETY: Both pointers are live Cog GObjects.
        unsafe { sys::cog_viewport_remove(self.as_ptr(), view.as_ptr()) };
    }

    /// Returns whether this viewport contains `view`.
    pub fn contains(&self, view: &View) -> bool {
        // SAFETY: Both pointers are live Cog GObjects.
        let contains = unsafe { sys::cog_viewport_contains(self.as_ptr(), view.as_ptr()) };
        raw::gboolean_to_bool(contains)
    }

    /// Returns the number of views in this viewport.
    pub fn len(&self) -> usize {
        // SAFETY: `self.ptr` owns a live CogViewport.
        unsafe { sys::cog_viewport_get_n_views(self.as_ptr()) as usize }
    }

    /// Returns whether this viewport contains no views.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the view at `index`, if it exists.
    pub fn nth_view(&self, index: usize) -> Option<View> {
        // SAFETY: `self.ptr` owns a live CogViewport. Cog returns null when the
        // index is out of bounds; otherwise it returns a borrowed CogView.
        let view = unsafe { sys::cog_viewport_get_nth_view(self.as_ptr(), index as sys::gsize) };
        View::from_borrowed(view, "cog_viewport_get_nth_view").ok()
    }

    /// Returns the currently visible view, if any.
    pub fn visible_view(&self) -> Option<View> {
        // SAFETY: `self.ptr` owns a live CogViewport. Cog returns a borrowed
        // CogView or null when no visible view exists.
        let view = unsafe { sys::cog_viewport_get_visible_view(self.as_ptr()) };
        View::from_borrowed(view, "cog_viewport_get_visible_view").ok()
    }

    /// Sets the currently visible view.
    pub fn set_visible_view(&self, view: &View) {
        // SAFETY: Both pointers are live Cog GObjects.
        unsafe { sys::cog_viewport_set_visible_view(self.as_ptr(), view.as_ptr()) };
    }

    /// Returns the underlying Cog pointer.
    pub fn as_ptr(&self) -> *mut sys::CogViewport {
        self.ptr.as_ptr()
    }
}

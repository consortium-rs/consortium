use std::{marker::PhantomData, os::raw::c_void, ptr::NonNull};

use cogcore_sys as sys;

use crate::{Error, Result};

pub(crate) struct ObjectPtr<T> {
    ptr: NonNull<T>,
    _not_send_sync: PhantomData<*mut T>,
}

impl<T> ObjectPtr<T> {
    pub(crate) fn from_owned(ptr: *mut T, context: &'static str) -> Result<Self> {
        let ptr = NonNull::new(ptr).ok_or(Error::Null(context))?;
        Ok(Self {
            ptr,
            _not_send_sync: PhantomData,
        })
    }

    pub(crate) fn from_borrowed(ptr: *mut T, context: &'static str) -> Result<Self> {
        let ptr = NonNull::new(ptr).ok_or(Error::Null(context))?;
        let raw = ptr.as_ptr().cast::<c_void>();
        // SAFETY: The pointer is a live GObject borrowed from Cog. Taking a
        // reference promotes it to owned wrapper state.
        let owned = unsafe { sys::g_object_ref(raw) };
        let ptr = NonNull::new(owned.cast::<T>()).ok_or(Error::Null(context))?;
        Ok(Self {
            ptr,
            _not_send_sync: PhantomData,
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T> Clone for ObjectPtr<T> {
    fn clone(&self) -> Self {
        let raw = self.ptr.as_ptr().cast::<c_void>();
        // SAFETY: The wrapper owns a live GObject reference. Cloning acquires
        // one additional reference for the cloned wrapper.
        let owned = unsafe { sys::g_object_ref(raw) };
        let ptr = NonNull::new(owned.cast::<T>()).expect("g_object_ref returned null");
        Self {
            ptr,
            _not_send_sync: PhantomData,
        }
    }
}

impl<T> Drop for ObjectPtr<T> {
    fn drop(&mut self) {
        let raw = self.ptr.as_ptr().cast::<c_void>();
        // SAFETY: ObjectPtr owns exactly one GObject reference and releases it
        // exactly once when dropped.
        unsafe { sys::g_object_unref(raw) };
    }
}

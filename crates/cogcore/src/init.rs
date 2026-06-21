use cogcore_sys as sys;

use crate::raw;

/// Initializes Cog with an optional platform name and module path.
pub fn init(platform_name: Option<&str>, module_path: Option<&str>) -> crate::Result<()> {
    let platform_name = raw::optional_cstring(platform_name)?;
    let module_path = raw::optional_cstring(module_path)?;

    // SAFETY: Both pointers are either null or valid NUL-terminated strings
    // that outlive the call.
    unsafe {
        sys::cog_init(
            raw::optional_ptr(platform_name.as_ref()),
            raw::optional_ptr(module_path.as_ref()),
        )
    };

    Ok(())
}

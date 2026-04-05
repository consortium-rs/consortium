// `TeeParam`, the postcard re-export, and the derive macros are ungated from the Linux
// restriction — they must be available on every target (TA code running in the TEE secure
// world is not Linux).  They ARE gated behind the `serde` feature because they pull in
// `serde` + `postcard`.  Error types and the TEE runtime (optee-teec) remain Linux-only.

/// Marker trait for types that can be passed through a TEE memref slot via `postcard`.
///
/// Derive it alongside `serde::Serialize` / `serde::Deserialize`:
///
/// ```rust,ignore
/// #[derive(TeeParam, Serialize, Deserialize)]
/// struct MyConfig {
///     threshold: u32,
///     mode: u8,
/// }
/// ```
///
/// `MAX_SIZE` controls how large a buffer the CA pre-allocates for inout/output slots.
/// The default from `#[derive(TeeParam)]` is 1024 bytes; override with a manual impl.
///
/// Requires the `serde` feature of `consortium-tee`.
#[cfg(feature = "serde")]
pub trait TeeParam: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Maximum serialized size in bytes.
    const MAX_SIZE: usize;
}

/// Re-exported `postcard` for use by generated `#[tee_command]` code.
///
/// Requires the `serde` feature of `consortium-tee`.
#[cfg(feature = "serde")]
#[doc(hidden)]
pub mod __private {
    pub use postcard;
}

pub use consortium_tee_proc_macros::{tee_command, tee_service};

/// Requires the `serde` feature of `consortium-tee`.
#[cfg(feature = "serde")]
pub use consortium_tee_proc_macros::TeeParam;

#[cfg(target_os = "linux")]
pub mod error;
#[cfg(target_os = "linux")]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};

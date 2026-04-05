// `TeeParam`, the postcard re-export, and the derive macros are ungated — they must
// be available on every target (TA code running in the TEE secure world is not Linux).
// Error types and the TEE runtime (optee-teec) are Linux-only.

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
pub trait TeeParam: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Maximum serialized size in bytes.
    const MAX_SIZE: usize;
}

/// Re-exported `postcard` for use by generated `#[tee_command]` code.
#[doc(hidden)]
pub mod __private {
    pub use postcard;
}

pub use consortium_tee_proc_macros::{TeeParam, tee_command, tee_service};

#[cfg(target_os = "linux")]
pub mod error;
#[cfg(target_os = "linux")]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};

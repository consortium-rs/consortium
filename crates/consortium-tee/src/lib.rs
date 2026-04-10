// `TeeParam` and the derive macros are ungated from the Linux restriction — they must be
// available on every target (TA code running in the TEE secure world is not Linux).
// Error types and the TEE runtime (optee-teec) remain Linux-only.

/// Marker trait for types that can be passed through a TEE memref slot via a codec.
///
/// Derive it alongside the serialization traits required by your codec:
///
/// ```rust,ignore
/// #[derive(TeeParam, Serialize, Deserialize)]
/// struct MyConfig {
///     threshold: u32,
///     mode: u8,
/// }
/// ```
///
/// When using `#[tee_command]`, any `TeeParam` parameters must be accompanied by a
/// `codec` attribute specifying which codec to use:
///
/// ```rust,ignore
/// #[tee_command(codec = PostcardCodec)]
/// fn process_config(ctx: &mut Context, config: MyConfig) -> Result<(), TeeError> {
///     // ...
/// }
/// ```
///
/// `MAX_SIZE` controls how large a buffer the CA pre-allocates for inout/output slots.
/// The default from `#[derive(TeeParam)]` is 1024 bytes; override with a manual impl.
pub trait TeeParam {
    /// Maximum serialized size in bytes.
    const MAX_SIZE: usize;
}

pub use consortium_tee_proc_macros::{TeeParam, tee_command, tee_service};

#[cfg(target_os = "linux")]
pub mod error;
#[cfg(target_os = "linux")]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};

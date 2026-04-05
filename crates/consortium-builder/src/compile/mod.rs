pub mod application;
pub mod firmware;
pub mod optee;
pub(crate) mod utils;
pub mod web;

use std::path::PathBuf;

/// Unified async compilation trait for firmware, application, and OP-TEE targets.
///
/// Each implementor owns its `Configuration` type (carries all build parameters),
/// a typed `Error`, and an `Output` (usually `PathBuf` to the built artifact).
///
/// The trait methods are free functions (take `config` by value or reference) so
/// callers can use any implementor with the same `.build(cfg)` / `.output_path(&cfg)` UX.
pub trait Compiler: Sized {
    /// All build parameters for this compiler.
    type Configuration;

    /// Typed error returned from build operations.
    type Error: std::error::Error + Send + Sync + 'static;

    /// The primary artifact produced (usually `PathBuf`).
    type Output;

    /// Run the full build and return the path to the primary artifact.
    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error>;

    /// Return the expected output path without building.
    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error>;
}

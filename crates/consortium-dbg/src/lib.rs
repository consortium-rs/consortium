#![cfg_attr(not(feature = "host"), no_std)]

pub mod config;
pub mod descriptor;
#[cfg(feature = "host")]
pub mod host;

/// Re-export of the defmt stream decoder, so host consumers (e.g.
/// `consortium-runtime-app`) can name [`host::DefmtSession`] callback types
/// (`Frame`) without a separate `defmt-decoder` dependency.
#[cfg(feature = "decode")]
pub use defmt_decoder;
/// Re-export of `defmt-parser`, whose [`Level`](defmt_parser::Level) is the type
/// returned by `Frame::level()` (defmt-decoder does not re-export it publicly).
#[cfg(feature = "decode")]
pub use defmt_parser;
#[cfg(feature = "logger")]
pub mod logger;
pub mod packet;
#[cfg(feature = "panic-handler")]
pub mod panic;
pub mod utils;

#[cfg(all(
    feature = "panic-handler",
    not(any(feature = "arch-cortex-m", feature = "arch-riscv"))
))]
compile_error!(
    "`panic-handler` requires exactly one arch feature: enable `arch-cortex-m` or `arch-riscv`"
);

#[cfg(all(feature = "arch-cortex-m", feature = "arch-riscv"))]
compile_error!("`arch-cortex-m` and `arch-riscv` are mutually exclusive");

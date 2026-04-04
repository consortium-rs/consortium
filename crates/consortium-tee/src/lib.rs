#![cfg(target_os = "linux")]
pub mod error;

pub use consortium_tee_proc_macros::{tee_command, tee_service};

#[cfg(not(target_os = "linux"))]
compile_error!("`consortium-tee` is only supported on Linux (requires /dev/tee0 and libteec)");

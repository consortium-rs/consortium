#![cfg(target_os = "linux")]
pub mod error;
pub mod param;

#[cfg(not(target_os = "linux"))]
compile_error!("`consortium-tee` is only supported on Linux (requires /dev/tee0 and libteec)");

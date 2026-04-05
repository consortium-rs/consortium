#![cfg(target_os = "linux")]
pub mod remoteproc;
pub use consortium_ipc_uio as ipc_uio;
pub use consortium_tee as tee;

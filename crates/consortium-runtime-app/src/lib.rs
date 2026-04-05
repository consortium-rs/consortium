#![cfg(target_os = "linux")]
pub mod remoteproc;
pub use consortium_ipc_uio as ipc_uio;
pub use consortium_tee as tee;

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_driver_hsem as hsem;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_driver_mu as mu;

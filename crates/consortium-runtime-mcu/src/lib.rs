#![no_std]

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_doorbell_mu as mu;

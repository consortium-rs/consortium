#![no_std]

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_driver_hsem as hsem;

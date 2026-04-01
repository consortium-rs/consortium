#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod chan;
pub mod channel;
pub mod doorbell;
pub(crate) mod future;
pub mod region;
pub mod regions;
pub mod transport;

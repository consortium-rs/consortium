#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

mod chan;
mod channel;
mod doorbell;
mod future;
mod region;
mod regions;
mod transport;

pub use chan::Chan;
pub use channel::Channel;
pub use doorbell::Doorbell;
pub use region::Region;
pub use regions::{DdrRegion, SerialRegion, SramRegion};
pub use transport::Transport;

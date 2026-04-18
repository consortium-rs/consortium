use super::descriptor::{DescState, MAGIC};

#[derive(Debug, thiserror::Error)]
pub enum DescriptorError {
    #[error("bad magic: expected 0x{:08X}, got 0x{got:08X}", MAGIC)]
    BadMagic { got: u32 },

    #[error("unsupported descriptor version: {0}")]
    BadVersion(u8),

    #[error("descriptor not ready: state is {0:?}")]
    NotReady(DescState),
}

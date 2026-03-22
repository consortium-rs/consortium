/// Descriptor layout in the DMA-coherent shared region.
///
/// ```text
/// Offset  Size  Owner   Field
/// 0x00    4     kernel  magic           0x434F_4E53 ("CONS")
/// 0x04    1     kernel  version
/// 0x05    1     kernel  num_channels
/// 0x06    2     kernel  reserved
/// 0x08    4     kernel  proposed_mtu
/// 0x0C    4     m-side  accepted_mtu
/// 0x10    4     kernel  effective_mtu
/// 0x14    4     both    state           (DescState)
/// 0x18    8     kernel  reserved
/// 0x20    16*N  kernel  channel table
/// ```
///
/// Each channel table entry (16 bytes):
/// ```text
/// +0x00  4  tx_offset   A→M slot byte offset in region
/// +0x04  4  tx_size     slot size (SLOT_HEADER + effective_mtu)
/// +0x08  4  rx_offset   M→A slot byte offset in region
/// +0x0C  4  rx_size
/// ```

pub const MAGIC: u32 = 0x434F_4E53;
pub const VERSION: u8 = 1;

const CHAN_TABLE_BASE: usize = 0x20;
const CHAN_ENTRY_SIZE: usize = 16;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescState {
    KernelInit = 0,
    Proposed   = 1,
    MReady     = 2,
    Ready      = 3,
    Suspended  = 4,
    Error      = 5,
}

impl DescState {
    fn from_u32(v: u32) -> Self {
        match v {
            0 => Self::KernelInit,
            1 => Self::Proposed,
            2 => Self::MReady,
            3 => Self::Ready,
            4 => Self::Suspended,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChannelLayout {
    pub tx_offset: usize,
    pub tx_size:   usize,
    pub rx_offset: usize,
    pub rx_size:   usize,
}

#[derive(Debug)]
pub struct Descriptor {
    pub num_channels:  u8,
    pub effective_mtu: usize,
    pub channels:      Vec<ChannelLayout>,
}

#[derive(Debug, thiserror::Error)]
pub enum DescriptorError {
    #[error("bad magic: expected 0x{:08X}, got 0x{got:08X}", MAGIC)]
    BadMagic { got: u32 },

    #[error("unsupported descriptor version: {0}")]
    BadVersion(u8),

    #[error("descriptor not ready: state is {0:?}")]
    NotReady(DescState),
}

/// Parse the descriptor from the start of the mmap'd region.
///
/// # Safety
/// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * 16` bytes.
/// All accesses use `read_volatile`; the region must be DMA-coherent.
pub unsafe fn parse(base: *const u8) -> Result<Descriptor, DescriptorError> {
    let magic   = unsafe { rv_u32(base, 0x00) };
    let version = unsafe { rv_u8(base,  0x04) };
    let num_ch  = unsafe { rv_u8(base,  0x05) };
    let eff_mtu = unsafe { rv_u32(base, 0x10) } as usize;
    let state   = DescState::from_u32(unsafe { rv_u32(base, 0x14) });

    if magic != MAGIC {
        return Err(DescriptorError::BadMagic { got: magic });
    }
    if version != VERSION {
        return Err(DescriptorError::BadVersion(version));
    }
    if state != DescState::Ready {
        return Err(DescriptorError::NotReady(state));
    }

    let channels = (0..num_ch as usize)
        .map(|i| {
            let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            ChannelLayout {
                tx_offset: unsafe { rv_u32(base, off)      } as usize,
                tx_size:   unsafe { rv_u32(base, off +  4) } as usize,
                rx_offset: unsafe { rv_u32(base, off +  8) } as usize,
                rx_size:   unsafe { rv_u32(base, off + 12) } as usize,
            }
        })
        .collect();

    Ok(Descriptor { num_channels: num_ch, effective_mtu: eff_mtu, channels })
}

unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { (base.add(offset) as *const u32).read_volatile() }
}

unsafe fn rv_u8(base: *const u8, offset: usize) -> u8 {
    unsafe { base.add(offset).read_volatile() }
}

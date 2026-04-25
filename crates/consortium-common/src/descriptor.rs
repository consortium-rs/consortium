#[repr(packed)]
#[derive(Clone, Copy, Debug)]
/// Shared descriptor header
///
/// This is the common header for all descriptors in the consortium. It is used to identify the descriptor type and version.
///
/// The `magic` field is a 4-byte identifier for the descriptor type. For example, "CDBG" for the debug descriptor, and "CIPC" for the IPC transport descriptor.
///
/// The `version` field is a 1-byte version number for the descriptor.
pub struct DescriptorHeader {
    pub magic: DescriptorMagic,
    pub version: u8,
    _reserved0: [u8; 1],
}

pub const DESCRIPTOR_HEADER_SIZE: usize = core::mem::size_of::<DescriptorHeader>();

#[derive(Debug, thiserror::Error)]
pub enum DescriptorHeaderError {
    #[error("wrong magic: got {got:?}, expected {expected:?}")]
    BadMagic {
        got: DescriptorMagic,
        expected: DescriptorMagic,
    },

    #[error("unknown magic: got {got:?}, expected one of {expected:?}")]
    UnknownMagic {
        got: DescriptorMagic,
        expected: [DescriptorMagic; 2],
    },

    #[error("bad version: got {0}, expected {1}")]
    BadVersion(u8, u8),
}

impl DescriptorHeader {
    pub const fn new(magic: DescriptorMagic, version: u8) -> Self {
        Self {
            magic,
            version,
            _reserved0: [0; 1],
        }
    }

    pub unsafe fn from_raw_ptr(ptr: *const u8) -> Self {
        Self {
            magic: unsafe { *ptr.cast::<DescriptorMagic>() },
            version: unsafe { *ptr.add(4).cast::<u8>() },
            _reserved0: [0; 1],
        }
    }

    pub unsafe fn write_to_raw_ptr(self, ptr: *mut u8) {
        unsafe { *ptr.cast::<DescriptorMagic>() = self.magic };
        unsafe { *ptr.add(4).cast::<u8>() = self.version };
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DescriptorMagic(u32);

pub const MAGIC_SIZE: usize = core::mem::size_of::<DescriptorMagic>();

impl DescriptorMagic {
    pub const IPC: Self = Self(u32::from_be_bytes(*b"CIPC"));
    pub const DEBUG: Self = Self(u32::from_be_bytes(*b"CDBG"));

    pub fn is_known(self) -> bool {
        matches!(self, Self::IPC | Self::DEBUG)
    }

    pub fn check(self, expected: Self) -> Result<(), DescriptorHeaderError> {
        if self == expected {
            Ok(())
        } else if self.is_known() {
            Err(DescriptorHeaderError::BadMagic {
                got: self,
                expected,
            })
        } else {
            Err(DescriptorHeaderError::UnknownMagic {
                got: self,
                expected: [Self::IPC, Self::DEBUG],
            })
        }
    }

    pub const fn new(magic: &[u8; 4]) -> Self {
        Self(u32::from_be_bytes(*magic))
    }
}

impl DescriptorHeader {
    pub fn check(
        self,
        expected_magic: DescriptorMagic,
        expected_version: u8,
    ) -> Result<(), DescriptorHeaderError> {
        self.magic.check(expected_magic)?;
        if self.version == expected_version {
            Ok(())
        } else {
            Err(DescriptorHeaderError::BadVersion(
                self.version,
                expected_version,
            ))
        }
    }
}

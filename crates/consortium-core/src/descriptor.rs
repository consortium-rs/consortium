//! The header every Consortium shared-memory structure starts with.
//!
//! Two cores agree on a memory window but not on when either of them boots. A
//! secondary that maps its window and finds arbitrary bytes needs to tell apart
//! several situations that all look like "not what I expected":
//!
//! - the primary has not published anything yet;
//! - this is a different Consortium structure (a `CDBG` log ring where a `CIPC`
//!   transport was expected — a plausible outcome of a misconfigured base address);
//! - this is a structure of the right kind but a version this build cannot parse;
//! - this is not Consortium memory at all.
//!
//! [`DescriptorHeader`] exists to make that distinction cheap: a 4-byte
//! [`DescriptorMagic`] plus a version byte at a fixed offset, readable before
//! anything else about the layout is assumed. [`DescriptorMagic::check`] returns
//! [`DescriptorHeaderError::BadMagic`] for a recognized-but-wrong structure and
//! [`DescriptorHeaderError::UnknownMagic`] for unrecognized bytes, so bring-up
//! failures point at the actual mistake.
//!
//! The header is `#[repr(C, packed)]` and the magic is stored big-endian, so the
//! four bytes read as ASCII in a memory dump (`43 49 50 43` = `CIPC`). Field access
//! goes through raw-pointer helpers ([`DescriptorHeader::from_raw_ptr`],
//! [`DescriptorHeader::write_to_raw_ptr`]) because a packed struct's fields cannot
//! be borrowed.
//!
//! Owners of the magics currently defined:
//!
//! | Magic  | Structure                            | Owner                             |
//! | ------ | ------------------------------------ | --------------------------------- |
//! | `CIPC` | descriptor, channel table, slots     | `consortium-ipc-transport-memory` |
//! | `CDBG` | defmt log ring descriptor            | `consortium-dbg`                  |
//!
//! This module owns only the header. The bytes after it belong to whichever crate
//! claims the magic.

#[repr(C, packed)]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

    /// Reads a `DescriptorHeader` from raw memory at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least 6 bytes and the memory must be readable.
    pub unsafe fn from_raw_ptr(ptr: *const u8) -> Self {
        Self {
            // SAFETY: caller guarantees ptr is valid for at least 6 bytes.
            magic: unsafe { *ptr.cast::<DescriptorMagic>() },
            // SAFETY: caller guarantees ptr is valid for at least 6 bytes.
            #[allow(clippy::multiple_unsafe_ops_per_block)]
            version: unsafe { *ptr.add(4).cast::<u8>() },
            _reserved0: [0; 1],
        }
    }

    /// Writes this `DescriptorHeader` to raw memory at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least 6 bytes and the memory must be writable.
    pub unsafe fn write_to_raw_ptr(self, ptr: *mut u8) {
        // SAFETY: caller guarantees ptr is valid for at least 6 bytes.
        unsafe { *ptr.cast::<DescriptorMagic>() = self.magic };
        // SAFETY: caller guarantees ptr is valid for at least 6 bytes.
        #[allow(clippy::multiple_unsafe_ops_per_block)]
        unsafe {
            *ptr.add(4).cast::<u8>() = self.version
        };
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DescriptorMagic(u32);

pub const MAGIC_SIZE: usize = core::mem::size_of::<DescriptorMagic>();

impl DescriptorMagic {
    pub const IPC: Self = Self(u32::from_be_bytes(*b"CIPC"));
    pub const DEBUG: Self = Self(u32::from_be_bytes(*b"CDBG"));

    /// Returns `true` if this magic is one of the recognized consortium descriptor types.
    pub fn is_known(self) -> bool {
        matches!(self, Self::IPC | Self::DEBUG)
    }

    /// Validates that this magic matches `expected`, returning a typed error if not.
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
    /// Validates both magic and version, returning a typed error if either does not match.
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

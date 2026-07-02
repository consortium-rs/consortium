#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER {
    ptr: *mut u8,
}
unsafe impl Send for IRQSTEER {}
unsafe impl Sync for IRQSTEER {}
impl IRQSTEER {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CHn_MASK(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHn_MASK, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHn_SET(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHn_SET, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHn_STATUS(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHn_STATUS, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHn_MINTDIS(self) -> crate::common::Reg<IRQSTEER_CHn_MINTDIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHn_MSTRSTAT(
        self,
    ) -> crate::common::Reg<IRQSTEER_CHn_MSTRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHn_MASK(pub u32);
impl IRQSTEER_CHn_MASK {
    #[doc = "Masks"]
    #[must_use]
    #[inline(always)]
    pub const fn MASKFLD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Masks"]
    #[inline(always)]
    pub const fn set_MASKFLD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQSTEER_CHn_MASK {
    #[inline(always)]
    fn default() -> IRQSTEER_CHn_MASK {
        IRQSTEER_CHn_MASK(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHn_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHn_MASK")
            .field("MASKFLD", &self.MASKFLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHn_MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHn_MASK {{ MASKFLD: {=u32:?} }}",
            self.MASKFLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHn_MINTDIS(pub u32);
impl IRQSTEER_CHn_MINTDIS {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IRQSTEER_CHn_MINTDIS {
    #[inline(always)]
    fn default() -> IRQSTEER_CHn_MINTDIS {
        IRQSTEER_CHn_MINTDIS(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHn_MINTDIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHn_MINTDIS")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHn_MINTDIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHn_MINTDIS {{ DISABLE: {=u16:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHn_MSTRSTAT(pub u32);
impl IRQSTEER_CHn_MSTRSTAT {
    #[must_use]
    #[inline(always)]
    pub const fn STATUS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IRQSTEER_CHn_MSTRSTAT {
    #[inline(always)]
    fn default() -> IRQSTEER_CHn_MSTRSTAT {
        IRQSTEER_CHn_MSTRSTAT(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHn_MSTRSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHn_MSTRSTAT")
            .field("STATUS", &self.STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHn_MSTRSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHn_MSTRSTAT {{ STATUS: {=bool:?} }}",
            self.STATUS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHn_SET(pub u32);
impl IRQSTEER_CHn_SET {
    #[doc = "Force Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEFLD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Force Interrupt"]
    #[inline(always)]
    pub const fn set_FORCEFLD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQSTEER_CHn_SET {
    #[inline(always)]
    fn default() -> IRQSTEER_CHn_SET {
        IRQSTEER_CHn_SET(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHn_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHn_SET")
            .field("FORCEFLD", &self.FORCEFLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHn_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHn_SET {{ FORCEFLD: {=u32:?} }}",
            self.FORCEFLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHn_STATUS(pub u32);
impl IRQSTEER_CHn_STATUS {
    #[doc = "Status of an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn STATUS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Status of an interrupt"]
    #[inline(always)]
    pub const fn set_STATUS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQSTEER_CHn_STATUS {
    #[inline(always)]
    fn default() -> IRQSTEER_CHn_STATUS {
        IRQSTEER_CHn_STATUS(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHn_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHn_STATUS")
            .field("STATUS", &self.STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHn_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHn_STATUS {{ STATUS: {=u32:?} }}",
            self.STATUS()
        )
    }
}

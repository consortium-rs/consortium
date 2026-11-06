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
    pub const fn CHN_MASK(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHN_MASK, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHN_SET(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHN_SET, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHN_STATUS(
        self,
        n: usize,
    ) -> crate::common::Reg<IRQSTEER_CHN_STATUS, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn CHN_MINTDIS(self) -> crate::common::Reg<IRQSTEER_CHN_MINTDIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn CHN_MSTRSTAT(
        self,
    ) -> crate::common::Reg<IRQSTEER_CHN_MSTRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHN_MASK(pub u32);
impl IRQSTEER_CHN_MASK {
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
impl Default for IRQSTEER_CHN_MASK {
    #[inline(always)]
    fn default() -> IRQSTEER_CHN_MASK {
        IRQSTEER_CHN_MASK(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHN_MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHN_MASK")
            .field("MASKFLD", &self.MASKFLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHN_MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHN_MASK {{ MASKFLD: {=u32:?} }}",
            self.MASKFLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHN_MINTDIS(pub u32);
impl IRQSTEER_CHN_MINTDIS {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for IRQSTEER_CHN_MINTDIS {
    #[inline(always)]
    fn default() -> IRQSTEER_CHN_MINTDIS {
        IRQSTEER_CHN_MINTDIS(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHN_MINTDIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHN_MINTDIS")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHN_MINTDIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHN_MINTDIS {{ DISABLE: {=u16:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHN_MSTRSTAT(pub u32);
impl IRQSTEER_CHN_MSTRSTAT {
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
impl Default for IRQSTEER_CHN_MSTRSTAT {
    #[inline(always)]
    fn default() -> IRQSTEER_CHN_MSTRSTAT {
        IRQSTEER_CHN_MSTRSTAT(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHN_MSTRSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHN_MSTRSTAT")
            .field("STATUS", &self.STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHN_MSTRSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHN_MSTRSTAT {{ STATUS: {=bool:?} }}",
            self.STATUS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHN_SET(pub u32);
impl IRQSTEER_CHN_SET {
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
impl Default for IRQSTEER_CHN_SET {
    #[inline(always)]
    fn default() -> IRQSTEER_CHN_SET {
        IRQSTEER_CHN_SET(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHN_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHN_SET")
            .field("FORCEFLD", &self.FORCEFLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHN_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHN_SET {{ FORCEFLD: {=u32:?} }}",
            self.FORCEFLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSTEER_CHN_STATUS(pub u32);
impl IRQSTEER_CHN_STATUS {
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
impl Default for IRQSTEER_CHN_STATUS {
    #[inline(always)]
    fn default() -> IRQSTEER_CHN_STATUS {
        IRQSTEER_CHN_STATUS(0)
    }
}
impl core::fmt::Debug for IRQSTEER_CHN_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSTEER_CHN_STATUS")
            .field("STATUS", &self.STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSTEER_CHN_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQSTEER_CHN_STATUS {{ STATUS: {=u32:?} }}",
            self.STATUS()
        )
    }
}

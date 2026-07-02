#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_GPR {
    ptr: *mut u8,
}
unsafe impl Send for IOMUXC_GPR {}
unsafe impl Sync for IOMUXC_GPR {}
impl IOMUXC_GPR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GPR0(self) -> crate::common::Reg<IOMUXC_GPR_GPR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_GPR_GPR0(pub u32);
impl IOMUXC_GPR_GPR0 {
    #[doc = "General purpose bits"]
    #[must_use]
    #[inline(always)]
    pub const fn GPR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "General purpose bits"]
    #[inline(always)]
    pub const fn set_GPR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IOMUXC_GPR_GPR0 {
    #[inline(always)]
    fn default() -> IOMUXC_GPR_GPR0 {
        IOMUXC_GPR_GPR0(0)
    }
}
impl core::fmt::Debug for IOMUXC_GPR_GPR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_GPR_GPR0")
            .field("GPR", &self.GPR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_GPR_GPR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IOMUXC_GPR_GPR0 {{ GPR: {=u16:?} }}", self.GPR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_GPR_GPR0_GPR(pub u32);
impl IOMUXC_GPR_GPR0_GPR {
    #[doc = "General purpose bits"]
    #[must_use]
    #[inline(always)]
    pub const fn LCK(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "General purpose bits"]
    #[inline(always)]
    pub const fn set_LCK(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for IOMUXC_GPR_GPR0_GPR {
    #[inline(always)]
    fn default() -> IOMUXC_GPR_GPR0_GPR {
        IOMUXC_GPR_GPR0_GPR(0)
    }
}
impl core::fmt::Debug for IOMUXC_GPR_GPR0_GPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_GPR_GPR0_GPR")
            .field("LCK", &self.LCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_GPR_GPR0_GPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IOMUXC_GPR_GPR0_GPR {{ LCK: {=u16:?} }}", self.LCK())
    }
}

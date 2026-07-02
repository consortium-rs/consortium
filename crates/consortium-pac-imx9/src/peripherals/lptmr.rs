#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR {
    ptr: *mut u8,
}
unsafe impl Send for LPTMR {}
unsafe impl Sync for LPTMR {}
impl LPTMR {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<LPTMR_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PSR(self) -> crate::common::Reg<LPTMR_PSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CMR(self) -> crate::common::Reg<LPTMR_CMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn CNR(self) -> crate::common::Reg<LPTMR_CNR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR_CMR(pub u32);
impl LPTMR_CMR {
    #[doc = "Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn COMPARE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Value"]
    #[inline(always)]
    pub const fn set_COMPARE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPTMR_CMR {
    #[inline(always)]
    fn default() -> LPTMR_CMR {
        LPTMR_CMR(0)
    }
}
impl core::fmt::Debug for LPTMR_CMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTMR_CMR")
            .field("COMPARE", &self.COMPARE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTMR_CMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTMR_CMR {{ COMPARE: {=u32:?} }}", self.COMPARE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR_CNR(pub u32);
impl LPTMR_CNR {
    #[doc = "Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn COUNTER(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter Value"]
    #[inline(always)]
    pub const fn set_COUNTER(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPTMR_CNR {
    #[inline(always)]
    fn default() -> LPTMR_CNR {
        LPTMR_CNR(0)
    }
}
impl core::fmt::Debug for LPTMR_CNR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTMR_CNR")
            .field("COUNTER", &self.COUNTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTMR_CNR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTMR_CNR {{ COUNTER: {=u32:?} }}", self.COUNTER())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR_CSR(pub u32);
impl LPTMR_CSR {
    #[must_use]
    #[inline(always)]
    pub const fn TEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TMS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TMS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TFC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TPP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TPP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TPS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TPS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TCF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TDRE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LPTMR_CSR {
    #[inline(always)]
    fn default() -> LPTMR_CSR {
        LPTMR_CSR(0)
    }
}
impl core::fmt::Debug for LPTMR_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTMR_CSR")
            .field("TEN", &self.TEN())
            .field("TMS", &self.TMS())
            .field("TFC", &self.TFC())
            .field("TPP", &self.TPP())
            .field("TPS", &self.TPS())
            .field("TIE", &self.TIE())
            .field("TCF", &self.TCF())
            .field("TDRE", &self.TDRE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTMR_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTMR_CSR {{ TEN: {=bool:?}, TMS: {=bool:?}, TFC: {=bool:?}, TPP: {=bool:?}, TPS: {=u8:?}, TIE: {=bool:?}, TCF: {=bool:?}, TDRE: {=bool:?} }}",
            self.TEN(),
            self.TMS(),
            self.TFC(),
            self.TPP(),
            self.TPS(),
            self.TIE(),
            self.TCF(),
            self.TDRE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTMR_PSR(pub u32);
impl LPTMR_PSR {
    #[must_use]
    #[inline(always)]
    pub const fn PCS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PCS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PBYP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRESCALE(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PRESCALE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
}
impl Default for LPTMR_PSR {
    #[inline(always)]
    fn default() -> LPTMR_PSR {
        LPTMR_PSR(0)
    }
}
impl core::fmt::Debug for LPTMR_PSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTMR_PSR")
            .field("PCS", &self.PCS())
            .field("PBYP", &self.PBYP())
            .field("PRESCALE", &self.PRESCALE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTMR_PSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTMR_PSR {{ PCS: {=u8:?}, PBYP: {=bool:?}, PRESCALE: {=u8:?} }}",
            self.PCS(),
            self.PBYP(),
            self.PRESCALE()
        )
    }
}

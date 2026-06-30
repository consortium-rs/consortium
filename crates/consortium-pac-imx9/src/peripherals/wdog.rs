#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG {
    ptr: *mut u8,
}
unsafe impl Send for WDOG {}
unsafe impl Sync for WDOG {}
impl WDOG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CS(self) -> crate::common::Reg<WDOG_CS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CNT(self) -> crate::common::Reg<WDOG_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TOVAL(self) -> crate::common::Reg<WDOG_TOVAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn WIN(self) -> crate::common::Reg<WDOG_WIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG_CNT(pub u32);
impl WDOG_CNT {
    #[doc = "Counter Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn CNTLOW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Counter Low Byte"]
    #[inline(always)]
    pub const fn set_CNTLOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Counter Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn CNTHIGH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Counter Low Byte"]
    #[inline(always)]
    pub const fn set_CNTHIGH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WDOG_CNT {
    #[inline(always)]
    fn default() -> WDOG_CNT {
        WDOG_CNT(0)
    }
}
impl core::fmt::Debug for WDOG_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG_CNT")
            .field("CNTLOW", &self.CNTLOW())
            .field("CNTHIGH", &self.CNTHIGH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOG_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDOG_CNT {{ CNTLOW: {=u8:?}, CNTHIGH: {=u8:?} }}",
            self.CNTLOW(),
            self.CNTHIGH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG_CS(pub u32);
impl WDOG_CS {
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAIT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DBG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TST(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UPDATE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UPDATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLK(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CLK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RCS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RCS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ULK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ULK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CMD32EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CMD32EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FLG(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WIN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLEAR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLEAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for WDOG_CS {
    #[inline(always)]
    fn default() -> WDOG_CS {
        WDOG_CS(0)
    }
}
impl core::fmt::Debug for WDOG_CS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG_CS")
            .field("STOP", &self.STOP())
            .field("WAIT", &self.WAIT())
            .field("DBG", &self.DBG())
            .field("TST", &self.TST())
            .field("UPDATE", &self.UPDATE())
            .field("INT", &self.INT())
            .field("EN", &self.EN())
            .field("CLK", &self.CLK())
            .field("RCS", &self.RCS())
            .field("ULK", &self.ULK())
            .field("PRES", &self.PRES())
            .field("CMD32EN", &self.CMD32EN())
            .field("FLG", &self.FLG())
            .field("WIN", &self.WIN())
            .field("CLEAR", &self.CLEAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOG_CS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDOG_CS {{ STOP: {=bool:?}, WAIT: {=bool:?}, DBG: {=bool:?}, TST: {=u8:?}, UPDATE: {=bool:?}, INT: {=bool:?}, EN: {=bool:?}, CLK: {=u8:?}, RCS: {=bool:?}, ULK: {=bool:?}, PRES: {=bool:?}, CMD32EN: {=bool:?}, FLG: {=bool:?}, WIN: {=bool:?}, CLEAR: {=bool:?} }}",
            self.STOP(),
            self.WAIT(),
            self.DBG(),
            self.TST(),
            self.UPDATE(),
            self.INT(),
            self.EN(),
            self.CLK(),
            self.RCS(),
            self.ULK(),
            self.PRES(),
            self.CMD32EN(),
            self.FLG(),
            self.WIN(),
            self.CLEAR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG_TOVAL(pub u32);
impl WDOG_TOVAL {
    #[doc = "Timeout Value Low"]
    #[must_use]
    #[inline(always)]
    pub const fn TOVALLOW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Value Low"]
    #[inline(always)]
    pub const fn set_TOVALLOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timeout Value Low"]
    #[must_use]
    #[inline(always)]
    pub const fn TOVALHIGH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Value Low"]
    #[inline(always)]
    pub const fn set_TOVALHIGH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WDOG_TOVAL {
    #[inline(always)]
    fn default() -> WDOG_TOVAL {
        WDOG_TOVAL(0)
    }
}
impl core::fmt::Debug for WDOG_TOVAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG_TOVAL")
            .field("TOVALLOW", &self.TOVALLOW())
            .field("TOVALHIGH", &self.TOVALHIGH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOG_TOVAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDOG_TOVAL {{ TOVALLOW: {=u8:?}, TOVALHIGH: {=u8:?} }}",
            self.TOVALLOW(),
            self.TOVALHIGH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WDOG_WIN(pub u32);
impl WDOG_WIN {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn WINLOW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_WINLOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn WINHIGH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_WINHIGH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WDOG_WIN {
    #[inline(always)]
    fn default() -> WDOG_WIN {
        WDOG_WIN(0)
    }
}
impl core::fmt::Debug for WDOG_WIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG_WIN")
            .field("WINLOW", &self.WINLOW())
            .field("WINHIGH", &self.WINHIGH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WDOG_WIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WDOG_WIN {{ WINLOW: {=u8:?}, WINHIGH: {=u8:?} }}",
            self.WINLOW(),
            self.WINHIGH()
        )
    }
}

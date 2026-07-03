#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT {
    ptr: *mut u8,
}
unsafe impl Send for LPIT {}
unsafe impl Sync for LPIT {}
impl LPIT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<LPIT_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<LPIT_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<LPIT_MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MIER(self) -> crate::common::Reg<LPIT_MIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SETTEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CLRTEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn CHANNEL_TVAL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn CHANNEL_CVAL(
        self,
        n: usize,
    ) -> crate::common::Reg<LPIT_CVAL_TMR_CUR, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 16usize) as _) }
    }
    #[inline(always)]
    pub const fn CHANNEL_TCTRL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize + n * 16usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_CVAL_TMR_CUR(pub u32);
impl LPIT_CVAL_TMR_CUR {
    #[doc = "Current Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current Timer Value"]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPIT_CVAL_TMR_CUR {
    #[inline(always)]
    fn default() -> LPIT_CVAL_TMR_CUR {
        LPIT_CVAL_TMR_CUR(0)
    }
}
impl core::fmt::Debug for LPIT_CVAL_TMR_CUR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_CVAL_TMR_CUR")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_CVAL_TMR_CUR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_CVAL_TMR_CUR {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MCR_DBG(pub u32);
impl LPIT_MCR_DBG {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for LPIT_MCR_DBG {
    #[inline(always)]
    fn default() -> LPIT_MCR_DBG {
        LPIT_MCR_DBG(0)
    }
}
impl core::fmt::Debug for LPIT_MCR_DBG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MCR_DBG")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MCR_DBG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_MCR_DBG {{ EN: {=bool:?} }}", self.EN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MCR_DOZE(pub u32);
impl LPIT_MCR_DOZE {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for LPIT_MCR_DOZE {
    #[inline(always)]
    fn default() -> LPIT_MCR_DOZE {
        LPIT_MCR_DOZE(0)
    }
}
impl core::fmt::Debug for LPIT_MCR_DOZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MCR_DOZE")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MCR_DOZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_MCR_DOZE {{ EN: {=bool:?} }}", self.EN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MCR_M(pub u32);
impl LPIT_MCR_M {
    #[must_use]
    #[inline(always)]
    pub const fn CEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPIT_MCR_M {
    #[inline(always)]
    fn default() -> LPIT_MCR_M {
        LPIT_MCR_M(0)
    }
}
impl core::fmt::Debug for LPIT_MCR_M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MCR_M")
            .field("CEN", &self.CEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MCR_M {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_MCR_M {{ CEN: {=bool:?} }}", self.CEN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MCR_SW(pub u32);
impl LPIT_MCR_SW {
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for LPIT_MCR_SW {
    #[inline(always)]
    fn default() -> LPIT_MCR_SW {
        LPIT_MCR_SW(0)
    }
}
impl core::fmt::Debug for LPIT_MCR_SW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MCR_SW")
            .field("RST", &self.RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MCR_SW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_MCR_SW {{ RST: {=bool:?} }}", self.RST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MIER(pub u32);
impl LPIT_MIER {
    #[must_use]
    #[inline(always)]
    pub const fn TIE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for LPIT_MIER {
    #[inline(always)]
    fn default() -> LPIT_MIER {
        LPIT_MIER(0)
    }
}
impl core::fmt::Debug for LPIT_MIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MIER")
            .field("TIE0", &self.TIE0())
            .field("TIE1", &self.TIE1())
            .field("TIE2", &self.TIE2())
            .field("TIE3", &self.TIE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPIT_MIER {{ TIE0: {=bool:?}, TIE1: {=bool:?}, TIE2: {=bool:?}, TIE3: {=bool:?} }}",
            self.TIE0(),
            self.TIE1(),
            self.TIE2(),
            self.TIE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_MSR(pub u32);
impl LPIT_MSR {
    #[must_use]
    #[inline(always)]
    pub const fn TIF0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIF0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIF1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIF1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIF2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIF2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIF3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIF3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for LPIT_MSR {
    #[inline(always)]
    fn default() -> LPIT_MSR {
        LPIT_MSR(0)
    }
}
impl core::fmt::Debug for LPIT_MSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_MSR")
            .field("TIF0", &self.TIF0())
            .field("TIF1", &self.TIF1())
            .field("TIF2", &self.TIF2())
            .field("TIF3", &self.TIF3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_MSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPIT_MSR {{ TIF0: {=bool:?}, TIF1: {=bool:?}, TIF2: {=bool:?}, TIF3: {=bool:?} }}",
            self.TIF0(),
            self.TIF1(),
            self.TIF2(),
            self.TIF3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_PARAM(pub u32);
impl LPIT_PARAM {
    #[doc = "Number of Timer Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn CHANNEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Timer Channels"]
    #[inline(always)]
    pub const fn set_CHANNEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for LPIT_PARAM {
    #[inline(always)]
    fn default() -> LPIT_PARAM {
        LPIT_PARAM(0)
    }
}
impl core::fmt::Debug for LPIT_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_PARAM")
            .field("CHANNEL", &self.CHANNEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_PARAM {{ CHANNEL: {=u8:?} }}", self.CHANNEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_PARAM_EXT(pub u32);
impl LPIT_PARAM_EXT {
    #[doc = "Number of Timer Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Timer Channels"]
    #[inline(always)]
    pub const fn set_TRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for LPIT_PARAM_EXT {
    #[inline(always)]
    fn default() -> LPIT_PARAM_EXT {
        LPIT_PARAM_EXT(0)
    }
}
impl core::fmt::Debug for LPIT_PARAM_EXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_PARAM_EXT")
            .field("TRIG", &self.TRIG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_PARAM_EXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_PARAM_EXT {{ TRIG: {=u8:?} }}", self.TRIG())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_TCTRL(pub u32);
impl LPIT_TCTRL {
    #[must_use]
    #[inline(always)]
    pub const fn CHAIN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CHAIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSOT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TSOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSOI(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TSOI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TROT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TROT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LPIT_TCTRL {
    #[inline(always)]
    fn default() -> LPIT_TCTRL {
        LPIT_TCTRL(0)
    }
}
impl core::fmt::Debug for LPIT_TCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_TCTRL")
            .field("CHAIN", &self.CHAIN())
            .field("MODE", &self.MODE())
            .field("TSOT", &self.TSOT())
            .field("TSOI", &self.TSOI())
            .field("TROT", &self.TROT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_TCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPIT_TCTRL {{ CHAIN: {=bool:?}, MODE: {=u8:?}, TSOT: {=bool:?}, TSOI: {=bool:?}, TROT: {=bool:?} }}",
            self.CHAIN(),
            self.MODE(),
            self.TSOT(),
            self.TSOI(),
            self.TROT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_TCTRL_T(pub u32);
impl LPIT_TCTRL_T {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPIT_TCTRL_T {
    #[inline(always)]
    fn default() -> LPIT_TCTRL_T {
        LPIT_TCTRL_T(0)
    }
}
impl core::fmt::Debug for LPIT_TCTRL_T {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_TCTRL_T")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_TCTRL_T {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_TCTRL_T {{ EN: {=bool:?} }}", self.EN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_TCTRL_TRG(pub u32);
impl LPIT_TCTRL_TRG {
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for LPIT_TCTRL_TRG {
    #[inline(always)]
    fn default() -> LPIT_TCTRL_TRG {
        LPIT_TCTRL_TRG(0)
    }
}
impl core::fmt::Debug for LPIT_TCTRL_TRG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_TCTRL_TRG")
            .field("SRC", &self.SRC())
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_TCTRL_TRG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPIT_TCTRL_TRG {{ SRC: {=bool:?}, SEL: {=u8:?} }}",
            self.SRC(),
            self.SEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_TVAL_TMR(pub u32);
impl LPIT_TVAL_TMR {
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPIT_TVAL_TMR {
    #[inline(always)]
    fn default() -> LPIT_TVAL_TMR {
        LPIT_TVAL_TMR(0)
    }
}
impl core::fmt::Debug for LPIT_TVAL_TMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_TVAL_TMR")
            .field("VAL", &self.VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_TVAL_TMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPIT_TVAL_TMR {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPIT_VERID(pub u32);
impl LPIT_VERID {
    #[doc = "Feature Number"]
    #[must_use]
    #[inline(always)]
    pub const fn FEATURE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Number"]
    #[inline(always)]
    pub const fn set_FEATURE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feature Number"]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Feature Number"]
    #[inline(always)]
    pub const fn set_MINOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_MAJOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPIT_VERID {
    #[inline(always)]
    fn default() -> LPIT_VERID {
        LPIT_VERID(0)
    }
}
impl core::fmt::Debug for LPIT_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPIT_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPIT_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPIT_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM {
    ptr: *mut u8,
}
unsafe impl Send for LPTIM {}
unsafe impl Sync for LPTIM {}
impl LPTIM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPTIM interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<LPTIM_ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LPTIM interrupt clear register"]
    #[inline(always)]
    pub const fn ICR(self) -> crate::common::Reg<LPTIM_ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LPTIM interrupt enable register"]
    #[inline(always)]
    pub const fn DIER(self) -> crate::common::Reg<LPTIM_DIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LPTIM configuration register"]
    #[inline(always)]
    pub const fn CFGR(self) -> crate::common::Reg<LPTIM_CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LPTIM control register"]
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<LPTIM_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LPTIM compare register 1"]
    #[inline(always)]
    pub const fn CCR1(self) -> crate::common::Reg<LPTIM_CCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LPTIM autoreload register"]
    #[inline(always)]
    pub const fn ARR(self) -> crate::common::Reg<LPTIM_ARR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LPTIM counter register"]
    #[inline(always)]
    pub const fn CNT(self) -> crate::common::Reg<LPTIM_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "LPTIM1 option register"]
    #[inline(always)]
    pub const fn OR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "LPTIM configuration register 2"]
    #[inline(always)]
    pub const fn CFGR2(self) -> crate::common::Reg<LPTIM_CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "LPTIM repetition register"]
    #[inline(always)]
    pub const fn RCR(self) -> crate::common::Reg<LPTIM_RCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "LPTIM capture/compare mode register 1"]
    #[inline(always)]
    pub const fn CCMR1(self) -> crate::common::Reg<LPTIM_CCMR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "LPTIM compare register 2"]
    #[inline(always)]
    pub const fn CCR2(self) -> crate::common::Reg<LPTIM_CCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "LPTIM peripheral hardware configuration register 2"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<LPTIM_HWCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "LPTIM peripheral hardware configuration register 1"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<LPTIM_HWCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "LPTIM peripheral version identification register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<LPTIM_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "LPTIM peripheral type identification register"]
    #[inline(always)]
    pub const fn PIDR(self) -> crate::common::Reg<LPTIM_PIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "LPTIM registers map size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<LPTIM_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_ARR(pub u32);
impl LPTIM_ARR {
    #[doc = "Auto reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn ARR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auto reload value"]
    #[inline(always)]
    pub const fn set_ARR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPTIM_ARR {
    #[inline(always)]
    fn default() -> LPTIM_ARR {
        LPTIM_ARR(0)
    }
}
impl core::fmt::Debug for LPTIM_ARR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_ARR")
            .field("ARR", &self.ARR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_ARR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_ARR {{ ARR: {=u16:?} }}", self.ARR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CCMR1(pub u32);
impl LPTIM_CCMR1 {
    #[doc = "Capture/compare 1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1SEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 selection"]
    #[inline(always)]
    pub const fn set_CC1SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/compare 1 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1E(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 output enable"]
    #[inline(always)]
    pub const fn set_CC1E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/compare 1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1P(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 1 output polarity"]
    #[inline(always)]
    pub const fn set_CC1P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 1 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn IC1PSC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler"]
    #[inline(always)]
    pub const fn set_IC1PSC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Input capture 1 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn IC1F(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 filter"]
    #[inline(always)]
    pub const fn set_IC1F(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Capture/compare 2 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2SEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 selection"]
    #[inline(always)]
    pub const fn set_CC2SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Capture/compare 2 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2E(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 output enable"]
    #[inline(always)]
    pub const fn set_CC2E(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Capture/compare 2 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2P(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 2 output polarity"]
    #[inline(always)]
    pub const fn set_CC2P(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Input capture 2 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn IC2PSC(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 2 prescaler"]
    #[inline(always)]
    pub const fn set_IC2PSC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Input capture 2 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn IC2F(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 2 filter"]
    #[inline(always)]
    pub const fn set_IC2F(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for LPTIM_CCMR1 {
    #[inline(always)]
    fn default() -> LPTIM_CCMR1 {
        LPTIM_CCMR1(0)
    }
}
impl core::fmt::Debug for LPTIM_CCMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CCMR1")
            .field("CC1SEL", &self.CC1SEL())
            .field("CC1E", &self.CC1E())
            .field("CC1P", &self.CC1P())
            .field("IC1PSC", &self.IC1PSC())
            .field("IC1F", &self.IC1F())
            .field("CC2SEL", &self.CC2SEL())
            .field("CC2E", &self.CC2E())
            .field("CC2P", &self.CC2P())
            .field("IC2PSC", &self.IC2PSC())
            .field("IC2F", &self.IC2F())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CCMR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_CCMR1 {{ CC1SEL: {=bool:?}, CC1E: {=bool:?}, CC1P: {=u8:?}, IC1PSC: {=u8:?}, IC1F: {=u8:?}, CC2SEL: {=bool:?}, CC2E: {=bool:?}, CC2P: {=u8:?}, IC2PSC: {=u8:?}, IC2F: {=u8:?} }}",
            self.CC1SEL(),
            self.CC1E(),
            self.CC1P(),
            self.IC1PSC(),
            self.IC1F(),
            self.CC2SEL(),
            self.CC2E(),
            self.CC2P(),
            self.IC2PSC(),
            self.IC2F()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CCR1(pub u32);
impl LPTIM_CCR1 {
    #[doc = "Capture/compare 1 value"]
    #[must_use]
    #[inline(always)]
    pub const fn CCR1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/compare 1 value"]
    #[inline(always)]
    pub const fn set_CCR1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPTIM_CCR1 {
    #[inline(always)]
    fn default() -> LPTIM_CCR1 {
        LPTIM_CCR1(0)
    }
}
impl core::fmt::Debug for LPTIM_CCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CCR1")
            .field("CCR1", &self.CCR1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_CCR1 {{ CCR1: {=u16:?} }}", self.CCR1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CCR2(pub u32);
impl LPTIM_CCR2 {
    #[doc = "Capture/compare 2 value"]
    #[must_use]
    #[inline(always)]
    pub const fn CCR2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/compare 2 value"]
    #[inline(always)]
    pub const fn set_CCR2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPTIM_CCR2 {
    #[inline(always)]
    fn default() -> LPTIM_CCR2 {
        LPTIM_CCR2(0)
    }
}
impl core::fmt::Debug for LPTIM_CCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CCR2")
            .field("CCR2", &self.CCR2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_CCR2 {{ CCR2: {=u16:?} }}", self.CCR2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CFGR(pub u32);
impl LPTIM_CFGR {
    #[doc = "Clock selector"]
    #[must_use]
    #[inline(always)]
    pub const fn CKSEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock selector"]
    #[inline(always)]
    pub const fn set_CKSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CKPOL(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Clock Polarity"]
    #[inline(always)]
    pub const fn set_CKPOL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Configurable digital filter for external clock"]
    #[must_use]
    #[inline(always)]
    pub const fn CKFLT(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for external clock"]
    #[inline(always)]
    pub const fn set_CKFLT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Configurable digital filter for trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn TRGFLT(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Configurable digital filter for trigger"]
    #[inline(always)]
    pub const fn set_TRGFLT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Clock prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn PRESC(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Clock prescaler"]
    #[inline(always)]
    pub const fn set_PRESC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Trigger selector"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGSEL(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Trigger selector"]
    #[inline(always)]
    pub const fn set_TRIGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Trigger enable and polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGEN(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "Trigger enable and polarity"]
    #[inline(always)]
    pub const fn set_TRIGEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "Timeout enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMOUT(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout enable"]
    #[inline(always)]
    pub const fn set_TIMOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Waveform shape"]
    #[must_use]
    #[inline(always)]
    pub const fn WAVE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape"]
    #[inline(always)]
    pub const fn set_WAVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Waveform shape polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn WAVPOL(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Waveform shape polarity"]
    #[inline(always)]
    pub const fn set_WAVPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Registers update mode"]
    #[must_use]
    #[inline(always)]
    pub const fn PRELOAD(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Registers update mode"]
    #[inline(always)]
    pub const fn set_PRELOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "counter mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn COUNTMODE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "counter mode enabled"]
    #[inline(always)]
    pub const fn set_COUNTMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Encoder mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ENC(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Encoder mode enable"]
    #[inline(always)]
    pub const fn set_ENC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for LPTIM_CFGR {
    #[inline(always)]
    fn default() -> LPTIM_CFGR {
        LPTIM_CFGR(0)
    }
}
impl core::fmt::Debug for LPTIM_CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CFGR")
            .field("CKSEL", &self.CKSEL())
            .field("CKPOL", &self.CKPOL())
            .field("CKFLT", &self.CKFLT())
            .field("TRGFLT", &self.TRGFLT())
            .field("PRESC", &self.PRESC())
            .field("TRIGSEL", &self.TRIGSEL())
            .field("TRIGEN", &self.TRIGEN())
            .field("TIMOUT", &self.TIMOUT())
            .field("WAVE", &self.WAVE())
            .field("WAVPOL", &self.WAVPOL())
            .field("PRELOAD", &self.PRELOAD())
            .field("COUNTMODE", &self.COUNTMODE())
            .field("ENC", &self.ENC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_CFGR {{ CKSEL: {=bool:?}, CKPOL: {=u8:?}, CKFLT: {=u8:?}, TRGFLT: {=u8:?}, PRESC: {=u8:?}, TRIGSEL: {=u8:?}, TRIGEN: {=u8:?}, TIMOUT: {=bool:?}, WAVE: {=bool:?}, WAVPOL: {=bool:?}, PRELOAD: {=bool:?}, COUNTMODE: {=bool:?}, ENC: {=bool:?} }}",
            self.CKSEL(),
            self.CKPOL(),
            self.CKFLT(),
            self.TRGFLT(),
            self.PRESC(),
            self.TRIGSEL(),
            self.TRIGEN(),
            self.TIMOUT(),
            self.WAVE(),
            self.WAVPOL(),
            self.PRELOAD(),
            self.COUNTMODE(),
            self.ENC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CFGR2(pub u32);
impl LPTIM_CFGR2 {
    #[doc = "LPTIM input 1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn IN1SEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "LPTIM input 1 selection"]
    #[inline(always)]
    pub const fn set_IN1SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "LPTIM input 2 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn IN2SEL(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "LPTIM input 2 selection"]
    #[inline(always)]
    pub const fn set_IN2SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "LPTIM input capture 1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn IC1SEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "LPTIM input capture 1 selection"]
    #[inline(always)]
    pub const fn set_IC1SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LPTIM input capture 2 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn IC2SEL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "LPTIM input capture 2 selection"]
    #[inline(always)]
    pub const fn set_IC2SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
}
impl Default for LPTIM_CFGR2 {
    #[inline(always)]
    fn default() -> LPTIM_CFGR2 {
        LPTIM_CFGR2(0)
    }
}
impl core::fmt::Debug for LPTIM_CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CFGR2")
            .field("IN1SEL", &self.IN1SEL())
            .field("IN2SEL", &self.IN2SEL())
            .field("IC1SEL", &self.IC1SEL())
            .field("IC2SEL", &self.IC2SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_CFGR2 {{ IN1SEL: {=u8:?}, IN2SEL: {=u8:?}, IC1SEL: {=u8:?}, IC2SEL: {=u8:?} }}",
            self.IN1SEL(),
            self.IN2SEL(),
            self.IC1SEL(),
            self.IC2SEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CNT(pub u32);
impl LPTIM_CNT {
    #[doc = "Counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPTIM_CNT {
    #[inline(always)]
    fn default() -> LPTIM_CNT {
        LPTIM_CNT(0)
    }
}
impl core::fmt::Debug for LPTIM_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CNT")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_CNT {{ CNT: {=u16:?} }}", self.CNT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_CR(pub u32);
impl LPTIM_CR {
    #[doc = "LPTIM enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM enable"]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM start in Single mode"]
    #[must_use]
    #[inline(always)]
    pub const fn SNGSTRT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM start in Single mode"]
    #[inline(always)]
    pub const fn set_SNGSTRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer start in Continuous mode"]
    #[must_use]
    #[inline(always)]
    pub const fn CNTSTRT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Timer start in Continuous mode"]
    #[inline(always)]
    pub const fn set_CNTSTRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Counter reset"]
    #[must_use]
    #[inline(always)]
    pub const fn COUNTRST(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Counter reset"]
    #[inline(always)]
    pub const fn set_COUNTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset after read enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTARE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset after read enable"]
    #[inline(always)]
    pub const fn set_RSTARE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for LPTIM_CR {
    #[inline(always)]
    fn default() -> LPTIM_CR {
        LPTIM_CR(0)
    }
}
impl core::fmt::Debug for LPTIM_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_CR")
            .field("ENABLE", &self.ENABLE())
            .field("SNGSTRT", &self.SNGSTRT())
            .field("CNTSTRT", &self.CNTSTRT())
            .field("COUNTRST", &self.COUNTRST())
            .field("RSTARE", &self.RSTARE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_CR {{ ENABLE: {=bool:?}, SNGSTRT: {=bool:?}, CNTSTRT: {=bool:?}, COUNTRST: {=bool:?}, RSTARE: {=bool:?} }}",
            self.ENABLE(),
            self.SNGSTRT(),
            self.CNTSTRT(),
            self.COUNTRST(),
            self.RSTARE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_DIER(pub u32);
impl LPTIM_DIER {
    #[doc = "Capture/compare 1 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1IE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 interrupt enable"]
    #[inline(always)]
    pub const fn set_CC1IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ARRMIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ARRMIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger valid edge Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EXTTRIGIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge Interrupt Enable"]
    #[inline(always)]
    pub const fn set_EXTTRIGIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register 1 update OK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP1OKIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 1 update OK interrupt enable"]
    #[inline(always)]
    pub const fn set_CMP1OKIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ARROKIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ARROKIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direction change to UP Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UPIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to UP Interrupt Enable"]
    #[inline(always)]
    pub const fn set_UPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Direction change to down Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to down Interrupt Enable"]
    #[inline(always)]
    pub const fn set_DOWNIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Update event interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UEIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Update event interrupt enable"]
    #[inline(always)]
    pub const fn set_UEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Repetition register update OK interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn REPOKIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repetition register update OK interrupt Enable"]
    #[inline(always)]
    pub const fn set_REPOKIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/compare 2 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2IE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 interrupt enable"]
    #[inline(always)]
    pub const fn set_CC2IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/compare 1 over-capture interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1OIE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 over-capture interrupt enable"]
    #[inline(always)]
    pub const fn set_CC1OIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture/compare 2 over-capture interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2OIE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 over-capture interrupt enable"]
    #[inline(always)]
    pub const fn set_CC2OIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Capture/compare 1 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1DE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 DMA request enable"]
    #[inline(always)]
    pub const fn set_CC1DE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Compare register 2 update OK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP2OKIE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 2 update OK interrupt enable"]
    #[inline(always)]
    pub const fn set_CMP2OKIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Update event DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UEDE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Update event DMA request enable"]
    #[inline(always)]
    pub const fn set_UEDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Capture/compare 2 DMA request enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2DE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 DMA request enable"]
    #[inline(always)]
    pub const fn set_CC2DE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LPTIM_DIER {
    #[inline(always)]
    fn default() -> LPTIM_DIER {
        LPTIM_DIER(0)
    }
}
impl core::fmt::Debug for LPTIM_DIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_DIER")
            .field("CC1IE", &self.CC1IE())
            .field("ARRMIE", &self.ARRMIE())
            .field("EXTTRIGIE", &self.EXTTRIGIE())
            .field("CMP1OKIE", &self.CMP1OKIE())
            .field("ARROKIE", &self.ARROKIE())
            .field("UPIE", &self.UPIE())
            .field("DOWNIE", &self.DOWNIE())
            .field("UEIE", &self.UEIE())
            .field("REPOKIE", &self.REPOKIE())
            .field("CC2IE", &self.CC2IE())
            .field("CC1OIE", &self.CC1OIE())
            .field("CC2OIE", &self.CC2OIE())
            .field("CC1DE", &self.CC1DE())
            .field("CMP2OKIE", &self.CMP2OKIE())
            .field("UEDE", &self.UEDE())
            .field("CC2DE", &self.CC2DE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_DIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_DIER {{ CC1IE: {=bool:?}, ARRMIE: {=bool:?}, EXTTRIGIE: {=bool:?}, CMP1OKIE: {=bool:?}, ARROKIE: {=bool:?}, UPIE: {=bool:?}, DOWNIE: {=bool:?}, UEIE: {=bool:?}, REPOKIE: {=bool:?}, CC2IE: {=bool:?}, CC1OIE: {=bool:?}, CC2OIE: {=bool:?}, CC1DE: {=bool:?}, CMP2OKIE: {=bool:?}, UEDE: {=bool:?}, CC2DE: {=bool:?} }}",
            self.CC1IE(),
            self.ARRMIE(),
            self.EXTTRIGIE(),
            self.CMP1OKIE(),
            self.ARROKIE(),
            self.UPIE(),
            self.DOWNIE(),
            self.UEIE(),
            self.REPOKIE(),
            self.CC2IE(),
            self.CC1OIE(),
            self.CC2OIE(),
            self.CC1DE(),
            self.CMP2OKIE(),
            self.UEDE(),
            self.CC2DE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_HWCFGR1(pub u32);
impl LPTIM_HWCFGR1 {
    #[doc = "peripheral hardware configuration 1"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "peripheral hardware configuration 1"]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "peripheral hardware configuration 2"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "peripheral hardware configuration 2"]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "peripheral hardware configuration 3"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "peripheral hardware configuration 3"]
    #[inline(always)]
    pub const fn set_CFG3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "peripheral hardware configuration 4"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "peripheral hardware configuration 4"]
    #[inline(always)]
    pub const fn set_CFG4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPTIM_HWCFGR1 {
    #[inline(always)]
    fn default() -> LPTIM_HWCFGR1 {
        LPTIM_HWCFGR1(0)
    }
}
impl core::fmt::Debug for LPTIM_HWCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_HWCFGR1")
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("CFG3", &self.CFG3())
            .field("CFG4", &self.CFG4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_HWCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_HWCFGR1 {{ CFG1: {=u8:?}, CFG2: {=u8:?}, CFG3: {=u8:?}, CFG4: {=u8:?} }}",
            self.CFG1(),
            self.CFG2(),
            self.CFG3(),
            self.CFG4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_HWCFGR2(pub u32);
impl LPTIM_HWCFGR2 {
    #[doc = "peripheral hardware configuration 1"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "peripheral hardware configuration 1"]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "peripheral hardware configuration 2"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "peripheral hardware configuration 2"]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "peripheral hardware configuration 3"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "peripheral hardware configuration 3"]
    #[inline(always)]
    pub const fn set_CFG3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for LPTIM_HWCFGR2 {
    #[inline(always)]
    fn default() -> LPTIM_HWCFGR2 {
        LPTIM_HWCFGR2(0)
    }
}
impl core::fmt::Debug for LPTIM_HWCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_HWCFGR2")
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("CFG3", &self.CFG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_HWCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_HWCFGR2 {{ CFG1: {=u8:?}, CFG2: {=u8:?}, CFG3: {=bool:?} }}",
            self.CFG1(),
            self.CFG2(),
            self.CFG3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_ICR(pub u32);
impl LPTIM_ICR {
    #[doc = "Capture/compare 1 clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1CF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 clear flag"]
    #[inline(always)]
    pub const fn set_CC1CF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ARRMCF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match clear flag"]
    #[inline(always)]
    pub const fn set_ARRMCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger valid edge clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EXTTRIGCF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger valid edge clear flag"]
    #[inline(always)]
    pub const fn set_EXTTRIGCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register 1 update OK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP1OKCF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 1 update OK clear flag"]
    #[inline(always)]
    pub const fn set_CMP1OKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ARROKCF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK clear flag"]
    #[inline(always)]
    pub const fn set_ARROKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Direction change to UP clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn UPCF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to UP clear flag"]
    #[inline(always)]
    pub const fn set_UPCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Direction change to down clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNCF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Direction change to down clear flag"]
    #[inline(always)]
    pub const fn set_DOWNCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Update event clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn UECF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Update event clear flag"]
    #[inline(always)]
    pub const fn set_UECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Repetition register update OK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn REPOKCF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repetition register update OK clear flag"]
    #[inline(always)]
    pub const fn set_REPOKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/compare 2 clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2CF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 clear flag"]
    #[inline(always)]
    pub const fn set_CC2CF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture/compare 1 over-capture clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1OCF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 1 over-capture clear flag"]
    #[inline(always)]
    pub const fn set_CC1OCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture/compare 2 over-capture clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2OCF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/compare 2 over-capture clear flag"]
    #[inline(always)]
    pub const fn set_CC2OCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Compare register 2 update OK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP2OKCF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 2 update OK clear flag"]
    #[inline(always)]
    pub const fn set_CMP2OKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt enable register update OK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn DIEROKCF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable register update OK clear flag"]
    #[inline(always)]
    pub const fn set_DIEROKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for LPTIM_ICR {
    #[inline(always)]
    fn default() -> LPTIM_ICR {
        LPTIM_ICR(0)
    }
}
impl core::fmt::Debug for LPTIM_ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_ICR")
            .field("CC1CF", &self.CC1CF())
            .field("ARRMCF", &self.ARRMCF())
            .field("EXTTRIGCF", &self.EXTTRIGCF())
            .field("CMP1OKCF", &self.CMP1OKCF())
            .field("ARROKCF", &self.ARROKCF())
            .field("UPCF", &self.UPCF())
            .field("DOWNCF", &self.DOWNCF())
            .field("UECF", &self.UECF())
            .field("REPOKCF", &self.REPOKCF())
            .field("CC2CF", &self.CC2CF())
            .field("CC1OCF", &self.CC1OCF())
            .field("CC2OCF", &self.CC2OCF())
            .field("CMP2OKCF", &self.CMP2OKCF())
            .field("DIEROKCF", &self.DIEROKCF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_ICR {{ CC1CF: {=bool:?}, ARRMCF: {=bool:?}, EXTTRIGCF: {=bool:?}, CMP1OKCF: {=bool:?}, ARROKCF: {=bool:?}, UPCF: {=bool:?}, DOWNCF: {=bool:?}, UECF: {=bool:?}, REPOKCF: {=bool:?}, CC2CF: {=bool:?}, CC1OCF: {=bool:?}, CC2OCF: {=bool:?}, CMP2OKCF: {=bool:?}, DIEROKCF: {=bool:?} }}",
            self.CC1CF(),
            self.ARRMCF(),
            self.EXTTRIGCF(),
            self.CMP1OKCF(),
            self.ARROKCF(),
            self.UPCF(),
            self.DOWNCF(),
            self.UECF(),
            self.REPOKCF(),
            self.CC2CF(),
            self.CC1OCF(),
            self.CC2OCF(),
            self.CMP2OKCF(),
            self.DIEROKCF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_ISR(pub u32);
impl LPTIM_ISR {
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1IF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub const fn set_CC1IF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Autoreload match"]
    #[must_use]
    #[inline(always)]
    pub const fn ARRM(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload match"]
    #[inline(always)]
    pub const fn set_ARRM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External trigger edge event"]
    #[must_use]
    #[inline(always)]
    pub const fn EXTTRIG(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External trigger edge event"]
    #[inline(always)]
    pub const fn set_EXTTRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compare register 1 update OK"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP1OK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 1 update OK"]
    #[inline(always)]
    pub const fn set_CMP1OK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Autoreload register update OK"]
    #[must_use]
    #[inline(always)]
    pub const fn ARROK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Autoreload register update OK"]
    #[inline(always)]
    pub const fn set_ARROK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Counter direction change down to up"]
    #[must_use]
    #[inline(always)]
    pub const fn UP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Counter direction change down to up"]
    #[inline(always)]
    pub const fn set_UP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Counter direction change up to down"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Counter direction change up to down"]
    #[inline(always)]
    pub const fn set_DOWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPTIM update event occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM update event occurred"]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Repetition register update OK"]
    #[must_use]
    #[inline(always)]
    pub const fn REPOK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Repetition register update OK"]
    #[inline(always)]
    pub const fn set_REPOK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2IF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub const fn set_CC2IF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture 1 over-capture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC1OF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Capture 1 over-capture flag"]
    #[inline(always)]
    pub const fn set_CC1OF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Capture 2 over-capture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CC2OF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Capture 2 over-capture flag"]
    #[inline(always)]
    pub const fn set_CC2OF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Compare register 2 update OK"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP2OK(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Compare register 2 update OK"]
    #[inline(always)]
    pub const fn set_CMP2OK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt enable register update OK"]
    #[must_use]
    #[inline(always)]
    pub const fn DIEROK(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable register update OK"]
    #[inline(always)]
    pub const fn set_DIEROK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for LPTIM_ISR {
    #[inline(always)]
    fn default() -> LPTIM_ISR {
        LPTIM_ISR(0)
    }
}
impl core::fmt::Debug for LPTIM_ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_ISR")
            .field("CC1IF", &self.CC1IF())
            .field("ARRM", &self.ARRM())
            .field("EXTTRIG", &self.EXTTRIG())
            .field("CMP1OK", &self.CMP1OK())
            .field("ARROK", &self.ARROK())
            .field("UP", &self.UP())
            .field("DOWN", &self.DOWN())
            .field("UE", &self.UE())
            .field("REPOK", &self.REPOK())
            .field("CC2IF", &self.CC2IF())
            .field("CC1OF", &self.CC1OF())
            .field("CC2OF", &self.CC2OF())
            .field("CMP2OK", &self.CMP2OK())
            .field("DIEROK", &self.DIEROK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_ISR {{ CC1IF: {=bool:?}, ARRM: {=bool:?}, EXTTRIG: {=bool:?}, CMP1OK: {=bool:?}, ARROK: {=bool:?}, UP: {=bool:?}, DOWN: {=bool:?}, UE: {=bool:?}, REPOK: {=bool:?}, CC2IF: {=bool:?}, CC1OF: {=bool:?}, CC2OF: {=bool:?}, CMP2OK: {=bool:?}, DIEROK: {=bool:?} }}",
            self.CC1IF(),
            self.ARRM(),
            self.EXTTRIG(),
            self.CMP1OK(),
            self.ARROK(),
            self.UP(),
            self.DOWN(),
            self.UE(),
            self.REPOK(),
            self.CC2IF(),
            self.CC1OF(),
            self.CC2OF(),
            self.CMP2OK(),
            self.DIEROK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_PIDR(pub u32);
impl LPTIM_PIDR {
    #[doc = "Peripheral type identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn P_ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral type identifier"]
    #[inline(always)]
    pub const fn set_P_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPTIM_PIDR {
    #[inline(always)]
    fn default() -> LPTIM_PIDR {
        LPTIM_PIDR(0)
    }
}
impl core::fmt::Debug for LPTIM_PIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_PIDR")
            .field("P_ID", &self.P_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_PIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_PIDR {{ P_ID: {=u32:?} }}", self.P_ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_RCR(pub u32);
impl LPTIM_RCR {
    #[doc = "Repetition register value"]
    #[must_use]
    #[inline(always)]
    pub const fn REP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Repetition register value"]
    #[inline(always)]
    pub const fn set_REP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for LPTIM_RCR {
    #[inline(always)]
    fn default() -> LPTIM_RCR {
        LPTIM_RCR(0)
    }
}
impl core::fmt::Debug for LPTIM_RCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_RCR")
            .field("REP", &self.REP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_RCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_RCR {{ REP: {=u8:?} }}", self.REP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_SIDR(pub u32);
impl LPTIM_SIDR {
    #[doc = "Registers map size identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn S_ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Registers map size identifier"]
    #[inline(always)]
    pub const fn set_S_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPTIM_SIDR {
    #[inline(always)]
    fn default() -> LPTIM_SIDR {
        LPTIM_SIDR(0)
    }
}
impl core::fmt::Debug for LPTIM_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_SIDR")
            .field("S_ID", &self.S_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPTIM_SIDR {{ S_ID: {=u32:?} }}", self.S_ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPTIM_VERR(pub u32);
impl LPTIM_VERR {
    #[doc = "Minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for LPTIM_VERR {
    #[inline(always)]
    fn default() -> LPTIM_VERR {
        LPTIM_VERR(0)
    }
}
impl core::fmt::Debug for LPTIM_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPTIM_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPTIM_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}

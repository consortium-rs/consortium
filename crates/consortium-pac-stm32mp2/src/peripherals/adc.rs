#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC {
    ptr: *mut u8,
}
unsafe impl Send for ADC {}
unsafe impl Sync for ADC {}
impl ADC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<ADC_ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ADC interrupt enable register"]
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<ADC_IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ADC control register"]
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<ADC_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ADC configuration register 1"]
    #[inline(always)]
    pub const fn CFGR1(self) -> crate::common::Reg<ADC_CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "ADC configuration register 2"]
    #[inline(always)]
    pub const fn CFGR2(self) -> crate::common::Reg<ADC_CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ADC sample time register 1"]
    #[inline(always)]
    pub const fn SMPR1(self) -> crate::common::Reg<ADC_SMPR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ADC sample time register 2"]
    #[inline(always)]
    pub const fn SMPR2(self) -> crate::common::Reg<ADC_SMPR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "ADC channel preselection register"]
    #[inline(always)]
    pub const fn PCSEL(self) -> crate::common::Reg<ADC_PCSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "ADC regular sequence register 1"]
    #[inline(always)]
    pub const fn SQR1(self) -> crate::common::Reg<ADC_SQR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "ADC regular sequence register 2"]
    #[inline(always)]
    pub const fn SQR2(self) -> crate::common::Reg<ADC_SQR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "ADC regular sequence register 3"]
    #[inline(always)]
    pub const fn SQR3(self) -> crate::common::Reg<ADC_SQR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "ADC regular sequence register 4"]
    #[inline(always)]
    pub const fn SQR4(self) -> crate::common::Reg<ADC_SQR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "ADC regular data register"]
    #[inline(always)]
    pub const fn DR(self) -> crate::common::Reg<ADC_DR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ADC injected sequence register"]
    #[inline(always)]
    pub const fn JSQR(self) -> crate::common::Reg<ADC_JSQR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "ADC offset configuration register 1"]
    #[inline(always)]
    pub const fn OFCFGR1(self) -> crate::common::Reg<ADC_OFCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ADC offset configuration register 2"]
    #[inline(always)]
    pub const fn OFCFGR2(self) -> crate::common::Reg<ADC_OFCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ADC offset configuration register 3"]
    #[inline(always)]
    pub const fn OFCFGR3(self) -> crate::common::Reg<ADC_OFCFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "ADC offset configuration register 4"]
    #[inline(always)]
    pub const fn OFCFGR4(self) -> crate::common::Reg<ADC_OFCFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "ADC offset register 1"]
    #[inline(always)]
    pub const fn OFR1(self) -> crate::common::Reg<ADC_OFR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "ADC offset register 2"]
    #[inline(always)]
    pub const fn OFR2(self) -> crate::common::Reg<ADC_OFR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "ADC offset register 3"]
    #[inline(always)]
    pub const fn OFR3(self) -> crate::common::Reg<ADC_OFR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "ADC offset register 4"]
    #[inline(always)]
    pub const fn OFR4(self) -> crate::common::Reg<ADC_OFR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "ADC gain compensation register"]
    #[inline(always)]
    pub const fn GCOMP(self) -> crate::common::Reg<ADC_GCOMP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "ADC injected data register 1"]
    #[inline(always)]
    pub const fn JDR1(self) -> crate::common::Reg<ADC_JDR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "ADC injected data register 2"]
    #[inline(always)]
    pub const fn JDR2(self) -> crate::common::Reg<ADC_JDR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "ADC injected data register 3"]
    #[inline(always)]
    pub const fn JDR3(self) -> crate::common::Reg<ADC_JDR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "ADC injected data register 4"]
    #[inline(always)]
    pub const fn JDR4(self) -> crate::common::Reg<ADC_JDR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn AWD2CR(self) -> crate::common::Reg<ADC_AWD2CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn AWD3CR(self) -> crate::common::Reg<ADC_AWD3CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "ADC analog watchdog 1 low threshold register"]
    #[inline(always)]
    pub const fn AWD1LTR(self) -> crate::common::Reg<ADC_AWD1LTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "ADC analog watchdog 1 high threshold register"]
    #[inline(always)]
    pub const fn AWD1HTR(self) -> crate::common::Reg<ADC_AWD1HTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "ADC analog watchdog 2 low threshold register"]
    #[inline(always)]
    pub const fn AWD2LTR(self) -> crate::common::Reg<ADC_AWD2LTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "ADC analog watchdog 2 high threshold register"]
    #[inline(always)]
    pub const fn AWD2HTR(self) -> crate::common::Reg<ADC_AWD2HTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 low threshold register"]
    #[inline(always)]
    pub const fn AWD3LTR(self) -> crate::common::Reg<ADC_AWD3LTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "ADC analog watchdog 3 high threshold register"]
    #[inline(always)]
    pub const fn AWD3HTR(self) -> crate::common::Reg<ADC_AWD3HTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "ADC differential mode selection register"]
    #[inline(always)]
    pub const fn DIFSEL(self) -> crate::common::Reg<ADC_DIFSEL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "ADC calibration factors"]
    #[inline(always)]
    pub const fn CALFACT(self) -> crate::common::Reg<ADC_CALFACT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "ADC option register"]
    #[inline(always)]
    pub const fn OR(self) -> crate::common::Reg<ADC_OR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD1HTR(pub u32);
impl ADC_AWD1HTR {
    #[doc = "ADC analog watchdog 1 threshold high"]
    #[must_use]
    #[inline(always)]
    pub const fn HTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 1 threshold high"]
    #[inline(always)]
    pub const fn set_HTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "ADC analog watchdog 1 filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn AWDFILT(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "ADC analog watchdog 1 filtering"]
    #[inline(always)]
    pub const fn set_AWDFILT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for ADC_AWD1HTR {
    #[inline(always)]
    fn default() -> ADC_AWD1HTR {
        ADC_AWD1HTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD1HTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD1HTR")
            .field("HTR", &self.HTR())
            .field("AWDFILT", &self.AWDFILT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD1HTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_AWD1HTR {{ HTR: {=u32:?}, AWDFILT: {=u8:?} }}",
            self.HTR(),
            self.AWDFILT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD1LTR(pub u32);
impl ADC_AWD1LTR {
    #[doc = "ADC analog watchdog 1 threshold low"]
    #[must_use]
    #[inline(always)]
    pub const fn LTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub const fn set_LTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD1LTR {
    #[inline(always)]
    fn default() -> ADC_AWD1LTR {
        ADC_AWD1LTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD1LTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD1LTR")
            .field("LTR", &self.LTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD1LTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD1LTR {{ LTR: {=u32:?} }}", self.LTR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD2CR(pub u32);
impl ADC_AWD2CR {
    #[doc = "ADC analog watchdog 2 monitored channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD2CH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 2 monitored channel selection"]
    #[inline(always)]
    pub const fn set_AWD2CH(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD2CR {
    #[inline(always)]
    fn default() -> ADC_AWD2CR {
        ADC_AWD2CR(0)
    }
}
impl core::fmt::Debug for ADC_AWD2CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD2CR")
            .field("AWD2CH", &self.AWD2CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD2CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD2CR {{ AWD2CH: {=u32:?} }}", self.AWD2CH())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD2HTR(pub u32);
impl ADC_AWD2HTR {
    #[doc = "ADC analog watchdog 2 threshold high"]
    #[must_use]
    #[inline(always)]
    pub const fn HTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 2 threshold high"]
    #[inline(always)]
    pub const fn set_HTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD2HTR {
    #[inline(always)]
    fn default() -> ADC_AWD2HTR {
        ADC_AWD2HTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD2HTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD2HTR")
            .field("HTR", &self.HTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD2HTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD2HTR {{ HTR: {=u32:?} }}", self.HTR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD2LTR(pub u32);
impl ADC_AWD2LTR {
    #[doc = "ADC analog watchdog 2 threshold low"]
    #[must_use]
    #[inline(always)]
    pub const fn LTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub const fn set_LTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD2LTR {
    #[inline(always)]
    fn default() -> ADC_AWD2LTR {
        ADC_AWD2LTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD2LTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD2LTR")
            .field("LTR", &self.LTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD2LTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD2LTR {{ LTR: {=u32:?} }}", self.LTR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD3CR(pub u32);
impl ADC_AWD3CR {
    #[doc = "ADC analog watchdog 3 monitored channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD3CH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 3 monitored channel selection"]
    #[inline(always)]
    pub const fn set_AWD3CH(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD3CR {
    #[inline(always)]
    fn default() -> ADC_AWD3CR {
        ADC_AWD3CR(0)
    }
}
impl core::fmt::Debug for ADC_AWD3CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD3CR")
            .field("AWD3CH", &self.AWD3CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD3CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD3CR {{ AWD3CH: {=u32:?} }}", self.AWD3CH())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD3HTR(pub u32);
impl ADC_AWD3HTR {
    #[doc = "ADC analog watchdog 3 threshold high"]
    #[must_use]
    #[inline(always)]
    pub const fn HTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub const fn set_HTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD3HTR {
    #[inline(always)]
    fn default() -> ADC_AWD3HTR {
        ADC_AWD3HTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD3HTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD3HTR")
            .field("HTR", &self.HTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD3HTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD3HTR {{ HTR: {=u32:?} }}", self.HTR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWD3LTR(pub u32);
impl ADC_AWD3LTR {
    #[doc = "ADC analog watchdog 3 threshold low"]
    #[must_use]
    #[inline(always)]
    pub const fn LTR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "ADC analog watchdog 3 threshold low"]
    #[inline(always)]
    pub const fn set_LTR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for ADC_AWD3LTR {
    #[inline(always)]
    fn default() -> ADC_AWD3LTR {
        ADC_AWD3LTR(0)
    }
}
impl core::fmt::Debug for ADC_AWD3LTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWD3LTR")
            .field("LTR", &self.LTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWD3LTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_AWD3LTR {{ LTR: {=u32:?} }}", self.LTR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CALFACT(pub u32);
impl ADC_CALFACT {
    #[doc = "ADC calibration factor in single-ended mode"]
    #[must_use]
    #[inline(always)]
    pub const fn CALFACT_S(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub const fn set_CALFACT_S(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "ADC calibration factor in differential mode"]
    #[must_use]
    #[inline(always)]
    pub const fn CALFACT_D(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "ADC calibration factor in differential mode"]
    #[inline(always)]
    pub const fn set_CALFACT_D(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "ADC calibration additional offset mode"]
    #[must_use]
    #[inline(always)]
    pub const fn CALADDOS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ADC calibration additional offset mode"]
    #[inline(always)]
    pub const fn set_CALADDOS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_CALFACT {
    #[inline(always)]
    fn default() -> ADC_CALFACT {
        ADC_CALFACT(0)
    }
}
impl core::fmt::Debug for ADC_CALFACT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALFACT")
            .field("CALFACT_S", &self.CALFACT_S())
            .field("CALFACT_D", &self.CALFACT_D())
            .field("CALADDOS", &self.CALADDOS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CALFACT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CALFACT {{ CALFACT_S: {=u16:?}, CALFACT_D: {=u16:?}, CALADDOS: {=bool:?} }}",
            self.CALFACT_S(),
            self.CALFACT_D(),
            self.CALADDOS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CFGR1(pub u32);
impl ADC_CFGR1 {
    #[doc = "ADC data management configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn DMNGT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADC data management configuration"]
    #[inline(always)]
    pub const fn set_DMNGT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADC data resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn RES(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "ADC data resolution"]
    #[inline(always)]
    pub const fn set_RES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "ADC group regular external trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn EXTSEL(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular external trigger source"]
    #[inline(always)]
    pub const fn set_EXTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "ADC group regular external trigger polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn EXTEN(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "ADC group regular external trigger polarity"]
    #[inline(always)]
    pub const fn set_EXTEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "ADC group regular overrun configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRMOD(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular overrun configuration"]
    #[inline(always)]
    pub const fn set_OVRMOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ADC group regular continuous conversion mode"]
    #[must_use]
    #[inline(always)]
    pub const fn CONT(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub const fn set_CONT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ADC low power auto wait"]
    #[must_use]
    #[inline(always)]
    pub const fn AUTDLY(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ADC low power auto wait"]
    #[inline(always)]
    pub const fn set_AUTDLY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ADC group regular sequencer discontinuous mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DISCEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub const fn set_DISCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ADC group regular sequencer discontinuous number of ranks"]
    #[must_use]
    #[inline(always)]
    pub const fn DISCNUM(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub const fn set_DISCNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "ADC group injected sequencer discontinuous mode"]
    #[must_use]
    #[inline(always)]
    pub const fn JDISCEN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub const fn set_JDISCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD1SGL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub const fn set_AWD1SGL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD1EN(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub const fn set_AWD1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ADC analog watchdog 1 enable on scope ADC group injected"]
    #[must_use]
    #[inline(always)]
    pub const fn JAWD1EN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub const fn set_JAWD1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC group injected automatic trigger mode"]
    #[must_use]
    #[inline(always)]
    pub const fn JAUTO(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub const fn set_JAUTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC analog watchdog 1 monitored channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD1CH(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub const fn set_AWD1CH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
}
impl Default for ADC_CFGR1 {
    #[inline(always)]
    fn default() -> ADC_CFGR1 {
        ADC_CFGR1(0)
    }
}
impl core::fmt::Debug for ADC_CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR1")
            .field("DMNGT", &self.DMNGT())
            .field("RES", &self.RES())
            .field("EXTSEL", &self.EXTSEL())
            .field("EXTEN", &self.EXTEN())
            .field("OVRMOD", &self.OVRMOD())
            .field("CONT", &self.CONT())
            .field("AUTDLY", &self.AUTDLY())
            .field("DISCEN", &self.DISCEN())
            .field("DISCNUM", &self.DISCNUM())
            .field("JDISCEN", &self.JDISCEN())
            .field("AWD1SGL", &self.AWD1SGL())
            .field("AWD1EN", &self.AWD1EN())
            .field("JAWD1EN", &self.JAWD1EN())
            .field("JAUTO", &self.JAUTO())
            .field("AWD1CH", &self.AWD1CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CFGR1 {{ DMNGT: {=u8:?}, RES: {=u8:?}, EXTSEL: {=u8:?}, EXTEN: {=u8:?}, OVRMOD: {=bool:?}, CONT: {=bool:?}, AUTDLY: {=bool:?}, DISCEN: {=bool:?}, DISCNUM: {=u8:?}, JDISCEN: {=bool:?}, AWD1SGL: {=bool:?}, AWD1EN: {=bool:?}, JAWD1EN: {=bool:?}, JAUTO: {=bool:?}, AWD1CH: {=u8:?} }}",
            self.DMNGT(),
            self.RES(),
            self.EXTSEL(),
            self.EXTEN(),
            self.OVRMOD(),
            self.CONT(),
            self.AUTDLY(),
            self.DISCEN(),
            self.DISCNUM(),
            self.JDISCEN(),
            self.AWD1SGL(),
            self.AWD1EN(),
            self.JAWD1EN(),
            self.JAUTO(),
            self.AWD1CH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CFGR2(pub u32);
impl ADC_CFGR2 {
    #[doc = "ADC oversampler enable on scope ADC group regular"]
    #[must_use]
    #[inline(always)]
    pub const fn ROVSE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub const fn set_ROVSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC oversampler enable on scope ADC group injected"]
    #[must_use]
    #[inline(always)]
    pub const fn JOVSE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub const fn set_JOVSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC oversampling shift"]
    #[must_use]
    #[inline(always)]
    pub const fn OVSS(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "ADC oversampling shift"]
    #[inline(always)]
    pub const fn set_OVSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[must_use]
    #[inline(always)]
    pub const fn TROVS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub const fn set_TROVS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ADC oversampling mode managing interlaced conversions of ADC group regular and group injected"]
    #[must_use]
    #[inline(always)]
    pub const fn ROVSM(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ADC oversampling mode managing interlaced conversions of ADC group regular and group injected"]
    #[inline(always)]
    pub const fn set_ROVSM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC bulb sampling mode"]
    #[must_use]
    #[inline(always)]
    pub const fn BULB(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ADC bulb sampling mode"]
    #[inline(always)]
    pub const fn set_BULB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ADC software trigger bit for sampling time control trigger mode"]
    #[must_use]
    #[inline(always)]
    pub const fn SWTRIG(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "ADC software trigger bit for sampling time control trigger mode"]
    #[inline(always)]
    pub const fn set_SWTRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ADC sampling time control trigger mode"]
    #[must_use]
    #[inline(always)]
    pub const fn SMPTRIG(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ADC sampling time control trigger mode"]
    #[inline(always)]
    pub const fn set_SMPTRIG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC oversampling ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn OVSR(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "ADC oversampling ratio"]
    #[inline(always)]
    pub const fn set_OVSR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "ADC left shift factor"]
    #[must_use]
    #[inline(always)]
    pub const fn LSHIFT(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "ADC left shift factor"]
    #[inline(always)]
    pub const fn set_LSHIFT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ADC_CFGR2 {
    #[inline(always)]
    fn default() -> ADC_CFGR2 {
        ADC_CFGR2(0)
    }
}
impl core::fmt::Debug for ADC_CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR2")
            .field("ROVSE", &self.ROVSE())
            .field("JOVSE", &self.JOVSE())
            .field("OVSS", &self.OVSS())
            .field("TROVS", &self.TROVS())
            .field("ROVSM", &self.ROVSM())
            .field("BULB", &self.BULB())
            .field("SWTRIG", &self.SWTRIG())
            .field("SMPTRIG", &self.SMPTRIG())
            .field("OVSR", &self.OVSR())
            .field("LSHIFT", &self.LSHIFT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CFGR2 {{ ROVSE: {=bool:?}, JOVSE: {=bool:?}, OVSS: {=u8:?}, TROVS: {=bool:?}, ROVSM: {=bool:?}, BULB: {=bool:?}, SWTRIG: {=bool:?}, SMPTRIG: {=bool:?}, OVSR: {=u16:?}, LSHIFT: {=u8:?} }}",
            self.ROVSE(),
            self.JOVSE(),
            self.OVSS(),
            self.TROVS(),
            self.ROVSM(),
            self.BULB(),
            self.SWTRIG(),
            self.SMPTRIG(),
            self.OVSR(),
            self.LSHIFT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CR(pub u32);
impl ADC_CR {
    #[doc = "ADC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC enable"]
    #[inline(always)]
    pub const fn set_ADEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC disable"]
    #[inline(always)]
    pub const fn set_ADDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC group regular conversion start"]
    #[must_use]
    #[inline(always)]
    pub const fn ADSTART(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular conversion start"]
    #[inline(always)]
    pub const fn set_ADSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC group injected conversion start"]
    #[must_use]
    #[inline(always)]
    pub const fn JADSTART(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected conversion start"]
    #[inline(always)]
    pub const fn set_JADSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC group regular conversion stop"]
    #[must_use]
    #[inline(always)]
    pub const fn ADSTP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular conversion stop"]
    #[inline(always)]
    pub const fn set_ADSTP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ADC group injected conversion stop"]
    #[must_use]
    #[inline(always)]
    pub const fn JADSTP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected conversion stop"]
    #[inline(always)]
    pub const fn set_JADSTP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ADC deep power down enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DEEPPWD(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ADC deep power down enable"]
    #[inline(always)]
    pub const fn set_DEEPPWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ADC differential mode for calibration"]
    #[must_use]
    #[inline(always)]
    pub const fn ADCALDIF(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ADC differential mode for calibration"]
    #[inline(always)]
    pub const fn set_ADCALDIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ADC calibration"]
    #[must_use]
    #[inline(always)]
    pub const fn ADCAL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ADC calibration"]
    #[inline(always)]
    pub const fn set_ADCAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_CR {
    #[inline(always)]
    fn default() -> ADC_CR {
        ADC_CR(0)
    }
}
impl core::fmt::Debug for ADC_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CR")
            .field("ADEN", &self.ADEN())
            .field("ADDIS", &self.ADDIS())
            .field("ADSTART", &self.ADSTART())
            .field("JADSTART", &self.JADSTART())
            .field("ADSTP", &self.ADSTP())
            .field("JADSTP", &self.JADSTP())
            .field("DEEPPWD", &self.DEEPPWD())
            .field("ADCALDIF", &self.ADCALDIF())
            .field("ADCAL", &self.ADCAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CR {{ ADEN: {=bool:?}, ADDIS: {=bool:?}, ADSTART: {=bool:?}, JADSTART: {=bool:?}, ADSTP: {=bool:?}, JADSTP: {=bool:?}, DEEPPWD: {=bool:?}, ADCALDIF: {=bool:?}, ADCAL: {=bool:?} }}",
            self.ADEN(),
            self.ADDIS(),
            self.ADSTART(),
            self.JADSTART(),
            self.ADSTP(),
            self.JADSTP(),
            self.DEEPPWD(),
            self.ADCALDIF(),
            self.ADCAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_DIFSEL(pub u32);
impl ADC_DIFSEL {
    #[doc = "ADC channel differential or single-ended mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn DIFSEL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "ADC channel differential or single-ended mode selection"]
    #[inline(always)]
    pub const fn set_DIFSEL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for ADC_DIFSEL {
    #[inline(always)]
    fn default() -> ADC_DIFSEL {
        ADC_DIFSEL(0)
    }
}
impl core::fmt::Debug for ADC_DIFSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DIFSEL")
            .field("DIFSEL", &self.DIFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_DIFSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_DIFSEL {{ DIFSEL: {=u32:?} }}", self.DIFSEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_DR(pub u32);
impl ADC_DR {
    #[doc = "ADC group regular conversion data"]
    #[must_use]
    #[inline(always)]
    pub const fn RDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADC group regular conversion data"]
    #[inline(always)]
    pub const fn set_RDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADC_DR {
    #[inline(always)]
    fn default() -> ADC_DR {
        ADC_DR(0)
    }
}
impl core::fmt::Debug for ADC_DR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DR")
            .field("RDATA", &self.RDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_DR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_DR {{ RDATA: {=u32:?} }}", self.RDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_GCOMP(pub u32);
impl ADC_GCOMP {
    #[doc = "Gain compensation coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn GCOMPCOEFF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Gain compensation coefficient"]
    #[inline(always)]
    pub const fn set_GCOMPCOEFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Gain compensation mode"]
    #[must_use]
    #[inline(always)]
    pub const fn GCOMP(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Gain compensation mode"]
    #[inline(always)]
    pub const fn set_GCOMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_GCOMP {
    #[inline(always)]
    fn default() -> ADC_GCOMP {
        ADC_GCOMP(0)
    }
}
impl core::fmt::Debug for ADC_GCOMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_GCOMP")
            .field("GCOMPCOEFF", &self.GCOMPCOEFF())
            .field("GCOMP", &self.GCOMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_GCOMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_GCOMP {{ GCOMPCOEFF: {=u16:?}, GCOMP: {=bool:?} }}",
            self.GCOMPCOEFF(),
            self.GCOMP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_IER(pub u32);
impl ADC_IER {
    #[doc = "ADC ready interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ADRDYIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC ready interrupt"]
    #[inline(always)]
    pub const fn set_ADRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC group regular end of sampling interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn EOSMPIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub const fn set_EOSMPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC group regular end of unitary conversion interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn EOCIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub const fn set_EOCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC group regular end of sequence conversions interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn EOSIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub const fn set_EOSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC group regular overrun interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular overrun interrupt"]
    #[inline(always)]
    pub const fn set_OVRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ADC group injected end of unitary conversion interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn JEOCIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    pub const fn set_JEOCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ADC group injected end of sequence conversions interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn JEOSIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    pub const fn set_JEOSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ADC analog watchdog 1 interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD1IE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub const fn set_AWD1IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ADC analog watchdog 2 interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD2IE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub const fn set_AWD2IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ADC analog watchdog 3 interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD3IE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub const fn set_AWD3IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ADC_IER {
    #[inline(always)]
    fn default() -> ADC_IER {
        ADC_IER(0)
    }
}
impl core::fmt::Debug for ADC_IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_IER")
            .field("ADRDYIE", &self.ADRDYIE())
            .field("EOSMPIE", &self.EOSMPIE())
            .field("EOCIE", &self.EOCIE())
            .field("EOSIE", &self.EOSIE())
            .field("OVRIE", &self.OVRIE())
            .field("JEOCIE", &self.JEOCIE())
            .field("JEOSIE", &self.JEOSIE())
            .field("AWD1IE", &self.AWD1IE())
            .field("AWD2IE", &self.AWD2IE())
            .field("AWD3IE", &self.AWD3IE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_IER {{ ADRDYIE: {=bool:?}, EOSMPIE: {=bool:?}, EOCIE: {=bool:?}, EOSIE: {=bool:?}, OVRIE: {=bool:?}, JEOCIE: {=bool:?}, JEOSIE: {=bool:?}, AWD1IE: {=bool:?}, AWD2IE: {=bool:?}, AWD3IE: {=bool:?} }}",
            self.ADRDYIE(),
            self.EOSMPIE(),
            self.EOCIE(),
            self.EOSIE(),
            self.OVRIE(),
            self.JEOCIE(),
            self.JEOSIE(),
            self.AWD1IE(),
            self.AWD2IE(),
            self.AWD3IE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_ISR(pub u32);
impl ADC_ISR {
    #[doc = "ADC ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ADRDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC ready flag"]
    #[inline(always)]
    pub const fn set_ADRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC group regular end of sampling flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EOSMP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of sampling flag"]
    #[inline(always)]
    pub const fn set_EOSMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC group regular end of unitary conversion flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EOC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub const fn set_EOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC group regular end of sequence conversions flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EOS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub const fn set_EOS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC group regular overrun flag"]
    #[must_use]
    #[inline(always)]
    pub const fn OVR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group regular overrun flag"]
    #[inline(always)]
    pub const fn set_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ADC group injected end of unitary conversion flag"]
    #[must_use]
    #[inline(always)]
    pub const fn JEOC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected end of unitary conversion flag"]
    #[inline(always)]
    pub const fn set_JEOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ADC group injected end of sequence conversions flag"]
    #[must_use]
    #[inline(always)]
    pub const fn JEOS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ADC group injected end of sequence conversions flag"]
    #[inline(always)]
    pub const fn set_JEOS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ADC analog watchdog 1 flag"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 1 flag"]
    #[inline(always)]
    pub const fn set_AWD1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ADC analog watchdog 2 flag"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 2 flag"]
    #[inline(always)]
    pub const fn set_AWD2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ADC analog watchdog 3 flag"]
    #[must_use]
    #[inline(always)]
    pub const fn AWD3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ADC analog watchdog 3 flag"]
    #[inline(always)]
    pub const fn set_AWD3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ADC_ISR {
    #[inline(always)]
    fn default() -> ADC_ISR {
        ADC_ISR(0)
    }
}
impl core::fmt::Debug for ADC_ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ISR")
            .field("ADRDY", &self.ADRDY())
            .field("EOSMP", &self.EOSMP())
            .field("EOC", &self.EOC())
            .field("EOS", &self.EOS())
            .field("OVR", &self.OVR())
            .field("JEOC", &self.JEOC())
            .field("JEOS", &self.JEOS())
            .field("AWD1", &self.AWD1())
            .field("AWD2", &self.AWD2())
            .field("AWD3", &self.AWD3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_ISR {{ ADRDY: {=bool:?}, EOSMP: {=bool:?}, EOC: {=bool:?}, EOS: {=bool:?}, OVR: {=bool:?}, JEOC: {=bool:?}, JEOS: {=bool:?}, AWD1: {=bool:?}, AWD2: {=bool:?}, AWD3: {=bool:?} }}",
            self.ADRDY(),
            self.EOSMP(),
            self.EOC(),
            self.EOS(),
            self.OVR(),
            self.JEOC(),
            self.JEOS(),
            self.AWD1(),
            self.AWD2(),
            self.AWD3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JDR1(pub u32);
impl ADC_JDR1 {
    #[doc = "ADC group injected sequencer rank 1 conversion data"]
    #[must_use]
    #[inline(always)]
    pub const fn JDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADC group injected sequencer rank 1 conversion data"]
    #[inline(always)]
    pub const fn set_JDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADC_JDR1 {
    #[inline(always)]
    fn default() -> ADC_JDR1 {
        ADC_JDR1(0)
    }
}
impl core::fmt::Debug for ADC_JDR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JDR1")
            .field("JDATA", &self.JDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JDR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_JDR1 {{ JDATA: {=u32:?} }}", self.JDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JDR2(pub u32);
impl ADC_JDR2 {
    #[doc = "ADC group injected sequencer rank 2 conversion data"]
    #[must_use]
    #[inline(always)]
    pub const fn JDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub const fn set_JDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADC_JDR2 {
    #[inline(always)]
    fn default() -> ADC_JDR2 {
        ADC_JDR2(0)
    }
}
impl core::fmt::Debug for ADC_JDR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JDR2")
            .field("JDATA", &self.JDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JDR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_JDR2 {{ JDATA: {=u32:?} }}", self.JDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JDR3(pub u32);
impl ADC_JDR3 {
    #[doc = "ADC group injected sequencer rank 3 conversion data"]
    #[must_use]
    #[inline(always)]
    pub const fn JDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub const fn set_JDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADC_JDR3 {
    #[inline(always)]
    fn default() -> ADC_JDR3 {
        ADC_JDR3(0)
    }
}
impl core::fmt::Debug for ADC_JDR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JDR3")
            .field("JDATA", &self.JDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JDR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_JDR3 {{ JDATA: {=u32:?} }}", self.JDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JDR4(pub u32);
impl ADC_JDR4 {
    #[doc = "ADC group injected sequencer rank 4 conversion data"]
    #[must_use]
    #[inline(always)]
    pub const fn JDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ADC group injected sequencer rank 4 conversion data"]
    #[inline(always)]
    pub const fn set_JDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ADC_JDR4 {
    #[inline(always)]
    fn default() -> ADC_JDR4 {
        ADC_JDR4(0)
    }
}
impl core::fmt::Debug for ADC_JDR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JDR4")
            .field("JDATA", &self.JDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JDR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_JDR4 {{ JDATA: {=u32:?} }}", self.JDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JSQR(pub u32);
impl ADC_JSQR {
    #[doc = "ADC group injected sequencer scan length"]
    #[must_use]
    #[inline(always)]
    pub const fn JL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADC group injected sequencer scan length"]
    #[inline(always)]
    pub const fn set_JL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADC group injected external trigger source"]
    #[must_use]
    #[inline(always)]
    pub const fn JEXTSEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group injected external trigger source"]
    #[inline(always)]
    pub const fn set_JEXTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "ADC group injected external trigger polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn JEXTEN(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "ADC group injected external trigger polarity"]
    #[inline(always)]
    pub const fn set_JEXTEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "ADC group injected sequencer rank 1"]
    #[must_use]
    #[inline(always)]
    pub const fn JSQ1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub const fn set_JSQ1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "ADC group injected sequencer rank 2"]
    #[must_use]
    #[inline(always)]
    pub const fn JSQ2(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub const fn set_JSQ2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "ADC group injected sequencer rank 3"]
    #[must_use]
    #[inline(always)]
    pub const fn JSQ3(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub const fn set_JSQ3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "ADC group injected sequencer rank 4"]
    #[must_use]
    #[inline(always)]
    pub const fn JSQ4(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub const fn set_JSQ4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for ADC_JSQR {
    #[inline(always)]
    fn default() -> ADC_JSQR {
        ADC_JSQR(0)
    }
}
impl core::fmt::Debug for ADC_JSQR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JSQR")
            .field("JL", &self.JL())
            .field("JEXTSEL", &self.JEXTSEL())
            .field("JEXTEN", &self.JEXTEN())
            .field("JSQ1", &self.JSQ1())
            .field("JSQ2", &self.JSQ2())
            .field("JSQ3", &self.JSQ3())
            .field("JSQ4", &self.JSQ4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JSQR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_JSQR {{ JL: {=u8:?}, JEXTSEL: {=u8:?}, JEXTEN: {=u8:?}, JSQ1: {=u8:?}, JSQ2: {=u8:?}, JSQ3: {=u8:?}, JSQ4: {=u8:?} }}",
            self.JL(),
            self.JEXTSEL(),
            self.JEXTEN(),
            self.JSQ1(),
            self.JSQ2(),
            self.JSQ3(),
            self.JSQ4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFCFGR1(pub u32);
impl ADC_OFCFGR1 {
    #[doc = "ADC offset instance 1 positive offset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn POSOFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 1 positive offset enable"]
    #[inline(always)]
    pub const fn set_POSOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC offset instance 1 unsigned saturation value"]
    #[must_use]
    #[inline(always)]
    pub const fn USAT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 1 unsigned saturation value"]
    #[inline(always)]
    pub const fn set_USAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC offset instance 1 signed satuaration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSAT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 1 signed satuaration enable"]
    #[inline(always)]
    pub const fn set_SSAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC offset instance 1 channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET_CH(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC offset instance 1 channel selection"]
    #[inline(always)]
    pub const fn set_OFFSET_CH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for ADC_OFCFGR1 {
    #[inline(always)]
    fn default() -> ADC_OFCFGR1 {
        ADC_OFCFGR1(0)
    }
}
impl core::fmt::Debug for ADC_OFCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFCFGR1")
            .field("POSOFF", &self.POSOFF())
            .field("USAT", &self.USAT())
            .field("SSAT", &self.SSAT())
            .field("OFFSET_CH", &self.OFFSET_CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_OFCFGR1 {{ POSOFF: {=bool:?}, USAT: {=bool:?}, SSAT: {=bool:?}, OFFSET_CH: {=u8:?} }}",
            self.POSOFF(),
            self.USAT(),
            self.SSAT(),
            self.OFFSET_CH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFCFGR2(pub u32);
impl ADC_OFCFGR2 {
    #[doc = "ADC offset instance 2 positive offset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn POSOFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 2 positive offset enable"]
    #[inline(always)]
    pub const fn set_POSOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC offset instance 2 unsigned saturation value"]
    #[must_use]
    #[inline(always)]
    pub const fn USAT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 2 unsigned saturation value"]
    #[inline(always)]
    pub const fn set_USAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC offset instance 2 signed satuaration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSAT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 2 signed satuaration enable"]
    #[inline(always)]
    pub const fn set_SSAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC offset instance 2 channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET_CH(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC offset instance 2 channel selection"]
    #[inline(always)]
    pub const fn set_OFFSET_CH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for ADC_OFCFGR2 {
    #[inline(always)]
    fn default() -> ADC_OFCFGR2 {
        ADC_OFCFGR2(0)
    }
}
impl core::fmt::Debug for ADC_OFCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFCFGR2")
            .field("POSOFF", &self.POSOFF())
            .field("USAT", &self.USAT())
            .field("SSAT", &self.SSAT())
            .field("OFFSET_CH", &self.OFFSET_CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_OFCFGR2 {{ POSOFF: {=bool:?}, USAT: {=bool:?}, SSAT: {=bool:?}, OFFSET_CH: {=u8:?} }}",
            self.POSOFF(),
            self.USAT(),
            self.SSAT(),
            self.OFFSET_CH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFCFGR3(pub u32);
impl ADC_OFCFGR3 {
    #[doc = "ADC offset instance 3 positive offset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn POSOFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 3 positive offset enable"]
    #[inline(always)]
    pub const fn set_POSOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC offset instance 3 unsigned saturation value"]
    #[must_use]
    #[inline(always)]
    pub const fn USAT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 3 unsigned saturation value"]
    #[inline(always)]
    pub const fn set_USAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC offset instance 3 signed satuaration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSAT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 3 signed satuaration enable"]
    #[inline(always)]
    pub const fn set_SSAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC offset instance 3 channel selection for the data offset"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET_CH(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC offset instance 3 channel selection for the data offset"]
    #[inline(always)]
    pub const fn set_OFFSET_CH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for ADC_OFCFGR3 {
    #[inline(always)]
    fn default() -> ADC_OFCFGR3 {
        ADC_OFCFGR3(0)
    }
}
impl core::fmt::Debug for ADC_OFCFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFCFGR3")
            .field("POSOFF", &self.POSOFF())
            .field("USAT", &self.USAT())
            .field("SSAT", &self.SSAT())
            .field("OFFSET_CH", &self.OFFSET_CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFCFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_OFCFGR3 {{ POSOFF: {=bool:?}, USAT: {=bool:?}, SSAT: {=bool:?}, OFFSET_CH: {=u8:?} }}",
            self.POSOFF(),
            self.USAT(),
            self.SSAT(),
            self.OFFSET_CH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFCFGR4(pub u32);
impl ADC_OFCFGR4 {
    #[doc = "ADC offset instance 4 positive offset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn POSOFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 4 positive offset enable"]
    #[inline(always)]
    pub const fn set_POSOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC offset instance 4 unsigned saturation value"]
    #[must_use]
    #[inline(always)]
    pub const fn USAT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 4 unsigned saturation value"]
    #[inline(always)]
    pub const fn set_USAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC offset instance 4 signed satuaration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSAT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ADC offset instance 4 signed satuaration enable"]
    #[inline(always)]
    pub const fn set_SSAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ADC offset instance 4 channel selection for the data offset"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET_CH(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC offset instance 4 channel selection for the data offset"]
    #[inline(always)]
    pub const fn set_OFFSET_CH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for ADC_OFCFGR4 {
    #[inline(always)]
    fn default() -> ADC_OFCFGR4 {
        ADC_OFCFGR4(0)
    }
}
impl core::fmt::Debug for ADC_OFCFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFCFGR4")
            .field("POSOFF", &self.POSOFF())
            .field("USAT", &self.USAT())
            .field("SSAT", &self.SSAT())
            .field("OFFSET_CH", &self.OFFSET_CH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFCFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_OFCFGR4 {{ POSOFF: {=bool:?}, USAT: {=bool:?}, SSAT: {=bool:?}, OFFSET_CH: {=u8:?} }}",
            self.POSOFF(),
            self.USAT(),
            self.SSAT(),
            self.OFFSET_CH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFR1(pub u32);
impl ADC_OFR1 {
    #[doc = "ADC offset instance 1 offset level"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "ADC offset instance 1 offset level"]
    #[inline(always)]
    pub const fn set_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for ADC_OFR1 {
    #[inline(always)]
    fn default() -> ADC_OFR1 {
        ADC_OFR1(0)
    }
}
impl core::fmt::Debug for ADC_OFR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR1")
            .field("OFFSET", &self.OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_OFR1 {{ OFFSET: {=u32:?} }}", self.OFFSET())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFR2(pub u32);
impl ADC_OFR2 {
    #[doc = "ADC offset instance 2 offset level"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "ADC offset instance 2 offset level"]
    #[inline(always)]
    pub const fn set_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for ADC_OFR2 {
    #[inline(always)]
    fn default() -> ADC_OFR2 {
        ADC_OFR2(0)
    }
}
impl core::fmt::Debug for ADC_OFR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR2")
            .field("OFFSET", &self.OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_OFR2 {{ OFFSET: {=u32:?} }}", self.OFFSET())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFR3(pub u32);
impl ADC_OFR3 {
    #[doc = "ADC offset instance 3 offset level"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "ADC offset instance 3 offset level"]
    #[inline(always)]
    pub const fn set_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for ADC_OFR3 {
    #[inline(always)]
    fn default() -> ADC_OFR3 {
        ADC_OFR3(0)
    }
}
impl core::fmt::Debug for ADC_OFR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR3")
            .field("OFFSET", &self.OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_OFR3 {{ OFFSET: {=u32:?} }}", self.OFFSET())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OFR4(pub u32);
impl ADC_OFR4 {
    #[doc = "ADC offset instance 4 offset level"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "ADC offset instance 4 offset level"]
    #[inline(always)]
    pub const fn set_OFFSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for ADC_OFR4 {
    #[inline(always)]
    fn default() -> ADC_OFR4 {
        ADC_OFR4(0)
    }
}
impl core::fmt::Debug for ADC_OFR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OFR4")
            .field("OFFSET", &self.OFFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OFR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_OFR4 {{ OFFSET: {=u32:?} }}", self.OFFSET())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_OR(pub u32);
impl ADC_OR {
    #[doc = "ADC internal path to internal reference voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn SELREF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC internal path to internal reference voltage"]
    #[inline(always)]
    pub const fn set_SELREF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC internal path to bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn SELBG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC internal path to bandgap"]
    #[inline(always)]
    pub const fn set_SELBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC internal path to VDDCORE"]
    #[must_use]
    #[inline(always)]
    pub const fn VDDCOREEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC internal path to VDDCORE"]
    #[inline(always)]
    pub const fn set_VDDCOREEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC internal path to VDDCPU"]
    #[must_use]
    #[inline(always)]
    pub const fn VDDCPUEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADC internal path to VDDCPU"]
    #[inline(always)]
    pub const fn set_VDDCPUEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC internal path to VDDGPU"]
    #[must_use]
    #[inline(always)]
    pub const fn VDDGPUEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ADC internal path to VDDGPU"]
    #[inline(always)]
    pub const fn set_VDDGPUEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for ADC_OR {
    #[inline(always)]
    fn default() -> ADC_OR {
        ADC_OR(0)
    }
}
impl core::fmt::Debug for ADC_OR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_OR")
            .field("SELREF", &self.SELREF())
            .field("SELBG", &self.SELBG())
            .field("VDDCOREEN", &self.VDDCOREEN())
            .field("VDDCPUEN", &self.VDDCPUEN())
            .field("VDDGPUEN", &self.VDDGPUEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_OR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_OR {{ SELREF: {=bool:?}, SELBG: {=bool:?}, VDDCOREEN: {=bool:?}, VDDCPUEN: {=bool:?}, VDDGPUEN: {=bool:?} }}",
            self.SELREF(),
            self.SELBG(),
            self.VDDCOREEN(),
            self.VDDCPUEN(),
            self.VDDGPUEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PCSEL(pub u32);
impl ADC_PCSEL {
    #[doc = "ADC channel preselection"]
    #[must_use]
    #[inline(always)]
    pub const fn PCSEL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "ADC channel preselection"]
    #[inline(always)]
    pub const fn set_PCSEL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for ADC_PCSEL {
    #[inline(always)]
    fn default() -> ADC_PCSEL {
        ADC_PCSEL(0)
    }
}
impl core::fmt::Debug for ADC_PCSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PCSEL")
            .field("PCSEL", &self.PCSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PCSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_PCSEL {{ PCSEL: {=u32:?} }}", self.PCSEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SMPR1(pub u32);
impl ADC_SMPR1 {
    #[doc = "ADC channel 0 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 0 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "ADC channel 1 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 1 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "ADC channel 2 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 2 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "ADC channel 3 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP3(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 3 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "ADC channel 4 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 4 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "ADC channel 5 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP5(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 5 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "ADC channel 6 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP6(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 6 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "ADC channel 7 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP7(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 7 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "ADC channel 8 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 8 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "ADC channel 9 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP9(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 9 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
}
impl Default for ADC_SMPR1 {
    #[inline(always)]
    fn default() -> ADC_SMPR1 {
        ADC_SMPR1(0)
    }
}
impl core::fmt::Debug for ADC_SMPR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SMPR1")
            .field("SMP0", &self.SMP0())
            .field("SMP1", &self.SMP1())
            .field("SMP2", &self.SMP2())
            .field("SMP3", &self.SMP3())
            .field("SMP4", &self.SMP4())
            .field("SMP5", &self.SMP5())
            .field("SMP6", &self.SMP6())
            .field("SMP7", &self.SMP7())
            .field("SMP8", &self.SMP8())
            .field("SMP9", &self.SMP9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SMPR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SMPR1 {{ SMP0: {=u8:?}, SMP1: {=u8:?}, SMP2: {=u8:?}, SMP3: {=u8:?}, SMP4: {=u8:?}, SMP5: {=u8:?}, SMP6: {=u8:?}, SMP7: {=u8:?}, SMP8: {=u8:?}, SMP9: {=u8:?} }}",
            self.SMP0(),
            self.SMP1(),
            self.SMP2(),
            self.SMP3(),
            self.SMP4(),
            self.SMP5(),
            self.SMP6(),
            self.SMP7(),
            self.SMP8(),
            self.SMP9()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SMPR2(pub u32);
impl ADC_SMPR2 {
    #[doc = "ADC channel 10 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "ADC channel 11 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP11(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "ADC channel 12 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP12(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "ADC channel 13 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP13(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "ADC channel 14 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "ADC channel 15 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP15(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "ADC channel 16 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP16(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 16 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "ADC channel 17 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP17(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 17 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "ADC channel 18 sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP18(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "ADC Channel 19 Sampling time selection"]
    #[must_use]
    #[inline(always)]
    pub const fn SMP19(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[doc = "ADC Channel 19 Sampling time selection"]
    #[inline(always)]
    pub const fn set_SMP19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
}
impl Default for ADC_SMPR2 {
    #[inline(always)]
    fn default() -> ADC_SMPR2 {
        ADC_SMPR2(0)
    }
}
impl core::fmt::Debug for ADC_SMPR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SMPR2")
            .field("SMP10", &self.SMP10())
            .field("SMP11", &self.SMP11())
            .field("SMP12", &self.SMP12())
            .field("SMP13", &self.SMP13())
            .field("SMP14", &self.SMP14())
            .field("SMP15", &self.SMP15())
            .field("SMP16", &self.SMP16())
            .field("SMP17", &self.SMP17())
            .field("SMP18", &self.SMP18())
            .field("SMP19", &self.SMP19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SMPR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SMPR2 {{ SMP10: {=u8:?}, SMP11: {=u8:?}, SMP12: {=u8:?}, SMP13: {=u8:?}, SMP14: {=u8:?}, SMP15: {=u8:?}, SMP16: {=u8:?}, SMP17: {=u8:?}, SMP18: {=u8:?}, SMP19: {=u8:?} }}",
            self.SMP10(),
            self.SMP11(),
            self.SMP12(),
            self.SMP13(),
            self.SMP14(),
            self.SMP15(),
            self.SMP16(),
            self.SMP17(),
            self.SMP18(),
            self.SMP19()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SQR1(pub u32);
impl ADC_SQR1 {
    #[doc = "ADC group regular sequencer scan length"]
    #[must_use]
    #[inline(always)]
    pub const fn L(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "ADC group regular sequencer scan length"]
    #[inline(always)]
    pub const fn set_L(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "ADC group regular sequencer rank 1"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 1"]
    #[inline(always)]
    pub const fn set_SQ1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "ADC group regular sequencer rank 2"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 2"]
    #[inline(always)]
    pub const fn set_SQ2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "ADC group regular sequencer rank 3"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 3"]
    #[inline(always)]
    pub const fn set_SQ3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "ADC group regular sequencer rank 4"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 4"]
    #[inline(always)]
    pub const fn set_SQ4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for ADC_SQR1 {
    #[inline(always)]
    fn default() -> ADC_SQR1 {
        ADC_SQR1(0)
    }
}
impl core::fmt::Debug for ADC_SQR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SQR1")
            .field("L", &self.L())
            .field("SQ1", &self.SQ1())
            .field("SQ2", &self.SQ2())
            .field("SQ3", &self.SQ3())
            .field("SQ4", &self.SQ4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SQR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SQR1 {{ L: {=u8:?}, SQ1: {=u8:?}, SQ2: {=u8:?}, SQ3: {=u8:?}, SQ4: {=u8:?} }}",
            self.L(),
            self.SQ1(),
            self.SQ2(),
            self.SQ3(),
            self.SQ4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SQR2(pub u32);
impl ADC_SQR2 {
    #[doc = "ADC group regular sequencer rank 5"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 5"]
    #[inline(always)]
    pub const fn set_SQ5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "ADC group regular sequencer rank 6"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 6"]
    #[inline(always)]
    pub const fn set_SQ6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "ADC group regular sequencer rank 7"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 7"]
    #[inline(always)]
    pub const fn set_SQ7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "ADC group regular sequencer rank 8"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ8(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 8"]
    #[inline(always)]
    pub const fn set_SQ8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "ADC group regular sequencer rank 9"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ9(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 9"]
    #[inline(always)]
    pub const fn set_SQ9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for ADC_SQR2 {
    #[inline(always)]
    fn default() -> ADC_SQR2 {
        ADC_SQR2(0)
    }
}
impl core::fmt::Debug for ADC_SQR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SQR2")
            .field("SQ5", &self.SQ5())
            .field("SQ6", &self.SQ6())
            .field("SQ7", &self.SQ7())
            .field("SQ8", &self.SQ8())
            .field("SQ9", &self.SQ9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SQR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SQR2 {{ SQ5: {=u8:?}, SQ6: {=u8:?}, SQ7: {=u8:?}, SQ8: {=u8:?}, SQ9: {=u8:?} }}",
            self.SQ5(),
            self.SQ6(),
            self.SQ7(),
            self.SQ8(),
            self.SQ9()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SQR3(pub u32);
impl ADC_SQR3 {
    #[doc = "ADC group regular sequencer rank 10"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ10(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 10"]
    #[inline(always)]
    pub const fn set_SQ10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "ADC group regular sequencer rank 11"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 11"]
    #[inline(always)]
    pub const fn set_SQ11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "ADC group regular sequencer rank 12"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ12(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 12"]
    #[inline(always)]
    pub const fn set_SQ12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "ADC group regular sequencer rank 13"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ13(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 13"]
    #[inline(always)]
    pub const fn set_SQ13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "ADC group regular sequencer rank 14"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 14"]
    #[inline(always)]
    pub const fn set_SQ14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for ADC_SQR3 {
    #[inline(always)]
    fn default() -> ADC_SQR3 {
        ADC_SQR3(0)
    }
}
impl core::fmt::Debug for ADC_SQR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SQR3")
            .field("SQ10", &self.SQ10())
            .field("SQ11", &self.SQ11())
            .field("SQ12", &self.SQ12())
            .field("SQ13", &self.SQ13())
            .field("SQ14", &self.SQ14())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SQR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SQR3 {{ SQ10: {=u8:?}, SQ11: {=u8:?}, SQ12: {=u8:?}, SQ13: {=u8:?}, SQ14: {=u8:?} }}",
            self.SQ10(),
            self.SQ11(),
            self.SQ12(),
            self.SQ13(),
            self.SQ14()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_SQR4(pub u32);
impl ADC_SQR4 {
    #[doc = "ADC group regular sequencer rank 15"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ15(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 15"]
    #[inline(always)]
    pub const fn set_SQ15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "ADC group regular sequencer rank 16"]
    #[must_use]
    #[inline(always)]
    pub const fn SQ16(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC group regular sequencer rank 16"]
    #[inline(always)]
    pub const fn set_SQ16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
}
impl Default for ADC_SQR4 {
    #[inline(always)]
    fn default() -> ADC_SQR4 {
        ADC_SQR4(0)
    }
}
impl core::fmt::Debug for ADC_SQR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_SQR4")
            .field("SQ15", &self.SQ15())
            .field("SQ16", &self.SQ16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_SQR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_SQR4 {{ SQ15: {=u8:?}, SQ16: {=u8:?} }}",
            self.SQ15(),
            self.SQ16()
        )
    }
}

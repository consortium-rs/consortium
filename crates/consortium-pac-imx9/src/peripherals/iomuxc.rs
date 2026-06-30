#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC {
    ptr: *mut u8,
}
unsafe impl Send for IOMUXC {}
unsafe impl Sync for IOMUXC {}
impl IOMUXC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SW_MUX_CTL_PAD(
        self,
        n: usize,
    ) -> crate::common::Reg<IOMUXC_SW_MUX_CTL_PAD, crate::common::RW> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SW_PAD_CTL_PAD(
        self,
        n: usize,
    ) -> crate::common::Reg<IOMUXC_SW_PAD_CTL_PAD, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SW_PAD_CTL_PAD_FCCU_ERR0(
        self,
    ) -> crate::common::Reg<IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[inline(always)]
    pub const fn SELECT_INPUT(
        self,
        n: usize,
    ) -> crate::common::Reg<IOMUXC_SELECT_INPUT, crate::common::RW> {
        assert!(n < 132usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SELECT_INPUT(pub u32);
impl IOMUXC_SELECT_INPUT {
    #[must_use]
    #[inline(always)]
    pub const fn DAISY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DAISY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for IOMUXC_SELECT_INPUT {
    #[inline(always)]
    fn default() -> IOMUXC_SELECT_INPUT {
        IOMUXC_SELECT_INPUT(0)
    }
}
impl core::fmt::Debug for IOMUXC_SELECT_INPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SELECT_INPUT")
            .field("DAISY", &self.DAISY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SELECT_INPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IOMUXC_SELECT_INPUT {{ DAISY: {=u8:?} }}", self.DAISY())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_MUX_CTL_PAD(pub u32);
impl IOMUXC_SW_MUX_CTL_PAD {
    #[must_use]
    #[inline(always)]
    pub const fn SION(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for IOMUXC_SW_MUX_CTL_PAD {
    #[inline(always)]
    fn default() -> IOMUXC_SW_MUX_CTL_PAD {
        IOMUXC_SW_MUX_CTL_PAD(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_MUX_CTL_PAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_MUX_CTL_PAD")
            .field("SION", &self.SION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_MUX_CTL_PAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_MUX_CTL_PAD {{ SION: {=bool:?} }}",
            self.SION()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_MUX_CTL_PAD_MUX(pub u32);
impl IOMUXC_SW_MUX_CTL_PAD_MUX {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for IOMUXC_SW_MUX_CTL_PAD_MUX {
    #[inline(always)]
    fn default() -> IOMUXC_SW_MUX_CTL_PAD_MUX {
        IOMUXC_SW_MUX_CTL_PAD_MUX(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_MUX_CTL_PAD_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_MUX_CTL_PAD_MUX")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_MUX_CTL_PAD_MUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_MUX_CTL_PAD_MUX {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_PAD_CTL_PAD(pub u32);
impl IOMUXC_SW_PAD_CTL_PAD {
    #[must_use]
    #[inline(always)]
    pub const fn DSE(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FSEL1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FSEL1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PU(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HYS(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HYS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMC(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SMC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Domain Access Field"]
    #[must_use]
    #[inline(always)]
    pub const fn APC(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain Access Field"]
    #[inline(always)]
    pub const fn set_APC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for IOMUXC_SW_PAD_CTL_PAD {
    #[inline(always)]
    fn default() -> IOMUXC_SW_PAD_CTL_PAD {
        IOMUXC_SW_PAD_CTL_PAD(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_PAD_CTL_PAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_PAD_CTL_PAD")
            .field("DSE", &self.DSE())
            .field("FSEL1", &self.FSEL1())
            .field("PU", &self.PU())
            .field("PD", &self.PD())
            .field("OD", &self.OD())
            .field("HYS", &self.HYS())
            .field("SMC", &self.SMC())
            .field("APC", &self.APC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_PAD_CTL_PAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_PAD_CTL_PAD {{ DSE: {=u8:?}, FSEL1: {=u8:?}, PU: {=bool:?}, PD: {=bool:?}, OD: {=bool:?}, HYS: {=bool:?}, SMC: {=u8:?}, APC: {=u8:?} }}",
            self.DSE(),
            self.FSEL1(),
            self.PU(),
            self.PD(),
            self.OD(),
            self.HYS(),
            self.SMC(),
            self.APC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_PAD_CTL_PAD_APC(pub u32);
impl IOMUXC_SW_PAD_CTL_PAD_APC {
    #[doc = "Domain Access Field"]
    #[must_use]
    #[inline(always)]
    pub const fn LCK(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain Access Field"]
    #[inline(always)]
    pub const fn set_LCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for IOMUXC_SW_PAD_CTL_PAD_APC {
    #[inline(always)]
    fn default() -> IOMUXC_SW_PAD_CTL_PAD_APC {
        IOMUXC_SW_PAD_CTL_PAD_APC(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_PAD_CTL_PAD_APC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_PAD_CTL_PAD_APC")
            .field("LCK", &self.LCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_PAD_CTL_PAD_APC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_PAD_CTL_PAD_APC {{ LCK: {=u8:?} }}",
            self.LCK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0(pub u32);
impl IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {
    #[must_use]
    #[inline(always)]
    pub const fn DSE(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FSEL1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FSEL1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PU(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HYS(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HYS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMC(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SMC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Domain Access Field"]
    #[must_use]
    #[inline(always)]
    pub const fn APC(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Domain Access Field"]
    #[inline(always)]
    pub const fn set_APC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {
    #[inline(always)]
    fn default() -> IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {
        IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0")
            .field("DSE", &self.DSE())
            .field("FSEL1", &self.FSEL1())
            .field("PU", &self.PU())
            .field("PD", &self.PD())
            .field("OD", &self.OD())
            .field("HYS", &self.HYS())
            .field("SMC", &self.SMC())
            .field("APC", &self.APC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0 {{ DSE: {=u8:?}, FSEL1: {=u8:?}, PU: {=bool:?}, PD: {=bool:?}, OD: {=bool:?}, HYS: {=bool:?}, SMC: {=u8:?}, APC: {=u8:?} }}",
            self.DSE(),
            self.FSEL1(),
            self.PU(),
            self.PD(),
            self.OD(),
            self.HYS(),
            self.SMC(),
            self.APC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC(pub u32);
impl IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {
    #[doc = "APC lock bits"]
    #[must_use]
    #[inline(always)]
    pub const fn LCK(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "APC lock bits"]
    #[inline(always)]
    pub const fn set_LCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {
    #[inline(always)]
    fn default() -> IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {
        IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC(0)
    }
}
impl core::fmt::Debug for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC")
            .field("LCK", &self.LCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IOMUXC_SW_PAD_CTL_PAD_FCCU_ERR0_APC {{ LCK: {=u8:?} }}",
            self.LCK()
        )
    }
}

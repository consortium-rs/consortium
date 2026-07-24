#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM {
    ptr: *mut u8,
}
unsafe impl Send for TPM {}
unsafe impl Sync for TPM {}
impl TPM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<TPM_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<TPM_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GLOBAL(self) -> crate::common::Reg<TPM_GLOBAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn SC(self) -> crate::common::Reg<TPM_SC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CNT(self) -> crate::common::Reg<TPM_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MOD(self) -> crate::common::Reg<TPM_MOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<TPM_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CONTROLS_CnSC(self, n: usize) -> crate::common::Reg<TPM_CnSC, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn CONTROLS_CnV(self, n: usize) -> crate::common::Reg<TPM_CnV, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn COMBINE(self) -> crate::common::Reg<TPM_COMBINE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIG(self) -> crate::common::Reg<TPM_TRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn POL(self) -> crate::common::Reg<TPM_POL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn FILTER(self) -> crate::common::Reg<TPM_FILTER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn QDCTRL(self) -> crate::common::Reg<TPM_QDCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn CONF(self) -> crate::common::Reg<TPM_CONF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_CNT(pub u32);
impl TPM_CNT {
    #[doc = "Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn COUNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter Value"]
    #[inline(always)]
    pub const fn set_COUNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TPM_CNT {
    #[inline(always)]
    fn default() -> TPM_CNT {
        TPM_CNT(0)
    }
}
impl core::fmt::Debug for TPM_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_CNT")
            .field("COUNT", &self.COUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TPM_CNT {{ COUNT: {=u32:?} }}", self.COUNT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_COMBINE(pub u32);
impl TPM_COMBINE {
    #[must_use]
    #[inline(always)]
    pub const fn COMBINE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMBINE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMSWAP0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMSWAP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMBINE1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMBINE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMSWAP1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMSWAP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for TPM_COMBINE {
    #[inline(always)]
    fn default() -> TPM_COMBINE {
        TPM_COMBINE(0)
    }
}
impl core::fmt::Debug for TPM_COMBINE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_COMBINE")
            .field("COMBINE0", &self.COMBINE0())
            .field("COMSWAP0", &self.COMSWAP0())
            .field("COMBINE1", &self.COMBINE1())
            .field("COMSWAP1", &self.COMSWAP1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_COMBINE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_COMBINE {{ COMBINE0: {=bool:?}, COMSWAP0: {=bool:?}, COMBINE1: {=bool:?}, COMSWAP1: {=bool:?} }}",
            self.COMBINE0(),
            self.COMSWAP0(),
            self.COMBINE1(),
            self.COMSWAP1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_CONF(pub u32);
impl TPM_CONF {
    #[must_use]
    #[inline(always)]
    pub const fn DOZEEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DBGMODE(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DBGMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GTBSYNC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GTBSYNC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GTBEEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GTBEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CSOT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CSOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CSOO(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CSOO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CROT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CROT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CPOT(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CPOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGPOL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRGPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGSRC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRGSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGSEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TRGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for TPM_CONF {
    #[inline(always)]
    fn default() -> TPM_CONF {
        TPM_CONF(0)
    }
}
impl core::fmt::Debug for TPM_CONF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_CONF")
            .field("DOZEEN", &self.DOZEEN())
            .field("DBGMODE", &self.DBGMODE())
            .field("GTBSYNC", &self.GTBSYNC())
            .field("GTBEEN", &self.GTBEEN())
            .field("CSOT", &self.CSOT())
            .field("CSOO", &self.CSOO())
            .field("CROT", &self.CROT())
            .field("CPOT", &self.CPOT())
            .field("TRGPOL", &self.TRGPOL())
            .field("TRGSRC", &self.TRGSRC())
            .field("TRGSEL", &self.TRGSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_CONF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_CONF {{ DOZEEN: {=bool:?}, DBGMODE: {=u8:?}, GTBSYNC: {=bool:?}, GTBEEN: {=bool:?}, CSOT: {=bool:?}, CSOO: {=bool:?}, CROT: {=bool:?}, CPOT: {=bool:?}, TRGPOL: {=bool:?}, TRGSRC: {=bool:?}, TRGSEL: {=u8:?} }}",
            self.DOZEEN(),
            self.DBGMODE(),
            self.GTBSYNC(),
            self.GTBEEN(),
            self.CSOT(),
            self.CSOO(),
            self.CROT(),
            self.CPOT(),
            self.TRGPOL(),
            self.TRGSRC(),
            self.TRGSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_CnSC(pub u32);
impl TPM_CnSC {
    #[must_use]
    #[inline(always)]
    pub const fn DMA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Edge or Level Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn ELSA(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Edge or Level Select A"]
    #[inline(always)]
    pub const fn set_ELSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Edge or Level Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn ELSB(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Edge or Level Select A"]
    #[inline(always)]
    pub const fn set_ELSB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Edge or Level Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn MSA(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Edge or Level Select B"]
    #[inline(always)]
    pub const fn set_MSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel Mode Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn MSB(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Mode Select A"]
    #[inline(always)]
    pub const fn set_MSB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel Mode Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn CHIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Mode Select B"]
    #[inline(always)]
    pub const fn set_CHIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CHF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CHF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for TPM_CnSC {
    #[inline(always)]
    fn default() -> TPM_CnSC {
        TPM_CnSC(0)
    }
}
impl core::fmt::Debug for TPM_CnSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_CnSC")
            .field("DMA", &self.DMA())
            .field("ELSA", &self.ELSA())
            .field("ELSB", &self.ELSB())
            .field("MSA", &self.MSA())
            .field("MSB", &self.MSB())
            .field("CHIE", &self.CHIE())
            .field("CHF", &self.CHF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_CnSC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_CnSC {{ DMA: {=bool:?}, ELSA: {=bool:?}, ELSB: {=bool:?}, MSA: {=bool:?}, MSB: {=bool:?}, CHIE: {=bool:?}, CHF: {=bool:?} }}",
            self.DMA(),
            self.ELSA(),
            self.ELSB(),
            self.MSA(),
            self.MSB(),
            self.CHIE(),
            self.CHF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_CnV(pub u32);
impl TPM_CnV {
    #[doc = "Channel Value"]
    #[must_use]
    #[inline(always)]
    pub const fn VAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel Value"]
    #[inline(always)]
    pub const fn set_VAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TPM_CnV {
    #[inline(always)]
    fn default() -> TPM_CnV {
        TPM_CnV(0)
    }
}
impl core::fmt::Debug for TPM_CnV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_CnV").field("VAL", &self.VAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_CnV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TPM_CnV {{ VAL: {=u32:?} }}", self.VAL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_FILTER(pub u32);
impl TPM_FILTER {
    #[doc = "Channel 0 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CH0FVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 0 Filter Value"]
    #[inline(always)]
    pub const fn set_CH0FVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Channel 0 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CH1FVAL(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 0 Filter Value"]
    #[inline(always)]
    pub const fn set_CH1FVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel 1 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CH2FVAL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 1 Filter Value"]
    #[inline(always)]
    pub const fn set_CH2FVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Channel 2 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CH3FVAL(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 2 Filter Value"]
    #[inline(always)]
    pub const fn set_CH3FVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for TPM_FILTER {
    #[inline(always)]
    fn default() -> TPM_FILTER {
        TPM_FILTER(0)
    }
}
impl core::fmt::Debug for TPM_FILTER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_FILTER")
            .field("CH0FVAL", &self.CH0FVAL())
            .field("CH1FVAL", &self.CH1FVAL())
            .field("CH2FVAL", &self.CH2FVAL())
            .field("CH3FVAL", &self.CH3FVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_FILTER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_FILTER {{ CH0FVAL: {=u8:?}, CH1FVAL: {=u8:?}, CH2FVAL: {=u8:?}, CH3FVAL: {=u8:?} }}",
            self.CH0FVAL(),
            self.CH1FVAL(),
            self.CH2FVAL(),
            self.CH3FVAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_GLOBAL(pub u32);
impl TPM_GLOBAL {
    #[must_use]
    #[inline(always)]
    pub const fn NOUPDATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOUPDATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
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
impl Default for TPM_GLOBAL {
    #[inline(always)]
    fn default() -> TPM_GLOBAL {
        TPM_GLOBAL(0)
    }
}
impl core::fmt::Debug for TPM_GLOBAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_GLOBAL")
            .field("NOUPDATE", &self.NOUPDATE())
            .field("RST", &self.RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_GLOBAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_GLOBAL {{ NOUPDATE: {=bool:?}, RST: {=bool:?} }}",
            self.NOUPDATE(),
            self.RST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_MOD(pub u32);
impl TPM_MOD {
    #[doc = "Modulo Value"]
    #[must_use]
    #[inline(always)]
    pub const fn MOD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Modulo Value"]
    #[inline(always)]
    pub const fn set_MOD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TPM_MOD {
    #[inline(always)]
    fn default() -> TPM_MOD {
        TPM_MOD(0)
    }
}
impl core::fmt::Debug for TPM_MOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_MOD").field("MOD", &self.MOD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_MOD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TPM_MOD {{ MOD: {=u32:?} }}", self.MOD())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_PARAM(pub u32);
impl TPM_PARAM {
    #[doc = "Channel Count"]
    #[must_use]
    #[inline(always)]
    pub const fn CHAN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Channel Count"]
    #[inline(always)]
    pub const fn set_CHAN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Channel Count"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Channel Count"]
    #[inline(always)]
    pub const fn set_TRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Trigger Count"]
    #[must_use]
    #[inline(always)]
    pub const fn WIDTH(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Count"]
    #[inline(always)]
    pub const fn set_WIDTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for TPM_PARAM {
    #[inline(always)]
    fn default() -> TPM_PARAM {
        TPM_PARAM(0)
    }
}
impl core::fmt::Debug for TPM_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_PARAM")
            .field("CHAN", &self.CHAN())
            .field("TRIG", &self.TRIG())
            .field("WIDTH", &self.WIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_PARAM {{ CHAN: {=u8:?}, TRIG: {=u8:?}, WIDTH: {=u8:?} }}",
            self.CHAN(),
            self.TRIG(),
            self.WIDTH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_POL(pub u32);
impl TPM_POL {
    #[must_use]
    #[inline(always)]
    pub const fn POL0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_POL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn POL1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_POL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn POL2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_POL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn POL3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_POL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TPM_POL {
    #[inline(always)]
    fn default() -> TPM_POL {
        TPM_POL(0)
    }
}
impl core::fmt::Debug for TPM_POL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_POL")
            .field("POL0", &self.POL0())
            .field("POL1", &self.POL1())
            .field("POL2", &self.POL2())
            .field("POL3", &self.POL3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_POL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_POL {{ POL0: {=bool:?}, POL1: {=bool:?}, POL2: {=bool:?}, POL3: {=bool:?} }}",
            self.POL0(),
            self.POL1(),
            self.POL2(),
            self.POL3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_QDCTRL(pub u32);
impl TPM_QDCTRL {
    #[must_use]
    #[inline(always)]
    pub const fn QUADEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_QUADEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TOFDIR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TOFDIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn QUADIR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_QUADIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn QUADMODE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_QUADMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TPM_QDCTRL {
    #[inline(always)]
    fn default() -> TPM_QDCTRL {
        TPM_QDCTRL(0)
    }
}
impl core::fmt::Debug for TPM_QDCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_QDCTRL")
            .field("QUADEN", &self.QUADEN())
            .field("TOFDIR", &self.TOFDIR())
            .field("QUADIR", &self.QUADIR())
            .field("QUADMODE", &self.QUADMODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_QDCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_QDCTRL {{ QUADEN: {=bool:?}, TOFDIR: {=bool:?}, QUADIR: {=bool:?}, QUADMODE: {=bool:?} }}",
            self.QUADEN(),
            self.TOFDIR(),
            self.QUADIR(),
            self.QUADMODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_SC(pub u32);
impl TPM_SC {
    #[must_use]
    #[inline(always)]
    pub const fn PS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CMOD(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CPWMS(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CPWMS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TOIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TOIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TOF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for TPM_SC {
    #[inline(always)]
    fn default() -> TPM_SC {
        TPM_SC(0)
    }
}
impl core::fmt::Debug for TPM_SC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_SC")
            .field("PS", &self.PS())
            .field("CMOD", &self.CMOD())
            .field("CPWMS", &self.CPWMS())
            .field("TOIE", &self.TOIE())
            .field("TOF", &self.TOF())
            .field("DMA", &self.DMA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_SC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_SC {{ PS: {=u8:?}, CMOD: {=u8:?}, CPWMS: {=bool:?}, TOIE: {=bool:?}, TOF: {=bool:?}, DMA: {=bool:?} }}",
            self.PS(),
            self.CMOD(),
            self.CPWMS(),
            self.TOIE(),
            self.TOF(),
            self.DMA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_STATUS(pub u32);
impl TPM_STATUS {
    #[must_use]
    #[inline(always)]
    pub const fn CH0F(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH1F(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH2F(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH2F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH3F(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH3F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TOF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for TPM_STATUS {
    #[inline(always)]
    fn default() -> TPM_STATUS {
        TPM_STATUS(0)
    }
}
impl core::fmt::Debug for TPM_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_STATUS")
            .field("CH0F", &self.CH0F())
            .field("CH1F", &self.CH1F())
            .field("CH2F", &self.CH2F())
            .field("CH3F", &self.CH3F())
            .field("TOF", &self.TOF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_STATUS {{ CH0F: {=bool:?}, CH1F: {=bool:?}, CH2F: {=bool:?}, CH3F: {=bool:?}, TOF: {=bool:?} }}",
            self.CH0F(),
            self.CH1F(),
            self.CH2F(),
            self.CH3F(),
            self.TOF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_TRIG(pub u32);
impl TPM_TRIG {
    #[must_use]
    #[inline(always)]
    pub const fn TRIG0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRIG0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRIG1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRIG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRIG2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRIG2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRIG3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRIG3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TPM_TRIG {
    #[inline(always)]
    fn default() -> TPM_TRIG {
        TPM_TRIG(0)
    }
}
impl core::fmt::Debug for TPM_TRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_TRIG")
            .field("TRIG0", &self.TRIG0())
            .field("TRIG1", &self.TRIG1())
            .field("TRIG2", &self.TRIG2())
            .field("TRIG3", &self.TRIG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_TRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_TRIG {{ TRIG0: {=bool:?}, TRIG1: {=bool:?}, TRIG2: {=bool:?}, TRIG3: {=bool:?} }}",
            self.TRIG0(),
            self.TRIG1(),
            self.TRIG2(),
            self.TRIG3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TPM_VERID(pub u32);
impl TPM_VERID {
    #[must_use]
    #[inline(always)]
    pub const fn FEATURE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_FEATURE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
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
impl Default for TPM_VERID {
    #[inline(always)]
    fn default() -> TPM_VERID {
        TPM_VERID(0)
    }
}
impl core::fmt::Debug for TPM_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TPM_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TPM_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TPM_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART {
    ptr: *mut u8,
}
unsafe impl Send for LPUART {}
unsafe impl Sync for LPUART {}
impl LPUART {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<LPUART_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<LPUART_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GLOBAL(self) -> crate::common::Reg<LPUART_GLOBAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PINCFG(self) -> crate::common::Reg<LPUART_PINCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn BAUD(self) -> crate::common::Reg<LPUART_BAUD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<LPUART_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<LPUART_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DATA(self) -> crate::common::Reg<LPUART_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn MATCH(self) -> crate::common::Reg<LPUART_MATCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MODIR(self) -> crate::common::Reg<LPUART_MODIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn FIFO(self) -> crate::common::Reg<LPUART_FIFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn WATER(self) -> crate::common::Reg<LPUART_WATER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn DATARO(self) -> crate::common::Reg<LPUART_DATARO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<LPUART_MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<LPUART_MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn REIR(self) -> crate::common::Reg<LPUART_REIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn TEIR(self) -> crate::common::Reg<LPUART_TEIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn HDCR(self) -> crate::common::Reg<LPUART_HDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn TOCR(self) -> crate::common::Reg<LPUART_TOCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn TOSR(self) -> crate::common::Reg<LPUART_TOSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn TIMEOUT(self, n: usize) -> crate::common::Reg<LPUART_TIMEOUT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TCBR(self, n: usize) -> crate::common::Reg<LPUART_TCBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TDBR(self, n: usize) -> crate::common::Reg<LPUART_TDBR, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_BAUD(pub u32);
impl LPUART_BAUD {
    #[doc = "Baud Rate Modulo Divisor"]
    #[must_use]
    #[inline(always)]
    pub const fn SBR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Baud Rate Modulo Divisor"]
    #[inline(always)]
    pub const fn set_SBR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Baud Rate Modulo Divisor"]
    #[must_use]
    #[inline(always)]
    pub const fn SBNS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Baud Rate Modulo Divisor"]
    #[inline(always)]
    pub const fn set_SBNS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEDGIE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEDGIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LBKDIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LBKDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RESYNCDIS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RESYNCDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOTHEDGE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOTHEDGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MATCFG(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MATCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIDMAE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDMAE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TDMAE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDMAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OSR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_OSR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn M10(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_M10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MAEN2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MAEN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MAEN1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MAEN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LPUART_BAUD {
    #[inline(always)]
    fn default() -> LPUART_BAUD {
        LPUART_BAUD(0)
    }
}
impl core::fmt::Debug for LPUART_BAUD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_BAUD")
            .field("SBR", &self.SBR())
            .field("SBNS", &self.SBNS())
            .field("RXEDGIE", &self.RXEDGIE())
            .field("LBKDIE", &self.LBKDIE())
            .field("RESYNCDIS", &self.RESYNCDIS())
            .field("BOTHEDGE", &self.BOTHEDGE())
            .field("MATCFG", &self.MATCFG())
            .field("RIDMAE", &self.RIDMAE())
            .field("RDMAE", &self.RDMAE())
            .field("TDMAE", &self.TDMAE())
            .field("OSR", &self.OSR())
            .field("M10", &self.M10())
            .field("MAEN2", &self.MAEN2())
            .field("MAEN1", &self.MAEN1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_BAUD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_BAUD {{ SBR: {=u16:?}, SBNS: {=bool:?}, RXEDGIE: {=bool:?}, LBKDIE: {=bool:?}, RESYNCDIS: {=bool:?}, BOTHEDGE: {=bool:?}, MATCFG: {=u8:?}, RIDMAE: {=bool:?}, RDMAE: {=bool:?}, TDMAE: {=bool:?}, OSR: {=u8:?}, M10: {=bool:?}, MAEN2: {=bool:?}, MAEN1: {=bool:?} }}",
            self.SBR(),
            self.SBNS(),
            self.RXEDGIE(),
            self.LBKDIE(),
            self.RESYNCDIS(),
            self.BOTHEDGE(),
            self.MATCFG(),
            self.RIDMAE(),
            self.RDMAE(),
            self.TDMAE(),
            self.OSR(),
            self.M10(),
            self.MAEN2(),
            self.MAEN1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_CTRL(pub u32);
impl LPUART_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn PT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ILT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ILT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAKE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAKE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn M(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_M(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSRC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOZEEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LOOPS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LOOPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IDLECFG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IDLECFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn M7(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_M7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MA2IE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MA2IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MA1IE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MA1IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SBK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SBK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RWU(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RWU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ILIE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ILIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TCIE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PEIE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FEIE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NEIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ORIE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ORIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXINV(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXDIR(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXDIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn R9T8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[inline(always)]
    pub const fn set_R9T8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn R8T9(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[inline(always)]
    pub const fn set_R8T9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LPUART_CTRL {
    #[inline(always)]
    fn default() -> LPUART_CTRL {
        LPUART_CTRL(0)
    }
}
impl core::fmt::Debug for LPUART_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_CTRL")
            .field("PT", &self.PT())
            .field("PE", &self.PE())
            .field("ILT", &self.ILT())
            .field("WAKE", &self.WAKE())
            .field("M", &self.M())
            .field("RSRC", &self.RSRC())
            .field("DOZEEN", &self.DOZEEN())
            .field("LOOPS", &self.LOOPS())
            .field("IDLECFG", &self.IDLECFG())
            .field("M7", &self.M7())
            .field("MA2IE", &self.MA2IE())
            .field("MA1IE", &self.MA1IE())
            .field("SBK", &self.SBK())
            .field("RWU", &self.RWU())
            .field("RE", &self.RE())
            .field("TE", &self.TE())
            .field("ILIE", &self.ILIE())
            .field("RIE", &self.RIE())
            .field("TCIE", &self.TCIE())
            .field("TIE", &self.TIE())
            .field("PEIE", &self.PEIE())
            .field("FEIE", &self.FEIE())
            .field("NEIE", &self.NEIE())
            .field("ORIE", &self.ORIE())
            .field("TXINV", &self.TXINV())
            .field("TXDIR", &self.TXDIR())
            .field("R9T8", &self.R9T8())
            .field("R8T9", &self.R8T9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_CTRL {{ PT: {=bool:?}, PE: {=bool:?}, ILT: {=bool:?}, WAKE: {=bool:?}, M: {=bool:?}, RSRC: {=bool:?}, DOZEEN: {=bool:?}, LOOPS: {=bool:?}, IDLECFG: {=u8:?}, M7: {=bool:?}, MA2IE: {=bool:?}, MA1IE: {=bool:?}, SBK: {=bool:?}, RWU: {=bool:?}, RE: {=bool:?}, TE: {=bool:?}, ILIE: {=bool:?}, RIE: {=bool:?}, TCIE: {=bool:?}, TIE: {=bool:?}, PEIE: {=bool:?}, FEIE: {=bool:?}, NEIE: {=bool:?}, ORIE: {=bool:?}, TXINV: {=bool:?}, TXDIR: {=bool:?}, R9T8: {=bool:?}, R8T9: {=bool:?} }}",
            self.PT(),
            self.PE(),
            self.ILT(),
            self.WAKE(),
            self.M(),
            self.RSRC(),
            self.DOZEEN(),
            self.LOOPS(),
            self.IDLECFG(),
            self.M7(),
            self.MA2IE(),
            self.MA1IE(),
            self.SBK(),
            self.RWU(),
            self.RE(),
            self.TE(),
            self.ILIE(),
            self.RIE(),
            self.TCIE(),
            self.TIE(),
            self.PEIE(),
            self.FEIE(),
            self.NEIE(),
            self.ORIE(),
            self.TXINV(),
            self.TXDIR(),
            self.R9T8(),
            self.R8T9()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_DATA(pub u32);
impl LPUART_DATA {
    #[must_use]
    #[inline(always)]
    pub const fn R0T0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_R0T0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn R1T1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    #[inline(always)]
    pub const fn set_R1T1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn R2T2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    #[inline(always)]
    pub const fn set_R2T2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn R3T3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    #[inline(always)]
    pub const fn set_R3T3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn R4T4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    #[inline(always)]
    pub const fn set_R4T4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn R5T5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    #[inline(always)]
    pub const fn set_R5T5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn R6T6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    #[inline(always)]
    pub const fn set_R6T6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn R7T7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    #[inline(always)]
    pub const fn set_R7T7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn R8T8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    #[inline(always)]
    pub const fn set_R8T8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn R9T9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    #[inline(always)]
    pub const fn set_R9T9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn LINBRK(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    #[inline(always)]
    pub const fn set_LINBRK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IDLINE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IDLINE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPT(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEMPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FRETSC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FRETSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PARITYE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PARITYE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOISY(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOISY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for LPUART_DATA {
    #[inline(always)]
    fn default() -> LPUART_DATA {
        LPUART_DATA(0)
    }
}
impl core::fmt::Debug for LPUART_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_DATA")
            .field("R0T0", &self.R0T0())
            .field("R1T1", &self.R1T1())
            .field("R2T2", &self.R2T2())
            .field("R3T3", &self.R3T3())
            .field("R4T4", &self.R4T4())
            .field("R5T5", &self.R5T5())
            .field("R6T6", &self.R6T6())
            .field("R7T7", &self.R7T7())
            .field("R8T8", &self.R8T8())
            .field("R9T9", &self.R9T9())
            .field("LINBRK", &self.LINBRK())
            .field("IDLINE", &self.IDLINE())
            .field("RXEMPT", &self.RXEMPT())
            .field("FRETSC", &self.FRETSC())
            .field("PARITYE", &self.PARITYE())
            .field("NOISY", &self.NOISY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_DATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_DATA {{ R0T0: {=bool:?}, R1T1: {=bool:?}, R2T2: {=bool:?}, R3T3: {=bool:?}, R4T4: {=bool:?}, R5T5: {=bool:?}, R6T6: {=bool:?}, R7T7: {=bool:?}, R8T8: {=bool:?}, R9T9: {=bool:?}, LINBRK: {=bool:?}, IDLINE: {=bool:?}, RXEMPT: {=bool:?}, FRETSC: {=bool:?}, PARITYE: {=bool:?}, NOISY: {=bool:?} }}",
            self.R0T0(),
            self.R1T1(),
            self.R2T2(),
            self.R3T3(),
            self.R4T4(),
            self.R5T5(),
            self.R6T6(),
            self.R7T7(),
            self.R8T8(),
            self.R9T9(),
            self.LINBRK(),
            self.IDLINE(),
            self.RXEMPT(),
            self.FRETSC(),
            self.PARITYE(),
            self.NOISY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_DATARO(pub u32);
impl LPUART_DATARO {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPUART_DATARO {
    #[inline(always)]
    fn default() -> LPUART_DATARO {
        LPUART_DATARO(0)
    }
}
impl core::fmt::Debug for LPUART_DATARO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_DATARO")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_DATARO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_DATARO {{ DATA: {=u16:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_FIFO(pub u32);
impl LPUART_FIFO {
    #[must_use]
    #[inline(always)]
    pub const fn RXFIFOSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXFIFOSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXFE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXFIFOSIZE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TXFIFOSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXFE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXUFE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXUFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXOFE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXOFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXIDEN(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXIDEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXFLUSH(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXFLUSH(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXUF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXOF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPT(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEMPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXEMPT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXEMPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for LPUART_FIFO {
    #[inline(always)]
    fn default() -> LPUART_FIFO {
        LPUART_FIFO(0)
    }
}
impl core::fmt::Debug for LPUART_FIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_FIFO")
            .field("RXFIFOSIZE", &self.RXFIFOSIZE())
            .field("RXFE", &self.RXFE())
            .field("TXFIFOSIZE", &self.TXFIFOSIZE())
            .field("TXFE", &self.TXFE())
            .field("RXUFE", &self.RXUFE())
            .field("TXOFE", &self.TXOFE())
            .field("RXIDEN", &self.RXIDEN())
            .field("RXFLUSH", &self.RXFLUSH())
            .field("TXFLUSH", &self.TXFLUSH())
            .field("RXUF", &self.RXUF())
            .field("TXOF", &self.TXOF())
            .field("RXEMPT", &self.RXEMPT())
            .field("TXEMPT", &self.TXEMPT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_FIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_FIFO {{ RXFIFOSIZE: {=u8:?}, RXFE: {=bool:?}, TXFIFOSIZE: {=u8:?}, TXFE: {=bool:?}, RXUFE: {=bool:?}, TXOFE: {=bool:?}, RXIDEN: {=u8:?}, RXFLUSH: {=bool:?}, TXFLUSH: {=bool:?}, RXUF: {=bool:?}, TXOF: {=bool:?}, RXEMPT: {=bool:?}, TXEMPT: {=bool:?} }}",
            self.RXFIFOSIZE(),
            self.RXFE(),
            self.TXFIFOSIZE(),
            self.TXFE(),
            self.RXUFE(),
            self.TXOFE(),
            self.RXIDEN(),
            self.RXFLUSH(),
            self.TXFLUSH(),
            self.RXUF(),
            self.TXOF(),
            self.RXEMPT(),
            self.TXEMPT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_GLOBAL(pub u32);
impl LPUART_GLOBAL {
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
impl Default for LPUART_GLOBAL {
    #[inline(always)]
    fn default() -> LPUART_GLOBAL {
        LPUART_GLOBAL(0)
    }
}
impl core::fmt::Debug for LPUART_GLOBAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_GLOBAL")
            .field("RST", &self.RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_GLOBAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_GLOBAL {{ RST: {=bool:?} }}", self.RST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_HDCR(pub u32);
impl LPUART_HDCR {
    #[must_use]
    #[inline(always)]
    pub const fn TXSTALL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXSEL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXWRMSK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXWRMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXMSK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RTS Extended"]
    #[must_use]
    #[inline(always)]
    pub const fn RTSEXT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RTS Extended"]
    #[inline(always)]
    pub const fn set_RTSEXT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for LPUART_HDCR {
    #[inline(always)]
    fn default() -> LPUART_HDCR {
        LPUART_HDCR(0)
    }
}
impl core::fmt::Debug for LPUART_HDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_HDCR")
            .field("TXSTALL", &self.TXSTALL())
            .field("RXSEL", &self.RXSEL())
            .field("RXWRMSK", &self.RXWRMSK())
            .field("RXMSK", &self.RXMSK())
            .field("RTSEXT", &self.RTSEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_HDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_HDCR {{ TXSTALL: {=bool:?}, RXSEL: {=bool:?}, RXWRMSK: {=bool:?}, RXMSK: {=bool:?}, RTSEXT: {=u8:?} }}",
            self.TXSTALL(),
            self.RXSEL(),
            self.RXWRMSK(),
            self.RXMSK(),
            self.RTSEXT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_MATCH(pub u32);
impl LPUART_MATCH {
    #[doc = "Match Address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn MA1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 1"]
    #[inline(always)]
    pub const fn set_MA1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Match Address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn MA2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 1"]
    #[inline(always)]
    pub const fn set_MA2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for LPUART_MATCH {
    #[inline(always)]
    fn default() -> LPUART_MATCH {
        LPUART_MATCH(0)
    }
}
impl core::fmt::Debug for LPUART_MATCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_MATCH")
            .field("MA1", &self.MA1())
            .field("MA2", &self.MA2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_MATCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_MATCH {{ MA1: {=u16:?}, MA2: {=u16:?} }}",
            self.MA1(),
            self.MA2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_MCR(pub u32);
impl LPUART_MCR {
    #[must_use]
    #[inline(always)]
    pub const fn CTS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DSR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DCD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DCD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DTR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RTS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for LPUART_MCR {
    #[inline(always)]
    fn default() -> LPUART_MCR {
        LPUART_MCR(0)
    }
}
impl core::fmt::Debug for LPUART_MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_MCR")
            .field("CTS", &self.CTS())
            .field("DSR", &self.DSR())
            .field("RIN", &self.RIN())
            .field("DCD", &self.DCD())
            .field("DTR", &self.DTR())
            .field("RTS", &self.RTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_MCR {{ CTS: {=bool:?}, DSR: {=bool:?}, RIN: {=bool:?}, DCD: {=bool:?}, DTR: {=bool:?}, RTS: {=bool:?} }}",
            self.CTS(),
            self.DSR(),
            self.RIN(),
            self.DCD(),
            self.DTR(),
            self.RTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_MODIR(pub u32);
impl LPUART_MODIR {
    #[must_use]
    #[inline(always)]
    pub const fn TXCTSE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXCTSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXRTSE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXRTSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXRTSPOL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXRTSPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXRTSE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXRTSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXCTSC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXCTSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXCTSSRC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXCTSSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive RTS Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn RTSWATER(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive RTS Configuration"]
    #[inline(always)]
    pub const fn set_RTSWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Receive RTS Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn TNP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive RTS Configuration"]
    #[inline(always)]
    pub const fn set_TNP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IREN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for LPUART_MODIR {
    #[inline(always)]
    fn default() -> LPUART_MODIR {
        LPUART_MODIR(0)
    }
}
impl core::fmt::Debug for LPUART_MODIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_MODIR")
            .field("TXCTSE", &self.TXCTSE())
            .field("TXRTSE", &self.TXRTSE())
            .field("TXRTSPOL", &self.TXRTSPOL())
            .field("RXRTSE", &self.RXRTSE())
            .field("TXCTSC", &self.TXCTSC())
            .field("TXCTSSRC", &self.TXCTSSRC())
            .field("RTSWATER", &self.RTSWATER())
            .field("TNP", &self.TNP())
            .field("IREN", &self.IREN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_MODIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_MODIR {{ TXCTSE: {=bool:?}, TXRTSE: {=bool:?}, TXRTSPOL: {=bool:?}, RXRTSE: {=bool:?}, TXCTSC: {=bool:?}, TXCTSSRC: {=bool:?}, RTSWATER: {=u8:?}, TNP: {=u8:?}, IREN: {=bool:?} }}",
            self.TXCTSE(),
            self.TXRTSE(),
            self.TXRTSPOL(),
            self.RXRTSE(),
            self.TXCTSC(),
            self.TXCTSSRC(),
            self.RTSWATER(),
            self.TNP(),
            self.IREN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_MSR(pub u32);
impl LPUART_MSR {
    #[must_use]
    #[inline(always)]
    pub const fn DCTS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DCTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DDSR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DDSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DRI(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DRI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DDCD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DDCD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CTS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DSR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DCD(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DCD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for LPUART_MSR {
    #[inline(always)]
    fn default() -> LPUART_MSR {
        LPUART_MSR(0)
    }
}
impl core::fmt::Debug for LPUART_MSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_MSR")
            .field("DCTS", &self.DCTS())
            .field("DDSR", &self.DDSR())
            .field("DRI", &self.DRI())
            .field("DDCD", &self.DDCD())
            .field("CTS", &self.CTS())
            .field("DSR", &self.DSR())
            .field("RIN", &self.RIN())
            .field("DCD", &self.DCD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_MSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_MSR {{ DCTS: {=bool:?}, DDSR: {=bool:?}, DRI: {=bool:?}, DDCD: {=bool:?}, CTS: {=bool:?}, DSR: {=bool:?}, RIN: {=bool:?}, DCD: {=bool:?} }}",
            self.DCTS(),
            self.DDSR(),
            self.DRI(),
            self.DDCD(),
            self.CTS(),
            self.DSR(),
            self.RIN(),
            self.DCD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_PARAM(pub u32);
impl LPUART_PARAM {
    #[doc = "Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFIFO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_TXFIFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFIFO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_RXFIFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for LPUART_PARAM {
    #[inline(always)]
    fn default() -> LPUART_PARAM {
        LPUART_PARAM(0)
    }
}
impl core::fmt::Debug for LPUART_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_PARAM")
            .field("TXFIFO", &self.TXFIFO())
            .field("RXFIFO", &self.RXFIFO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_PARAM {{ TXFIFO: {=u8:?}, RXFIFO: {=u8:?} }}",
            self.TXFIFO(),
            self.RXFIFO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_PINCFG(pub u32);
impl LPUART_PINCFG {
    #[must_use]
    #[inline(always)]
    pub const fn TRGSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TRGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for LPUART_PINCFG {
    #[inline(always)]
    fn default() -> LPUART_PINCFG {
        LPUART_PINCFG(0)
    }
}
impl core::fmt::Debug for LPUART_PINCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_PINCFG")
            .field("TRGSEL", &self.TRGSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_PINCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_PINCFG {{ TRGSEL: {=u8:?} }}", self.TRGSEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_REIR(pub u32);
impl LPUART_REIR {
    #[doc = "Idle Time"]
    #[must_use]
    #[inline(always)]
    pub const fn IDTIME(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Idle Time"]
    #[inline(always)]
    pub const fn set_IDTIME(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for LPUART_REIR {
    #[inline(always)]
    fn default() -> LPUART_REIR {
        LPUART_REIR(0)
    }
}
impl core::fmt::Debug for LPUART_REIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_REIR")
            .field("IDTIME", &self.IDTIME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_REIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_REIR {{ IDTIME: {=u16:?} }}", self.IDTIME())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_STAT(pub u32);
impl LPUART_STAT {
    #[must_use]
    #[inline(always)]
    pub const fn LBKFE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LBKFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AME(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TSF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MA2F(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MA2F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MA1F(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MA1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OR(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IDLE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDRF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TC(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TDRE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RAF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LBKDE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LBKDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BRK13(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BRK13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RWUID(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RWUID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXINV(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSBF(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEDGIF(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEDGIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LBKDIF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LBKDIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LPUART_STAT {
    #[inline(always)]
    fn default() -> LPUART_STAT {
        LPUART_STAT(0)
    }
}
impl core::fmt::Debug for LPUART_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_STAT")
            .field("LBKFE", &self.LBKFE())
            .field("AME", &self.AME())
            .field("MSF", &self.MSF())
            .field("TSF", &self.TSF())
            .field("MA2F", &self.MA2F())
            .field("MA1F", &self.MA1F())
            .field("PF", &self.PF())
            .field("FE", &self.FE())
            .field("NF", &self.NF())
            .field("OR", &self.OR())
            .field("IDLE", &self.IDLE())
            .field("RDRF", &self.RDRF())
            .field("TC", &self.TC())
            .field("TDRE", &self.TDRE())
            .field("RAF", &self.RAF())
            .field("LBKDE", &self.LBKDE())
            .field("BRK13", &self.BRK13())
            .field("RWUID", &self.RWUID())
            .field("RXINV", &self.RXINV())
            .field("MSBF", &self.MSBF())
            .field("RXEDGIF", &self.RXEDGIF())
            .field("LBKDIF", &self.LBKDIF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_STAT {{ LBKFE: {=bool:?}, AME: {=bool:?}, MSF: {=bool:?}, TSF: {=bool:?}, MA2F: {=bool:?}, MA1F: {=bool:?}, PF: {=bool:?}, FE: {=bool:?}, NF: {=bool:?}, OR: {=bool:?}, IDLE: {=bool:?}, RDRF: {=bool:?}, TC: {=bool:?}, TDRE: {=bool:?}, RAF: {=bool:?}, LBKDE: {=bool:?}, BRK13: {=bool:?}, RWUID: {=bool:?}, RXINV: {=bool:?}, MSBF: {=bool:?}, RXEDGIF: {=bool:?}, LBKDIF: {=bool:?} }}",
            self.LBKFE(),
            self.AME(),
            self.MSF(),
            self.TSF(),
            self.MA2F(),
            self.MA1F(),
            self.PF(),
            self.FE(),
            self.NF(),
            self.OR(),
            self.IDLE(),
            self.RDRF(),
            self.TC(),
            self.TDRE(),
            self.RAF(),
            self.LBKDE(),
            self.BRK13(),
            self.RWUID(),
            self.RXINV(),
            self.MSBF(),
            self.RXEDGIF(),
            self.LBKDIF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TCBR(pub u32);
impl LPUART_TCBR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LPUART_TCBR {
    #[inline(always)]
    fn default() -> LPUART_TCBR {
        LPUART_TCBR(0)
    }
}
impl core::fmt::Debug for LPUART_TCBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TCBR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TCBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_TCBR {{ DATA: {=u16:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TDBR(pub u32);
impl LPUART_TDBR {
    #[doc = "Data0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data0"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data1"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data1"]
    #[inline(always)]
    pub const fn set_DATA2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data2"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data2"]
    #[inline(always)]
    pub const fn set_DATA3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPUART_TDBR {
    #[inline(always)]
    fn default() -> LPUART_TDBR {
        LPUART_TDBR(0)
    }
}
impl core::fmt::Debug for LPUART_TDBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TDBR")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .field("DATA2", &self.DATA2())
            .field("DATA3", &self.DATA3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TDBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_TDBR {{ DATA0: {=u8:?}, DATA1: {=u8:?}, DATA2: {=u8:?}, DATA3: {=u8:?} }}",
            self.DATA0(),
            self.DATA1(),
            self.DATA2(),
            self.DATA3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TEIR(pub u32);
impl LPUART_TEIR {
    #[doc = "Idle Time"]
    #[must_use]
    #[inline(always)]
    pub const fn IDTIME(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Idle Time"]
    #[inline(always)]
    pub const fn set_IDTIME(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for LPUART_TEIR {
    #[inline(always)]
    fn default() -> LPUART_TEIR {
        LPUART_TEIR(0)
    }
}
impl core::fmt::Debug for LPUART_TEIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TEIR")
            .field("IDTIME", &self.IDTIME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TEIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPUART_TEIR {{ IDTIME: {=u16:?} }}", self.IDTIME())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TIMEOUT(pub u32);
impl LPUART_TIMEOUT {
    #[doc = "Timeout Value"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Timeout Value"]
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Timeout Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Timeout Value"]
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for LPUART_TIMEOUT {
    #[inline(always)]
    fn default() -> LPUART_TIMEOUT {
        LPUART_TIMEOUT(0)
    }
}
impl core::fmt::Debug for LPUART_TIMEOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TIMEOUT")
            .field("TIMEOUT", &self.TIMEOUT())
            .field("CFG", &self.CFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TIMEOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_TIMEOUT {{ TIMEOUT: {=u16:?}, CFG: {=u8:?} }}",
            self.TIMEOUT(),
            self.CFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TOCR(pub u32);
impl LPUART_TOCR {
    #[doc = "Timeout Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TOEN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Enable"]
    #[inline(always)]
    pub const fn set_TOEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Timeout Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TOIE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Enable"]
    #[inline(always)]
    pub const fn set_TOIE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for LPUART_TOCR {
    #[inline(always)]
    fn default() -> LPUART_TOCR {
        LPUART_TOCR(0)
    }
}
impl core::fmt::Debug for LPUART_TOCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TOCR")
            .field("TOEN", &self.TOEN())
            .field("TOIE", &self.TOIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TOCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_TOCR {{ TOEN: {=u8:?}, TOIE: {=u8:?} }}",
            self.TOEN(),
            self.TOIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_TOSR(pub u32);
impl LPUART_TOSR {
    #[doc = "Timeout Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn TOZ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Zero"]
    #[inline(always)]
    pub const fn set_TOZ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Timeout Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn TOF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Zero"]
    #[inline(always)]
    pub const fn set_TOF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for LPUART_TOSR {
    #[inline(always)]
    fn default() -> LPUART_TOSR {
        LPUART_TOSR(0)
    }
}
impl core::fmt::Debug for LPUART_TOSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_TOSR")
            .field("TOZ", &self.TOZ())
            .field("TOF", &self.TOF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_TOSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_TOSR {{ TOZ: {=u8:?}, TOF: {=u8:?} }}",
            self.TOZ(),
            self.TOF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_VERID(pub u32);
impl LPUART_VERID {
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
impl Default for LPUART_VERID {
    #[inline(always)]
    fn default() -> LPUART_VERID {
        LPUART_VERID(0)
    }
}
impl core::fmt::Debug for LPUART_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPUART_WATER(pub u32);
impl LPUART_WATER {
    #[doc = "Transmit Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn TXWATER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit Watermark"]
    #[inline(always)]
    pub const fn set_TXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Transmit Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCOUNT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Watermark"]
    #[inline(always)]
    pub const fn set_TXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Transmit Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn RXWATER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit Counter"]
    #[inline(always)]
    pub const fn set_RXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Receive Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCOUNT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive Watermark"]
    #[inline(always)]
    pub const fn set_RXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for LPUART_WATER {
    #[inline(always)]
    fn default() -> LPUART_WATER {
        LPUART_WATER(0)
    }
}
impl core::fmt::Debug for LPUART_WATER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_WATER")
            .field("TXWATER", &self.TXWATER())
            .field("TXCOUNT", &self.TXCOUNT())
            .field("RXWATER", &self.RXWATER())
            .field("RXCOUNT", &self.RXCOUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPUART_WATER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPUART_WATER {{ TXWATER: {=u8:?}, TXCOUNT: {=u8:?}, RXWATER: {=u8:?}, RXCOUNT: {=u8:?} }}",
            self.TXWATER(),
            self.TXCOUNT(),
            self.RXWATER(),
            self.RXCOUNT()
        )
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}

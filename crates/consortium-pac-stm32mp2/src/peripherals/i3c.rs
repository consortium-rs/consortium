#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C {
    ptr: *mut u8,
}
unsafe impl Send for I3C {}
unsafe impl Sync for I3C {}
impl I3C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I3C message control register"]
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<I3C_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I3C configuration register"]
    #[inline(always)]
    pub const fn CFGR(self) -> crate::common::Reg<I3C_CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "I3C receive data byte register"]
    #[inline(always)]
    pub const fn RDR(self) -> crate::common::Reg<I3C_RDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "I3C receive data word register"]
    #[inline(always)]
    pub const fn RDWR(self) -> crate::common::Reg<I3C_RDWR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "I3C transmit data byte register"]
    #[inline(always)]
    pub const fn TDR(self) -> crate::common::Reg<I3C_TDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "I3C transmit data word register"]
    #[inline(always)]
    pub const fn TDWR(self) -> crate::common::Reg<I3C_TDWR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "I3C IBI payload data register"]
    #[inline(always)]
    pub const fn IBIDR(self) -> crate::common::Reg<I3C_IBIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "I3C target transmit configuration register"]
    #[inline(always)]
    pub const fn TGTTDR(self) -> crate::common::Reg<I3C_TGTTDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "I3C status register"]
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<I3C_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "I3C status error register"]
    #[inline(always)]
    pub const fn SER(self) -> crate::common::Reg<I3C_SER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "I3C received message register"]
    #[inline(always)]
    pub const fn RMR(self) -> crate::common::Reg<I3C_RMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "I3C event register"]
    #[inline(always)]
    pub const fn EVR(self) -> crate::common::Reg<I3C_EVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "I3C interrupt enable register"]
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<I3C_IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "I3C clear event register"]
    #[inline(always)]
    pub const fn CEVR(self) -> crate::common::Reg<I3C_CEVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "I3C own device characteristics register"]
    #[inline(always)]
    pub const fn DEVR0(self) -> crate::common::Reg<I3C_DEVR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "I3C device x (0<=x<=4) register"]
    #[inline(always)]
    pub const fn DEVRX(self, n: usize) -> crate::common::Reg<I3C_DEVRX, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "I3C maximum read length register"]
    #[inline(always)]
    pub const fn MAXRLR(self) -> crate::common::Reg<I3C_MAXRLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "I3C maximum write length register"]
    #[inline(always)]
    pub const fn MAXWLR(self) -> crate::common::Reg<I3C_MAXWLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "I3C timing register 0"]
    #[inline(always)]
    pub const fn TIMINGR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C timing register 1"]
    #[inline(always)]
    pub const fn TIMINGR1(self) -> crate::common::Reg<I3C_TIMINGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "I3C timing register 2"]
    #[inline(always)]
    pub const fn TIMINGR2(self) -> crate::common::Reg<I3C_TIMINGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "I3C bus characteristics register"]
    #[inline(always)]
    pub const fn BCR(self) -> crate::common::Reg<I3C_BCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "I3C device characteristics register"]
    #[inline(always)]
    pub const fn DCR(self) -> crate::common::Reg<I3C_DCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "I3C get capability register"]
    #[inline(always)]
    pub const fn GETCAPR(self) -> crate::common::Reg<I3C_GETCAPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "I3C controller-role capability register"]
    #[inline(always)]
    pub const fn CRCAPR(self) -> crate::common::Reg<I3C_CRCAPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "I3C get capability register"]
    #[inline(always)]
    pub const fn GETMXDSR(self) -> crate::common::Reg<I3C_GETMXDSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "I3C extended provisioned ID register"]
    #[inline(always)]
    pub const fn EPIDR(self) -> crate::common::Reg<I3C_EPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "I3C hardware configuration register"]
    #[inline(always)]
    pub const fn HWCFGR(self) -> crate::common::Reg<I3C_HWCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "I3C version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<I3C_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "I3C IP identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<I3C_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "I3C IP size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<I3C_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_BCR(pub u32);
impl I3C_BCR {
    #[doc = "Bus Characteristics"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bus Characteristics"]
    #[inline(always)]
    pub const fn set_BCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Max Data Speed Limitation"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Max Data Speed Limitation"]
    #[inline(always)]
    pub const fn set_BCR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IBI Request capable"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IBI Request capable"]
    #[inline(always)]
    pub const fn set_BCR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IBI Payload additional Mandatory Data Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IBI Payload additional Mandatory Data Byte"]
    #[inline(always)]
    pub const fn set_BCR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Device Role shared during Dynamic Address Assignment"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Device Role shared during Dynamic Address Assignment"]
    #[inline(always)]
    pub const fn set_BCR6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for I3C_BCR {
    #[inline(always)]
    fn default() -> I3C_BCR {
        I3C_BCR(0)
    }
}
impl core::fmt::Debug for I3C_BCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_BCR")
            .field("BCR", &self.BCR())
            .field("BCR0", &self.BCR0())
            .field("BCR1", &self.BCR1())
            .field("BCR2", &self.BCR2())
            .field("BCR6", &self.BCR6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_BCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_BCR {{ BCR: {=u8:?}, BCR0: {=bool:?}, BCR1: {=bool:?}, BCR2: {=bool:?}, BCR6: {=bool:?} }}",
            self.BCR(),
            self.BCR0(),
            self.BCR1(),
            self.BCR2(),
            self.BCR6()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_CEVR(pub u32);
impl I3C_CEVR {
    #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CFCF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "clear frame complete flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_CFCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRXTGTENDF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "clear target-initiated read end flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CRXTGTENDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "clear error flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CERRF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "clear error flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_CERRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "clear IBI request flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CIBIF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "clear IBI request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CIBIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "clear IBI end flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CIBIENDF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "clear IBI end flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CIBIENDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "clear controller-role request flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CCRF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "clear controller-role request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CCRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "clear controller-role update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CCRUPDF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "clear controller-role update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CCRUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "clear hot-join flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CHJF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "clear hot-join flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CHJF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "clear wakeup flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CWKPF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "clear wakeup flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CWKPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CGETF(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "clear GETxxx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CGETF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CSTAF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "clear GETSTATUS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CSTAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CDAUPDF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CDAUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CMWLUPDF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "clear SETMWL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CMWLUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CMRLUPDF(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "clear SETMRL CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CMRLUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "clear reset pattern flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRSTF(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "clear reset pattern flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CASUPDF(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clear ENTASx CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CASUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CINTUPDF(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "clear ENEC/DISEC CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CINTUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CDEFF(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "clear DEFTGTS CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CDEFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CGRPF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "clear DEFGRPA CCC flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CGRPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_CEVR {
    #[inline(always)]
    fn default() -> I3C_CEVR {
        I3C_CEVR(0)
    }
}
impl core::fmt::Debug for I3C_CEVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_CEVR")
            .field("CFCF", &self.CFCF())
            .field("CRXTGTENDF", &self.CRXTGTENDF())
            .field("CERRF", &self.CERRF())
            .field("CIBIF", &self.CIBIF())
            .field("CIBIENDF", &self.CIBIENDF())
            .field("CCRF", &self.CCRF())
            .field("CCRUPDF", &self.CCRUPDF())
            .field("CHJF", &self.CHJF())
            .field("CWKPF", &self.CWKPF())
            .field("CGETF", &self.CGETF())
            .field("CSTAF", &self.CSTAF())
            .field("CDAUPDF", &self.CDAUPDF())
            .field("CMWLUPDF", &self.CMWLUPDF())
            .field("CMRLUPDF", &self.CMRLUPDF())
            .field("CRSTF", &self.CRSTF())
            .field("CASUPDF", &self.CASUPDF())
            .field("CINTUPDF", &self.CINTUPDF())
            .field("CDEFF", &self.CDEFF())
            .field("CGRPF", &self.CGRPF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_CEVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_CEVR {{ CFCF: {=bool:?}, CRXTGTENDF: {=bool:?}, CERRF: {=bool:?}, CIBIF: {=bool:?}, CIBIENDF: {=bool:?}, CCRF: {=bool:?}, CCRUPDF: {=bool:?}, CHJF: {=bool:?}, CWKPF: {=bool:?}, CGETF: {=bool:?}, CSTAF: {=bool:?}, CDAUPDF: {=bool:?}, CMWLUPDF: {=bool:?}, CMRLUPDF: {=bool:?}, CRSTF: {=bool:?}, CASUPDF: {=bool:?}, CINTUPDF: {=bool:?}, CDEFF: {=bool:?}, CGRPF: {=bool:?} }}",
            self.CFCF(),
            self.CRXTGTENDF(),
            self.CERRF(),
            self.CIBIF(),
            self.CIBIENDF(),
            self.CCRF(),
            self.CCRUPDF(),
            self.CHJF(),
            self.CWKPF(),
            self.CGETF(),
            self.CSTAF(),
            self.CDAUPDF(),
            self.CMWLUPDF(),
            self.CMRLUPDF(),
            self.CRSTF(),
            self.CASUPDF(),
            self.CINTUPDF(),
            self.CDEFF(),
            self.CGRPF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_CFGR(pub u32);
impl I3C_CFGR {
    #[doc = "I3C enable (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C enable (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "initial controller/target role"]
    #[must_use]
    #[inline(always)]
    pub const fn CRINIT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "initial controller/target role"]
    #[inline(always)]
    pub const fn set_CRINIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no arbitrable header after a START (when I3C is acting as a controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn NOARBH(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no arbitrable header after a START (when I3C is acting as a controller)"]
    #[inline(always)]
    pub const fn set_NOARBH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HDR reset pattern enable (when I3C is acting as a controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTPTRN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HDR reset pattern enable (when I3C is acting as a controller)"]
    #[inline(always)]
    pub const fn set_RSTPTRN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn EXITPTRN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "HDR Exit Pattern enable (when I3C is acting as a controller)"]
    #[inline(always)]
    pub const fn set_EXITPTRN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn HKSDAEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "High-keeper enable on SDA line (when I3C is acting as a controller)"]
    #[inline(always)]
    pub const fn set_HKSDAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hot Join request acknowledge (when I3C is acting as a controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn HJACK(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hot Join request acknowledge (when I3C is acting as a controller)"]
    #[inline(always)]
    pub const fn set_HJACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO DMA request enable (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFLUSH(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO flush (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXTHRES(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO threshold (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXTHRES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO DMA request enable (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFLUSH(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO flush (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXTHRES(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO threshold (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXTHRES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "S-FIFO DMA request enable (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMAEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "S-FIFO DMA request enable (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_SDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "S-FIFO flush (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn SFLUSH(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "S-FIFO flush (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_SFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn SMODE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "S-FIFO enable / status receive mode (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_SMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "transmit mode (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn TMODE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "transmit mode (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_TMODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "C-FIFO DMA request enable (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CDMAEN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO DMA request enable (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "C-FIFO flush (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CFLUSH(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO flush (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CFLUSH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "frame transfer set (a"]
    #[must_use]
    #[inline(always)]
    pub const fn TSFSET(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "frame transfer set (a"]
    #[inline(always)]
    pub const fn set_TSFSET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for I3C_CFGR {
    #[inline(always)]
    fn default() -> I3C_CFGR {
        I3C_CFGR(0)
    }
}
impl core::fmt::Debug for I3C_CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_CFGR")
            .field("EN", &self.EN())
            .field("CRINIT", &self.CRINIT())
            .field("NOARBH", &self.NOARBH())
            .field("RSTPTRN", &self.RSTPTRN())
            .field("EXITPTRN", &self.EXITPTRN())
            .field("HKSDAEN", &self.HKSDAEN())
            .field("HJACK", &self.HJACK())
            .field("RXDMAEN", &self.RXDMAEN())
            .field("RXFLUSH", &self.RXFLUSH())
            .field("RXTHRES", &self.RXTHRES())
            .field("TXDMAEN", &self.TXDMAEN())
            .field("TXFLUSH", &self.TXFLUSH())
            .field("TXTHRES", &self.TXTHRES())
            .field("SDMAEN", &self.SDMAEN())
            .field("SFLUSH", &self.SFLUSH())
            .field("SMODE", &self.SMODE())
            .field("TMODE", &self.TMODE())
            .field("CDMAEN", &self.CDMAEN())
            .field("CFLUSH", &self.CFLUSH())
            .field("TSFSET", &self.TSFSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_CFGR {{ EN: {=bool:?}, CRINIT: {=bool:?}, NOARBH: {=bool:?}, RSTPTRN: {=bool:?}, EXITPTRN: {=bool:?}, HKSDAEN: {=bool:?}, HJACK: {=bool:?}, RXDMAEN: {=bool:?}, RXFLUSH: {=bool:?}, RXTHRES: {=bool:?}, TXDMAEN: {=bool:?}, TXFLUSH: {=bool:?}, TXTHRES: {=bool:?}, SDMAEN: {=bool:?}, SFLUSH: {=bool:?}, SMODE: {=bool:?}, TMODE: {=bool:?}, CDMAEN: {=bool:?}, CFLUSH: {=bool:?}, TSFSET: {=bool:?} }}",
            self.EN(),
            self.CRINIT(),
            self.NOARBH(),
            self.RSTPTRN(),
            self.EXITPTRN(),
            self.HKSDAEN(),
            self.HJACK(),
            self.RXDMAEN(),
            self.RXFLUSH(),
            self.RXTHRES(),
            self.TXDMAEN(),
            self.TXFLUSH(),
            self.TXTHRES(),
            self.SDMAEN(),
            self.SFLUSH(),
            self.SMODE(),
            self.TMODE(),
            self.CDMAEN(),
            self.CFLUSH(),
            self.TSFSET()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_CR(pub u32);
impl I3C_CR {
    #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as contr"]
    #[must_use]
    #[inline(always)]
    pub const fn DCNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "count of data to transfer during a read or write message, in bytes (whatever I3C is acting as contr"]
    #[inline(always)]
    pub const fn set_DCNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "8-Bit CCC code"]
    #[must_use]
    #[inline(always)]
    pub const fn CCC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "8-Bit CCC code"]
    #[inline(always)]
    pub const fn set_CCC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "read / non-write message (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn RNW(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "read / non-write message (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_RNW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn ADD(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "7-bit I3C dynamic / I2C static target address (when I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_ADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
    #[doc = "message type (whatever I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MTYPE(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x0f;
        val as u8
    }
    #[doc = "message type (whatever I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_MTYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
    }
    #[doc = "message end type (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn MEND(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "message end type (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_MEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_CR {
    #[inline(always)]
    fn default() -> I3C_CR {
        I3C_CR(0)
    }
}
impl core::fmt::Debug for I3C_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_CR")
            .field("DCNT", &self.DCNT())
            .field("CCC", &self.CCC())
            .field("RNW", &self.RNW())
            .field("ADD", &self.ADD())
            .field("MTYPE", &self.MTYPE())
            .field("MEND", &self.MEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_CR {{ DCNT: {=u16:?}, CCC: {=u8:?}, RNW: {=bool:?}, ADD: {=u8:?}, MTYPE: {=u8:?}, MEND: {=bool:?} }}",
            self.DCNT(),
            self.CCC(),
            self.RNW(),
            self.ADD(),
            self.MTYPE(),
            self.MEND()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_CRCAPR(pub u32);
impl I3C_CRCAPR {
    #[doc = "delayed controller-role hand-off"]
    #[must_use]
    #[inline(always)]
    pub const fn CAPDHOFF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "delayed controller-role hand-off"]
    #[inline(always)]
    pub const fn set_CAPDHOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "group management support (when acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CAPGRP(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "group management support (when acting as controller)"]
    #[inline(always)]
    pub const fn set_CAPGRP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for I3C_CRCAPR {
    #[inline(always)]
    fn default() -> I3C_CRCAPR {
        I3C_CRCAPR(0)
    }
}
impl core::fmt::Debug for I3C_CRCAPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_CRCAPR")
            .field("CAPDHOFF", &self.CAPDHOFF())
            .field("CAPGRP", &self.CAPGRP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_CRCAPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_CRCAPR {{ CAPDHOFF: {=bool:?}, CAPGRP: {=bool:?} }}",
            self.CAPDHOFF(),
            self.CAPGRP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_DCR(pub u32);
impl I3C_DCR {
    #[doc = "device characteristics ID"]
    #[must_use]
    #[inline(always)]
    pub const fn DCR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "device characteristics ID"]
    #[inline(always)]
    pub const fn set_DCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_DCR {
    #[inline(always)]
    fn default() -> I3C_DCR {
        I3C_DCR(0)
    }
}
impl core::fmt::Debug for I3C_DCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_DCR").field("DCR", &self.DCR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_DCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_DCR {{ DCR: {=u8:?} }}", self.DCR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_DEVR0(pub u32);
impl I3C_DEVR0 {
    #[doc = "dynamic address is valid (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn DAVAL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dynamic address is valid (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_DAVAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7-bit dynamic address"]
    #[must_use]
    #[inline(always)]
    pub const fn DA(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "7-bit dynamic address"]
    #[inline(always)]
    pub const fn set_DA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "IBI request enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IBI request enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_IBIEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "controller-role request enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CREN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role request enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "hot-join request enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn HJEN(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "hot-join request enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_HJEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "activity state (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn AS(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "activity state (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_AS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "reset action/level on received reset pattern (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTACT(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "reset action/level on received reset pattern (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_RSTACT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "reset action is valid (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTVAL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "reset action is valid (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_RSTVAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for I3C_DEVR0 {
    #[inline(always)]
    fn default() -> I3C_DEVR0 {
        I3C_DEVR0(0)
    }
}
impl core::fmt::Debug for I3C_DEVR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_DEVR0")
            .field("DAVAL", &self.DAVAL())
            .field("DA", &self.DA())
            .field("IBIEN", &self.IBIEN())
            .field("CREN", &self.CREN())
            .field("HJEN", &self.HJEN())
            .field("AS", &self.AS())
            .field("RSTACT", &self.RSTACT())
            .field("RSTVAL", &self.RSTVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_DEVR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_DEVR0 {{ DAVAL: {=bool:?}, DA: {=u8:?}, IBIEN: {=bool:?}, CREN: {=bool:?}, HJEN: {=bool:?}, AS: {=u8:?}, RSTACT: {=u8:?}, RSTVAL: {=bool:?} }}",
            self.DAVAL(),
            self.DA(),
            self.IBIEN(),
            self.CREN(),
            self.HJEN(),
            self.AS(),
            self.RSTACT(),
            self.RSTVAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_DEVRX(pub u32);
impl I3C_DEVRX {
    #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn DA(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "assigned I3C dynamic address to target x (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_DA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "IBI request acknowledge (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIACK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IBI request acknowledge (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_IBIACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "controller-role request acknowledge (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRACK(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role request acknowledge (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CRACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "IBI data enable (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDEN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IBI data enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_IBIDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn SUSP(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "suspend/stop I3C transfer on received IBI (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_SUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DA\\[6:0\\] write disabled (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn DIS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DA\\[6:0\\] write disabled (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_DIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_DEVRX {
    #[inline(always)]
    fn default() -> I3C_DEVRX {
        I3C_DEVRX(0)
    }
}
impl core::fmt::Debug for I3C_DEVRX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_DEVRX")
            .field("DA", &self.DA())
            .field("IBIACK", &self.IBIACK())
            .field("CRACK", &self.CRACK())
            .field("IBIDEN", &self.IBIDEN())
            .field("SUSP", &self.SUSP())
            .field("DIS", &self.DIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_DEVRX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_DEVRX {{ DA: {=u8:?}, IBIACK: {=bool:?}, CRACK: {=bool:?}, IBIDEN: {=bool:?}, SUSP: {=bool:?}, DIS: {=bool:?} }}",
            self.DA(),
            self.IBIACK(),
            self.CRACK(),
            self.IBIDEN(),
            self.SUSP(),
            self.DIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_EPIDR(pub u32);
impl I3C_EPIDR {
    #[doc = "4-bit MIPI Instance ID"]
    #[must_use]
    #[inline(always)]
    pub const fn MIPIID(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "4-bit MIPI Instance ID"]
    #[inline(always)]
    pub const fn set_MIPIID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "provisioned ID type selector"]
    #[must_use]
    #[inline(always)]
    pub const fn IDTSEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "provisioned ID type selector"]
    #[inline(always)]
    pub const fn set_IDTSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "15-bit MIPI manufacturer ID"]
    #[must_use]
    #[inline(always)]
    pub const fn MIPIMID(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "15-bit MIPI manufacturer ID"]
    #[inline(always)]
    pub const fn set_MIPIMID(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for I3C_EPIDR {
    #[inline(always)]
    fn default() -> I3C_EPIDR {
        I3C_EPIDR(0)
    }
}
impl core::fmt::Debug for I3C_EPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_EPIDR")
            .field("MIPIID", &self.MIPIID())
            .field("IDTSEL", &self.IDTSEL())
            .field("MIPIMID", &self.MIPIMID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_EPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_EPIDR {{ MIPIID: {=u8:?}, IDTSEL: {=bool:?}, MIPIMID: {=u16:?} }}",
            self.MIPIID(),
            self.IDTSEL(),
            self.MIPIMID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_EVR(pub u32);
impl I3C_EVR {
    #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO empty flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_CFEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFEF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO empty flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXFEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "C-FIFO not full flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CFNFF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO not full flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CFNFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "S-FIFO not empty flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn SFNEF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "S-FIFO not empty flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_SFNEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFNFF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO not full flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXFNFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFNEF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO not empty flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXFNEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXLASTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "last written data byte/word flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXLASTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXLASTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "last read data byte/word flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXLASTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "frame complete flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn FCF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "frame complete flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_FCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "target-initiated read end flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXTGTENDF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "target-initiated read end flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_RXTGTENDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "flag (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "flag (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_ERRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IBI flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IBI flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_IBIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "IBI end flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIENDF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IBI end flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_IBIENDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "controller-role request flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role request flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "controller-role update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRUPDF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CRUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "hot-join flag (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn HJF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "hot-join flag (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_HJF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "wakeup/missed start flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn WKPF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "wakeup/missed start flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_WKPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "get flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn GETF(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "get flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_GETF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "get status flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn STAF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "get status flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_STAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "dynamic address update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn DAUPDF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "dynamic address update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_DAUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "maximum write length update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MWLUPDF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "maximum write length update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MWLUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "maximum read length update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MRLUPDF(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "maximum read length update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MRLUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "reset pattern flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTF(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "reset pattern flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "activity state update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn ASUPDF(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "activity state update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_ASUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn INTUPDF(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt/controller-role/hot-join update flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_INTUPDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DEFTGTS flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn DEFF(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DEFTGTS flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_DEFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "group addressing flag (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn GRPF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "group addressing flag (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_GRPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_EVR {
    #[inline(always)]
    fn default() -> I3C_EVR {
        I3C_EVR(0)
    }
}
impl core::fmt::Debug for I3C_EVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_EVR")
            .field("CFEF", &self.CFEF())
            .field("TXFEF", &self.TXFEF())
            .field("CFNFF", &self.CFNFF())
            .field("SFNEF", &self.SFNEF())
            .field("TXFNFF", &self.TXFNFF())
            .field("RXFNEF", &self.RXFNEF())
            .field("TXLASTF", &self.TXLASTF())
            .field("RXLASTF", &self.RXLASTF())
            .field("FCF", &self.FCF())
            .field("RXTGTENDF", &self.RXTGTENDF())
            .field("ERRF", &self.ERRF())
            .field("IBIF", &self.IBIF())
            .field("IBIENDF", &self.IBIENDF())
            .field("CRF", &self.CRF())
            .field("CRUPDF", &self.CRUPDF())
            .field("HJF", &self.HJF())
            .field("WKPF", &self.WKPF())
            .field("GETF", &self.GETF())
            .field("STAF", &self.STAF())
            .field("DAUPDF", &self.DAUPDF())
            .field("MWLUPDF", &self.MWLUPDF())
            .field("MRLUPDF", &self.MRLUPDF())
            .field("RSTF", &self.RSTF())
            .field("ASUPDF", &self.ASUPDF())
            .field("INTUPDF", &self.INTUPDF())
            .field("DEFF", &self.DEFF())
            .field("GRPF", &self.GRPF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_EVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_EVR {{ CFEF: {=bool:?}, TXFEF: {=bool:?}, CFNFF: {=bool:?}, SFNEF: {=bool:?}, TXFNFF: {=bool:?}, RXFNEF: {=bool:?}, TXLASTF: {=bool:?}, RXLASTF: {=bool:?}, FCF: {=bool:?}, RXTGTENDF: {=bool:?}, ERRF: {=bool:?}, IBIF: {=bool:?}, IBIENDF: {=bool:?}, CRF: {=bool:?}, CRUPDF: {=bool:?}, HJF: {=bool:?}, WKPF: {=bool:?}, GETF: {=bool:?}, STAF: {=bool:?}, DAUPDF: {=bool:?}, MWLUPDF: {=bool:?}, MRLUPDF: {=bool:?}, RSTF: {=bool:?}, ASUPDF: {=bool:?}, INTUPDF: {=bool:?}, DEFF: {=bool:?}, GRPF: {=bool:?} }}",
            self.CFEF(),
            self.TXFEF(),
            self.CFNFF(),
            self.SFNEF(),
            self.TXFNFF(),
            self.RXFNEF(),
            self.TXLASTF(),
            self.RXLASTF(),
            self.FCF(),
            self.RXTGTENDF(),
            self.ERRF(),
            self.IBIF(),
            self.IBIENDF(),
            self.CRF(),
            self.CRUPDF(),
            self.HJF(),
            self.WKPF(),
            self.GETF(),
            self.STAF(),
            self.DAUPDF(),
            self.MWLUPDF(),
            self.MRLUPDF(),
            self.RSTF(),
            self.ASUPDF(),
            self.INTUPDF(),
            self.DEFF(),
            self.GRPF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_GETCAPR(pub u32);
impl I3C_GETCAPR {
    #[doc = "IBI MDB support for pending read notification"]
    #[must_use]
    #[inline(always)]
    pub const fn CAPPEND(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "IBI MDB support for pending read notification"]
    #[inline(always)]
    pub const fn set_CAPPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for I3C_GETCAPR {
    #[inline(always)]
    fn default() -> I3C_GETCAPR {
        I3C_GETCAPR(0)
    }
}
impl core::fmt::Debug for I3C_GETCAPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_GETCAPR")
            .field("CAPPEND", &self.CAPPEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_GETCAPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_GETCAPR {{ CAPPEND: {=bool:?} }}", self.CAPPEND())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_GETMXDSR(pub u32);
impl I3C_GETMXDSR {
    #[doc = "controller hand-off activity state"]
    #[must_use]
    #[inline(always)]
    pub const fn HOFFAS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "controller hand-off activity state"]
    #[inline(always)]
    pub const fn set_HOFFAS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "GETMXDS CCC format"]
    #[must_use]
    #[inline(always)]
    pub const fn FMT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "GETMXDS CCC format"]
    #[inline(always)]
    pub const fn set_FMT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)"]
    #[must_use]
    #[inline(always)]
    pub const fn RDTURN(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)"]
    #[inline(always)]
    pub const fn set_RDTURN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "clock-to-data turnaround time (tSCO)"]
    #[must_use]
    #[inline(always)]
    pub const fn TSCO(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "clock-to-data turnaround time (tSCO)"]
    #[inline(always)]
    pub const fn set_TSCO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for I3C_GETMXDSR {
    #[inline(always)]
    fn default() -> I3C_GETMXDSR {
        I3C_GETMXDSR(0)
    }
}
impl core::fmt::Debug for I3C_GETMXDSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_GETMXDSR")
            .field("HOFFAS", &self.HOFFAS())
            .field("FMT", &self.FMT())
            .field("RDTURN", &self.RDTURN())
            .field("TSCO", &self.TSCO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_GETMXDSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_GETMXDSR {{ HOFFAS: {=u8:?}, FMT: {=u8:?}, RDTURN: {=u8:?}, TSCO: {=bool:?} }}",
            self.HOFFAS(),
            self.FMT(),
            self.RDTURN(),
            self.TSCO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_HWCFGR(pub u32);
impl I3C_HWCFGR {
    #[doc = "C-FIFO size, in multiple of (32-bit) words"]
    #[must_use]
    #[inline(always)]
    pub const fn CSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "C-FIFO size, in multiple of (32-bit) words"]
    #[inline(always)]
    pub const fn set_CSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "S-FIFO size, in multiple of (32-bit) words"]
    #[must_use]
    #[inline(always)]
    pub const fn SSIZE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "S-FIFO size, in multiple of (32-bit) words"]
    #[inline(always)]
    pub const fn set_SSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "TX-FIFO size, in multiple of 4 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn TSIZE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "TX-FIFO size, in multiple of 4 bytes"]
    #[inline(always)]
    pub const fn set_TSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "RX-FIFO size, in multiple of 4 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn RSIZE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "RX-FIFO size, in multiple of 4 bytes"]
    #[inline(always)]
    pub const fn set_RSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "number of targets whose the IBI/controller-role request can be managed in parallel by this I3C IP,"]
    #[must_use]
    #[inline(always)]
    pub const fn NBT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "number of targets whose the IBI/controller-role request can be managed in parallel by this I3C IP,"]
    #[inline(always)]
    pub const fn set_NBT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for I3C_HWCFGR {
    #[inline(always)]
    fn default() -> I3C_HWCFGR {
        I3C_HWCFGR(0)
    }
}
impl core::fmt::Debug for I3C_HWCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_HWCFGR")
            .field("CSIZE", &self.CSIZE())
            .field("SSIZE", &self.SSIZE())
            .field("TSIZE", &self.TSIZE())
            .field("RSIZE", &self.RSIZE())
            .field("NBT", &self.NBT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_HWCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_HWCFGR {{ CSIZE: {=u8:?}, SSIZE: {=u8:?}, TSIZE: {=u8:?}, RSIZE: {=u8:?}, NBT: {=u8:?} }}",
            self.CSIZE(),
            self.SSIZE(),
            self.TSIZE(),
            self.RSIZE(),
            self.NBT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_IBIDR(pub u32);
impl I3C_IBIDR {
    #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDB0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit IBI payload data (earliest byte on I3C bus, i"]
    #[inline(always)]
    pub const fn set_IBIDB0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "32-bit IBI payload data"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDBx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit IBI payload data"]
    #[inline(always)]
    pub const fn set_IBIDBx(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
    #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDB1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB0\\[7:0\\])"]
    #[inline(always)]
    pub const fn set_IBIDB1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDB2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit IBI payload data (next byte on I3C bus after IBIDB1\\[7:0\\])"]
    #[inline(always)]
    pub const fn set_IBIDB2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "8-bit IBI payload data (latest byte on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDB3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit IBI payload data (latest byte on I3C bus)"]
    #[inline(always)]
    pub const fn set_IBIDB3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_IBIDR {
    #[inline(always)]
    fn default() -> I3C_IBIDR {
        I3C_IBIDR(0)
    }
}
impl core::fmt::Debug for I3C_IBIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_IBIDR")
            .field("IBIDB0", &self.IBIDB0())
            .field("IBIDBx", &self.IBIDBx())
            .field("IBIDB1", &self.IBIDB1())
            .field("IBIDB2", &self.IBIDB2())
            .field("IBIDB3", &self.IBIDB3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_IBIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_IBIDR {{ IBIDB0: {=u8:?}, IBIDBx: {=u32:?}, IBIDB1: {=u8:?}, IBIDB2: {=u8:?}, IBIDB3: {=u8:?} }}",
            self.IBIDB0(),
            self.IBIDBx(),
            self.IBIDB1(),
            self.IBIDB2(),
            self.IBIDB3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_IER(pub u32);
impl I3C_IER {
    #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CFNFIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_CFNFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn SFNEIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_SFNEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFNFIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_TXFNFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFNEIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_RXFNEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn FCIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_FCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn RXTGTENDIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "target-initiated read end interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_RXTGTENDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "error interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub const fn set_ERRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IBI request interrupt enable (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "IBI request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_IBIIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "IBI end interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIENDIE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IBI end interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_IBIENDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRIE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_CRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "controller-role update interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn CRUPDIE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "controller-role update interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_CRUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "hot-join interrupt enable (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn HJIE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "hot-join interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_HJIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "wakeup interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn WKPIE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "wakeup interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_WKPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn GETIE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "GETxxx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_GETIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn STAIE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_STAIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn DAUPDIE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_DAUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MWLUPDIE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SETMWL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MWLUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MRLUPDIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "SETMRL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MRLUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "reset pattern interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RSTIE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "reset pattern interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_RSTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn ASUPDIE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ENTASx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_ASUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn INTUPDIE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_INTUPDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn DEFIE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_DEFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn GRPIE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_GRPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_IER {
    #[inline(always)]
    fn default() -> I3C_IER {
        I3C_IER(0)
    }
}
impl core::fmt::Debug for I3C_IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_IER")
            .field("CFNFIE", &self.CFNFIE())
            .field("SFNEIE", &self.SFNEIE())
            .field("TXFNFIE", &self.TXFNFIE())
            .field("RXFNEIE", &self.RXFNEIE())
            .field("FCIE", &self.FCIE())
            .field("RXTGTENDIE", &self.RXTGTENDIE())
            .field("ERRIE", &self.ERRIE())
            .field("IBIIE", &self.IBIIE())
            .field("IBIENDIE", &self.IBIENDIE())
            .field("CRIE", &self.CRIE())
            .field("CRUPDIE", &self.CRUPDIE())
            .field("HJIE", &self.HJIE())
            .field("WKPIE", &self.WKPIE())
            .field("GETIE", &self.GETIE())
            .field("STAIE", &self.STAIE())
            .field("DAUPDIE", &self.DAUPDIE())
            .field("MWLUPDIE", &self.MWLUPDIE())
            .field("MRLUPDIE", &self.MRLUPDIE())
            .field("RSTIE", &self.RSTIE())
            .field("ASUPDIE", &self.ASUPDIE())
            .field("INTUPDIE", &self.INTUPDIE())
            .field("DEFIE", &self.DEFIE())
            .field("GRPIE", &self.GRPIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_IER {{ CFNFIE: {=bool:?}, SFNEIE: {=bool:?}, TXFNFIE: {=bool:?}, RXFNEIE: {=bool:?}, FCIE: {=bool:?}, RXTGTENDIE: {=bool:?}, ERRIE: {=bool:?}, IBIIE: {=bool:?}, IBIENDIE: {=bool:?}, CRIE: {=bool:?}, CRUPDIE: {=bool:?}, HJIE: {=bool:?}, WKPIE: {=bool:?}, GETIE: {=bool:?}, STAIE: {=bool:?}, DAUPDIE: {=bool:?}, MWLUPDIE: {=bool:?}, MRLUPDIE: {=bool:?}, RSTIE: {=bool:?}, ASUPDIE: {=bool:?}, INTUPDIE: {=bool:?}, DEFIE: {=bool:?}, GRPIE: {=bool:?} }}",
            self.CFNFIE(),
            self.SFNEIE(),
            self.TXFNFIE(),
            self.RXFNEIE(),
            self.FCIE(),
            self.RXTGTENDIE(),
            self.ERRIE(),
            self.IBIIE(),
            self.IBIENDIE(),
            self.CRIE(),
            self.CRUPDIE(),
            self.HJIE(),
            self.WKPIE(),
            self.GETIE(),
            self.STAIE(),
            self.DAUPDIE(),
            self.MWLUPDIE(),
            self.MRLUPDIE(),
            self.RSTIE(),
            self.ASUPDIE(),
            self.INTUPDIE(),
            self.DEFIE(),
            self.GRPIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_IPIDR(pub u32);
impl I3C_IPIDR {
    #[doc = "IP identification"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP identification"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for I3C_IPIDR {
    #[inline(always)]
    fn default() -> I3C_IPIDR {
        I3C_IPIDR(0)
    }
}
impl core::fmt::Debug for I3C_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_IPIDR").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_IPIDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MAXRLR(pub u32);
impl I3C_MAXRLR {
    #[doc = "maximum data read length (when I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "maximum data read length (when I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IBI payload data size, in bytes (when I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "IBI payload data size, in bytes (when I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_IBIP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for I3C_MAXRLR {
    #[inline(always)]
    fn default() -> I3C_MAXRLR {
        I3C_MAXRLR(0)
    }
}
impl core::fmt::Debug for I3C_MAXRLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MAXRLR")
            .field("MRL", &self.MRL())
            .field("IBIP", &self.IBIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MAXRLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MAXRLR {{ MRL: {=u16:?}, IBIP: {=u8:?} }}",
            self.MRL(),
            self.IBIP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MAXWLR(pub u32);
impl I3C_MAXWLR {
    #[doc = "maximum data write length (when I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn MWL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "maximum data write length (when I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_MWL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MAXWLR {
    #[inline(always)]
    fn default() -> I3C_MAXWLR {
        I3C_MAXWLR(0)
    }
}
impl core::fmt::Debug for I3C_MAXWLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MAXWLR")
            .field("MWL", &self.MWL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MAXWLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MAXWLR {{ MWL: {=u16:?} }}", self.MWL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_RDR(pub u32);
impl I3C_RDR {
    #[doc = "8-bit received data on I3C bus"]
    #[must_use]
    #[inline(always)]
    pub const fn RDB0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit received data on I3C bus"]
    #[inline(always)]
    pub const fn set_RDB0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_RDR {
    #[inline(always)]
    fn default() -> I3C_RDR {
        I3C_RDR(0)
    }
}
impl core::fmt::Debug for I3C_RDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_RDR")
            .field("RDB0", &self.RDB0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_RDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_RDR {{ RDB0: {=u8:?} }}", self.RDB0())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_RDWR(pub u32);
impl I3C_RDWR {
    #[doc = "8-bit received data (earliest byte on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn RDB0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit received data (earliest byte on I3C bus)"]
    #[inline(always)]
    pub const fn set_RDB0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "32-bit received data"]
    #[must_use]
    #[inline(always)]
    pub const fn RDBx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit received data"]
    #[inline(always)]
    pub const fn set_RDBx(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
    #[doc = "8-bit received data (next byte after RDB0 on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn RDB1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit received data (next byte after RDB0 on I3C bus)"]
    #[inline(always)]
    pub const fn set_RDB1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "8-bit received data (next byte after RDB1 on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn RDB2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit received data (next byte after RDB1 on I3C bus)"]
    #[inline(always)]
    pub const fn set_RDB2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "8-bit received data (latest byte on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn RDB3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit received data (latest byte on I3C bus)"]
    #[inline(always)]
    pub const fn set_RDB3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_RDWR {
    #[inline(always)]
    fn default() -> I3C_RDWR {
        I3C_RDWR(0)
    }
}
impl core::fmt::Debug for I3C_RDWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_RDWR")
            .field("RDB0", &self.RDB0())
            .field("RDBx", &self.RDBx())
            .field("RDB1", &self.RDB1())
            .field("RDB2", &self.RDB2())
            .field("RDB3", &self.RDB3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_RDWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_RDWR {{ RDB0: {=u8:?}, RDBx: {=u32:?}, RDB1: {=u8:?}, RDB2: {=u8:?}, RDB3: {=u8:?} }}",
            self.RDB0(),
            self.RDBx(),
            self.RDB1(),
            self.RDB2(),
            self.RDB3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_RMR(pub u32);
impl I3C_RMR {
    #[doc = "IBI received payload data count (when the I3C is configured as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIRDCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "IBI received payload data count (when the I3C is configured as controller)"]
    #[inline(always)]
    pub const fn set_IBIRDCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "received CCC code (when the I3C is configured as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn RCODE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "received CCC code (when the I3C is configured as target)"]
    #[inline(always)]
    pub const fn set_RCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "received target address (when the I3C is configured as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn RADD(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "received target address (when the I3C is configured as controller)"]
    #[inline(always)]
    pub const fn set_RADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
}
impl Default for I3C_RMR {
    #[inline(always)]
    fn default() -> I3C_RMR {
        I3C_RMR(0)
    }
}
impl core::fmt::Debug for I3C_RMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_RMR")
            .field("IBIRDCNT", &self.IBIRDCNT())
            .field("RCODE", &self.RCODE())
            .field("RADD", &self.RADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_RMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_RMR {{ IBIRDCNT: {=u8:?}, RCODE: {=u8:?}, RADD: {=u8:?} }}",
            self.IBIRDCNT(),
            self.RCODE(),
            self.RADD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SER(pub u32);
impl I3C_SER {
    #[doc = "protocol error code/type"]
    #[must_use]
    #[inline(always)]
    pub const fn CODERR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "protocol error code/type"]
    #[inline(always)]
    pub const fn set_CODERR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "protocol error"]
    #[must_use]
    #[inline(always)]
    pub const fn PERR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "protocol error"]
    #[inline(always)]
    pub const fn set_PERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SCL stall error (when the I3C is acting as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn STALL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SCL stall error (when the I3C is acting as target)"]
    #[inline(always)]
    pub const fn set_STALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX-FIFO overrun or TX-FIFO underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn DOVR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX-FIFO overrun or TX-FIFO underrun"]
    #[inline(always)]
    pub const fn set_DOVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn COVR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_COVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "address not acknowledged (when the I3C is configured as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn ANACK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "address not acknowledged (when the I3C is configured as controller)"]
    #[inline(always)]
    pub const fn set_ANACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "data not acknowledged (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn DNACK(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "data not acknowledged (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_DNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "data error (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn DERR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "data error (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_DERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for I3C_SER {
    #[inline(always)]
    fn default() -> I3C_SER {
        I3C_SER(0)
    }
}
impl core::fmt::Debug for I3C_SER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SER")
            .field("CODERR", &self.CODERR())
            .field("PERR", &self.PERR())
            .field("STALL", &self.STALL())
            .field("DOVR", &self.DOVR())
            .field("COVR", &self.COVR())
            .field("ANACK", &self.ANACK())
            .field("DNACK", &self.DNACK())
            .field("DERR", &self.DERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SER {{ CODERR: {=u8:?}, PERR: {=bool:?}, STALL: {=bool:?}, DOVR: {=bool:?}, COVR: {=bool:?}, ANACK: {=bool:?}, DNACK: {=bool:?}, DERR: {=bool:?} }}",
            self.CODERR(),
            self.PERR(),
            self.STALL(),
            self.DOVR(),
            self.COVR(),
            self.ANACK(),
            self.DNACK(),
            self.DERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SIDR(pub u32);
impl I3C_SIDR {
    #[doc = "IP address space size identification"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP address space size identification"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for I3C_SIDR {
    #[inline(always)]
    fn default() -> I3C_SIDR {
        I3C_SIDR(0)
    }
}
impl core::fmt::Debug for I3C_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SR(pub u32);
impl I3C_SR {
    #[doc = "data counter"]
    #[must_use]
    #[inline(always)]
    pub const fn XDCNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "data counter"]
    #[inline(always)]
    pub const fn set_XDCNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as co"]
    #[must_use]
    #[inline(always)]
    pub const fn ABT(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "a private read message is completed/aborted prematurely by the target (when the I3C is acting as co"]
    #[inline(always)]
    pub const fn set_ABT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "message direction"]
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "message direction"]
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn MID(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "message identifier/counter of a given frame (when the I3C is acting as controller)"]
    #[inline(always)]
    pub const fn set_MID(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_SR {
    #[inline(always)]
    fn default() -> I3C_SR {
        I3C_SR(0)
    }
}
impl core::fmt::Debug for I3C_SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SR")
            .field("XDCNT", &self.XDCNT())
            .field("ABT", &self.ABT())
            .field("DIR", &self.DIR())
            .field("MID", &self.MID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SR {{ XDCNT: {=u16:?}, ABT: {=bool:?}, DIR: {=bool:?}, MID: {=u8:?} }}",
            self.XDCNT(),
            self.ABT(),
            self.DIR(),
            self.MID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_TDR(pub u32);
impl I3C_TDR {
    #[doc = "8-bit data to transmit on I3C bus"]
    #[must_use]
    #[inline(always)]
    pub const fn TDB0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit data to transmit on I3C bus"]
    #[inline(always)]
    pub const fn set_TDB0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_TDR {
    #[inline(always)]
    fn default() -> I3C_TDR {
        I3C_TDR(0)
    }
}
impl core::fmt::Debug for I3C_TDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TDR")
            .field("TDB0", &self.TDB0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_TDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_TDR {{ TDB0: {=u8:?} }}", self.TDB0())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_TDWR(pub u32);
impl I3C_TDWR {
    #[doc = "8-bit transmit data (earliest byte on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn TDB0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data (earliest byte on I3C bus)"]
    #[inline(always)]
    pub const fn set_TDB0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "32-bit transmit data"]
    #[must_use]
    #[inline(always)]
    pub const fn TDBx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit transmit data"]
    #[inline(always)]
    pub const fn set_TDBx(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
    #[doc = "8-bit transmit data (next byte after TDB0\\[7:0\\] on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn TDB1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data (next byte after TDB0\\[7:0\\] on I3C bus)"]
    #[inline(always)]
    pub const fn set_TDB1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "8-bit transmit data (next byte after TDB1\\[7:0\\] on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn TDB2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data (next byte after TDB1\\[7:0\\] on I3C bus)"]
    #[inline(always)]
    pub const fn set_TDB2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "8-bit transmit data (latest byte on I3C bus)"]
    #[must_use]
    #[inline(always)]
    pub const fn TDB3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data (latest byte on I3C bus)"]
    #[inline(always)]
    pub const fn set_TDB3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_TDWR {
    #[inline(always)]
    fn default() -> I3C_TDWR {
        I3C_TDWR(0)
    }
}
impl core::fmt::Debug for I3C_TDWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TDWR")
            .field("TDB0", &self.TDB0())
            .field("TDBx", &self.TDBx())
            .field("TDB1", &self.TDB1())
            .field("TDB2", &self.TDB2())
            .field("TDB3", &self.TDB3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_TDWR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_TDWR {{ TDB0: {=u8:?}, TDBx: {=u32:?}, TDB1: {=u8:?}, TDB2: {=u8:?}, TDB3: {=u8:?} }}",
            self.TDB0(),
            self.TDBx(),
            self.TDB1(),
            self.TDB2(),
            self.TDB3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_TGTTDR(pub u32);
impl I3C_TGTTDR {
    #[doc = "transmit data counter, in bytes (when I3C is configured as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn TGTTDCNT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "transmit data counter, in bytes (when I3C is configured as target)"]
    #[inline(always)]
    pub const fn set_TGTTDCNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "preload of the TX-FIFO (when I3C is configured as target)"]
    #[must_use]
    #[inline(always)]
    pub const fn PRELOAD(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "preload of the TX-FIFO (when I3C is configured as target)"]
    #[inline(always)]
    pub const fn set_PRELOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3C_TGTTDR {
    #[inline(always)]
    fn default() -> I3C_TGTTDR {
        I3C_TGTTDR(0)
    }
}
impl core::fmt::Debug for I3C_TGTTDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TGTTDR")
            .field("TGTTDCNT", &self.TGTTDCNT())
            .field("PRELOAD", &self.PRELOAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_TGTTDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_TGTTDR {{ TGTTDCNT: {=u16:?}, PRELOAD: {=bool:?} }}",
            self.TGTTDCNT(),
            self.PRELOAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_TIMINGR1(pub u32);
impl I3C_TIMINGR1 {
    #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set"]
    #[must_use]
    #[inline(always)]
    pub const fn AVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set"]
    #[inline(always)]
    pub const fn set_AVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "activity state of the new controller (when I3C is acting as - active- controller)"]
    #[must_use]
    #[inline(always)]
    pub const fn ASNCR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "activity state of the new controller (when I3C is acting as - active- controller)"]
    #[inline(always)]
    pub const fn set_ASNCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time ("]
    #[must_use]
    #[inline(always)]
    pub const fn FREE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time ("]
    #[inline(always)]
    pub const fn set_FREE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for I3C_TIMINGR1 {
    #[inline(always)]
    fn default() -> I3C_TIMINGR1 {
        I3C_TIMINGR1(0)
    }
}
impl core::fmt::Debug for I3C_TIMINGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TIMINGR1")
            .field("AVAL", &self.AVAL())
            .field("ASNCR", &self.ASNCR())
            .field("FREE", &self.FREE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_TIMINGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_TIMINGR1 {{ AVAL: {=u8:?}, ASNCR: {=u8:?}, FREE: {=u8:?} }}",
            self.AVAL(),
            self.ASNCR(),
            self.FREE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_TIMINGR2(pub u32);
impl I3C_TIMINGR2 {
    #[doc = "Controller clock stall on T-bit phase of Data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STALLT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controller clock stall on T-bit phase of Data enable"]
    #[inline(always)]
    pub const fn set_STALLT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "controller clock stall on PAR phase of Data enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STALLD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "controller clock stall on PAR phase of Data enable"]
    #[inline(always)]
    pub const fn set_STALLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "controller clock stall on PAR phase of CCC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STALLC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "controller clock stall on PAR phase of CCC enable"]
    #[inline(always)]
    pub const fn set_STALLC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "controller clock stall enable on ACK phase"]
    #[must_use]
    #[inline(always)]
    pub const fn STALLA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "controller clock stall enable on ACK phase"]
    #[inline(always)]
    pub const fn set_STALLA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "controller clock stall time, in number of kernel clock cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn STALL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "controller clock stall time, in number of kernel clock cycles"]
    #[inline(always)]
    pub const fn set_STALL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_TIMINGR2 {
    #[inline(always)]
    fn default() -> I3C_TIMINGR2 {
        I3C_TIMINGR2(0)
    }
}
impl core::fmt::Debug for I3C_TIMINGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_TIMINGR2")
            .field("STALLT", &self.STALLT())
            .field("STALLD", &self.STALLD())
            .field("STALLC", &self.STALLC())
            .field("STALLA", &self.STALLA())
            .field("STALL", &self.STALL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_TIMINGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_TIMINGR2 {{ STALLT: {=bool:?}, STALLD: {=bool:?}, STALLC: {=bool:?}, STALLA: {=bool:?}, STALL: {=u8:?} }}",
            self.STALLT(),
            self.STALLD(),
            self.STALLC(),
            self.STALLA(),
            self.STALL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_VERR(pub u32);
impl I3C_VERR {
    #[doc = "minor IP revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "minor IP revision"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "major IP revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "major IP revision"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for I3C_VERR {
    #[inline(always)]
    fn default() -> I3C_VERR {
        I3C_VERR(0)
    }
}
impl core::fmt::Debug for I3C_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
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

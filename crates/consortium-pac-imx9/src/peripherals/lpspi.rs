#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI {
    ptr: *mut u8,
}
unsafe impl Send for LPSPI {}
unsafe impl Sync for LPSPI {}
impl LPSPI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<LPSPI_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<LPSPI_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<LPSPI_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<LPSPI_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<LPSPI_IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DER(self) -> crate::common::Reg<LPSPI_DER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn CFGR0(self) -> crate::common::Reg<LPSPI_CFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CFGR1(self) -> crate::common::Reg<LPSPI_CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn DMR0(self) -> crate::common::Reg<LPSPI_DMR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn DMR1(self) -> crate::common::Reg<LPSPI_DMR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn CCR(self) -> crate::common::Reg<LPSPI_CCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn CCR1(self) -> crate::common::Reg<LPSPI_CCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn FCR(self) -> crate::common::Reg<LPSPI_FCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn FSR(self) -> crate::common::Reg<LPSPI_FSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<LPSPI_TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn TDR(self) -> crate::common::Reg<LPSPI_TDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn RSR(self) -> crate::common::Reg<LPSPI_RSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn RDR(self) -> crate::common::Reg<LPSPI_RDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn RDROR(self) -> crate::common::Reg<LPSPI_RDROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn TCBR(self) -> crate::common::Reg<LPSPI_TCBR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[inline(always)]
    pub const fn TDBR(self, n: usize) -> crate::common::Reg<LPSPI_TDBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn RDBR(self, n: usize) -> crate::common::Reg<LPSPI_RDBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_CCR(pub u32);
impl LPSPI_CCR {
    #[doc = "SCK Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn SCKDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Divider"]
    #[inline(always)]
    pub const fn set_SCKDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCK Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn DBT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Divider"]
    #[inline(always)]
    pub const fn set_DBT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Delay Between Transfers"]
    #[must_use]
    #[inline(always)]
    pub const fn PCSSCK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Delay Between Transfers"]
    #[inline(always)]
    pub const fn set_PCSSCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PCS-to-SCK Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn SCKPCS(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PCS-to-SCK Delay"]
    #[inline(always)]
    pub const fn set_SCKPCS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPSPI_CCR {
    #[inline(always)]
    fn default() -> LPSPI_CCR {
        LPSPI_CCR(0)
    }
}
impl core::fmt::Debug for LPSPI_CCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_CCR")
            .field("SCKDIV", &self.SCKDIV())
            .field("DBT", &self.DBT())
            .field("PCSSCK", &self.PCSSCK())
            .field("SCKPCS", &self.SCKPCS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_CCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_CCR {{ SCKDIV: {=u8:?}, DBT: {=u8:?}, PCSSCK: {=u8:?}, SCKPCS: {=u8:?} }}",
            self.SCKDIV(),
            self.DBT(),
            self.PCSSCK(),
            self.SCKPCS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_CCR1(pub u32);
impl LPSPI_CCR1 {
    #[doc = "SCK Setup"]
    #[must_use]
    #[inline(always)]
    pub const fn SCKSET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Setup"]
    #[inline(always)]
    pub const fn set_SCKSET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCK Setup"]
    #[must_use]
    #[inline(always)]
    pub const fn SCKHLD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Setup"]
    #[inline(always)]
    pub const fn set_SCKHLD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "SCK Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn PCSPCS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Hold"]
    #[inline(always)]
    pub const fn set_PCSPCS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PCS to PCS Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn SCKSCK(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PCS to PCS Delay"]
    #[inline(always)]
    pub const fn set_SCKSCK(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPSPI_CCR1 {
    #[inline(always)]
    fn default() -> LPSPI_CCR1 {
        LPSPI_CCR1(0)
    }
}
impl core::fmt::Debug for LPSPI_CCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_CCR1")
            .field("SCKSET", &self.SCKSET())
            .field("SCKHLD", &self.SCKHLD())
            .field("PCSPCS", &self.PCSPCS())
            .field("SCKSCK", &self.SCKSCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_CCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_CCR1 {{ SCKSET: {=u8:?}, SCKHLD: {=u8:?}, PCSPCS: {=u8:?}, SCKSCK: {=u8:?} }}",
            self.SCKSET(),
            self.SCKHLD(),
            self.PCSPCS(),
            self.SCKSCK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_CFGR0(pub u32);
impl LPSPI_CFGR0 {
    #[must_use]
    #[inline(always)]
    pub const fn CIRFIFO(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIRFIFO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDMO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDMO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for LPSPI_CFGR0 {
    #[inline(always)]
    fn default() -> LPSPI_CFGR0 {
        LPSPI_CFGR0(0)
    }
}
impl core::fmt::Debug for LPSPI_CFGR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_CFGR0")
            .field("CIRFIFO", &self.CIRFIFO())
            .field("RDMO", &self.RDMO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_CFGR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_CFGR0 {{ CIRFIFO: {=bool:?}, RDMO: {=bool:?} }}",
            self.CIRFIFO(),
            self.RDMO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_CFGR1(pub u32);
impl LPSPI_CFGR1 {
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SAMPLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SAMPLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AUTOPCS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AUTOPCS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOSTALL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PARTIAL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PARTIAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PCSPOL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PCSPOL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MATCFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MATCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PINCFG(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PINCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OUTCFG(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OUTCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for LPSPI_CFGR1 {
    #[inline(always)]
    fn default() -> LPSPI_CFGR1 {
        LPSPI_CFGR1(0)
    }
}
impl core::fmt::Debug for LPSPI_CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_CFGR1")
            .field("MASTER", &self.MASTER())
            .field("SAMPLE", &self.SAMPLE())
            .field("AUTOPCS", &self.AUTOPCS())
            .field("NOSTALL", &self.NOSTALL())
            .field("PARTIAL", &self.PARTIAL())
            .field("PCSPOL", &self.PCSPOL())
            .field("MATCFG", &self.MATCFG())
            .field("PINCFG", &self.PINCFG())
            .field("OUTCFG", &self.OUTCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_CFGR1 {{ MASTER: {=bool:?}, SAMPLE: {=bool:?}, AUTOPCS: {=bool:?}, NOSTALL: {=bool:?}, PARTIAL: {=bool:?}, PCSPOL: {=u8:?}, MATCFG: {=u8:?}, PINCFG: {=u8:?}, OUTCFG: {=bool:?} }}",
            self.MASTER(),
            self.SAMPLE(),
            self.AUTOPCS(),
            self.NOSTALL(),
            self.PARTIAL(),
            self.PCSPOL(),
            self.MATCFG(),
            self.PINCFG(),
            self.OUTCFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_CR(pub u32);
impl LPSPI_CR {
    #[must_use]
    #[inline(always)]
    pub const fn MEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MEN(&mut self, val: bool) {
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
    #[must_use]
    #[inline(always)]
    pub const fn DOZEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DBGEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DBGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RRF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for LPSPI_CR {
    #[inline(always)]
    fn default() -> LPSPI_CR {
        LPSPI_CR(0)
    }
}
impl core::fmt::Debug for LPSPI_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_CR")
            .field("MEN", &self.MEN())
            .field("RST", &self.RST())
            .field("DOZEN", &self.DOZEN())
            .field("DBGEN", &self.DBGEN())
            .field("RTF", &self.RTF())
            .field("RRF", &self.RRF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_CR {{ MEN: {=bool:?}, RST: {=bool:?}, DOZEN: {=bool:?}, DBGEN: {=bool:?}, RTF: {=bool:?}, RRF: {=bool:?} }}",
            self.MEN(),
            self.RST(),
            self.DOZEN(),
            self.DBGEN(),
            self.RTF(),
            self.RRF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_DER(pub u32);
impl LPSPI_DER {
    #[must_use]
    #[inline(always)]
    pub const fn TDDE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDDE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FCDE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FCDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for LPSPI_DER {
    #[inline(always)]
    fn default() -> LPSPI_DER {
        LPSPI_DER(0)
    }
}
impl core::fmt::Debug for LPSPI_DER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_DER")
            .field("TDDE", &self.TDDE())
            .field("RDDE", &self.RDDE())
            .field("FCDE", &self.FCDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_DER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_DER {{ TDDE: {=bool:?}, RDDE: {=bool:?}, FCDE: {=bool:?} }}",
            self.TDDE(),
            self.RDDE(),
            self.FCDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_DMR0(pub u32);
impl LPSPI_DMR0 {
    #[doc = "Match 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCH0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_MATCH0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_DMR0 {
    #[inline(always)]
    fn default() -> LPSPI_DMR0 {
        LPSPI_DMR0(0)
    }
}
impl core::fmt::Debug for LPSPI_DMR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_DMR0")
            .field("MATCH0", &self.MATCH0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_DMR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_DMR0 {{ MATCH0: {=u32:?} }}", self.MATCH0())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_DMR1(pub u32);
impl LPSPI_DMR1 {
    #[doc = "Match 1 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCH1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 1 Value"]
    #[inline(always)]
    pub const fn set_MATCH1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_DMR1 {
    #[inline(always)]
    fn default() -> LPSPI_DMR1 {
        LPSPI_DMR1(0)
    }
}
impl core::fmt::Debug for LPSPI_DMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_DMR1")
            .field("MATCH1", &self.MATCH1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_DMR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_DMR1 {{ MATCH1: {=u32:?} }}", self.MATCH1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_FCR(pub u32);
impl LPSPI_FCR {
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn TXWATER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_TXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn RXWATER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_RXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for LPSPI_FCR {
    #[inline(always)]
    fn default() -> LPSPI_FCR {
        LPSPI_FCR(0)
    }
}
impl core::fmt::Debug for LPSPI_FCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_FCR")
            .field("TXWATER", &self.TXWATER())
            .field("RXWATER", &self.RXWATER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_FCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_FCR {{ TXWATER: {=u8:?}, RXWATER: {=u8:?} }}",
            self.TXWATER(),
            self.RXWATER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_FSR(pub u32);
impl LPSPI_FSR {
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCOUNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_TXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCOUNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_RXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for LPSPI_FSR {
    #[inline(always)]
    fn default() -> LPSPI_FSR {
        LPSPI_FSR(0)
    }
}
impl core::fmt::Debug for LPSPI_FSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_FSR")
            .field("TXCOUNT", &self.TXCOUNT())
            .field("RXCOUNT", &self.RXCOUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_FSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_FSR {{ TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?} }}",
            self.TXCOUNT(),
            self.RXCOUNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_IER(pub u32);
impl LPSPI_IER {
    #[must_use]
    #[inline(always)]
    pub const fn TDIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WCIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FCIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TCIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TEIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn REIE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMIE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for LPSPI_IER {
    #[inline(always)]
    fn default() -> LPSPI_IER {
        LPSPI_IER(0)
    }
}
impl core::fmt::Debug for LPSPI_IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_IER")
            .field("TDIE", &self.TDIE())
            .field("RDIE", &self.RDIE())
            .field("WCIE", &self.WCIE())
            .field("FCIE", &self.FCIE())
            .field("TCIE", &self.TCIE())
            .field("TEIE", &self.TEIE())
            .field("REIE", &self.REIE())
            .field("DMIE", &self.DMIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_IER {{ TDIE: {=bool:?}, RDIE: {=bool:?}, WCIE: {=bool:?}, FCIE: {=bool:?}, TCIE: {=bool:?}, TEIE: {=bool:?}, REIE: {=bool:?}, DMIE: {=bool:?} }}",
            self.TDIE(),
            self.RDIE(),
            self.WCIE(),
            self.FCIE(),
            self.TCIE(),
            self.TEIE(),
            self.REIE(),
            self.DMIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_PARAM(pub u32);
impl LPSPI_PARAM {
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
    #[doc = "Receive FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn PCSNUM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_PCSNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for LPSPI_PARAM {
    #[inline(always)]
    fn default() -> LPSPI_PARAM {
        LPSPI_PARAM(0)
    }
}
impl core::fmt::Debug for LPSPI_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_PARAM")
            .field("TXFIFO", &self.TXFIFO())
            .field("RXFIFO", &self.RXFIFO())
            .field("PCSNUM", &self.PCSNUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_PARAM {{ TXFIFO: {=u8:?}, RXFIFO: {=u8:?}, PCSNUM: {=u8:?} }}",
            self.TXFIFO(),
            self.RXFIFO(),
            self.PCSNUM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_RDBR(pub u32);
impl LPSPI_RDBR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_RDBR {
    #[inline(always)]
    fn default() -> LPSPI_RDBR {
        LPSPI_RDBR(0)
    }
}
impl core::fmt::Debug for LPSPI_RDBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_RDBR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_RDBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_RDBR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_RDR(pub u32);
impl LPSPI_RDR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_RDR {
    #[inline(always)]
    fn default() -> LPSPI_RDR {
        LPSPI_RDR(0)
    }
}
impl core::fmt::Debug for LPSPI_RDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_RDR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_RDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_RDR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_RDROR(pub u32);
impl LPSPI_RDROR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_RDROR {
    #[inline(always)]
    fn default() -> LPSPI_RDROR {
        LPSPI_RDROR(0)
    }
}
impl core::fmt::Debug for LPSPI_RDROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_RDROR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_RDROR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_RDROR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_RSR(pub u32);
impl LPSPI_RSR {
    #[must_use]
    #[inline(always)]
    pub const fn SOF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for LPSPI_RSR {
    #[inline(always)]
    fn default() -> LPSPI_RSR {
        LPSPI_RSR(0)
    }
}
impl core::fmt::Debug for LPSPI_RSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_RSR")
            .field("SOF", &self.SOF())
            .field("RXEMPTY", &self.RXEMPTY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_RSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_RSR {{ SOF: {=bool:?}, RXEMPTY: {=bool:?} }}",
            self.SOF(),
            self.RXEMPTY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_SR(pub u32);
impl LPSPI_SR {
    #[must_use]
    #[inline(always)]
    pub const fn TDF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WCF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FCF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TCF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TEF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn REF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MBF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for LPSPI_SR {
    #[inline(always)]
    fn default() -> LPSPI_SR {
        LPSPI_SR(0)
    }
}
impl core::fmt::Debug for LPSPI_SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_SR")
            .field("TDF", &self.TDF())
            .field("RDF", &self.RDF())
            .field("WCF", &self.WCF())
            .field("FCF", &self.FCF())
            .field("TCF", &self.TCF())
            .field("TEF", &self.TEF())
            .field("REF", &self.REF())
            .field("DMF", &self.DMF())
            .field("MBF", &self.MBF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_SR {{ TDF: {=bool:?}, RDF: {=bool:?}, WCF: {=bool:?}, FCF: {=bool:?}, TCF: {=bool:?}, TEF: {=bool:?}, REF: {=bool:?}, DMF: {=bool:?}, MBF: {=bool:?} }}",
            self.TDF(),
            self.RDF(),
            self.WCF(),
            self.FCF(),
            self.TCF(),
            self.TEF(),
            self.REF(),
            self.DMF(),
            self.MBF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_TCBR(pub u32);
impl LPSPI_TCBR {
    #[doc = "Command Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_TCBR {
    #[inline(always)]
    fn default() -> LPSPI_TCBR {
        LPSPI_TCBR(0)
    }
}
impl core::fmt::Debug for LPSPI_TCBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_TCBR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_TCBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_TCBR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_TCR(pub u32);
impl LPSPI_TCR {
    #[doc = "Frame Size"]
    #[must_use]
    #[inline(always)]
    pub const fn FRAMESZ(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_FRAMESZ(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Frame Size"]
    #[must_use]
    #[inline(always)]
    pub const fn TXMSK(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_TXMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXMSK(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CONTC(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CONTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CONT(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CONT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BYSW(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BYSW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LSBF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LSBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PCS(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PCS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRESCALE(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PRESCALE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CPHA(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CPHA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CPOL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LPSPI_TCR {
    #[inline(always)]
    fn default() -> LPSPI_TCR {
        LPSPI_TCR(0)
    }
}
impl core::fmt::Debug for LPSPI_TCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_TCR")
            .field("FRAMESZ", &self.FRAMESZ())
            .field("TXMSK", &self.TXMSK())
            .field("RXMSK", &self.RXMSK())
            .field("CONTC", &self.CONTC())
            .field("CONT", &self.CONT())
            .field("BYSW", &self.BYSW())
            .field("LSBF", &self.LSBF())
            .field("PCS", &self.PCS())
            .field("PRESCALE", &self.PRESCALE())
            .field("CPHA", &self.CPHA())
            .field("CPOL", &self.CPOL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_TCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_TCR {{ FRAMESZ: {=u16:?}, TXMSK: {=bool:?}, RXMSK: {=bool:?}, CONTC: {=bool:?}, CONT: {=bool:?}, BYSW: {=bool:?}, LSBF: {=bool:?}, PCS: {=u8:?}, PRESCALE: {=u8:?}, CPHA: {=bool:?}, CPOL: {=bool:?} }}",
            self.FRAMESZ(),
            self.TXMSK(),
            self.RXMSK(),
            self.CONTC(),
            self.CONT(),
            self.BYSW(),
            self.LSBF(),
            self.PCS(),
            self.PRESCALE(),
            self.CPHA(),
            self.CPOL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_TDBR(pub u32);
impl LPSPI_TDBR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_TDBR {
    #[inline(always)]
    fn default() -> LPSPI_TDBR {
        LPSPI_TDBR(0)
    }
}
impl core::fmt::Debug for LPSPI_TDBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_TDBR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_TDBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_TDBR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_TDR(pub u32);
impl LPSPI_TDR {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LPSPI_TDR {
    #[inline(always)]
    fn default() -> LPSPI_TDR {
        LPSPI_TDR(0)
    }
}
impl core::fmt::Debug for LPSPI_TDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_TDR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_TDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPSPI_TDR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPSPI_VERID(pub u32);
impl LPSPI_VERID {
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
impl Default for LPSPI_VERID {
    #[inline(always)]
    fn default() -> LPSPI_VERID {
        LPSPI_VERID(0)
    }
}
impl core::fmt::Debug for LPSPI_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPSPI_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPSPI_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPSPI_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}

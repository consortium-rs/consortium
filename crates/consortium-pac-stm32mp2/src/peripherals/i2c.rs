#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C {
    ptr: *mut u8,
}
unsafe impl Send for I2C {}
unsafe impl Sync for I2C {}
impl I2C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I2C Control register 1"]
    #[inline(always)]
    pub const fn CR1(self) -> crate::common::Reg<I2C_CR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I2C Control register 2"]
    #[inline(always)]
    pub const fn CR2(self) -> crate::common::Reg<I2C_CR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "I2C Own address 1 register"]
    #[inline(always)]
    pub const fn OAR1(self) -> crate::common::Reg<I2C_OAR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "I2C Own address 2 register"]
    #[inline(always)]
    pub const fn OAR2(self) -> crate::common::Reg<I2C_OAR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "I2C Timing register"]
    #[inline(always)]
    pub const fn TIMINGR(self) -> crate::common::Reg<I2C_TIMINGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "I2C Timeout register"]
    #[inline(always)]
    pub const fn TIMEOUTR(self) -> crate::common::Reg<I2C_TIMEOUTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "I2C Interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<I2C_ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "I2C Interrupt clear register"]
    #[inline(always)]
    pub const fn ICR(self) -> crate::common::Reg<I2C_ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "I2C PEC register"]
    #[inline(always)]
    pub const fn PECR(self) -> crate::common::Reg<I2C_PECR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "I2C Receive data register"]
    #[inline(always)]
    pub const fn RXDR(self) -> crate::common::Reg<I2C_RXDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "I2C Transmit data register"]
    #[inline(always)]
    pub const fn TXDR(self) -> crate::common::Reg<I2C_TXDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "I2C Autonomous mode control register"]
    #[inline(always)]
    pub const fn AUTOCR(self) -> crate::common::Reg<I2C_AUTOCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "I2C hardware configuration register"]
    #[inline(always)]
    pub const fn HWCFGR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "I2C version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<I2C_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "I2C identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<I2C_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "I2C size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<I2C_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_AUTOCR(pub u32);
impl I2C_AUTOCR {
    #[doc = "DMA request enable on Transfer Complete event"]
    #[must_use]
    #[inline(always)]
    pub const fn TCDMAEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA request enable on Transfer Complete event"]
    #[inline(always)]
    pub const fn set_TCDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA request enable on Transfer Complete Reload event"]
    #[must_use]
    #[inline(always)]
    pub const fn TCRDMAEN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA request enable on Transfer Complete Reload event"]
    #[inline(always)]
    pub const fn set_TCRDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGSEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger selection"]
    #[inline(always)]
    pub const fn set_TRIGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGPOL(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger polarity"]
    #[inline(always)]
    pub const fn set_TRIGPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Trigger enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger enable"]
    #[inline(always)]
    pub const fn set_TRIGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for I2C_AUTOCR {
    #[inline(always)]
    fn default() -> I2C_AUTOCR {
        I2C_AUTOCR(0)
    }
}
impl core::fmt::Debug for I2C_AUTOCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_AUTOCR")
            .field("TCDMAEN", &self.TCDMAEN())
            .field("TCRDMAEN", &self.TCRDMAEN())
            .field("TRIGSEL", &self.TRIGSEL())
            .field("TRIGPOL", &self.TRIGPOL())
            .field("TRIGEN", &self.TRIGEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_AUTOCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_AUTOCR {{ TCDMAEN: {=bool:?}, TCRDMAEN: {=bool:?}, TRIGSEL: {=u8:?}, TRIGPOL: {=bool:?}, TRIGEN: {=bool:?} }}",
            self.TCDMAEN(),
            self.TCRDMAEN(),
            self.TRIGSEL(),
            self.TRIGPOL(),
            self.TRIGEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_CR1(pub u32);
impl I2C_CR1 {
    #[doc = "Peripheral enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral enable"]
    #[inline(always)]
    pub const fn set_PE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX interrupt enable"]
    #[inline(always)]
    pub const fn set_TXIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RX interrupt enable"]
    #[inline(always)]
    pub const fn set_RXIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address match interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address match interrupt enable"]
    #[inline(always)]
    pub const fn set_ADDRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NACK received interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn NACKIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK received interrupt enable"]
    #[inline(always)]
    pub const fn set_NACKIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "STOP detection interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STOPIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection interrupt enable"]
    #[inline(always)]
    pub const fn set_STOPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TCIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_TCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Errors interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Errors interrupt enable"]
    #[inline(always)]
    pub const fn set_ERRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Digital noise filter"]
    #[must_use]
    #[inline(always)]
    pub const fn DNF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Digital noise filter"]
    #[inline(always)]
    pub const fn set_DNF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Analog noise filter OFF"]
    #[must_use]
    #[inline(always)]
    pub const fn ANFOFF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Analog noise filter OFF"]
    #[inline(always)]
    pub const fn set_ANFOFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SWRST(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub const fn set_SWRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA transmission requests enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAEN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA transmission requests enable"]
    #[inline(always)]
    pub const fn set_TXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA reception requests enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA reception requests enable"]
    #[inline(always)]
    pub const fn set_RXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Slave byte control"]
    #[must_use]
    #[inline(always)]
    pub const fn SBC(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave byte control"]
    #[inline(always)]
    pub const fn set_SBC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Clock stretching disable"]
    #[must_use]
    #[inline(always)]
    pub const fn NOSTRETCH(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Clock stretching disable"]
    #[inline(always)]
    pub const fn set_NOSTRETCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wakeup from STOP enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WUPEN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Wakeup from STOP enable"]
    #[inline(always)]
    pub const fn set_WUPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "General call enable"]
    #[must_use]
    #[inline(always)]
    pub const fn GCEN(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "General call enable"]
    #[inline(always)]
    pub const fn set_GCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SMBus host address enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SMBHEN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus host address enable"]
    #[inline(always)]
    pub const fn set_SMBHEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SMBus device default address enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SMBDEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus device default address enable"]
    #[inline(always)]
    pub const fn set_SMBDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SMBus alert enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ALERTEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus alert enable"]
    #[inline(always)]
    pub const fn set_ALERTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PEC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PECEN(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PEC enable"]
    #[inline(always)]
    pub const fn set_PECEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Fast-mode Plus 20mA drive enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FMP(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Fast-mode Plus 20mA drive enable"]
    #[inline(always)]
    pub const fn set_FMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Address match flag (ADDR) automatic clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRACLR(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    pub const fn set_ADDRACLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "STOP detection flag (STOPF) automatic clear"]
    #[must_use]
    #[inline(always)]
    pub const fn STOPFACLR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    pub const fn set_STOPFACLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I2C_CR1 {
    #[inline(always)]
    fn default() -> I2C_CR1 {
        I2C_CR1(0)
    }
}
impl core::fmt::Debug for I2C_CR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR1")
            .field("PE", &self.PE())
            .field("TXIE", &self.TXIE())
            .field("RXIE", &self.RXIE())
            .field("ADDRIE", &self.ADDRIE())
            .field("NACKIE", &self.NACKIE())
            .field("STOPIE", &self.STOPIE())
            .field("TCIE", &self.TCIE())
            .field("ERRIE", &self.ERRIE())
            .field("DNF", &self.DNF())
            .field("ANFOFF", &self.ANFOFF())
            .field("SWRST", &self.SWRST())
            .field("TXDMAEN", &self.TXDMAEN())
            .field("RXDMAEN", &self.RXDMAEN())
            .field("SBC", &self.SBC())
            .field("NOSTRETCH", &self.NOSTRETCH())
            .field("WUPEN", &self.WUPEN())
            .field("GCEN", &self.GCEN())
            .field("SMBHEN", &self.SMBHEN())
            .field("SMBDEN", &self.SMBDEN())
            .field("ALERTEN", &self.ALERTEN())
            .field("PECEN", &self.PECEN())
            .field("FMP", &self.FMP())
            .field("ADDRACLR", &self.ADDRACLR())
            .field("STOPFACLR", &self.STOPFACLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_CR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_CR1 {{ PE: {=bool:?}, TXIE: {=bool:?}, RXIE: {=bool:?}, ADDRIE: {=bool:?}, NACKIE: {=bool:?}, STOPIE: {=bool:?}, TCIE: {=bool:?}, ERRIE: {=bool:?}, DNF: {=u8:?}, ANFOFF: {=bool:?}, SWRST: {=bool:?}, TXDMAEN: {=bool:?}, RXDMAEN: {=bool:?}, SBC: {=bool:?}, NOSTRETCH: {=bool:?}, WUPEN: {=bool:?}, GCEN: {=bool:?}, SMBHEN: {=bool:?}, SMBDEN: {=bool:?}, ALERTEN: {=bool:?}, PECEN: {=bool:?}, FMP: {=bool:?}, ADDRACLR: {=bool:?}, STOPFACLR: {=bool:?} }}",
            self.PE(),
            self.TXIE(),
            self.RXIE(),
            self.ADDRIE(),
            self.NACKIE(),
            self.STOPIE(),
            self.TCIE(),
            self.ERRIE(),
            self.DNF(),
            self.ANFOFF(),
            self.SWRST(),
            self.TXDMAEN(),
            self.RXDMAEN(),
            self.SBC(),
            self.NOSTRETCH(),
            self.WUPEN(),
            self.GCEN(),
            self.SMBHEN(),
            self.SMBDEN(),
            self.ALERTEN(),
            self.PECEN(),
            self.FMP(),
            self.ADDRACLR(),
            self.STOPFACLR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_CR2(pub u32);
impl I2C_CR2 {
    #[doc = "Slave address (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn SADD(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Slave address (master mode)"]
    #[inline(always)]
    pub const fn set_SADD(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Transfer direction (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn RD_WRN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer direction (master mode)"]
    #[inline(always)]
    pub const fn set_RD_WRN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "10-bit addressing mode (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn ADD10(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub const fn set_ADD10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "10-bit address header only read direction (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn HEAD10R(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "10-bit address header only read direction (master mode)"]
    #[inline(always)]
    pub const fn set_HEAD10R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "START generation"]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "START generation"]
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "STOP generation (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "STOP generation (master mode)"]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "NACK generation (slave mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn NACK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NACK generation (slave mode)"]
    #[inline(always)]
    pub const fn set_NACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Number of bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn NBYTES(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of bytes"]
    #[inline(always)]
    pub const fn set_NBYTES(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NBYTES reload mode"]
    #[must_use]
    #[inline(always)]
    pub const fn RELOAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES reload mode"]
    #[inline(always)]
    pub const fn set_RELOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Automatic end mode (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOEND(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic end mode (master mode)"]
    #[inline(always)]
    pub const fn set_AUTOEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Packet error checking byte"]
    #[must_use]
    #[inline(always)]
    pub const fn PECBYTE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Packet error checking byte"]
    #[inline(always)]
    pub const fn set_PECBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for I2C_CR2 {
    #[inline(always)]
    fn default() -> I2C_CR2 {
        I2C_CR2(0)
    }
}
impl core::fmt::Debug for I2C_CR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR2")
            .field("SADD", &self.SADD())
            .field("RD_WRN", &self.RD_WRN())
            .field("ADD10", &self.ADD10())
            .field("HEAD10R", &self.HEAD10R())
            .field("START", &self.START())
            .field("STOP", &self.STOP())
            .field("NACK", &self.NACK())
            .field("NBYTES", &self.NBYTES())
            .field("RELOAD", &self.RELOAD())
            .field("AUTOEND", &self.AUTOEND())
            .field("PECBYTE", &self.PECBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_CR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_CR2 {{ SADD: {=u16:?}, RD_WRN: {=bool:?}, ADD10: {=bool:?}, HEAD10R: {=bool:?}, START: {=bool:?}, STOP: {=bool:?}, NACK: {=bool:?}, NBYTES: {=u8:?}, RELOAD: {=bool:?}, AUTOEND: {=bool:?}, PECBYTE: {=bool:?} }}",
            self.SADD(),
            self.RD_WRN(),
            self.ADD10(),
            self.HEAD10R(),
            self.START(),
            self.STOP(),
            self.NACK(),
            self.NBYTES(),
            self.RELOAD(),
            self.AUTOEND(),
            self.PECBYTE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_ICR(pub u32);
impl I2C_ICR {
    #[doc = "Address matched clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRCF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address matched clear flag"]
    #[inline(always)]
    pub const fn set_ADDRCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NACK clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn NACKCF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK clear flag"]
    #[inline(always)]
    pub const fn set_NACKCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "STOP detection clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STOPCF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection clear flag"]
    #[inline(always)]
    pub const fn set_STOPCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Bus error clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BERRCF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error clear flag"]
    #[inline(always)]
    pub const fn set_BERRCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration lost clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ARLOCF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost clear flag"]
    #[inline(always)]
    pub const fn set_ARLOCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun/Underrun clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRCF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/Underrun clear flag"]
    #[inline(always)]
    pub const fn set_OVRCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PAC error clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PECCF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PAC error clear flag"]
    #[inline(always)]
    pub const fn set_PECCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timeout clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMOUTCF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout clear flag"]
    #[inline(always)]
    pub const fn set_TIMOUTCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Alert clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ALERTCF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Alert clear flag"]
    #[inline(always)]
    pub const fn set_ALERTCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for I2C_ICR {
    #[inline(always)]
    fn default() -> I2C_ICR {
        I2C_ICR(0)
    }
}
impl core::fmt::Debug for I2C_ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_ICR")
            .field("ADDRCF", &self.ADDRCF())
            .field("NACKCF", &self.NACKCF())
            .field("STOPCF", &self.STOPCF())
            .field("BERRCF", &self.BERRCF())
            .field("ARLOCF", &self.ARLOCF())
            .field("OVRCF", &self.OVRCF())
            .field("PECCF", &self.PECCF())
            .field("TIMOUTCF", &self.TIMOUTCF())
            .field("ALERTCF", &self.ALERTCF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_ICR {{ ADDRCF: {=bool:?}, NACKCF: {=bool:?}, STOPCF: {=bool:?}, BERRCF: {=bool:?}, ARLOCF: {=bool:?}, OVRCF: {=bool:?}, PECCF: {=bool:?}, TIMOUTCF: {=bool:?}, ALERTCF: {=bool:?} }}",
            self.ADDRCF(),
            self.NACKCF(),
            self.STOPCF(),
            self.BERRCF(),
            self.ARLOCF(),
            self.OVRCF(),
            self.PECCF(),
            self.TIMOUTCF(),
            self.ALERTCF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_IPIDR(pub u32);
impl I2C_IPIDR {
    #[doc = "IP Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP Identifier"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for I2C_IPIDR {
    #[inline(always)]
    fn default() -> I2C_IPIDR {
        I2C_IPIDR(0)
    }
}
impl core::fmt::Debug for I2C_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_IPIDR").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I2C_IPIDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_ISR(pub u32);
impl I2C_ISR {
    #[doc = "Transmit data register empty"]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit data register empty"]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn TXIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt status"]
    #[inline(always)]
    pub const fn set_TXIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive data register not empty"]
    #[must_use]
    #[inline(always)]
    pub const fn RXNE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data register not empty"]
    #[inline(always)]
    pub const fn set_RXNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Address matched (slave mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Address matched (slave mode)"]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "NACK received flag"]
    #[must_use]
    #[inline(always)]
    pub const fn NACKF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NACK received flag"]
    #[inline(always)]
    pub const fn set_NACKF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "STOP detection flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STOPF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "STOP detection flag"]
    #[inline(always)]
    pub const fn set_STOPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn TC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete (master mode)"]
    #[inline(always)]
    pub const fn set_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer complete reload"]
    #[must_use]
    #[inline(always)]
    pub const fn TCR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete reload"]
    #[inline(always)]
    pub const fn set_TCR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn BERR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error"]
    #[inline(always)]
    pub const fn set_BERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration lost"]
    #[must_use]
    #[inline(always)]
    pub const fn ARLO(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost"]
    #[inline(always)]
    pub const fn set_ARLO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun/Underrun"]
    #[must_use]
    #[inline(always)]
    pub const fn OVR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun/Underrun"]
    #[inline(always)]
    pub const fn set_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PEC error in reception"]
    #[must_use]
    #[inline(always)]
    pub const fn PECERR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PEC error in reception"]
    #[inline(always)]
    pub const fn set_PECERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timeout or Tlow detection flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout or Tlow detection flag"]
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SMBus alert"]
    #[must_use]
    #[inline(always)]
    pub const fn ALERT(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SMBus alert"]
    #[inline(always)]
    pub const fn set_ALERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bus busy"]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Transfer direction (slave mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer direction (slave mode)"]
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Address match code (slave mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDCODE(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[doc = "Address match code (slave mode)"]
    #[inline(always)]
    pub const fn set_ADDCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
}
impl Default for I2C_ISR {
    #[inline(always)]
    fn default() -> I2C_ISR {
        I2C_ISR(0)
    }
}
impl core::fmt::Debug for I2C_ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_ISR")
            .field("TXE", &self.TXE())
            .field("TXIS", &self.TXIS())
            .field("RXNE", &self.RXNE())
            .field("ADDR", &self.ADDR())
            .field("NACKF", &self.NACKF())
            .field("STOPF", &self.STOPF())
            .field("TC", &self.TC())
            .field("TCR", &self.TCR())
            .field("BERR", &self.BERR())
            .field("ARLO", &self.ARLO())
            .field("OVR", &self.OVR())
            .field("PECERR", &self.PECERR())
            .field("TIMEOUT", &self.TIMEOUT())
            .field("ALERT", &self.ALERT())
            .field("BUSY", &self.BUSY())
            .field("DIR", &self.DIR())
            .field("ADDCODE", &self.ADDCODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_ISR {{ TXE: {=bool:?}, TXIS: {=bool:?}, RXNE: {=bool:?}, ADDR: {=bool:?}, NACKF: {=bool:?}, STOPF: {=bool:?}, TC: {=bool:?}, TCR: {=bool:?}, BERR: {=bool:?}, ARLO: {=bool:?}, OVR: {=bool:?}, PECERR: {=bool:?}, TIMEOUT: {=bool:?}, ALERT: {=bool:?}, BUSY: {=bool:?}, DIR: {=bool:?}, ADDCODE: {=u8:?} }}",
            self.TXE(),
            self.TXIS(),
            self.RXNE(),
            self.ADDR(),
            self.NACKF(),
            self.STOPF(),
            self.TC(),
            self.TCR(),
            self.BERR(),
            self.ARLO(),
            self.OVR(),
            self.PECERR(),
            self.TIMEOUT(),
            self.ALERT(),
            self.BUSY(),
            self.DIR(),
            self.ADDCODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_OAR1(pub u32);
impl I2C_OAR1 {
    #[doc = "Interface own address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn OA1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interface own address 1"]
    #[inline(always)]
    pub const fn set_OA1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Own address 1 10-bit mode"]
    #[must_use]
    #[inline(always)]
    pub const fn OA1MODE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Own address 1 10-bit mode"]
    #[inline(always)]
    pub const fn set_OA1MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Own address 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OA1EN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Own address 1 enable"]
    #[inline(always)]
    pub const fn set_OA1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for I2C_OAR1 {
    #[inline(always)]
    fn default() -> I2C_OAR1 {
        I2C_OAR1(0)
    }
}
impl core::fmt::Debug for I2C_OAR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_OAR1")
            .field("OA1", &self.OA1())
            .field("OA1MODE", &self.OA1MODE())
            .field("OA1EN", &self.OA1EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_OAR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_OAR1 {{ OA1: {=u16:?}, OA1MODE: {=bool:?}, OA1EN: {=bool:?} }}",
            self.OA1(),
            self.OA1MODE(),
            self.OA1EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_OAR2(pub u32);
impl I2C_OAR2 {
    #[doc = "Interface own address 2"]
    #[must_use]
    #[inline(always)]
    pub const fn OA2(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Interface own address 2"]
    #[inline(always)]
    pub const fn set_OA2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Own address 2 masks"]
    #[must_use]
    #[inline(always)]
    pub const fn OA2MSK(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Own address 2 masks"]
    #[inline(always)]
    pub const fn set_OA2MSK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Own address 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OA2EN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Own address 2 enable"]
    #[inline(always)]
    pub const fn set_OA2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for I2C_OAR2 {
    #[inline(always)]
    fn default() -> I2C_OAR2 {
        I2C_OAR2(0)
    }
}
impl core::fmt::Debug for I2C_OAR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_OAR2")
            .field("OA2", &self.OA2())
            .field("OA2MSK", &self.OA2MSK())
            .field("OA2EN", &self.OA2EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_OAR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_OAR2 {{ OA2: {=u8:?}, OA2MSK: {=u8:?}, OA2EN: {=bool:?} }}",
            self.OA2(),
            self.OA2MSK(),
            self.OA2EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_PECR(pub u32);
impl I2C_PECR {
    #[doc = "PEC register"]
    #[must_use]
    #[inline(always)]
    pub const fn PEC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PEC register"]
    #[inline(always)]
    pub const fn set_PEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I2C_PECR {
    #[inline(always)]
    fn default() -> I2C_PECR {
        I2C_PECR(0)
    }
}
impl core::fmt::Debug for I2C_PECR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_PECR")
            .field("PEC", &self.PEC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_PECR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I2C_PECR {{ PEC: {=u8:?} }}", self.PEC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_RXDR(pub u32);
impl I2C_RXDR {
    #[doc = "8-bit receive data"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit receive data"]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I2C_RXDR {
    #[inline(always)]
    fn default() -> I2C_RXDR {
        I2C_RXDR(0)
    }
}
impl core::fmt::Debug for I2C_RXDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_RXDR")
            .field("RXDATA", &self.RXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_RXDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I2C_RXDR {{ RXDATA: {=u8:?} }}", self.RXDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_SIDR(pub u32);
impl I2C_SIDR {
    #[doc = "Size Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size Identifier"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for I2C_SIDR {
    #[inline(always)]
    fn default() -> I2C_SIDR {
        I2C_SIDR(0)
    }
}
impl core::fmt::Debug for I2C_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I2C_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_TIMEOUTR(pub u32);
impl I2C_TIMEOUTR {
    #[doc = "Bus timeout A"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUTA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus timeout A"]
    #[inline(always)]
    pub const fn set_TIMEOUTA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Idle clock timeout detection"]
    #[must_use]
    #[inline(always)]
    pub const fn TIDLE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Idle clock timeout detection"]
    #[inline(always)]
    pub const fn set_TIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clock timeout enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMOUTEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Clock timeout enable"]
    #[inline(always)]
    pub const fn set_TIMOUTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Bus timeout B"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUTB(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus timeout B"]
    #[inline(always)]
    pub const fn set_TIMEOUTB(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Extended clock timeout enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEXTEN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended clock timeout enable"]
    #[inline(always)]
    pub const fn set_TEXTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I2C_TIMEOUTR {
    #[inline(always)]
    fn default() -> I2C_TIMEOUTR {
        I2C_TIMEOUTR(0)
    }
}
impl core::fmt::Debug for I2C_TIMEOUTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TIMEOUTR")
            .field("TIMEOUTA", &self.TIMEOUTA())
            .field("TIDLE", &self.TIDLE())
            .field("TIMOUTEN", &self.TIMOUTEN())
            .field("TIMEOUTB", &self.TIMEOUTB())
            .field("TEXTEN", &self.TEXTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_TIMEOUTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_TIMEOUTR {{ TIMEOUTA: {=u16:?}, TIDLE: {=bool:?}, TIMOUTEN: {=bool:?}, TIMEOUTB: {=u16:?}, TEXTEN: {=bool:?} }}",
            self.TIMEOUTA(),
            self.TIDLE(),
            self.TIMOUTEN(),
            self.TIMEOUTB(),
            self.TEXTEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_TIMINGR(pub u32);
impl I2C_TIMINGR {
    #[doc = "SCL low period (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn SCLL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCL low period (master mode)"]
    #[inline(always)]
    pub const fn set_SCLL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCL high period (master mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn SCLH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCL high period (master mode)"]
    #[inline(always)]
    pub const fn set_SCLH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn SDADEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Data hold time"]
    #[inline(always)]
    pub const fn set_SDADEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Data setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn SCLDEL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Data setup time"]
    #[inline(always)]
    pub const fn set_SCLDEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Timings prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn PRESC(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Timings prescaler"]
    #[inline(always)]
    pub const fn set_PRESC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for I2C_TIMINGR {
    #[inline(always)]
    fn default() -> I2C_TIMINGR {
        I2C_TIMINGR(0)
    }
}
impl core::fmt::Debug for I2C_TIMINGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TIMINGR")
            .field("SCLL", &self.SCLL())
            .field("SCLH", &self.SCLH())
            .field("SDADEL", &self.SDADEL())
            .field("SCLDEL", &self.SCLDEL())
            .field("PRESC", &self.PRESC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_TIMINGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_TIMINGR {{ SCLL: {=u8:?}, SCLH: {=u8:?}, SDADEL: {=u8:?}, SCLDEL: {=u8:?}, PRESC: {=u8:?} }}",
            self.SCLL(),
            self.SCLH(),
            self.SDADEL(),
            self.SCLDEL(),
            self.PRESC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_TXDR(pub u32);
impl I2C_TXDR {
    #[doc = "8-bit transmit data"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit transmit data"]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I2C_TXDR {
    #[inline(always)]
    fn default() -> I2C_TXDR {
        I2C_TXDR(0)
    }
}
impl core::fmt::Debug for I2C_TXDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TXDR")
            .field("TXDATA", &self.TXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_TXDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I2C_TXDR {{ TXDATA: {=u8:?} }}", self.TXDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C_VERR(pub u32);
impl I2C_VERR {
    #[doc = "Minor Revision of the IP"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Revision of the IP"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Major Revision of the IP"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Revision of the IP"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for I2C_VERR {
    #[inline(always)]
    fn default() -> I2C_VERR {
        I2C_VERR(0)
    }
}
impl core::fmt::Debug for I2C_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I2C_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I2C_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}

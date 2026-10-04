#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI {
    ptr: *mut u8,
}
unsafe impl Send for SPI {}
unsafe impl Sync for SPI {}
impl SPI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPI/I2S Control register 1"]
    #[inline(always)]
    pub const fn CR1(self) -> crate::common::Reg<SPI_CR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SPI Control register 2"]
    #[inline(always)]
    pub const fn CR2(self) -> crate::common::Reg<SPI_CR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SPI Configuration register 1"]
    #[inline(always)]
    pub const fn CFG1(self) -> crate::common::Reg<SPI_CFG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SPI Configuration register 2"]
    #[inline(always)]
    pub const fn CFG2(self) -> crate::common::Reg<SPI_CFG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPI/I2S Interrupt Enable register"]
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<SPI_IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SPI Status register"]
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<SPI_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SPI/I2S Interrupt/Status Flags Clear register"]
    #[inline(always)]
    pub const fn IFCR(self) -> crate::common::Reg<SPI_IFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SPI/I2S Autonomous Mode Control register"]
    #[inline(always)]
    pub const fn AUTOCR(self) -> crate::common::Reg<SPI_AUTOCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPI/I2S Transmit data register"]
    #[inline(always)]
    pub const fn TXDR(self) -> crate::common::Reg<SPI_TXDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SPI/I2S Receive data register"]
    #[inline(always)]
    pub const fn RXDR(self) -> crate::common::Reg<SPI_RXDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SPI CRC Polynomial register"]
    #[inline(always)]
    pub const fn CRCPOLY(self) -> crate::common::Reg<SPI_CRCPOLY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SPI Transmitter CRC register"]
    #[inline(always)]
    pub const fn TXCRC(self) -> crate::common::Reg<SPI_TXCRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SPI Receiver CRC register"]
    #[inline(always)]
    pub const fn RXCRC(self) -> crate::common::Reg<SPI_RXCRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SPI Underrun data register"]
    #[inline(always)]
    pub const fn UDRDR(self) -> crate::common::Reg<SPI_UDRDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SPI/I2S configuration register"]
    #[inline(always)]
    pub const fn I2SCFGR(self) -> crate::common::Reg<SPI_I2SCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SPI/I2S hardware configuration register 2"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<SPI_HWCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "SPI/I2S hardware configuration register 1"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<SPI_HWCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "SPI/I2S Version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<SPI_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "SPI/I2S Identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<SPI_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "SPI/I2S Size Identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<SPI_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_AUTOCR(pub u32);
impl SPI_AUTOCR {
    #[doc = "CTRIGSEL \\[3:0\\]: Trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGSEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CTRIGSEL \\[3:0\\]: Trigger selection"]
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
    #[doc = "Trigger of CSTART control enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger of CSTART control enable"]
    #[inline(always)]
    pub const fn set_TRIGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for SPI_AUTOCR {
    #[inline(always)]
    fn default() -> SPI_AUTOCR {
        SPI_AUTOCR(0)
    }
}
impl core::fmt::Debug for SPI_AUTOCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_AUTOCR")
            .field("TRIGSEL", &self.TRIGSEL())
            .field("TRIGPOL", &self.TRIGPOL())
            .field("TRIGEN", &self.TRIGEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_AUTOCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_AUTOCR {{ TRIGSEL: {=u8:?}, TRIGPOL: {=bool:?}, TRIGEN: {=bool:?} }}",
            self.TRIGSEL(),
            self.TRIGPOL(),
            self.TRIGEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_CFG1(pub u32);
impl SPI_CFG1 {
    #[doc = "DSIZE\\[4:0\\]: Bits number in single SPI data frame"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DSIZE\\[4:0\\]: Bits number in single SPI data frame"]
    #[inline(always)]
    pub const fn set_DSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "FTHVL \\[3:0\\]: FIFO threshold level"]
    #[must_use]
    #[inline(always)]
    pub const fn FTHLV(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "FTHVL \\[3:0\\]: FIFO threshold level"]
    #[inline(always)]
    pub const fn set_FTHLV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "Behavior of Slave transmitter at underrun condition"]
    #[must_use]
    #[inline(always)]
    pub const fn UDRCFG(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Behavior of Slave transmitter at underrun condition"]
    #[inline(always)]
    pub const fn set_UDRCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Rx DMA stream enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAEN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Rx DMA stream enable"]
    #[inline(always)]
    pub const fn set_RXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx DMA stream enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx DMA stream enable"]
    #[inline(always)]
    pub const fn set_TXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CRCSIZE \\[4:0\\]: Length of CRC frame to be transacted and compared"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCSIZE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "CRCSIZE \\[4:0\\]: Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub const fn set_CRCSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Hardware CRC computation enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware CRC computation enable"]
    #[inline(always)]
    pub const fn set_CRCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Master baud rate prescaler setting"]
    #[must_use]
    #[inline(always)]
    pub const fn MBR(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Master baud rate prescaler setting"]
    #[inline(always)]
    pub const fn set_MBR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "Bypass of the prescaler at master baud rate clock generator"]
    #[must_use]
    #[inline(always)]
    pub const fn BPASS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the prescaler at master baud rate clock generator"]
    #[inline(always)]
    pub const fn set_BPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SPI_CFG1 {
    #[inline(always)]
    fn default() -> SPI_CFG1 {
        SPI_CFG1(0)
    }
}
impl core::fmt::Debug for SPI_CFG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CFG1")
            .field("DSIZE", &self.DSIZE())
            .field("FTHLV", &self.FTHLV())
            .field("UDRCFG", &self.UDRCFG())
            .field("RXDMAEN", &self.RXDMAEN())
            .field("TXDMAEN", &self.TXDMAEN())
            .field("CRCSIZE", &self.CRCSIZE())
            .field("CRCEN", &self.CRCEN())
            .field("MBR", &self.MBR())
            .field("BPASS", &self.BPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_CFG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_CFG1 {{ DSIZE: {=u8:?}, FTHLV: {=u8:?}, UDRCFG: {=bool:?}, RXDMAEN: {=bool:?}, TXDMAEN: {=bool:?}, CRCSIZE: {=u8:?}, CRCEN: {=bool:?}, MBR: {=u8:?}, BPASS: {=bool:?} }}",
            self.DSIZE(),
            self.FTHLV(),
            self.UDRCFG(),
            self.RXDMAEN(),
            self.TXDMAEN(),
            self.CRCSIZE(),
            self.CRCEN(),
            self.MBR(),
            self.BPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_CFG2(pub u32);
impl SPI_CFG2 {
    #[doc = "Master SS Idleness"]
    #[must_use]
    #[inline(always)]
    pub const fn MSSI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Master SS Idleness"]
    #[inline(always)]
    pub const fn set_MSSI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Master Inter-Data Idleness"]
    #[must_use]
    #[inline(always)]
    pub const fn MIDI(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Master Inter-Data Idleness"]
    #[inline(always)]
    pub const fn set_MIDI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "RDY signal input master management"]
    #[must_use]
    #[inline(always)]
    pub const fn RDIMM(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "RDY signal input master management"]
    #[inline(always)]
    pub const fn set_RDIMM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "RDY signal input/output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn RDIOP(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RDY signal input/output polarity"]
    #[inline(always)]
    pub const fn set_RDIOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Swap functionality of MISO and MOSI pins"]
    #[must_use]
    #[inline(always)]
    pub const fn IOSWP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub const fn set_IOSWP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "COMM \\[1:0\\]: SPI Communication Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn COMM(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "COMM \\[1:0\\]: SPI Communication Mode"]
    #[inline(always)]
    pub const fn set_COMM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "SP\\[2:0\\]: Serial Protocol"]
    #[must_use]
    #[inline(always)]
    pub const fn SP(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "SP\\[2:0\\]: Serial Protocol"]
    #[inline(always)]
    pub const fn set_SP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "SPI Master"]
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Data frame format"]
    #[must_use]
    #[inline(always)]
    pub const fn LSBFRST(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Data frame format"]
    #[inline(always)]
    pub const fn set_LSBFRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Clock Phase"]
    #[must_use]
    #[inline(always)]
    pub const fn CPHA(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Phase"]
    #[inline(always)]
    pub const fn set_CPHA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CPOL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Polarity"]
    #[inline(always)]
    pub const fn set_CPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Software management of SS signal input"]
    #[must_use]
    #[inline(always)]
    pub const fn SSM(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Software management of SS signal input"]
    #[inline(always)]
    pub const fn set_SSM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SS input/output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn SSIOP(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "SS input/output polarity"]
    #[inline(always)]
    pub const fn set_SSIOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "SS output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSOE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "SS output enable"]
    #[inline(always)]
    pub const fn set_SSOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "SS output management in master mode"]
    #[must_use]
    #[inline(always)]
    pub const fn SSOM(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SS output management in master mode"]
    #[inline(always)]
    pub const fn set_SSOM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Alternate function GPIOs control"]
    #[must_use]
    #[inline(always)]
    pub const fn AFCNTR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate function GPIOs control"]
    #[inline(always)]
    pub const fn set_AFCNTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SPI_CFG2 {
    #[inline(always)]
    fn default() -> SPI_CFG2 {
        SPI_CFG2(0)
    }
}
impl core::fmt::Debug for SPI_CFG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CFG2")
            .field("MSSI", &self.MSSI())
            .field("MIDI", &self.MIDI())
            .field("RDIMM", &self.RDIMM())
            .field("RDIOP", &self.RDIOP())
            .field("IOSWP", &self.IOSWP())
            .field("COMM", &self.COMM())
            .field("SP", &self.SP())
            .field("MASTER", &self.MASTER())
            .field("LSBFRST", &self.LSBFRST())
            .field("CPHA", &self.CPHA())
            .field("CPOL", &self.CPOL())
            .field("SSM", &self.SSM())
            .field("SSIOP", &self.SSIOP())
            .field("SSOE", &self.SSOE())
            .field("SSOM", &self.SSOM())
            .field("AFCNTR", &self.AFCNTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_CFG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_CFG2 {{ MSSI: {=u8:?}, MIDI: {=u8:?}, RDIMM: {=bool:?}, RDIOP: {=bool:?}, IOSWP: {=bool:?}, COMM: {=u8:?}, SP: {=u8:?}, MASTER: {=bool:?}, LSBFRST: {=bool:?}, CPHA: {=bool:?}, CPOL: {=bool:?}, SSM: {=bool:?}, SSIOP: {=bool:?}, SSOE: {=bool:?}, SSOM: {=bool:?}, AFCNTR: {=bool:?} }}",
            self.MSSI(),
            self.MIDI(),
            self.RDIMM(),
            self.RDIOP(),
            self.IOSWP(),
            self.COMM(),
            self.SP(),
            self.MASTER(),
            self.LSBFRST(),
            self.CPHA(),
            self.CPOL(),
            self.SSM(),
            self.SSIOP(),
            self.SSOE(),
            self.SSOM(),
            self.AFCNTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_CR1(pub u32);
impl SPI_CR1 {
    #[doc = "Serial Peripheral Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Peripheral Enable"]
    #[inline(always)]
    pub const fn set_SPE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master automatic suspension in Receive mode"]
    #[must_use]
    #[inline(always)]
    pub const fn MASRX(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master automatic suspension in Receive mode"]
    #[inline(always)]
    pub const fn set_MASRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Master transfer start"]
    #[must_use]
    #[inline(always)]
    pub const fn CSTART(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Master transfer start"]
    #[inline(always)]
    pub const fn set_CSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Master SUSPend request"]
    #[must_use]
    #[inline(always)]
    pub const fn CSUSP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Master SUSPend request"]
    #[inline(always)]
    pub const fn set_CSUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Rx/Tx direction at Half-duplex mode"]
    #[must_use]
    #[inline(always)]
    pub const fn HDDIR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub const fn set_HDDIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Internal SS signal input level"]
    #[must_use]
    #[inline(always)]
    pub const fn SSI(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Internal SS signal input level"]
    #[inline(always)]
    pub const fn set_SSI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "32-bit CRC polynomial configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn CRC33_17(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub const fn set_CRC33_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CRC calculation initialization pattern control for receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn RCRCINI(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub const fn set_RCRCINI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CRC calculation initialization pattern control for transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn TCRCINI(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub const fn set_TCRCINI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Locking the AF configuration of associated IOs"]
    #[must_use]
    #[inline(always)]
    pub const fn IOLOCK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub const fn set_IOLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for SPI_CR1 {
    #[inline(always)]
    fn default() -> SPI_CR1 {
        SPI_CR1(0)
    }
}
impl core::fmt::Debug for SPI_CR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CR1")
            .field("SPE", &self.SPE())
            .field("MASRX", &self.MASRX())
            .field("CSTART", &self.CSTART())
            .field("CSUSP", &self.CSUSP())
            .field("HDDIR", &self.HDDIR())
            .field("SSI", &self.SSI())
            .field("CRC33_17", &self.CRC33_17())
            .field("RCRCINI", &self.RCRCINI())
            .field("TCRCINI", &self.TCRCINI())
            .field("IOLOCK", &self.IOLOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_CR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_CR1 {{ SPE: {=bool:?}, MASRX: {=bool:?}, CSTART: {=bool:?}, CSUSP: {=bool:?}, HDDIR: {=bool:?}, SSI: {=bool:?}, CRC33_17: {=bool:?}, RCRCINI: {=bool:?}, TCRCINI: {=bool:?}, IOLOCK: {=bool:?} }}",
            self.SPE(),
            self.MASRX(),
            self.CSTART(),
            self.CSUSP(),
            self.HDDIR(),
            self.SSI(),
            self.CRC33_17(),
            self.RCRCINI(),
            self.TCRCINI(),
            self.IOLOCK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_CR2(pub u32);
impl SPI_CR2 {
    #[doc = "Number of data at current transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn TSIZE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data at current transfer"]
    #[inline(always)]
    pub const fn set_TSIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SPI_CR2 {
    #[inline(always)]
    fn default() -> SPI_CR2 {
        SPI_CR2(0)
    }
}
impl core::fmt::Debug for SPI_CR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CR2")
            .field("TSIZE", &self.TSIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_CR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_CR2 {{ TSIZE: {=u16:?} }}", self.TSIZE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_CRCPOLY(pub u32);
impl SPI_CRCPOLY {
    #[doc = "CRC Polynomial register"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCPOLY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CRC Polynomial register"]
    #[inline(always)]
    pub const fn set_CRCPOLY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_CRCPOLY {
    #[inline(always)]
    fn default() -> SPI_CRCPOLY {
        SPI_CRCPOLY(0)
    }
}
impl core::fmt::Debug for SPI_CRCPOLY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CRCPOLY")
            .field("CRCPOLY", &self.CRCPOLY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_CRCPOLY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_CRCPOLY {{ CRCPOLY: {=u32:?} }}", self.CRCPOLY())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_HWCFGR1(pub u32);
impl SPI_HWCFGR1 {
    #[doc = "TxFIFO size"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFCFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "TxFIFO size"]
    #[inline(always)]
    pub const fn set_TXFCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RxFIFO size"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFCFG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "RxFIFO size"]
    #[inline(always)]
    pub const fn set_RXFCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "CRC configuration for SPI"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCCFG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "CRC configuration for SPI"]
    #[inline(always)]
    pub const fn set_CRCCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "I2S configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn I2SCFG(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "I2S configuration"]
    #[inline(always)]
    pub const fn set_I2SCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "SPI data size configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn DSCFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "SPI data size configuration"]
    #[inline(always)]
    pub const fn set_DSCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Autonomous trigger feature configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn TRGCFG(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Autonomous trigger feature configuration"]
    #[inline(always)]
    pub const fn set_TRGCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "SPI2S full feature version configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn FULLCFG(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "SPI2S full feature version configuration"]
    #[inline(always)]
    pub const fn set_FULLCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for SPI_HWCFGR1 {
    #[inline(always)]
    fn default() -> SPI_HWCFGR1 {
        SPI_HWCFGR1(0)
    }
}
impl core::fmt::Debug for SPI_HWCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_HWCFGR1")
            .field("TXFCFG", &self.TXFCFG())
            .field("RXFCFG", &self.RXFCFG())
            .field("CRCCFG", &self.CRCCFG())
            .field("I2SCFG", &self.I2SCFG())
            .field("DSCFG", &self.DSCFG())
            .field("TRGCFG", &self.TRGCFG())
            .field("FULLCFG", &self.FULLCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_HWCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_HWCFGR1 {{ TXFCFG: {=u8:?}, RXFCFG: {=u8:?}, CRCCFG: {=u8:?}, I2SCFG: {=u8:?}, DSCFG: {=u8:?}, TRGCFG: {=u8:?}, FULLCFG: {=u8:?} }}",
            self.TXFCFG(),
            self.RXFCFG(),
            self.CRCCFG(),
            self.I2SCFG(),
            self.DSCFG(),
            self.TRGCFG(),
            self.FULLCFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_HWCFGR2(pub u32);
impl SPI_HWCFGR2 {
    #[doc = "Number of option bits available 0 to 31"]
    #[must_use]
    #[inline(always)]
    pub const fn OPBCFG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of option bits available 0 to 31"]
    #[inline(always)]
    pub const fn set_OPBCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SPI_HWCFGR2 {
    #[inline(always)]
    fn default() -> SPI_HWCFGR2 {
        SPI_HWCFGR2(0)
    }
}
impl core::fmt::Debug for SPI_HWCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_HWCFGR2")
            .field("OPBCFG", &self.OPBCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_HWCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_HWCFGR2 {{ OPBCFG: {=u8:?} }}", self.OPBCFG())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_I2SCFGR(pub u32);
impl SPI_I2SCFGR {
    #[doc = "I2S mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn I2SMOD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2S mode selection"]
    #[inline(always)]
    pub const fn set_I2SMOD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2SCFGR\\[1:0\\] bits (I2S configuration mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn I2SCFG(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "I2SCFGR\\[1:0\\] bits (I2S configuration mode)"]
    #[inline(always)]
    pub const fn set_I2SCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "I2SSTD\\[1:0\\] I2S standard selection"]
    #[must_use]
    #[inline(always)]
    pub const fn I2SSTD(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "I2SSTD\\[1:0\\] I2S standard selection"]
    #[inline(always)]
    pub const fn set_I2SSTD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PCM frame synchronization"]
    #[must_use]
    #[inline(always)]
    pub const fn PCMSYNC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PCM frame synchronization"]
    #[inline(always)]
    pub const fn set_PCMSYNC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DATLEN\\[1:0\\] Data length to be transferred"]
    #[must_use]
    #[inline(always)]
    pub const fn DATLEN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "DATLEN\\[1:0\\] Data length to be transferred"]
    #[inline(always)]
    pub const fn set_DATLEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Channel length (number of bits per audio channel)"]
    #[must_use]
    #[inline(always)]
    pub const fn CHLEN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub const fn set_CHLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Serial audio clock polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CKPOL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Serial audio clock polarity"]
    #[inline(always)]
    pub const fn set_CKPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fixed channel length in SLAVE"]
    #[must_use]
    #[inline(always)]
    pub const fn FIXCH(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed channel length in SLAVE"]
    #[inline(always)]
    pub const fn set_FIXCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Word select inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn WSINV(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Word select inversion"]
    #[inline(always)]
    pub const fn set_WSINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data format"]
    #[must_use]
    #[inline(always)]
    pub const fn DATFMT(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data format"]
    #[inline(always)]
    pub const fn set_DATFMT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "I2S Linear prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn I2SDIV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "I2S Linear prescaler"]
    #[inline(always)]
    pub const fn set_I2SDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Odd factor for the prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ODD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Odd factor for the prescaler"]
    #[inline(always)]
    pub const fn set_ODD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Master Clock Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MCKOE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Master Clock Output Enable"]
    #[inline(always)]
    pub const fn set_MCKOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for SPI_I2SCFGR {
    #[inline(always)]
    fn default() -> SPI_I2SCFGR {
        SPI_I2SCFGR(0)
    }
}
impl core::fmt::Debug for SPI_I2SCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_I2SCFGR")
            .field("I2SMOD", &self.I2SMOD())
            .field("I2SCFG", &self.I2SCFG())
            .field("I2SSTD", &self.I2SSTD())
            .field("PCMSYNC", &self.PCMSYNC())
            .field("DATLEN", &self.DATLEN())
            .field("CHLEN", &self.CHLEN())
            .field("CKPOL", &self.CKPOL())
            .field("FIXCH", &self.FIXCH())
            .field("WSINV", &self.WSINV())
            .field("DATFMT", &self.DATFMT())
            .field("I2SDIV", &self.I2SDIV())
            .field("ODD", &self.ODD())
            .field("MCKOE", &self.MCKOE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_I2SCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_I2SCFGR {{ I2SMOD: {=bool:?}, I2SCFG: {=u8:?}, I2SSTD: {=u8:?}, PCMSYNC: {=bool:?}, DATLEN: {=u8:?}, CHLEN: {=bool:?}, CKPOL: {=bool:?}, FIXCH: {=bool:?}, WSINV: {=bool:?}, DATFMT: {=bool:?}, I2SDIV: {=u8:?}, ODD: {=bool:?}, MCKOE: {=bool:?} }}",
            self.I2SMOD(),
            self.I2SCFG(),
            self.I2SSTD(),
            self.PCMSYNC(),
            self.DATLEN(),
            self.CHLEN(),
            self.CKPOL(),
            self.FIXCH(),
            self.WSINV(),
            self.DATFMT(),
            self.I2SDIV(),
            self.ODD(),
            self.MCKOE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_IER(pub u32);
impl SPI_IER {
    #[doc = "RXP Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXPIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RXP Interrupt Enable"]
    #[inline(always)]
    pub const fn set_RXPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TXP interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXPIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TXP interrupt enable"]
    #[inline(always)]
    pub const fn set_TXPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DXP interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DXPIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DXP interrupt enable"]
    #[inline(always)]
    pub const fn set_DXPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "EOT/SUSP/TXC interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EOTIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EOT/SUSP/TXC interrupt enable"]
    #[inline(always)]
    pub const fn set_EOTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TXTF interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXTFIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TXTF interrupt enable"]
    #[inline(always)]
    pub const fn set_TXTFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "UDR interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UDRIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "UDR interrupt enable"]
    #[inline(always)]
    pub const fn set_UDRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OVR interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OVR interrupt enable"]
    #[inline(always)]
    pub const fn set_OVRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CRC error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCEIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CRC error interrupt enable"]
    #[inline(always)]
    pub const fn set_CRCEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TI Frame Error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIFREIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TI Frame Error interrupt enable"]
    #[inline(always)]
    pub const fn set_TIFREIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mode Fault interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MODFIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault interrupt enable"]
    #[inline(always)]
    pub const fn set_MODFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for SPI_IER {
    #[inline(always)]
    fn default() -> SPI_IER {
        SPI_IER(0)
    }
}
impl core::fmt::Debug for SPI_IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_IER")
            .field("RXPIE", &self.RXPIE())
            .field("TXPIE", &self.TXPIE())
            .field("DXPIE", &self.DXPIE())
            .field("EOTIE", &self.EOTIE())
            .field("TXTFIE", &self.TXTFIE())
            .field("UDRIE", &self.UDRIE())
            .field("OVRIE", &self.OVRIE())
            .field("CRCEIE", &self.CRCEIE())
            .field("TIFREIE", &self.TIFREIE())
            .field("MODFIE", &self.MODFIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_IER {{ RXPIE: {=bool:?}, TXPIE: {=bool:?}, DXPIE: {=bool:?}, EOTIE: {=bool:?}, TXTFIE: {=bool:?}, UDRIE: {=bool:?}, OVRIE: {=bool:?}, CRCEIE: {=bool:?}, TIFREIE: {=bool:?}, MODFIE: {=bool:?} }}",
            self.RXPIE(),
            self.TXPIE(),
            self.DXPIE(),
            self.EOTIE(),
            self.TXTFIE(),
            self.UDRIE(),
            self.OVRIE(),
            self.CRCEIE(),
            self.TIFREIE(),
            self.MODFIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_IFCR(pub u32);
impl SPI_IFCR {
    #[doc = "End Of Transfer flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn EOTC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "End Of Transfer flag clear"]
    #[inline(always)]
    pub const fn set_EOTC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmission Transfer Filled flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn TXTFC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Transfer Filled flag clear"]
    #[inline(always)]
    pub const fn set_TXTFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Underrun flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn UDRC(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun flag clear"]
    #[inline(always)]
    pub const fn set_UDRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overrun flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun flag clear"]
    #[inline(always)]
    pub const fn set_OVRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CRC Error flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCEC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error flag clear"]
    #[inline(always)]
    pub const fn set_CRCEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TI frame format error flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn TIFREC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TI frame format error flag clear"]
    #[inline(always)]
    pub const fn set_TIFREC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mode Fault flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn MODFC(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault flag clear"]
    #[inline(always)]
    pub const fn set_MODFC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SUSPend flag clear"]
    #[must_use]
    #[inline(always)]
    pub const fn SUSPC(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SUSPend flag clear"]
    #[inline(always)]
    pub const fn set_SUSPC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for SPI_IFCR {
    #[inline(always)]
    fn default() -> SPI_IFCR {
        SPI_IFCR(0)
    }
}
impl core::fmt::Debug for SPI_IFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_IFCR")
            .field("EOTC", &self.EOTC())
            .field("TXTFC", &self.TXTFC())
            .field("UDRC", &self.UDRC())
            .field("OVRC", &self.OVRC())
            .field("CRCEC", &self.CRCEC())
            .field("TIFREC", &self.TIFREC())
            .field("MODFC", &self.MODFC())
            .field("SUSPC", &self.SUSPC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_IFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_IFCR {{ EOTC: {=bool:?}, TXTFC: {=bool:?}, UDRC: {=bool:?}, OVRC: {=bool:?}, CRCEC: {=bool:?}, TIFREC: {=bool:?}, MODFC: {=bool:?}, SUSPC: {=bool:?} }}",
            self.EOTC(),
            self.TXTFC(),
            self.UDRC(),
            self.OVRC(),
            self.CRCEC(),
            self.TIFREC(),
            self.MODFC(),
            self.SUSPC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_IPIDR(pub u32);
impl SPI_IPIDR {
    #[doc = "MCD identification code which identifies the IP"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MCD identification code which identifies the IP"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_IPIDR {
    #[inline(always)]
    fn default() -> SPI_IPIDR {
        SPI_IPIDR(0)
    }
}
impl core::fmt::Debug for SPI_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_IPIDR").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_IPIDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_RXCRC(pub u32);
impl SPI_RXCRC {
    #[doc = "CRC Register for receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCRC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CRC Register for receiver"]
    #[inline(always)]
    pub const fn set_RXCRC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_RXCRC {
    #[inline(always)]
    fn default() -> SPI_RXCRC {
        SPI_RXCRC(0)
    }
}
impl core::fmt::Debug for SPI_RXCRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_RXCRC")
            .field("RXCRC", &self.RXCRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_RXCRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_RXCRC {{ RXCRC: {=u32:?} }}", self.RXCRC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_RXDR(pub u32);
impl SPI_RXDR {
    #[doc = "Receive Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn set_RXDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_RXDR {
    #[inline(always)]
    fn default() -> SPI_RXDR {
        SPI_RXDR(0)
    }
}
impl core::fmt::Debug for SPI_RXDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_RXDR")
            .field("RXDR", &self.RXDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_RXDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_RXDR {{ RXDR: {=u32:?} }}", self.RXDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_SIDR(pub u32);
impl SPI_SIDR {
    #[doc = "Size identification"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size identification"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_SIDR {
    #[inline(always)]
    fn default() -> SPI_SIDR {
        SPI_SIDR(0)
    }
}
impl core::fmt::Debug for SPI_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_SR(pub u32);
impl SPI_SR {
    #[doc = "Rx-Packet available"]
    #[must_use]
    #[inline(always)]
    pub const fn RXP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx-Packet available"]
    #[inline(always)]
    pub const fn set_RXP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tx-Packet space available"]
    #[must_use]
    #[inline(always)]
    pub const fn TXP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tx-Packet space available"]
    #[inline(always)]
    pub const fn set_TXP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Duplex Packet available"]
    #[must_use]
    #[inline(always)]
    pub const fn DXP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Duplex Packet available"]
    #[inline(always)]
    pub const fn set_DXP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "End of transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn EOT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "End of transfer"]
    #[inline(always)]
    pub const fn set_EOT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmission Transfer Filled"]
    #[must_use]
    #[inline(always)]
    pub const fn TXTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Transfer Filled"]
    #[inline(always)]
    pub const fn set_TXTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Underrun at Slave transmission mode"]
    #[must_use]
    #[inline(always)]
    pub const fn UDR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun at Slave transmission mode"]
    #[inline(always)]
    pub const fn set_UDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Overrun Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn OVR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error Detected"]
    #[inline(always)]
    pub const fn set_OVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CRC Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error Detected"]
    #[inline(always)]
    pub const fn set_CRCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TI frame format error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn TIFRE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TI frame format error Detected"]
    #[inline(always)]
    pub const fn set_TIFRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mode Fault Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn MODF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Mode Fault Detected"]
    #[inline(always)]
    pub const fn set_MODF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Suspension status, SUSP is set by hardware"]
    #[must_use]
    #[inline(always)]
    pub const fn SUSP(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Suspension status, SUSP is set by hardware"]
    #[inline(always)]
    pub const fn set_SUSP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TxFIFO transmission complete"]
    #[must_use]
    #[inline(always)]
    pub const fn TXC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TxFIFO transmission complete"]
    #[inline(always)]
    pub const fn set_TXC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "RxFIFO Packing Level"]
    #[must_use]
    #[inline(always)]
    pub const fn RXPLVL(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "RxFIFO Packing Level"]
    #[inline(always)]
    pub const fn set_RXPLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Rx FIFO Word Not Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn RXWNE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO Word Not Empty"]
    #[inline(always)]
    pub const fn set_RXWNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Number of data frames remaining in TSIZE"]
    #[must_use]
    #[inline(always)]
    pub const fn CTSIZE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data frames remaining in TSIZE"]
    #[inline(always)]
    pub const fn set_CTSIZE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SPI_SR {
    #[inline(always)]
    fn default() -> SPI_SR {
        SPI_SR(0)
    }
}
impl core::fmt::Debug for SPI_SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SR")
            .field("RXP", &self.RXP())
            .field("TXP", &self.TXP())
            .field("DXP", &self.DXP())
            .field("EOT", &self.EOT())
            .field("TXTF", &self.TXTF())
            .field("UDR", &self.UDR())
            .field("OVR", &self.OVR())
            .field("CRCE", &self.CRCE())
            .field("TIFRE", &self.TIFRE())
            .field("MODF", &self.MODF())
            .field("SUSP", &self.SUSP())
            .field("TXC", &self.TXC())
            .field("RXPLVL", &self.RXPLVL())
            .field("RXWNE", &self.RXWNE())
            .field("CTSIZE", &self.CTSIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_SR {{ RXP: {=bool:?}, TXP: {=bool:?}, DXP: {=bool:?}, EOT: {=bool:?}, TXTF: {=bool:?}, UDR: {=bool:?}, OVR: {=bool:?}, CRCE: {=bool:?}, TIFRE: {=bool:?}, MODF: {=bool:?}, SUSP: {=bool:?}, TXC: {=bool:?}, RXPLVL: {=u8:?}, RXWNE: {=bool:?}, CTSIZE: {=u16:?} }}",
            self.RXP(),
            self.TXP(),
            self.DXP(),
            self.EOT(),
            self.TXTF(),
            self.UDR(),
            self.OVR(),
            self.CRCE(),
            self.TIFRE(),
            self.MODF(),
            self.SUSP(),
            self.TXC(),
            self.RXPLVL(),
            self.RXWNE(),
            self.CTSIZE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_TXCRC(pub u32);
impl SPI_TXCRC {
    #[doc = "CRC Register for transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCRC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CRC Register for transmitter"]
    #[inline(always)]
    pub const fn set_TXCRC(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_TXCRC {
    #[inline(always)]
    fn default() -> SPI_TXCRC {
        SPI_TXCRC(0)
    }
}
impl core::fmt::Debug for SPI_TXCRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_TXCRC")
            .field("TXCRC", &self.TXCRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_TXCRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_TXCRC {{ TXCRC: {=u32:?} }}", self.TXCRC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_TXDR(pub u32);
impl SPI_TXDR {
    #[doc = "Transmit Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn set_TXDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_TXDR {
    #[inline(always)]
    fn default() -> SPI_TXDR {
        SPI_TXDR(0)
    }
}
impl core::fmt::Debug for SPI_TXDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_TXDR")
            .field("TXDR", &self.TXDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_TXDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_TXDR {{ TXDR: {=u32:?} }}", self.TXDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_UDRDR(pub u32);
impl SPI_UDRDR {
    #[doc = "Data at slave underrun condition"]
    #[must_use]
    #[inline(always)]
    pub const fn UDRDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data at slave underrun condition"]
    #[inline(always)]
    pub const fn set_UDRDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SPI_UDRDR {
    #[inline(always)]
    fn default() -> SPI_UDRDR {
        SPI_UDRDR(0)
    }
}
impl core::fmt::Debug for SPI_UDRDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_UDRDR")
            .field("UDRDR", &self.UDRDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_UDRDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SPI_UDRDR {{ UDRDR: {=u32:?} }}", self.UDRDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_VERR(pub u32);
impl SPI_VERR {
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
impl Default for SPI_VERR {
    #[inline(always)]
    fn default() -> SPI_VERR {
        SPI_VERR(0)
    }
}
impl core::fmt::Debug for SPI_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}

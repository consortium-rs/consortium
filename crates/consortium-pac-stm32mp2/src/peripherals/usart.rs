#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART {
    ptr: *mut u8,
}
unsafe impl Send for USART {}
unsafe impl Sync for USART {}
impl USART {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USART Control register 1"]
    #[inline(always)]
    pub const fn CR1(self) -> crate::common::Reg<USART_CR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USART Control register 2"]
    #[inline(always)]
    pub const fn CR2(self) -> crate::common::Reg<USART_CR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USART Control register 3"]
    #[inline(always)]
    pub const fn CR3(self) -> crate::common::Reg<USART_CR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USART Baud rate register"]
    #[inline(always)]
    pub const fn BRR(self) -> crate::common::Reg<USART_BRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USART Guard time and prescaler register"]
    #[inline(always)]
    pub const fn GTPR(self) -> crate::common::Reg<USART_GTPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USART Receiver timeout register"]
    #[inline(always)]
    pub const fn RTOR(self) -> crate::common::Reg<USART_RTOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USART Request register"]
    #[inline(always)]
    pub const fn RQR(self) -> crate::common::Reg<USART_RQR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USART Interrupt and status register"]
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<USART_ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USART Interrupt flag Clear register"]
    #[inline(always)]
    pub const fn ICR(self) -> crate::common::Reg<USART_ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USART Receive Data register"]
    #[inline(always)]
    pub const fn RDR(self) -> crate::common::Reg<USART_RDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USART Transmit Data register"]
    #[inline(always)]
    pub const fn TDR(self) -> crate::common::Reg<USART_TDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USART Prescaler register"]
    #[inline(always)]
    pub const fn PRESC(self) -> crate::common::Reg<USART_PRESC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USART Autonomous mode Control register"]
    #[inline(always)]
    pub const fn AUTOCR(self) -> crate::common::Reg<USART_AUTOCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "USART Hardware Configuration register 2"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<USART_HWCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "USART Hardware Configuration register 1"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<USART_HWCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "USART version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<USART_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "USART Identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<USART_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "USART Size Identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<USART_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_AUTOCR(pub u32);
impl USART_AUTOCR {
    #[doc = "TDN transmission data number"]
    #[must_use]
    #[inline(always)]
    pub const fn TDN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TDN transmission data number"]
    #[inline(always)]
    pub const fn set_TDN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger polarity bit"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGPOL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger polarity bit"]
    #[inline(always)]
    pub const fn set_TRIGPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Trigger enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGEN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger enable bit"]
    #[inline(always)]
    pub const fn set_TRIGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Idle frame transmission disable bit after enabling the transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn IDLEDIS(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Idle frame transmission disable bit after enabling the transmitter"]
    #[inline(always)]
    pub const fn set_IDLEDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Trigger selection bits"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGSEL(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger selection bits"]
    #[inline(always)]
    pub const fn set_TRIGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
    }
}
impl Default for USART_AUTOCR {
    #[inline(always)]
    fn default() -> USART_AUTOCR {
        USART_AUTOCR(0)
    }
}
impl core::fmt::Debug for USART_AUTOCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_AUTOCR")
            .field("TDN", &self.TDN())
            .field("TRIGPOL", &self.TRIGPOL())
            .field("TRIGEN", &self.TRIGEN())
            .field("IDLEDIS", &self.IDLEDIS())
            .field("TRIGSEL", &self.TRIGSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_AUTOCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_AUTOCR {{ TDN: {=u16:?}, TRIGPOL: {=bool:?}, TRIGEN: {=bool:?}, IDLEDIS: {=bool:?}, TRIGSEL: {=u8:?} }}",
            self.TDN(),
            self.TRIGPOL(),
            self.TRIGEN(),
            self.IDLEDIS(),
            self.TRIGSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_BRR(pub u32);
impl USART_BRR {
    #[doc = "USART baud rate"]
    #[must_use]
    #[inline(always)]
    pub const fn BRR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "USART baud rate"]
    #[inline(always)]
    pub const fn set_BRR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "LPUART baud rate division (LPUARTDIV)"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    pub const fn set_LPUART(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for USART_BRR {
    #[inline(always)]
    fn default() -> USART_BRR {
        USART_BRR(0)
    }
}
impl core::fmt::Debug for USART_BRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_BRR")
            .field("BRR", &self.BRR())
            .field("LPUART", &self.LPUART())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_BRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_BRR {{ BRR: {=u16:?}, LPUART: {=u32:?} }}",
            self.BRR(),
            self.LPUART()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_CR1(pub u32);
impl USART_CR1 {
    #[doc = "USART enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART enable"]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART enable in low-power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn UESM(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART enable in low-power mode"]
    #[inline(always)]
    pub const fn set_UESM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receiver enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver enable"]
    #[inline(always)]
    pub const fn set_RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmitter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter enable"]
    #[inline(always)]
    pub const fn set_TE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IDLE interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IDLEIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IDLE interrupt enable"]
    #[inline(always)]
    pub const fn set_IDLEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXFIFO not empty interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFNEIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO not empty interrupt enable"]
    #[inline(always)]
    pub const fn set_RXFNEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmission complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TCIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete interrupt enable"]
    #[inline(always)]
    pub const fn set_TCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TXFIFO not full interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFNFIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO not full interrupt enable"]
    #[inline(always)]
    pub const fn set_TXFNFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PE interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PEIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PE interrupt enable"]
    #[inline(always)]
    pub const fn set_PEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parity selection"]
    #[must_use]
    #[inline(always)]
    pub const fn PS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Parity selection"]
    #[inline(always)]
    pub const fn set_PS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Parity control enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PCE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Parity control enable"]
    #[inline(always)]
    pub const fn set_PCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receiver wakeup method"]
    #[must_use]
    #[inline(always)]
    pub const fn WAKE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver wakeup method"]
    #[inline(always)]
    pub const fn set_WAKE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word length"]
    #[must_use]
    #[inline(always)]
    pub const fn M(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Word length"]
    #[inline(always)]
    pub const fn set_M(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 12usize)) | (((val as u32) & 0x0001_ffff) << 12usize);
    }
    #[doc = "Word length"]
    #[must_use]
    #[inline(always)]
    pub const fn M0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word length"]
    #[inline(always)]
    pub const fn set_M0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Mute mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MME(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Mute mode enable"]
    #[inline(always)]
    pub const fn set_MME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Character match interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CMIE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Character match interrupt enable"]
    #[inline(always)]
    pub const fn set_CMIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Oversampling mode"]
    #[must_use]
    #[inline(always)]
    pub const fn OVER8(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Oversampling mode"]
    #[inline(always)]
    pub const fn set_OVER8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Driver Enable deassertion time"]
    #[must_use]
    #[inline(always)]
    pub const fn DEDT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Driver Enable deassertion time"]
    #[inline(always)]
    pub const fn set_DEDT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Driver Enable assertion time"]
    #[must_use]
    #[inline(always)]
    pub const fn DEAT(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "Driver Enable assertion time"]
    #[inline(always)]
    pub const fn set_DEAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Receiver timeout interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTOIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout interrupt enable"]
    #[inline(always)]
    pub const fn set_RTOIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "End of Block interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EOBIE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "End of Block interrupt enable"]
    #[inline(always)]
    pub const fn set_EOBIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Word length"]
    #[must_use]
    #[inline(always)]
    pub const fn M1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Word length"]
    #[inline(always)]
    pub const fn set_M1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "FIFO mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FIFOEN(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO mode enable"]
    #[inline(always)]
    pub const fn set_FIFOEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "TXFIFO empty interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFEIE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO empty interrupt enable"]
    #[inline(always)]
    pub const fn set_TXFEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "RXFIFO Full interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFFIE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO Full interrupt enable"]
    #[inline(always)]
    pub const fn set_RXFFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for USART_CR1 {
    #[inline(always)]
    fn default() -> USART_CR1 {
        USART_CR1(0)
    }
}
impl core::fmt::Debug for USART_CR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_CR1")
            .field("UE", &self.UE())
            .field("UESM", &self.UESM())
            .field("RE", &self.RE())
            .field("TE", &self.TE())
            .field("IDLEIE", &self.IDLEIE())
            .field("RXFNEIE", &self.RXFNEIE())
            .field("TCIE", &self.TCIE())
            .field("TXFNFIE", &self.TXFNFIE())
            .field("PEIE", &self.PEIE())
            .field("PS", &self.PS())
            .field("PCE", &self.PCE())
            .field("WAKE", &self.WAKE())
            .field("M", &self.M())
            .field("M0", &self.M0())
            .field("MME", &self.MME())
            .field("CMIE", &self.CMIE())
            .field("OVER8", &self.OVER8())
            .field("DEDT", &self.DEDT())
            .field("DEAT", &self.DEAT())
            .field("RTOIE", &self.RTOIE())
            .field("EOBIE", &self.EOBIE())
            .field("M1", &self.M1())
            .field("FIFOEN", &self.FIFOEN())
            .field("TXFEIE", &self.TXFEIE())
            .field("RXFFIE", &self.RXFFIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_CR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_CR1 {{ UE: {=bool:?}, UESM: {=bool:?}, RE: {=bool:?}, TE: {=bool:?}, IDLEIE: {=bool:?}, RXFNEIE: {=bool:?}, TCIE: {=bool:?}, TXFNFIE: {=bool:?}, PEIE: {=bool:?}, PS: {=bool:?}, PCE: {=bool:?}, WAKE: {=bool:?}, M: {=u32:?}, M0: {=bool:?}, MME: {=bool:?}, CMIE: {=bool:?}, OVER8: {=bool:?}, DEDT: {=u8:?}, DEAT: {=u8:?}, RTOIE: {=bool:?}, EOBIE: {=bool:?}, M1: {=bool:?}, FIFOEN: {=bool:?}, TXFEIE: {=bool:?}, RXFFIE: {=bool:?} }}",
            self.UE(),
            self.UESM(),
            self.RE(),
            self.TE(),
            self.IDLEIE(),
            self.RXFNEIE(),
            self.TCIE(),
            self.TXFNFIE(),
            self.PEIE(),
            self.PS(),
            self.PCE(),
            self.WAKE(),
            self.M(),
            self.M0(),
            self.MME(),
            self.CMIE(),
            self.OVER8(),
            self.DEDT(),
            self.DEAT(),
            self.RTOIE(),
            self.EOBIE(),
            self.M1(),
            self.FIFOEN(),
            self.TXFEIE(),
            self.RXFFIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_CR2(pub u32);
impl USART_CR2 {
    #[doc = "Synchronous Slave mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SLVEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous Slave mode enable"]
    #[inline(always)]
    pub const fn set_SLVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "7-bit Address Detection/4-bit Address Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDM7(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub const fn set_ADDM7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LIN break detection length"]
    #[must_use]
    #[inline(always)]
    pub const fn LBDL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection length"]
    #[inline(always)]
    pub const fn set_LBDL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LIN break detection interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LBDIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection interrupt enable"]
    #[inline(always)]
    pub const fn set_LBDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Last bit clock pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn LBCL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Last bit clock pulse"]
    #[inline(always)]
    pub const fn set_LBCL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock phase"]
    #[must_use]
    #[inline(always)]
    pub const fn CPHA(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock phase"]
    #[inline(always)]
    pub const fn set_CPHA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Clock polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn CPOL(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Clock polarity"]
    #[inline(always)]
    pub const fn set_CPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Clock enable"]
    #[inline(always)]
    pub const fn set_CLKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "stop bits"]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "stop bits"]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "LIN mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LINEN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "LIN mode enable"]
    #[inline(always)]
    pub const fn set_LINEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Swap TX/RX pins"]
    #[must_use]
    #[inline(always)]
    pub const fn SWAP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Swap TX/RX pins"]
    #[inline(always)]
    pub const fn set_SWAP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RX pin active level inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn RXINV(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RX pin active level inversion"]
    #[inline(always)]
    pub const fn set_RXINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX pin active level inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn TXINV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX pin active level inversion"]
    #[inline(always)]
    pub const fn set_TXINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Binary data inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn DATAINV(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Binary data inversion"]
    #[inline(always)]
    pub const fn set_DATAINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Most significant bit first"]
    #[must_use]
    #[inline(always)]
    pub const fn MSBFIRST(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Most significant bit first"]
    #[inline(always)]
    pub const fn set_MSBFIRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Auto baud rate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ABREN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate enable"]
    #[inline(always)]
    pub const fn set_ABREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Auto baud rate mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ABRMODE(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Auto baud rate mode"]
    #[inline(always)]
    pub const fn set_ABRMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "Receiver timeout enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTOEN(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout enable"]
    #[inline(always)]
    pub const fn set_RTOEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Address of the USART node"]
    #[must_use]
    #[inline(always)]
    pub const fn ADD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address of the USART node"]
    #[inline(always)]
    pub const fn set_ADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for USART_CR2 {
    #[inline(always)]
    fn default() -> USART_CR2 {
        USART_CR2(0)
    }
}
impl core::fmt::Debug for USART_CR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_CR2")
            .field("SLVEN", &self.SLVEN())
            .field("ADDM7", &self.ADDM7())
            .field("LBDL", &self.LBDL())
            .field("LBDIE", &self.LBDIE())
            .field("LBCL", &self.LBCL())
            .field("CPHA", &self.CPHA())
            .field("CPOL", &self.CPOL())
            .field("CLKEN", &self.CLKEN())
            .field("STOP", &self.STOP())
            .field("LINEN", &self.LINEN())
            .field("SWAP", &self.SWAP())
            .field("RXINV", &self.RXINV())
            .field("TXINV", &self.TXINV())
            .field("DATAINV", &self.DATAINV())
            .field("MSBFIRST", &self.MSBFIRST())
            .field("ABREN", &self.ABREN())
            .field("ABRMODE", &self.ABRMODE())
            .field("RTOEN", &self.RTOEN())
            .field("ADD", &self.ADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_CR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_CR2 {{ SLVEN: {=bool:?}, ADDM7: {=bool:?}, LBDL: {=bool:?}, LBDIE: {=bool:?}, LBCL: {=bool:?}, CPHA: {=bool:?}, CPOL: {=bool:?}, CLKEN: {=bool:?}, STOP: {=u8:?}, LINEN: {=bool:?}, SWAP: {=bool:?}, RXINV: {=bool:?}, TXINV: {=bool:?}, DATAINV: {=bool:?}, MSBFIRST: {=bool:?}, ABREN: {=bool:?}, ABRMODE: {=u8:?}, RTOEN: {=bool:?}, ADD: {=u8:?} }}",
            self.SLVEN(),
            self.ADDM7(),
            self.LBDL(),
            self.LBDIE(),
            self.LBCL(),
            self.CPHA(),
            self.CPOL(),
            self.CLKEN(),
            self.STOP(),
            self.LINEN(),
            self.SWAP(),
            self.RXINV(),
            self.TXINV(),
            self.DATAINV(),
            self.MSBFIRST(),
            self.ABREN(),
            self.ABRMODE(),
            self.RTOEN(),
            self.ADD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_CR3(pub u32);
impl USART_CR3 {
    #[doc = "Error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn set_EIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IrDA mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IREN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IrDA mode enable"]
    #[inline(always)]
    pub const fn set_IREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IrDA low-power"]
    #[must_use]
    #[inline(always)]
    pub const fn IRLP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IrDA low-power"]
    #[inline(always)]
    pub const fn set_IRLP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Half-duplex selection"]
    #[must_use]
    #[inline(always)]
    pub const fn HDSEL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Half-duplex selection"]
    #[inline(always)]
    pub const fn set_HDSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Smartcard NACK enable"]
    #[must_use]
    #[inline(always)]
    pub const fn NACK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Smartcard NACK enable"]
    #[inline(always)]
    pub const fn set_NACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Smartcard mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SCEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Smartcard mode enable"]
    #[inline(always)]
    pub const fn set_SCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA enable receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn DMAR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable receiver"]
    #[inline(always)]
    pub const fn set_DMAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA enable transmitter"]
    #[must_use]
    #[inline(always)]
    pub const fn DMAT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable transmitter"]
    #[inline(always)]
    pub const fn set_DMAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RTS enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTSE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RTS enable"]
    #[inline(always)]
    pub const fn set_RTSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CTSE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS enable"]
    #[inline(always)]
    pub const fn set_CTSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CTSIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS interrupt enable"]
    #[inline(always)]
    pub const fn set_CTSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "One sample bit method enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ONEBIT(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "One sample bit method enable"]
    #[inline(always)]
    pub const fn set_ONEBIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Overrun Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRDIS(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Disable"]
    #[inline(always)]
    pub const fn set_OVRDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA Disable on Reception Error"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Disable on Reception Error"]
    #[inline(always)]
    pub const fn set_DDRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Driver enable mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DEM(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Driver enable mode"]
    #[inline(always)]
    pub const fn set_DEM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Driver enable polarity selection"]
    #[must_use]
    #[inline(always)]
    pub const fn DEP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Driver enable polarity selection"]
    #[inline(always)]
    pub const fn set_DEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Smartcard auto-retry count"]
    #[must_use]
    #[inline(always)]
    pub const fn SCARCNT(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Smartcard auto-retry count"]
    #[inline(always)]
    pub const fn set_SCARCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "TXFIFO threshold interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFTIE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub const fn set_TXFTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Transmission Complete before guard time, interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TCBGTIE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete before guard time, interrupt enable"]
    #[inline(always)]
    pub const fn set_TCBGTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Receive FIFO threshold configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFTCFG(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO threshold configuration"]
    #[inline(always)]
    pub const fn set_RXFTCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "RXFIFO threshold interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFTIE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub const fn set_RXFTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TXFIFO threshold configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFTCFG(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "TXFIFO threshold configuration"]
    #[inline(always)]
    pub const fn set_TXFTCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for USART_CR3 {
    #[inline(always)]
    fn default() -> USART_CR3 {
        USART_CR3(0)
    }
}
impl core::fmt::Debug for USART_CR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_CR3")
            .field("EIE", &self.EIE())
            .field("IREN", &self.IREN())
            .field("IRLP", &self.IRLP())
            .field("HDSEL", &self.HDSEL())
            .field("NACK", &self.NACK())
            .field("SCEN", &self.SCEN())
            .field("DMAR", &self.DMAR())
            .field("DMAT", &self.DMAT())
            .field("RTSE", &self.RTSE())
            .field("CTSE", &self.CTSE())
            .field("CTSIE", &self.CTSIE())
            .field("ONEBIT", &self.ONEBIT())
            .field("OVRDIS", &self.OVRDIS())
            .field("DDRE", &self.DDRE())
            .field("DEM", &self.DEM())
            .field("DEP", &self.DEP())
            .field("SCARCNT", &self.SCARCNT())
            .field("TXFTIE", &self.TXFTIE())
            .field("TCBGTIE", &self.TCBGTIE())
            .field("RXFTCFG", &self.RXFTCFG())
            .field("RXFTIE", &self.RXFTIE())
            .field("TXFTCFG", &self.TXFTCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_CR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_CR3 {{ EIE: {=bool:?}, IREN: {=bool:?}, IRLP: {=bool:?}, HDSEL: {=bool:?}, NACK: {=bool:?}, SCEN: {=bool:?}, DMAR: {=bool:?}, DMAT: {=bool:?}, RTSE: {=bool:?}, CTSE: {=bool:?}, CTSIE: {=bool:?}, ONEBIT: {=bool:?}, OVRDIS: {=bool:?}, DDRE: {=bool:?}, DEM: {=bool:?}, DEP: {=bool:?}, SCARCNT: {=u8:?}, TXFTIE: {=bool:?}, TCBGTIE: {=bool:?}, RXFTCFG: {=u8:?}, RXFTIE: {=bool:?}, TXFTCFG: {=u8:?} }}",
            self.EIE(),
            self.IREN(),
            self.IRLP(),
            self.HDSEL(),
            self.NACK(),
            self.SCEN(),
            self.DMAR(),
            self.DMAT(),
            self.RTSE(),
            self.CTSE(),
            self.CTSIE(),
            self.ONEBIT(),
            self.OVRDIS(),
            self.DDRE(),
            self.DEM(),
            self.DEP(),
            self.SCARCNT(),
            self.TXFTIE(),
            self.TCBGTIE(),
            self.RXFTCFG(),
            self.RXFTIE(),
            self.TXFTCFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_GTPR(pub u32);
impl USART_GTPR {
    #[doc = "Prescaler value"]
    #[must_use]
    #[inline(always)]
    pub const fn PSC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn set_PSC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Guard time value"]
    #[must_use]
    #[inline(always)]
    pub const fn GT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Guard time value"]
    #[inline(always)]
    pub const fn set_GT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for USART_GTPR {
    #[inline(always)]
    fn default() -> USART_GTPR {
        USART_GTPR(0)
    }
}
impl core::fmt::Debug for USART_GTPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_GTPR")
            .field("PSC", &self.PSC())
            .field("GT", &self.GT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_GTPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_GTPR {{ PSC: {=u8:?}, GT: {=u8:?} }}",
            self.PSC(),
            self.GT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_HWCFGR1(pub u32);
impl USART_HWCFGR1 {
    #[doc = "USART hardware configuration 1"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 1"]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "USART hardware configuration 2"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 2"]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "USART hardware configuration 3"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 3"]
    #[inline(always)]
    pub const fn set_CFG3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "USART hardware configuration 4"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 4"]
    #[inline(always)]
    pub const fn set_CFG4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "USART hardware configuration 5"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 5"]
    #[inline(always)]
    pub const fn set_CFG5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "USART hardware configuration 6"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG6(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 6"]
    #[inline(always)]
    pub const fn set_CFG6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "USART hardware configuration 7"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 7"]
    #[inline(always)]
    pub const fn set_CFG7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "USART hardware configuration 8"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG8(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 8"]
    #[inline(always)]
    pub const fn set_CFG8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for USART_HWCFGR1 {
    #[inline(always)]
    fn default() -> USART_HWCFGR1 {
        USART_HWCFGR1(0)
    }
}
impl core::fmt::Debug for USART_HWCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_HWCFGR1")
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("CFG3", &self.CFG3())
            .field("CFG4", &self.CFG4())
            .field("CFG5", &self.CFG5())
            .field("CFG6", &self.CFG6())
            .field("CFG7", &self.CFG7())
            .field("CFG8", &self.CFG8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_HWCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_HWCFGR1 {{ CFG1: {=u8:?}, CFG2: {=u8:?}, CFG3: {=u8:?}, CFG4: {=u8:?}, CFG5: {=u8:?}, CFG6: {=u8:?}, CFG7: {=u8:?}, CFG8: {=u8:?} }}",
            self.CFG1(),
            self.CFG2(),
            self.CFG3(),
            self.CFG4(),
            self.CFG5(),
            self.CFG6(),
            self.CFG7(),
            self.CFG8()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_HWCFGR2(pub u32);
impl USART_HWCFGR2 {
    #[doc = "USART hardware configuration 1"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 1"]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "USART hardware configuration 2"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 2"]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "USART hardware configuration 3"]
    #[must_use]
    #[inline(always)]
    pub const fn CFG3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "USART hardware configuration 3"]
    #[inline(always)]
    pub const fn set_CFG3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for USART_HWCFGR2 {
    #[inline(always)]
    fn default() -> USART_HWCFGR2 {
        USART_HWCFGR2(0)
    }
}
impl core::fmt::Debug for USART_HWCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_HWCFGR2")
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("CFG3", &self.CFG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_HWCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_HWCFGR2 {{ CFG1: {=u8:?}, CFG2: {=u8:?}, CFG3: {=u8:?} }}",
            self.CFG1(),
            self.CFG2(),
            self.CFG3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_ICR(pub u32);
impl USART_ICR {
    #[doc = "Parity error clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PECF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error clear flag"]
    #[inline(always)]
    pub const fn set_PECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Framing error clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn FECF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error clear flag"]
    #[inline(always)]
    pub const fn set_FECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Noise detected clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn NECF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Noise detected clear flag"]
    #[inline(always)]
    pub const fn set_NECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ORECF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error clear flag"]
    #[inline(always)]
    pub const fn set_ORECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line detected clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IDLECF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line detected clear flag"]
    #[inline(always)]
    pub const fn set_IDLECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TXFIFO empty clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFECF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO empty clear flag"]
    #[inline(always)]
    pub const fn set_TXFECF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmission complete clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TCCF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete clear flag"]
    #[inline(always)]
    pub const fn set_TCCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmission complete before Guard time clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TCBGTCF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete before Guard time clear flag"]
    #[inline(always)]
    pub const fn set_TCBGTCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LIN break detection clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LBDCF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection clear flag"]
    #[inline(always)]
    pub const fn set_LBDCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CTSCF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS clear flag"]
    #[inline(always)]
    pub const fn set_CTSCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver timeout clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RTOCF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout clear flag"]
    #[inline(always)]
    pub const fn set_RTOCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "End of block clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EOBCF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "End of block clear flag"]
    #[inline(always)]
    pub const fn set_EOBCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI slave underrun clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn UDRCF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI slave underrun clear flag"]
    #[inline(always)]
    pub const fn set_UDRCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Character match clear flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CMCF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Character match clear flag"]
    #[inline(always)]
    pub const fn set_CMCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for USART_ICR {
    #[inline(always)]
    fn default() -> USART_ICR {
        USART_ICR(0)
    }
}
impl core::fmt::Debug for USART_ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_ICR")
            .field("PECF", &self.PECF())
            .field("FECF", &self.FECF())
            .field("NECF", &self.NECF())
            .field("ORECF", &self.ORECF())
            .field("IDLECF", &self.IDLECF())
            .field("TXFECF", &self.TXFECF())
            .field("TCCF", &self.TCCF())
            .field("TCBGTCF", &self.TCBGTCF())
            .field("LBDCF", &self.LBDCF())
            .field("CTSCF", &self.CTSCF())
            .field("RTOCF", &self.RTOCF())
            .field("EOBCF", &self.EOBCF())
            .field("UDRCF", &self.UDRCF())
            .field("CMCF", &self.CMCF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_ICR {{ PECF: {=bool:?}, FECF: {=bool:?}, NECF: {=bool:?}, ORECF: {=bool:?}, IDLECF: {=bool:?}, TXFECF: {=bool:?}, TCCF: {=bool:?}, TCBGTCF: {=bool:?}, LBDCF: {=bool:?}, CTSCF: {=bool:?}, RTOCF: {=bool:?}, EOBCF: {=bool:?}, UDRCF: {=bool:?}, CMCF: {=bool:?} }}",
            self.PECF(),
            self.FECF(),
            self.NECF(),
            self.ORECF(),
            self.IDLECF(),
            self.TXFECF(),
            self.TCCF(),
            self.TCBGTCF(),
            self.LBDCF(),
            self.CTSCF(),
            self.RTOCF(),
            self.EOBCF(),
            self.UDRCF(),
            self.CMCF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_IPIDR(pub u32);
impl USART_IPIDR {
    #[doc = "Peripheral identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral identifier"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for USART_IPIDR {
    #[inline(always)]
    fn default() -> USART_IPIDR {
        USART_IPIDR(0)
    }
}
impl core::fmt::Debug for USART_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_IPIDR")
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USART_IPIDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_ISR(pub u32);
impl USART_ISR {
    #[doc = "Parity error"]
    #[must_use]
    #[inline(always)]
    pub const fn PE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub const fn set_PE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Framing error"]
    #[must_use]
    #[inline(always)]
    pub const fn FE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error"]
    #[inline(always)]
    pub const fn set_FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Noise detection flag"]
    #[must_use]
    #[inline(always)]
    pub const fn NE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Noise detection flag"]
    #[inline(always)]
    pub const fn set_NE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error"]
    #[must_use]
    #[inline(always)]
    pub const fn ORE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub const fn set_ORE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Idle line detected"]
    #[must_use]
    #[inline(always)]
    pub const fn IDLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Idle line detected"]
    #[inline(always)]
    pub const fn set_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RXFIFO not empty"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFNE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO not empty"]
    #[inline(always)]
    pub const fn set_RXFNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmission complete"]
    #[must_use]
    #[inline(always)]
    pub const fn TC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete"]
    #[inline(always)]
    pub const fn set_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "0x00000080"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFNF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0x00000080"]
    #[inline(always)]
    pub const fn set_TXFNF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LIN break detection flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LBDF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LIN break detection flag"]
    #[inline(always)]
    pub const fn set_LBDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CTS interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CTSIF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CTS interrupt flag"]
    #[inline(always)]
    pub const fn set_CTSIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CTS flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CTS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CTS flag"]
    #[inline(always)]
    pub const fn set_CTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receiver timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn RTOF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver timeout"]
    #[inline(always)]
    pub const fn set_RTOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "End of block flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EOBF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "End of block flag"]
    #[inline(always)]
    pub const fn set_EOBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPI slave underrun error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn UDR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPI slave underrun error flag"]
    #[inline(always)]
    pub const fn set_UDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Auto baud rate error"]
    #[must_use]
    #[inline(always)]
    pub const fn ABRE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate error"]
    #[inline(always)]
    pub const fn set_ABRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto baud rate flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ABRF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate flag"]
    #[inline(always)]
    pub const fn set_ABRF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Busy flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Busy flag"]
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Character match flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CMF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Character match flag"]
    #[inline(always)]
    pub const fn set_CMF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Send break flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SBKF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Send break flag"]
    #[inline(always)]
    pub const fn set_SBKF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Receiver wakeup from Mute mode"]
    #[must_use]
    #[inline(always)]
    pub const fn RWU(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver wakeup from Mute mode"]
    #[inline(always)]
    pub const fn set_RWU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Transmit enable acknowledge flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TEACK(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit enable acknowledge flag"]
    #[inline(always)]
    pub const fn set_TEACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive enable acknowledge flag"]
    #[must_use]
    #[inline(always)]
    pub const fn REACK(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Receive enable acknowledge flag"]
    #[inline(always)]
    pub const fn set_REACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TXFIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO Empty"]
    #[inline(always)]
    pub const fn set_TXFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "RXFIFO Full"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO Full"]
    #[inline(always)]
    pub const fn set_RXFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Transmission complete before guard time flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TCBGT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete before guard time flag"]
    #[inline(always)]
    pub const fn set_TCBGT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RXFIFO threshold flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFT(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "RXFIFO threshold flag"]
    #[inline(always)]
    pub const fn set_RXFT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "TXFIFO threshold flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFT(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "TXFIFO threshold flag"]
    #[inline(always)]
    pub const fn set_TXFT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for USART_ISR {
    #[inline(always)]
    fn default() -> USART_ISR {
        USART_ISR(0)
    }
}
impl core::fmt::Debug for USART_ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_ISR")
            .field("PE", &self.PE())
            .field("FE", &self.FE())
            .field("NE", &self.NE())
            .field("ORE", &self.ORE())
            .field("IDLE", &self.IDLE())
            .field("RXFNE", &self.RXFNE())
            .field("TC", &self.TC())
            .field("TXFNF", &self.TXFNF())
            .field("LBDF", &self.LBDF())
            .field("CTSIF", &self.CTSIF())
            .field("CTS", &self.CTS())
            .field("RTOF", &self.RTOF())
            .field("EOBF", &self.EOBF())
            .field("UDR", &self.UDR())
            .field("ABRE", &self.ABRE())
            .field("ABRF", &self.ABRF())
            .field("BUSY", &self.BUSY())
            .field("CMF", &self.CMF())
            .field("SBKF", &self.SBKF())
            .field("RWU", &self.RWU())
            .field("TEACK", &self.TEACK())
            .field("REACK", &self.REACK())
            .field("TXFE", &self.TXFE())
            .field("RXFF", &self.RXFF())
            .field("TCBGT", &self.TCBGT())
            .field("RXFT", &self.RXFT())
            .field("TXFT", &self.TXFT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_ISR {{ PE: {=bool:?}, FE: {=bool:?}, NE: {=bool:?}, ORE: {=bool:?}, IDLE: {=bool:?}, RXFNE: {=bool:?}, TC: {=bool:?}, TXFNF: {=bool:?}, LBDF: {=bool:?}, CTSIF: {=bool:?}, CTS: {=bool:?}, RTOF: {=bool:?}, EOBF: {=bool:?}, UDR: {=bool:?}, ABRE: {=bool:?}, ABRF: {=bool:?}, BUSY: {=bool:?}, CMF: {=bool:?}, SBKF: {=bool:?}, RWU: {=bool:?}, TEACK: {=bool:?}, REACK: {=bool:?}, TXFE: {=bool:?}, RXFF: {=bool:?}, TCBGT: {=bool:?}, RXFT: {=bool:?}, TXFT: {=bool:?} }}",
            self.PE(),
            self.FE(),
            self.NE(),
            self.ORE(),
            self.IDLE(),
            self.RXFNE(),
            self.TC(),
            self.TXFNF(),
            self.LBDF(),
            self.CTSIF(),
            self.CTS(),
            self.RTOF(),
            self.EOBF(),
            self.UDR(),
            self.ABRE(),
            self.ABRF(),
            self.BUSY(),
            self.CMF(),
            self.SBKF(),
            self.RWU(),
            self.TEACK(),
            self.REACK(),
            self.TXFE(),
            self.RXFF(),
            self.TCBGT(),
            self.RXFT(),
            self.TXFT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_PRESC(pub u32);
impl USART_PRESC {
    #[doc = "Clock prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn PRESCALER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock prescaler"]
    #[inline(always)]
    pub const fn set_PRESCALER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for USART_PRESC {
    #[inline(always)]
    fn default() -> USART_PRESC {
        USART_PRESC(0)
    }
}
impl core::fmt::Debug for USART_PRESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_PRESC")
            .field("PRESCALER", &self.PRESCALER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_PRESC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USART_PRESC {{ PRESCALER: {=u8:?} }}", self.PRESCALER())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_RDR(pub u32);
impl USART_RDR {
    #[doc = "Receive data value"]
    #[must_use]
    #[inline(always)]
    pub const fn RDR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Receive data value"]
    #[inline(always)]
    pub const fn set_RDR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for USART_RDR {
    #[inline(always)]
    fn default() -> USART_RDR {
        USART_RDR(0)
    }
}
impl core::fmt::Debug for USART_RDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_RDR")
            .field("RDR", &self.RDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_RDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USART_RDR {{ RDR: {=u16:?} }}", self.RDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_RQR(pub u32);
impl USART_RQR {
    #[doc = "Auto baud rate request"]
    #[must_use]
    #[inline(always)]
    pub const fn ABRRQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate request"]
    #[inline(always)]
    pub const fn set_ABRRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Send break request"]
    #[must_use]
    #[inline(always)]
    pub const fn SBKRQ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Send break request"]
    #[inline(always)]
    pub const fn set_SBKRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mute mode request"]
    #[must_use]
    #[inline(always)]
    pub const fn MMRQ(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mute mode request"]
    #[inline(always)]
    pub const fn set_MMRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive data flush request"]
    #[must_use]
    #[inline(always)]
    pub const fn RXFRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data flush request"]
    #[inline(always)]
    pub const fn set_RXFRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit data flush request"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFRQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit data flush request"]
    #[inline(always)]
    pub const fn set_TXFRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for USART_RQR {
    #[inline(always)]
    fn default() -> USART_RQR {
        USART_RQR(0)
    }
}
impl core::fmt::Debug for USART_RQR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_RQR")
            .field("ABRRQ", &self.ABRRQ())
            .field("SBKRQ", &self.SBKRQ())
            .field("MMRQ", &self.MMRQ())
            .field("RXFRQ", &self.RXFRQ())
            .field("TXFRQ", &self.TXFRQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_RQR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_RQR {{ ABRRQ: {=bool:?}, SBKRQ: {=bool:?}, MMRQ: {=bool:?}, RXFRQ: {=bool:?}, TXFRQ: {=bool:?} }}",
            self.ABRRQ(),
            self.SBKRQ(),
            self.MMRQ(),
            self.RXFRQ(),
            self.TXFRQ()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_RTOR(pub u32);
impl USART_RTOR {
    #[doc = "Receiver timeout value"]
    #[must_use]
    #[inline(always)]
    pub const fn RTO(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receiver timeout value"]
    #[inline(always)]
    pub const fn set_RTO(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Block Length"]
    #[must_use]
    #[inline(always)]
    pub const fn BLEN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Block Length"]
    #[inline(always)]
    pub const fn set_BLEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for USART_RTOR {
    #[inline(always)]
    fn default() -> USART_RTOR {
        USART_RTOR(0)
    }
}
impl core::fmt::Debug for USART_RTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_RTOR")
            .field("RTO", &self.RTO())
            .field("BLEN", &self.BLEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_RTOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_RTOR {{ RTO: {=u32:?}, BLEN: {=u8:?} }}",
            self.RTO(),
            self.BLEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_SIDR(pub u32);
impl USART_SIDR {
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
impl Default for USART_SIDR {
    #[inline(always)]
    fn default() -> USART_SIDR {
        USART_SIDR(0)
    }
}
impl core::fmt::Debug for USART_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USART_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_TDR(pub u32);
impl USART_TDR {
    #[doc = "Transmit data value"]
    #[must_use]
    #[inline(always)]
    pub const fn TDR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Transmit data value"]
    #[inline(always)]
    pub const fn set_TDR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for USART_TDR {
    #[inline(always)]
    fn default() -> USART_TDR {
        USART_TDR(0)
    }
}
impl core::fmt::Debug for USART_TDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_TDR")
            .field("TDR", &self.TDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_TDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USART_TDR {{ TDR: {=u16:?} }}", self.TDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART_VERR(pub u32);
impl USART_VERR {
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
impl Default for USART_VERR {
    #[inline(always)]
    fn default() -> USART_VERR {
        USART_VERR(0)
    }
}
impl core::fmt::Debug for USART_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USART_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USART_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN {
    ptr: *mut u8,
}
unsafe impl Send for FDCAN {}
unsafe impl Sync for FDCAN {}
impl FDCAN {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FDCAN Core Release register"]
    #[inline(always)]
    pub const fn CREL(self) -> crate::common::Reg<FDCAN_CREL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FDCAN Endian register"]
    #[inline(always)]
    pub const fn ENDN(self) -> crate::common::Reg<FDCAN_ENDN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FDCAN Data Bit Timing & Prescaler register"]
    #[inline(always)]
    pub const fn DBTP(self) -> crate::common::Reg<FDCAN_DBTP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "FDCAN Test register"]
    #[inline(always)]
    pub const fn TEST(self) -> crate::common::Reg<FDCAN_TEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FDCAN RAM Watchdog register"]
    #[inline(always)]
    pub const fn RWD(self) -> crate::common::Reg<FDCAN_RWD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "FDCAN CC Control register"]
    #[inline(always)]
    pub const fn CCCR(self) -> crate::common::Reg<FDCAN_CCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "FDCAN Nominal Bit Timing & Prescaler register"]
    #[inline(always)]
    pub const fn NBTP(self) -> crate::common::Reg<FDCAN_NBTP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Configuration register"]
    #[inline(always)]
    pub const fn TSCC(self) -> crate::common::Reg<FDCAN_TSCC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "FDCAN Timestamp Counter Value register"]
    #[inline(always)]
    pub const fn TSCV(self) -> crate::common::Reg<FDCAN_TSCV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Configuration register"]
    #[inline(always)]
    pub const fn TOCC(self) -> crate::common::Reg<FDCAN_TOCC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "FDCAN Timeout Counter Value register"]
    #[inline(always)]
    pub const fn TOCV(self) -> crate::common::Reg<FDCAN_TOCV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "FDCAN Error Counter register"]
    #[inline(always)]
    pub const fn ECR(self) -> crate::common::Reg<FDCAN_ECR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "FDCAN Protocol Status register"]
    #[inline(always)]
    pub const fn PSR(self) -> crate::common::Reg<FDCAN_PSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "FDCAN Transmitter Delay Compensation register"]
    #[inline(always)]
    pub const fn TDCR(self) -> crate::common::Reg<FDCAN_TDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "FDCAN Interrupt register"]
    #[inline(always)]
    pub const fn IR(self) -> crate::common::Reg<FDCAN_IR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "FDCAN Interrupt Enable register"]
    #[inline(always)]
    pub const fn IE(self) -> crate::common::Reg<FDCAN_IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Select register"]
    #[inline(always)]
    pub const fn ILS(self) -> crate::common::Reg<FDCAN_ILS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "FDCAN Interrupt Line Enable register"]
    #[inline(always)]
    pub const fn ILE(self) -> crate::common::Reg<FDCAN_ILE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "FDCAN Global Filter Configuration register"]
    #[inline(always)]
    pub const fn GFC(self) -> crate::common::Reg<FDCAN_GFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "FDCAN Standard ID Filter Configuration register"]
    #[inline(always)]
    pub const fn SIDFC(self) -> crate::common::Reg<FDCAN_SIDFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FDCAN Extended ID Filter Configuration register"]
    #[inline(always)]
    pub const fn XIDFC(self) -> crate::common::Reg<FDCAN_XIDFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "FDCAN Extended ID AND Mask register"]
    #[inline(always)]
    pub const fn XIDAM(self) -> crate::common::Reg<FDCAN_XIDAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FDCAN High Priority Message Status register"]
    #[inline(always)]
    pub const fn HPMS(self) -> crate::common::Reg<FDCAN_HPMS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "FDCAN New Data 1 register"]
    #[inline(always)]
    pub const fn NDAT1(self) -> crate::common::Reg<FDCAN_NDAT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "FDCAN New Data 2 register"]
    #[inline(always)]
    pub const fn NDAT2(self) -> crate::common::Reg<FDCAN_NDAT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 0 Configuration register"]
    #[inline(always)]
    pub const fn RXF0C(self) -> crate::common::Reg<FDCAN_RXF0C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 0 Status register"]
    #[inline(always)]
    pub const fn RXF0S(self) -> crate::common::Reg<FDCAN_RXF0S, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 0 Acknowledge register"]
    #[inline(always)]
    pub const fn RXF0A(self) -> crate::common::Reg<FDCAN_RXF0A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "FDCAN Rx Buffer Configuration register"]
    #[inline(always)]
    pub const fn RXBC(self) -> crate::common::Reg<FDCAN_RXBC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 1 Configuration register"]
    #[inline(always)]
    pub const fn RXF1C(self) -> crate::common::Reg<FDCAN_RXF1C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 1 Status register"]
    #[inline(always)]
    pub const fn RXF1S(self) -> crate::common::Reg<FDCAN_RXF1S, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "FDCAN Rx FIFO 1 Acknowledge register"]
    #[inline(always)]
    pub const fn RXF1A(self) -> crate::common::Reg<FDCAN_RXF1A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "FDCAN Rx Buffer/FIFO Element Size Configuration register"]
    #[inline(always)]
    pub const fn RXESC(self) -> crate::common::Reg<FDCAN_RXESC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Configuration register"]
    #[inline(always)]
    pub const fn TXBC(self) -> crate::common::Reg<FDCAN_TXBC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "FDCAN Tx FIFO/Queue Status register"]
    #[inline(always)]
    pub const fn TXFQS(self) -> crate::common::Reg<FDCAN_TXFQS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Element Size Configuration register"]
    #[inline(always)]
    pub const fn TXESC(self) -> crate::common::Reg<FDCAN_TXESC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Request Pending register"]
    #[inline(always)]
    pub const fn TXBRP(self) -> crate::common::Reg<FDCAN_TXBRP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Add Request register"]
    #[inline(always)]
    pub const fn TXBAR(self) -> crate::common::Reg<FDCAN_TXBAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Request register"]
    #[inline(always)]
    pub const fn TXBCR(self) -> crate::common::Reg<FDCAN_TXBCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Occurred register"]
    #[inline(always)]
    pub const fn TXBTO(self) -> crate::common::Reg<FDCAN_TXBTO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished register"]
    #[inline(always)]
    pub const fn TXBCF(self) -> crate::common::Reg<FDCAN_TXBCF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Transmission Interrupt Enable register"]
    #[inline(always)]
    pub const fn TXBTIE(self) -> crate::common::Reg<FDCAN_TXBTIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable register"]
    #[inline(always)]
    pub const fn TXBCIE(self) -> crate::common::Reg<FDCAN_TXBCIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Configuration register"]
    #[inline(always)]
    pub const fn TXEFC(self) -> crate::common::Reg<FDCAN_TXEFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Status register"]
    #[inline(always)]
    pub const fn TXEFS(self) -> crate::common::Reg<FDCAN_TXEFS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "FDCAN Tx Event FIFO Acknowledge register"]
    #[inline(always)]
    pub const fn TXEFA(self) -> crate::common::Reg<FDCAN_TXEFA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_CCCR(pub u32);
impl FDCAN_CCCR {
    #[doc = "Initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn INIT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization"]
    #[inline(always)]
    pub const fn set_INIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Configuration Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CCE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration Change Enable"]
    #[inline(always)]
    pub const fn set_CCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ASM Restricted Operation Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ASM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ASM Restricted Operation Mode"]
    #[inline(always)]
    pub const fn set_ASM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Stop Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn CSA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Acknowledge"]
    #[inline(always)]
    pub const fn set_CSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Stop Request"]
    #[must_use]
    #[inline(always)]
    pub const fn CSR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Request"]
    #[inline(always)]
    pub const fn set_CSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bus Monitoring Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn MON(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Monitoring Mode"]
    #[inline(always)]
    pub const fn set_MON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Automatic Retransmission"]
    #[must_use]
    #[inline(always)]
    pub const fn DAR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Automatic Retransmission"]
    #[inline(always)]
    pub const fn set_DAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEST(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test Mode Enable"]
    #[inline(always)]
    pub const fn set_TEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "FD Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FDOE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FD Operation Enable"]
    #[inline(always)]
    pub const fn set_FDOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FDCAN Bit Rate Switching"]
    #[must_use]
    #[inline(always)]
    pub const fn BRSE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FDCAN Bit Rate Switching"]
    #[inline(always)]
    pub const fn set_BRSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protocol Exception Handling Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn PXHD(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Handling Disable"]
    #[inline(always)]
    pub const fn set_PXHD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Edge Filtering during Bus Integration"]
    #[must_use]
    #[inline(always)]
    pub const fn EFBI(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub const fn set_EFBI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Two CAN bit times Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn TXP(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Two CAN bit times Pause"]
    #[inline(always)]
    pub const fn set_TXP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Non ISO Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn NISO(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Non ISO Operation"]
    #[inline(always)]
    pub const fn set_NISO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for FDCAN_CCCR {
    #[inline(always)]
    fn default() -> FDCAN_CCCR {
        FDCAN_CCCR(0)
    }
}
impl core::fmt::Debug for FDCAN_CCCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CCCR")
            .field("INIT", &self.INIT())
            .field("CCE", &self.CCE())
            .field("ASM", &self.ASM())
            .field("CSA", &self.CSA())
            .field("CSR", &self.CSR())
            .field("MON", &self.MON())
            .field("DAR", &self.DAR())
            .field("TEST", &self.TEST())
            .field("FDOE", &self.FDOE())
            .field("BRSE", &self.BRSE())
            .field("PXHD", &self.PXHD())
            .field("EFBI", &self.EFBI())
            .field("TXP", &self.TXP())
            .field("NISO", &self.NISO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_CCCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_CCCR {{ INIT: {=bool:?}, CCE: {=bool:?}, ASM: {=bool:?}, CSA: {=bool:?}, CSR: {=bool:?}, MON: {=bool:?}, DAR: {=bool:?}, TEST: {=bool:?}, FDOE: {=bool:?}, BRSE: {=bool:?}, PXHD: {=bool:?}, EFBI: {=bool:?}, TXP: {=bool:?}, NISO: {=bool:?} }}",
            self.INIT(),
            self.CCE(),
            self.ASM(),
            self.CSA(),
            self.CSR(),
            self.MON(),
            self.DAR(),
            self.TEST(),
            self.FDOE(),
            self.BRSE(),
            self.PXHD(),
            self.EFBI(),
            self.TXP(),
            self.NISO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_CREL(pub u32);
impl FDCAN_CREL {
    #[doc = "Timestamp Day"]
    #[must_use]
    #[inline(always)]
    pub const fn DAY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timestamp Day"]
    #[inline(always)]
    pub const fn set_DAY(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timestamp Month"]
    #[must_use]
    #[inline(always)]
    pub const fn MON(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timestamp Month"]
    #[inline(always)]
    pub const fn set_MON(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Timestamp Year"]
    #[must_use]
    #[inline(always)]
    pub const fn YEAR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp Year"]
    #[inline(always)]
    pub const fn set_YEAR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Sub-step of Core release"]
    #[must_use]
    #[inline(always)]
    pub const fn SUBSTEP(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Sub-step of Core release"]
    #[inline(always)]
    pub const fn set_SUBSTEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Step of Core release"]
    #[must_use]
    #[inline(always)]
    pub const fn STEP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Step of Core release"]
    #[inline(always)]
    pub const fn set_STEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Core release"]
    #[must_use]
    #[inline(always)]
    pub const fn REL(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Core release"]
    #[inline(always)]
    pub const fn set_REL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for FDCAN_CREL {
    #[inline(always)]
    fn default() -> FDCAN_CREL {
        FDCAN_CREL(0)
    }
}
impl core::fmt::Debug for FDCAN_CREL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_CREL")
            .field("DAY", &self.DAY())
            .field("MON", &self.MON())
            .field("YEAR", &self.YEAR())
            .field("SUBSTEP", &self.SUBSTEP())
            .field("STEP", &self.STEP())
            .field("REL", &self.REL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_CREL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_CREL {{ DAY: {=u8:?}, MON: {=u8:?}, YEAR: {=u8:?}, SUBSTEP: {=u8:?}, STEP: {=u8:?}, REL: {=u8:?} }}",
            self.DAY(),
            self.MON(),
            self.YEAR(),
            self.SUBSTEP(),
            self.STEP(),
            self.REL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_DBTP(pub u32);
impl FDCAN_DBTP {
    #[doc = "Synchronization Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn DSJW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Synchronization Jump Width"]
    #[inline(always)]
    pub const fn set_DSJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data time segment after sample point"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data time segment after sample point"]
    #[inline(always)]
    pub const fn set_DTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data time segment before sample point"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Data time segment before sample point"]
    #[inline(always)]
    pub const fn set_DTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Data BIt Rate Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn DBRP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data BIt Rate Prescaler"]
    #[inline(always)]
    pub const fn set_DBRP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transceiver Delay Compensation"]
    #[must_use]
    #[inline(always)]
    pub const fn TDC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation"]
    #[inline(always)]
    pub const fn set_TDC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for FDCAN_DBTP {
    #[inline(always)]
    fn default() -> FDCAN_DBTP {
        FDCAN_DBTP(0)
    }
}
impl core::fmt::Debug for FDCAN_DBTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_DBTP")
            .field("DSJW", &self.DSJW())
            .field("DTSEG2", &self.DTSEG2())
            .field("DTSEG1", &self.DTSEG1())
            .field("DBRP", &self.DBRP())
            .field("TDC", &self.TDC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_DBTP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_DBTP {{ DSJW: {=u8:?}, DTSEG2: {=u8:?}, DTSEG1: {=u8:?}, DBRP: {=u8:?}, TDC: {=bool:?} }}",
            self.DSJW(),
            self.DTSEG2(),
            self.DTSEG1(),
            self.DBRP(),
            self.TDC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_ECR(pub u32);
impl FDCAN_ECR {
    #[doc = "Transmit Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn TEC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter"]
    #[inline(always)]
    pub const fn set_TEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn REC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive Error Counter"]
    #[inline(always)]
    pub const fn set_REC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive Error Passive"]
    #[must_use]
    #[inline(always)]
    pub const fn RP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Passive"]
    #[inline(always)]
    pub const fn set_RP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN Error Logging"]
    #[must_use]
    #[inline(always)]
    pub const fn CEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CAN Error Logging"]
    #[inline(always)]
    pub const fn set_CEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for FDCAN_ECR {
    #[inline(always)]
    fn default() -> FDCAN_ECR {
        FDCAN_ECR(0)
    }
}
impl core::fmt::Debug for FDCAN_ECR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ECR")
            .field("TEC", &self.TEC())
            .field("REC", &self.REC())
            .field("RP", &self.RP())
            .field("CEL", &self.CEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_ECR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_ECR {{ TEC: {=u8:?}, REC: {=u8:?}, RP: {=bool:?}, CEL: {=u8:?} }}",
            self.TEC(),
            self.REC(),
            self.RP(),
            self.CEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_ENDN(pub u32);
impl FDCAN_ENDN {
    #[doc = "TEndiannes Test Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ETV(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TEndiannes Test Value"]
    #[inline(always)]
    pub const fn set_ETV(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_ENDN {
    #[inline(always)]
    fn default() -> FDCAN_ENDN {
        FDCAN_ENDN(0)
    }
}
impl core::fmt::Debug for FDCAN_ENDN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ENDN")
            .field("ETV", &self.ETV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_ENDN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_ENDN {{ ETV: {=u32:?} }}", self.ETV())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_GFC(pub u32);
impl FDCAN_GFC {
    #[doc = "Reject Remote Frames Extended"]
    #[must_use]
    #[inline(always)]
    pub const fn RRFE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reject Remote Frames Extended"]
    #[inline(always)]
    pub const fn set_RRFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reject Remote Frames Standard"]
    #[must_use]
    #[inline(always)]
    pub const fn RRFS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reject Remote Frames Standard"]
    #[inline(always)]
    pub const fn set_RRFS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Accept Non-matching Frames Extended"]
    #[must_use]
    #[inline(always)]
    pub const fn ANFE(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub const fn set_ANFE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Accept Non-matching Frames Standard"]
    #[must_use]
    #[inline(always)]
    pub const fn ANFS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub const fn set_ANFS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for FDCAN_GFC {
    #[inline(always)]
    fn default() -> FDCAN_GFC {
        FDCAN_GFC(0)
    }
}
impl core::fmt::Debug for FDCAN_GFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_GFC")
            .field("RRFE", &self.RRFE())
            .field("RRFS", &self.RRFS())
            .field("ANFE", &self.ANFE())
            .field("ANFS", &self.ANFS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_GFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_GFC {{ RRFE: {=bool:?}, RRFS: {=bool:?}, ANFE: {=u8:?}, ANFS: {=u8:?} }}",
            self.RRFE(),
            self.RRFS(),
            self.ANFE(),
            self.ANFS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_HPMS(pub u32);
impl FDCAN_HPMS {
    #[doc = "Buffer Index"]
    #[must_use]
    #[inline(always)]
    pub const fn BIDX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Buffer Index"]
    #[inline(always)]
    pub const fn set_BIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Message Storage Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn MSI(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Message Storage Indicator"]
    #[inline(always)]
    pub const fn set_MSI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Filter Index"]
    #[must_use]
    #[inline(always)]
    pub const fn FIDX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Filter Index"]
    #[inline(always)]
    pub const fn set_FIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Filter List"]
    #[must_use]
    #[inline(always)]
    pub const fn FLST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Filter List"]
    #[inline(always)]
    pub const fn set_FLST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for FDCAN_HPMS {
    #[inline(always)]
    fn default() -> FDCAN_HPMS {
        FDCAN_HPMS(0)
    }
}
impl core::fmt::Debug for FDCAN_HPMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_HPMS")
            .field("BIDX", &self.BIDX())
            .field("MSI", &self.MSI())
            .field("FIDX", &self.FIDX())
            .field("FLST", &self.FLST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_HPMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_HPMS {{ BIDX: {=u8:?}, MSI: {=u8:?}, FIDX: {=u8:?}, FLST: {=bool:?} }}",
            self.BIDX(),
            self.MSI(),
            self.FIDX(),
            self.FLST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_IE(pub u32);
impl FDCAN_IE {
    #[doc = "Rx FIFO 0 New Message Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0NE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub const fn set_RF0NE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0WE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached Enable"]
    #[inline(always)]
    pub const fn set_RF0WE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0FE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub const fn set_RF0FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0LE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub const fn set_RF0LE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1NE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub const fn set_RF1NE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1WE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub const fn set_RF1WE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1FE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full Enable"]
    #[inline(always)]
    pub const fn set_RF1FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1LE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub const fn set_RF1LE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HPME(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message Enable"]
    #[inline(always)]
    pub const fn set_HPME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TCE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed Enable"]
    #[inline(always)]
    pub const fn set_TCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TCFE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub const fn set_TCFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TFEE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty Enable"]
    #[inline(always)]
    pub const fn set_TFEE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIFO New Entry Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFNE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub const fn set_TEFNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFWE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub const fn set_TEFWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFFE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub const fn set_TEFFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Element Lost Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFLE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub const fn set_TEFLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TSWE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound Enable"]
    #[inline(always)]
    pub const fn set_TSWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MRAFE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure Enable"]
    #[inline(always)]
    pub const fn set_MRAFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TOOE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred Enable"]
    #[inline(always)]
    pub const fn set_TOOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DRXE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub const fn set_DRXE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Error Logging Overflow Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ELOE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow Enable"]
    #[inline(always)]
    pub const fn set_ELOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EPE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Enable"]
    #[inline(always)]
    pub const fn set_EPE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EWE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status Enable"]
    #[inline(always)]
    pub const fn set_EWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn BOE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status Enable"]
    #[inline(always)]
    pub const fn set_BOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WDIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Enable"]
    #[inline(always)]
    pub const fn set_WDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PEAE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub const fn set_PEAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PEDE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub const fn set_PEDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to Reserved Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ARAE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to Reserved Address Enable"]
    #[inline(always)]
    pub const fn set_ARAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for FDCAN_IE {
    #[inline(always)]
    fn default() -> FDCAN_IE {
        FDCAN_IE(0)
    }
}
impl core::fmt::Debug for FDCAN_IE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IE")
            .field("RF0NE", &self.RF0NE())
            .field("RF0WE", &self.RF0WE())
            .field("RF0FE", &self.RF0FE())
            .field("RF0LE", &self.RF0LE())
            .field("RF1NE", &self.RF1NE())
            .field("RF1WE", &self.RF1WE())
            .field("RF1FE", &self.RF1FE())
            .field("RF1LE", &self.RF1LE())
            .field("HPME", &self.HPME())
            .field("TCE", &self.TCE())
            .field("TCFE", &self.TCFE())
            .field("TFEE", &self.TFEE())
            .field("TEFNE", &self.TEFNE())
            .field("TEFWE", &self.TEFWE())
            .field("TEFFE", &self.TEFFE())
            .field("TEFLE", &self.TEFLE())
            .field("TSWE", &self.TSWE())
            .field("MRAFE", &self.MRAFE())
            .field("TOOE", &self.TOOE())
            .field("DRXE", &self.DRXE())
            .field("ELOE", &self.ELOE())
            .field("EPE", &self.EPE())
            .field("EWE", &self.EWE())
            .field("BOE", &self.BOE())
            .field("WDIE", &self.WDIE())
            .field("PEAE", &self.PEAE())
            .field("PEDE", &self.PEDE())
            .field("ARAE", &self.ARAE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_IE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_IE {{ RF0NE: {=bool:?}, RF0WE: {=bool:?}, RF0FE: {=bool:?}, RF0LE: {=bool:?}, RF1NE: {=bool:?}, RF1WE: {=bool:?}, RF1FE: {=bool:?}, RF1LE: {=bool:?}, HPME: {=bool:?}, TCE: {=bool:?}, TCFE: {=bool:?}, TFEE: {=bool:?}, TEFNE: {=bool:?}, TEFWE: {=bool:?}, TEFFE: {=bool:?}, TEFLE: {=bool:?}, TSWE: {=bool:?}, MRAFE: {=bool:?}, TOOE: {=bool:?}, DRXE: {=bool:?}, ELOE: {=bool:?}, EPE: {=bool:?}, EWE: {=bool:?}, BOE: {=bool:?}, WDIE: {=bool:?}, PEAE: {=bool:?}, PEDE: {=bool:?}, ARAE: {=bool:?} }}",
            self.RF0NE(),
            self.RF0WE(),
            self.RF0FE(),
            self.RF0LE(),
            self.RF1NE(),
            self.RF1WE(),
            self.RF1FE(),
            self.RF1LE(),
            self.HPME(),
            self.TCE(),
            self.TCFE(),
            self.TFEE(),
            self.TEFNE(),
            self.TEFWE(),
            self.TEFFE(),
            self.TEFLE(),
            self.TSWE(),
            self.MRAFE(),
            self.TOOE(),
            self.DRXE(),
            self.ELOE(),
            self.EPE(),
            self.EWE(),
            self.BOE(),
            self.WDIE(),
            self.PEAE(),
            self.PEDE(),
            self.ARAE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_ILE(pub u32);
impl FDCAN_ILE {
    #[doc = "Enable Interrupt Line 0"]
    #[must_use]
    #[inline(always)]
    pub const fn EINT0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt Line 0"]
    #[inline(always)]
    pub const fn set_EINT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Interrupt Line 1"]
    #[must_use]
    #[inline(always)]
    pub const fn EINT1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt Line 1"]
    #[inline(always)]
    pub const fn set_EINT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for FDCAN_ILE {
    #[inline(always)]
    fn default() -> FDCAN_ILE {
        FDCAN_ILE(0)
    }
}
impl core::fmt::Debug for FDCAN_ILE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILE")
            .field("EINT0", &self.EINT0())
            .field("EINT1", &self.EINT1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_ILE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_ILE {{ EINT0: {=bool:?}, EINT1: {=bool:?} }}",
            self.EINT0(),
            self.EINT1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_ILS(pub u32);
impl FDCAN_ILS {
    #[doc = "Rx FIFO 0 New Message Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0NL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message Line"]
    #[inline(always)]
    pub const fn set_RF0NL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0WL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached Line"]
    #[inline(always)]
    pub const fn set_RF0WL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0FL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full Line"]
    #[inline(always)]
    pub const fn set_RF0FL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0LL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost Line"]
    #[inline(always)]
    pub const fn set_RF0LL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1NL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message Line"]
    #[inline(always)]
    pub const fn set_RF1NL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1WL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached Line"]
    #[inline(always)]
    pub const fn set_RF1WL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1FL(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full Line"]
    #[inline(always)]
    pub const fn set_RF1FL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost Line"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1LL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost Line"]
    #[inline(always)]
    pub const fn set_RF1LL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message Line"]
    #[must_use]
    #[inline(always)]
    pub const fn HPML(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message Line"]
    #[inline(always)]
    pub const fn set_HPML(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TCL(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed Line"]
    #[inline(always)]
    pub const fn set_TCL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TCFL(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished Line"]
    #[inline(always)]
    pub const fn set_TCFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TFEL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty Line"]
    #[inline(always)]
    pub const fn set_TFEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIFO New Entry Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFNL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO New Entry Line"]
    #[inline(always)]
    pub const fn set_TEFNL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFWL(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached Line"]
    #[inline(always)]
    pub const fn set_TEFWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFFL(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full Line"]
    #[inline(always)]
    pub const fn set_TEFFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Element Lost Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFLL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost Line"]
    #[inline(always)]
    pub const fn set_TEFLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TSWL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound Line"]
    #[inline(always)]
    pub const fn set_TSWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure Line"]
    #[must_use]
    #[inline(always)]
    pub const fn MRAFL(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure Line"]
    #[inline(always)]
    pub const fn set_MRAFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred Line"]
    #[must_use]
    #[inline(always)]
    pub const fn TOOL(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred Line"]
    #[inline(always)]
    pub const fn set_TOOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer Line"]
    #[must_use]
    #[inline(always)]
    pub const fn DRXL(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer Line"]
    #[inline(always)]
    pub const fn set_DRXL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Error Logging Overflow Line"]
    #[must_use]
    #[inline(always)]
    pub const fn ELOL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow Line"]
    #[inline(always)]
    pub const fn set_ELOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive Line"]
    #[must_use]
    #[inline(always)]
    pub const fn EPL(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive Line"]
    #[inline(always)]
    pub const fn set_EPL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status Line"]
    #[must_use]
    #[inline(always)]
    pub const fn EWL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status Line"]
    #[inline(always)]
    pub const fn set_EWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status Line"]
    #[must_use]
    #[inline(always)]
    pub const fn BOL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status Line"]
    #[inline(always)]
    pub const fn set_BOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt Line"]
    #[must_use]
    #[inline(always)]
    pub const fn WDIL(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Line"]
    #[inline(always)]
    pub const fn set_WDIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase Line"]
    #[must_use]
    #[inline(always)]
    pub const fn PEAL(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub const fn set_PEAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase Line"]
    #[must_use]
    #[inline(always)]
    pub const fn PEDL(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub const fn set_PEDL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to Reserved Address Line"]
    #[must_use]
    #[inline(always)]
    pub const fn ARAL(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to Reserved Address Line"]
    #[inline(always)]
    pub const fn set_ARAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for FDCAN_ILS {
    #[inline(always)]
    fn default() -> FDCAN_ILS {
        FDCAN_ILS(0)
    }
}
impl core::fmt::Debug for FDCAN_ILS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILS")
            .field("RF0NL", &self.RF0NL())
            .field("RF0WL", &self.RF0WL())
            .field("RF0FL", &self.RF0FL())
            .field("RF0LL", &self.RF0LL())
            .field("RF1NL", &self.RF1NL())
            .field("RF1WL", &self.RF1WL())
            .field("RF1FL", &self.RF1FL())
            .field("RF1LL", &self.RF1LL())
            .field("HPML", &self.HPML())
            .field("TCL", &self.TCL())
            .field("TCFL", &self.TCFL())
            .field("TFEL", &self.TFEL())
            .field("TEFNL", &self.TEFNL())
            .field("TEFWL", &self.TEFWL())
            .field("TEFFL", &self.TEFFL())
            .field("TEFLL", &self.TEFLL())
            .field("TSWL", &self.TSWL())
            .field("MRAFL", &self.MRAFL())
            .field("TOOL", &self.TOOL())
            .field("DRXL", &self.DRXL())
            .field("ELOL", &self.ELOL())
            .field("EPL", &self.EPL())
            .field("EWL", &self.EWL())
            .field("BOL", &self.BOL())
            .field("WDIL", &self.WDIL())
            .field("PEAL", &self.PEAL())
            .field("PEDL", &self.PEDL())
            .field("ARAL", &self.ARAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_ILS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_ILS {{ RF0NL: {=bool:?}, RF0WL: {=bool:?}, RF0FL: {=bool:?}, RF0LL: {=bool:?}, RF1NL: {=bool:?}, RF1WL: {=bool:?}, RF1FL: {=bool:?}, RF1LL: {=bool:?}, HPML: {=bool:?}, TCL: {=bool:?}, TCFL: {=bool:?}, TFEL: {=bool:?}, TEFNL: {=bool:?}, TEFWL: {=bool:?}, TEFFL: {=bool:?}, TEFLL: {=bool:?}, TSWL: {=bool:?}, MRAFL: {=bool:?}, TOOL: {=bool:?}, DRXL: {=bool:?}, ELOL: {=bool:?}, EPL: {=bool:?}, EWL: {=bool:?}, BOL: {=bool:?}, WDIL: {=bool:?}, PEAL: {=bool:?}, PEDL: {=bool:?}, ARAL: {=bool:?} }}",
            self.RF0NL(),
            self.RF0WL(),
            self.RF0FL(),
            self.RF0LL(),
            self.RF1NL(),
            self.RF1WL(),
            self.RF1FL(),
            self.RF1LL(),
            self.HPML(),
            self.TCL(),
            self.TCFL(),
            self.TFEL(),
            self.TEFNL(),
            self.TEFWL(),
            self.TEFFL(),
            self.TEFLL(),
            self.TSWL(),
            self.MRAFL(),
            self.TOOL(),
            self.DRXL(),
            self.ELOL(),
            self.EPL(),
            self.EWL(),
            self.BOL(),
            self.WDIL(),
            self.PEAL(),
            self.PEDL(),
            self.ARAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_IR(pub u32);
impl FDCAN_IR {
    #[doc = "Rx FIFO 0 New Message"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 New Message"]
    #[inline(always)]
    pub const fn set_RF0N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 Watermark Reached"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0W(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub const fn set_RF0W(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 Full"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0F(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full"]
    #[inline(always)]
    pub const fn set_RF0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 Message Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0L(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub const fn set_RF0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 New Message"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1N(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 New Message"]
    #[inline(always)]
    pub const fn set_RF1N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 Watermark Reached"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1W(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub const fn set_RF1W(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 Full"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1F(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full"]
    #[inline(always)]
    pub const fn set_RF1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 Message Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1L(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub const fn set_RF1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High Priority Message"]
    #[must_use]
    #[inline(always)]
    pub const fn HPM(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High Priority Message"]
    #[inline(always)]
    pub const fn set_HPM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission Completed"]
    #[must_use]
    #[inline(always)]
    pub const fn TC(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Completed"]
    #[inline(always)]
    pub const fn set_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission Cancellation Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn TCF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Cancellation Finished"]
    #[inline(always)]
    pub const fn set_TCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn TFE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO Empty"]
    #[inline(always)]
    pub const fn set_TFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx Event FIFO New Entry"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO New Entry"]
    #[inline(always)]
    pub const fn set_TEFN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx Event FIFO Watermark Reached"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFW(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub const fn set_TEFW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx Event FIFO Full"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Full"]
    #[inline(always)]
    pub const fn set_TEFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Event FIFO Element Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub const fn set_TEFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp Wraparound"]
    #[must_use]
    #[inline(always)]
    pub const fn TSW(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Wraparound"]
    #[inline(always)]
    pub const fn set_TSW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM Access Failure"]
    #[must_use]
    #[inline(always)]
    pub const fn MRAF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM Access Failure"]
    #[inline(always)]
    pub const fn set_MRAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout Occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn TOO(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Occurred"]
    #[inline(always)]
    pub const fn set_TOO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored to Dedicated Rx Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn DRX(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub const fn set_DRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Error Logging Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn ELO(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error Logging Overflow"]
    #[inline(always)]
    pub const fn set_ELO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error Passive"]
    #[must_use]
    #[inline(always)]
    pub const fn EP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive"]
    #[inline(always)]
    pub const fn set_EP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning Status"]
    #[must_use]
    #[inline(always)]
    pub const fn EW(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status"]
    #[inline(always)]
    pub const fn set_EW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status"]
    #[must_use]
    #[inline(always)]
    pub const fn BO(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status"]
    #[inline(always)]
    pub const fn set_BO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn WDI(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt"]
    #[inline(always)]
    pub const fn set_WDI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol Error in Arbitration Phase"]
    #[must_use]
    #[inline(always)]
    pub const fn PEA(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Arbitration Phase"]
    #[inline(always)]
    pub const fn set_PEA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol Error in Data Phase"]
    #[must_use]
    #[inline(always)]
    pub const fn PED(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Error in Data Phase"]
    #[inline(always)]
    pub const fn set_PED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to Reserved Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ARA(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to Reserved Address"]
    #[inline(always)]
    pub const fn set_ARA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for FDCAN_IR {
    #[inline(always)]
    fn default() -> FDCAN_IR {
        FDCAN_IR(0)
    }
}
impl core::fmt::Debug for FDCAN_IR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IR")
            .field("RF0N", &self.RF0N())
            .field("RF0W", &self.RF0W())
            .field("RF0F", &self.RF0F())
            .field("RF0L", &self.RF0L())
            .field("RF1N", &self.RF1N())
            .field("RF1W", &self.RF1W())
            .field("RF1F", &self.RF1F())
            .field("RF1L", &self.RF1L())
            .field("HPM", &self.HPM())
            .field("TC", &self.TC())
            .field("TCF", &self.TCF())
            .field("TFE", &self.TFE())
            .field("TEFN", &self.TEFN())
            .field("TEFW", &self.TEFW())
            .field("TEFF", &self.TEFF())
            .field("TEFL", &self.TEFL())
            .field("TSW", &self.TSW())
            .field("MRAF", &self.MRAF())
            .field("TOO", &self.TOO())
            .field("DRX", &self.DRX())
            .field("ELO", &self.ELO())
            .field("EP", &self.EP())
            .field("EW", &self.EW())
            .field("BO", &self.BO())
            .field("WDI", &self.WDI())
            .field("PEA", &self.PEA())
            .field("PED", &self.PED())
            .field("ARA", &self.ARA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_IR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_IR {{ RF0N: {=bool:?}, RF0W: {=bool:?}, RF0F: {=bool:?}, RF0L: {=bool:?}, RF1N: {=bool:?}, RF1W: {=bool:?}, RF1F: {=bool:?}, RF1L: {=bool:?}, HPM: {=bool:?}, TC: {=bool:?}, TCF: {=bool:?}, TFE: {=bool:?}, TEFN: {=bool:?}, TEFW: {=bool:?}, TEFF: {=bool:?}, TEFL: {=bool:?}, TSW: {=bool:?}, MRAF: {=bool:?}, TOO: {=bool:?}, DRX: {=bool:?}, ELO: {=bool:?}, EP: {=bool:?}, EW: {=bool:?}, BO: {=bool:?}, WDI: {=bool:?}, PEA: {=bool:?}, PED: {=bool:?}, ARA: {=bool:?} }}",
            self.RF0N(),
            self.RF0W(),
            self.RF0F(),
            self.RF0L(),
            self.RF1N(),
            self.RF1W(),
            self.RF1F(),
            self.RF1L(),
            self.HPM(),
            self.TC(),
            self.TCF(),
            self.TFE(),
            self.TEFN(),
            self.TEFW(),
            self.TEFF(),
            self.TEFL(),
            self.TSW(),
            self.MRAF(),
            self.TOO(),
            self.DRX(),
            self.ELO(),
            self.EP(),
            self.EW(),
            self.BO(),
            self.WDI(),
            self.PEA(),
            self.PED(),
            self.ARA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_NBTP(pub u32);
impl FDCAN_NBTP {
    #[doc = "Nominal Time segment after sample point"]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time segment after sample point"]
    #[inline(always)]
    pub const fn set_NTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Nominal Time segment before sample point"]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal Time segment before sample point"]
    #[inline(always)]
    pub const fn set_NTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Bit Rate Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn NBRP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Bit Rate Prescaler"]
    #[inline(always)]
    pub const fn set_NBRP(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Nominal (Re)Synchronization Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn NSJW(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub const fn set_NSJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for FDCAN_NBTP {
    #[inline(always)]
    fn default() -> FDCAN_NBTP {
        FDCAN_NBTP(0)
    }
}
impl core::fmt::Debug for FDCAN_NBTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NBTP")
            .field("NTSEG2", &self.NTSEG2())
            .field("NTSEG1", &self.NTSEG1())
            .field("NBRP", &self.NBRP())
            .field("NSJW", &self.NSJW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_NBTP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_NBTP {{ NTSEG2: {=u8:?}, NTSEG1: {=u8:?}, NBRP: {=u16:?}, NSJW: {=u8:?} }}",
            self.NTSEG2(),
            self.NTSEG1(),
            self.NBRP(),
            self.NSJW()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_NDAT1(pub u32);
impl FDCAN_NDAT1 {
    #[doc = "New Data flag of Rx Buffer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ND0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 0"]
    #[inline(always)]
    pub const fn set_ND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "New Data flag of Rx Buffer 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ND1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 1"]
    #[inline(always)]
    pub const fn set_ND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "New Data flag of Rx Buffer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ND2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 2"]
    #[inline(always)]
    pub const fn set_ND2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "New Data flag of Rx Buffer 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ND3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 3"]
    #[inline(always)]
    pub const fn set_ND3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "New Data flag of Rx Buffer 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ND4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 4"]
    #[inline(always)]
    pub const fn set_ND4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "New Data flag of Rx Buffer 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ND5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 5"]
    #[inline(always)]
    pub const fn set_ND5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "New Data flag of Rx Buffer 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ND6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 6"]
    #[inline(always)]
    pub const fn set_ND6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "New Data flag of Rx Buffer 7"]
    #[must_use]
    #[inline(always)]
    pub const fn ND7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 7"]
    #[inline(always)]
    pub const fn set_ND7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "New Data flag of Rx Buffer 8"]
    #[must_use]
    #[inline(always)]
    pub const fn ND8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 8"]
    #[inline(always)]
    pub const fn set_ND8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "New Data flag of Rx Buffer 9"]
    #[must_use]
    #[inline(always)]
    pub const fn ND9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 9"]
    #[inline(always)]
    pub const fn set_ND9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "New Data flag of Rx Buffer 10"]
    #[must_use]
    #[inline(always)]
    pub const fn ND10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 10"]
    #[inline(always)]
    pub const fn set_ND10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "New Data flag of Rx Buffer 11"]
    #[must_use]
    #[inline(always)]
    pub const fn ND11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 11"]
    #[inline(always)]
    pub const fn set_ND11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "New Data flag of Rx Buffer 12"]
    #[must_use]
    #[inline(always)]
    pub const fn ND12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 12"]
    #[inline(always)]
    pub const fn set_ND12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "New Data flag of Rx Buffer 13"]
    #[must_use]
    #[inline(always)]
    pub const fn ND13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 13"]
    #[inline(always)]
    pub const fn set_ND13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "New Data flag of Rx Buffer 14"]
    #[must_use]
    #[inline(always)]
    pub const fn ND14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 14"]
    #[inline(always)]
    pub const fn set_ND14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "New Data flag of Rx Buffer 15"]
    #[must_use]
    #[inline(always)]
    pub const fn ND15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 15"]
    #[inline(always)]
    pub const fn set_ND15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "New Data flag of Rx Buffer 16"]
    #[must_use]
    #[inline(always)]
    pub const fn ND16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 16"]
    #[inline(always)]
    pub const fn set_ND16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "New Data flag of Rx Buffer 17"]
    #[must_use]
    #[inline(always)]
    pub const fn ND17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 17"]
    #[inline(always)]
    pub const fn set_ND17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "New Data flag of Rx Buffer 18"]
    #[must_use]
    #[inline(always)]
    pub const fn ND18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 18"]
    #[inline(always)]
    pub const fn set_ND18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "New Data flag of Rx Buffer 19"]
    #[must_use]
    #[inline(always)]
    pub const fn ND19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 19"]
    #[inline(always)]
    pub const fn set_ND19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "New Data flag of Rx Buffer 20"]
    #[must_use]
    #[inline(always)]
    pub const fn ND20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 20"]
    #[inline(always)]
    pub const fn set_ND20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "New Data flag of Rx Buffer 21"]
    #[must_use]
    #[inline(always)]
    pub const fn ND21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 21"]
    #[inline(always)]
    pub const fn set_ND21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "New Data flag of Rx Buffer 22"]
    #[must_use]
    #[inline(always)]
    pub const fn ND22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 22"]
    #[inline(always)]
    pub const fn set_ND22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "New Data flag of Rx Buffer 23"]
    #[must_use]
    #[inline(always)]
    pub const fn ND23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 23"]
    #[inline(always)]
    pub const fn set_ND23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "New Data flag of Rx Buffer 24"]
    #[must_use]
    #[inline(always)]
    pub const fn ND24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 24"]
    #[inline(always)]
    pub const fn set_ND24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "New Data flag of Rx Buffer 25"]
    #[must_use]
    #[inline(always)]
    pub const fn ND25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 25"]
    #[inline(always)]
    pub const fn set_ND25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "New Data flag of Rx Buffer 26"]
    #[must_use]
    #[inline(always)]
    pub const fn ND26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 26"]
    #[inline(always)]
    pub const fn set_ND26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "New Data flag of Rx Buffer 27"]
    #[must_use]
    #[inline(always)]
    pub const fn ND27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 27"]
    #[inline(always)]
    pub const fn set_ND27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "New Data flag of Rx Buffer 28"]
    #[must_use]
    #[inline(always)]
    pub const fn ND28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 28"]
    #[inline(always)]
    pub const fn set_ND28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "New Data flag of Rx Buffer 29"]
    #[must_use]
    #[inline(always)]
    pub const fn ND29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 29"]
    #[inline(always)]
    pub const fn set_ND29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "New Data flag of Rx Buffer 30"]
    #[must_use]
    #[inline(always)]
    pub const fn ND30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 30"]
    #[inline(always)]
    pub const fn set_ND30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "New Data flag of Rx Buffer 31"]
    #[must_use]
    #[inline(always)]
    pub const fn ND31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 31"]
    #[inline(always)]
    pub const fn set_ND31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FDCAN_NDAT1 {
    #[inline(always)]
    fn default() -> FDCAN_NDAT1 {
        FDCAN_NDAT1(0)
    }
}
impl core::fmt::Debug for FDCAN_NDAT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NDAT1")
            .field("ND0", &self.ND0())
            .field("ND1", &self.ND1())
            .field("ND2", &self.ND2())
            .field("ND3", &self.ND3())
            .field("ND4", &self.ND4())
            .field("ND5", &self.ND5())
            .field("ND6", &self.ND6())
            .field("ND7", &self.ND7())
            .field("ND8", &self.ND8())
            .field("ND9", &self.ND9())
            .field("ND10", &self.ND10())
            .field("ND11", &self.ND11())
            .field("ND12", &self.ND12())
            .field("ND13", &self.ND13())
            .field("ND14", &self.ND14())
            .field("ND15", &self.ND15())
            .field("ND16", &self.ND16())
            .field("ND17", &self.ND17())
            .field("ND18", &self.ND18())
            .field("ND19", &self.ND19())
            .field("ND20", &self.ND20())
            .field("ND21", &self.ND21())
            .field("ND22", &self.ND22())
            .field("ND23", &self.ND23())
            .field("ND24", &self.ND24())
            .field("ND25", &self.ND25())
            .field("ND26", &self.ND26())
            .field("ND27", &self.ND27())
            .field("ND28", &self.ND28())
            .field("ND29", &self.ND29())
            .field("ND30", &self.ND30())
            .field("ND31", &self.ND31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_NDAT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_NDAT1 {{ ND0: {=bool:?}, ND1: {=bool:?}, ND2: {=bool:?}, ND3: {=bool:?}, ND4: {=bool:?}, ND5: {=bool:?}, ND6: {=bool:?}, ND7: {=bool:?}, ND8: {=bool:?}, ND9: {=bool:?}, ND10: {=bool:?}, ND11: {=bool:?}, ND12: {=bool:?}, ND13: {=bool:?}, ND14: {=bool:?}, ND15: {=bool:?}, ND16: {=bool:?}, ND17: {=bool:?}, ND18: {=bool:?}, ND19: {=bool:?}, ND20: {=bool:?}, ND21: {=bool:?}, ND22: {=bool:?}, ND23: {=bool:?}, ND24: {=bool:?}, ND25: {=bool:?}, ND26: {=bool:?}, ND27: {=bool:?}, ND28: {=bool:?}, ND29: {=bool:?}, ND30: {=bool:?}, ND31: {=bool:?} }}",
            self.ND0(),
            self.ND1(),
            self.ND2(),
            self.ND3(),
            self.ND4(),
            self.ND5(),
            self.ND6(),
            self.ND7(),
            self.ND8(),
            self.ND9(),
            self.ND10(),
            self.ND11(),
            self.ND12(),
            self.ND13(),
            self.ND14(),
            self.ND15(),
            self.ND16(),
            self.ND17(),
            self.ND18(),
            self.ND19(),
            self.ND20(),
            self.ND21(),
            self.ND22(),
            self.ND23(),
            self.ND24(),
            self.ND25(),
            self.ND26(),
            self.ND27(),
            self.ND28(),
            self.ND29(),
            self.ND30(),
            self.ND31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_NDAT2(pub u32);
impl FDCAN_NDAT2 {
    #[doc = "New Data flag of Rx Buffer 32"]
    #[must_use]
    #[inline(always)]
    pub const fn ND32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 32"]
    #[inline(always)]
    pub const fn set_ND32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "New Data flag of Rx Buffer 33"]
    #[must_use]
    #[inline(always)]
    pub const fn ND33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 33"]
    #[inline(always)]
    pub const fn set_ND33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "New Data flag of Rx Buffer 34"]
    #[must_use]
    #[inline(always)]
    pub const fn ND34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 34"]
    #[inline(always)]
    pub const fn set_ND34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "New Data flag of Rx Buffer 35"]
    #[must_use]
    #[inline(always)]
    pub const fn ND35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 35"]
    #[inline(always)]
    pub const fn set_ND35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "New Data flag of Rx Buffer 36"]
    #[must_use]
    #[inline(always)]
    pub const fn ND36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 36"]
    #[inline(always)]
    pub const fn set_ND36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "New Data flag of Rx Buffer 37"]
    #[must_use]
    #[inline(always)]
    pub const fn ND37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 37"]
    #[inline(always)]
    pub const fn set_ND37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "New Data flag of Rx Buffer 38"]
    #[must_use]
    #[inline(always)]
    pub const fn ND38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 38"]
    #[inline(always)]
    pub const fn set_ND38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "New Data flag of Rx Buffer 39"]
    #[must_use]
    #[inline(always)]
    pub const fn ND39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 39"]
    #[inline(always)]
    pub const fn set_ND39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "New Data flag of Rx Buffer 40"]
    #[must_use]
    #[inline(always)]
    pub const fn ND40(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 40"]
    #[inline(always)]
    pub const fn set_ND40(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "New Data flag of Rx Buffer 41"]
    #[must_use]
    #[inline(always)]
    pub const fn ND41(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 41"]
    #[inline(always)]
    pub const fn set_ND41(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "New Data flag of Rx Buffer 42"]
    #[must_use]
    #[inline(always)]
    pub const fn ND42(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 42"]
    #[inline(always)]
    pub const fn set_ND42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "New Data flag of Rx Buffer 43"]
    #[must_use]
    #[inline(always)]
    pub const fn ND43(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 43"]
    #[inline(always)]
    pub const fn set_ND43(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "New Data flag of Rx Buffer 44"]
    #[must_use]
    #[inline(always)]
    pub const fn ND44(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 44"]
    #[inline(always)]
    pub const fn set_ND44(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "New Data flag of Rx Buffer 45"]
    #[must_use]
    #[inline(always)]
    pub const fn ND45(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 45"]
    #[inline(always)]
    pub const fn set_ND45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "New Data flag of Rx Buffer 46"]
    #[must_use]
    #[inline(always)]
    pub const fn ND46(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 46"]
    #[inline(always)]
    pub const fn set_ND46(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "New Data flag of Rx Buffer 47"]
    #[must_use]
    #[inline(always)]
    pub const fn ND47(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 47"]
    #[inline(always)]
    pub const fn set_ND47(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "New Data flag of Rx Buffer 48"]
    #[must_use]
    #[inline(always)]
    pub const fn ND48(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 48"]
    #[inline(always)]
    pub const fn set_ND48(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "New Data flag of Rx Buffer 49"]
    #[must_use]
    #[inline(always)]
    pub const fn ND49(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 49"]
    #[inline(always)]
    pub const fn set_ND49(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "New Data flag of Rx Buffer 50"]
    #[must_use]
    #[inline(always)]
    pub const fn ND50(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 50"]
    #[inline(always)]
    pub const fn set_ND50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "New Data flag of Rx Buffer 51"]
    #[must_use]
    #[inline(always)]
    pub const fn ND51(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 51"]
    #[inline(always)]
    pub const fn set_ND51(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "New Data flag of Rx Buffer 52"]
    #[must_use]
    #[inline(always)]
    pub const fn ND52(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 52"]
    #[inline(always)]
    pub const fn set_ND52(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "New Data flag of Rx Buffer 53"]
    #[must_use]
    #[inline(always)]
    pub const fn ND53(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 53"]
    #[inline(always)]
    pub const fn set_ND53(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "New Data flag of Rx Buffer 54"]
    #[must_use]
    #[inline(always)]
    pub const fn ND54(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 54"]
    #[inline(always)]
    pub const fn set_ND54(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "New Data flag of Rx Buffer 55"]
    #[must_use]
    #[inline(always)]
    pub const fn ND55(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 55"]
    #[inline(always)]
    pub const fn set_ND55(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "New Data flag of Rx Buffer 56"]
    #[must_use]
    #[inline(always)]
    pub const fn ND56(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 56"]
    #[inline(always)]
    pub const fn set_ND56(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "New Data flag of Rx Buffer 57"]
    #[must_use]
    #[inline(always)]
    pub const fn ND57(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 57"]
    #[inline(always)]
    pub const fn set_ND57(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "New Data flag of Rx Buffer 58"]
    #[must_use]
    #[inline(always)]
    pub const fn ND58(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 58"]
    #[inline(always)]
    pub const fn set_ND58(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "New Data flag of Rx Buffer 59"]
    #[must_use]
    #[inline(always)]
    pub const fn ND59(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 59"]
    #[inline(always)]
    pub const fn set_ND59(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "New Data flag of Rx Buffer 60"]
    #[must_use]
    #[inline(always)]
    pub const fn ND60(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 60"]
    #[inline(always)]
    pub const fn set_ND60(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "New Data flag of Rx Buffer 61"]
    #[must_use]
    #[inline(always)]
    pub const fn ND61(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 61"]
    #[inline(always)]
    pub const fn set_ND61(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "New Data flag of Rx Buffer 62"]
    #[must_use]
    #[inline(always)]
    pub const fn ND62(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 62"]
    #[inline(always)]
    pub const fn set_ND62(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "New Data flag of Rx Buffer 63"]
    #[must_use]
    #[inline(always)]
    pub const fn ND63(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "New Data flag of Rx Buffer 63"]
    #[inline(always)]
    pub const fn set_ND63(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FDCAN_NDAT2 {
    #[inline(always)]
    fn default() -> FDCAN_NDAT2 {
        FDCAN_NDAT2(0)
    }
}
impl core::fmt::Debug for FDCAN_NDAT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NDAT2")
            .field("ND32", &self.ND32())
            .field("ND33", &self.ND33())
            .field("ND34", &self.ND34())
            .field("ND35", &self.ND35())
            .field("ND36", &self.ND36())
            .field("ND37", &self.ND37())
            .field("ND38", &self.ND38())
            .field("ND39", &self.ND39())
            .field("ND40", &self.ND40())
            .field("ND41", &self.ND41())
            .field("ND42", &self.ND42())
            .field("ND43", &self.ND43())
            .field("ND44", &self.ND44())
            .field("ND45", &self.ND45())
            .field("ND46", &self.ND46())
            .field("ND47", &self.ND47())
            .field("ND48", &self.ND48())
            .field("ND49", &self.ND49())
            .field("ND50", &self.ND50())
            .field("ND51", &self.ND51())
            .field("ND52", &self.ND52())
            .field("ND53", &self.ND53())
            .field("ND54", &self.ND54())
            .field("ND55", &self.ND55())
            .field("ND56", &self.ND56())
            .field("ND57", &self.ND57())
            .field("ND58", &self.ND58())
            .field("ND59", &self.ND59())
            .field("ND60", &self.ND60())
            .field("ND61", &self.ND61())
            .field("ND62", &self.ND62())
            .field("ND63", &self.ND63())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_NDAT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_NDAT2 {{ ND32: {=bool:?}, ND33: {=bool:?}, ND34: {=bool:?}, ND35: {=bool:?}, ND36: {=bool:?}, ND37: {=bool:?}, ND38: {=bool:?}, ND39: {=bool:?}, ND40: {=bool:?}, ND41: {=bool:?}, ND42: {=bool:?}, ND43: {=bool:?}, ND44: {=bool:?}, ND45: {=bool:?}, ND46: {=bool:?}, ND47: {=bool:?}, ND48: {=bool:?}, ND49: {=bool:?}, ND50: {=bool:?}, ND51: {=bool:?}, ND52: {=bool:?}, ND53: {=bool:?}, ND54: {=bool:?}, ND55: {=bool:?}, ND56: {=bool:?}, ND57: {=bool:?}, ND58: {=bool:?}, ND59: {=bool:?}, ND60: {=bool:?}, ND61: {=bool:?}, ND62: {=bool:?}, ND63: {=bool:?} }}",
            self.ND32(),
            self.ND33(),
            self.ND34(),
            self.ND35(),
            self.ND36(),
            self.ND37(),
            self.ND38(),
            self.ND39(),
            self.ND40(),
            self.ND41(),
            self.ND42(),
            self.ND43(),
            self.ND44(),
            self.ND45(),
            self.ND46(),
            self.ND47(),
            self.ND48(),
            self.ND49(),
            self.ND50(),
            self.ND51(),
            self.ND52(),
            self.ND53(),
            self.ND54(),
            self.ND55(),
            self.ND56(),
            self.ND57(),
            self.ND58(),
            self.ND59(),
            self.ND60(),
            self.ND61(),
            self.ND62(),
            self.ND63()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_PSR(pub u32);
impl FDCAN_PSR {
    #[doc = "Last Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn LEC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last Error Code"]
    #[inline(always)]
    pub const fn set_LEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Activity"]
    #[must_use]
    #[inline(always)]
    pub const fn ACT(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Activity"]
    #[inline(always)]
    pub const fn set_ACT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Error Passive"]
    #[must_use]
    #[inline(always)]
    pub const fn EP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive"]
    #[inline(always)]
    pub const fn set_EP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning Status"]
    #[must_use]
    #[inline(always)]
    pub const fn EW(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Status"]
    #[inline(always)]
    pub const fn set_EW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus_Off Status"]
    #[must_use]
    #[inline(always)]
    pub const fn BO(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status"]
    #[inline(always)]
    pub const fn set_BO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Data Last Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn DLEC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data Last Error Code"]
    #[inline(always)]
    pub const fn set_DLEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "ESI flag of last received FDCAN Message"]
    #[must_use]
    #[inline(always)]
    pub const fn RESI(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub const fn set_RESI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRS flag of last received FDCAN Message"]
    #[must_use]
    #[inline(always)]
    pub const fn RBRS(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub const fn set_RBRS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Received FDCAN Message"]
    #[must_use]
    #[inline(always)]
    pub const fn REDL(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Received FDCAN Message"]
    #[inline(always)]
    pub const fn set_REDL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol Exception Event"]
    #[must_use]
    #[inline(always)]
    pub const fn PXE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Event"]
    #[inline(always)]
    pub const fn set_PXE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmitter Delay Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn TDCV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub const fn set_TDCV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for FDCAN_PSR {
    #[inline(always)]
    fn default() -> FDCAN_PSR {
        FDCAN_PSR(0)
    }
}
impl core::fmt::Debug for FDCAN_PSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_PSR")
            .field("LEC", &self.LEC())
            .field("ACT", &self.ACT())
            .field("EP", &self.EP())
            .field("EW", &self.EW())
            .field("BO", &self.BO())
            .field("DLEC", &self.DLEC())
            .field("RESI", &self.RESI())
            .field("RBRS", &self.RBRS())
            .field("REDL", &self.REDL())
            .field("PXE", &self.PXE())
            .field("TDCV", &self.TDCV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_PSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_PSR {{ LEC: {=u8:?}, ACT: {=u8:?}, EP: {=bool:?}, EW: {=bool:?}, BO: {=bool:?}, DLEC: {=u8:?}, RESI: {=bool:?}, RBRS: {=bool:?}, REDL: {=bool:?}, PXE: {=bool:?}, TDCV: {=u8:?} }}",
            self.LEC(),
            self.ACT(),
            self.EP(),
            self.EW(),
            self.BO(),
            self.DLEC(),
            self.RESI(),
            self.RBRS(),
            self.REDL(),
            self.PXE(),
            self.TDCV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RWD(pub u32);
impl FDCAN_RWD {
    #[doc = "Watchdog configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn WDC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog configuration"]
    #[inline(always)]
    pub const fn set_WDC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Watchdog value"]
    #[must_use]
    #[inline(always)]
    pub const fn WDV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Watchdog value"]
    #[inline(always)]
    pub const fn set_WDV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for FDCAN_RWD {
    #[inline(always)]
    fn default() -> FDCAN_RWD {
        FDCAN_RWD(0)
    }
}
impl core::fmt::Debug for FDCAN_RWD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RWD")
            .field("WDC", &self.WDC())
            .field("WDV", &self.WDV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RWD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RWD {{ WDC: {=u8:?}, WDV: {=u8:?} }}",
            self.WDC(),
            self.WDV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXBC(pub u32);
impl FDCAN_RXBC {
    #[doc = "Rx Buffer Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn RBSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx Buffer Start Address"]
    #[inline(always)]
    pub const fn set_RBSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
}
impl Default for FDCAN_RXBC {
    #[inline(always)]
    fn default() -> FDCAN_RXBC {
        FDCAN_RXBC(0)
    }
}
impl core::fmt::Debug for FDCAN_RXBC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXBC")
            .field("RBSA", &self.RBSA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXBC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_RXBC {{ RBSA: {=u16:?} }}", self.RBSA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXESC(pub u32);
impl FDCAN_RXESC {
    #[doc = "Rx FIFO 1 Data Field Size"]
    #[must_use]
    #[inline(always)]
    pub const fn F0DS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    pub const fn set_F0DS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Rx FIFO 0 Data Field Size"]
    #[must_use]
    #[inline(always)]
    pub const fn F1DS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    pub const fn set_F1DS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Rx Buffer Data Field Size"]
    #[must_use]
    #[inline(always)]
    pub const fn RBDS(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Rx Buffer Data Field Size"]
    #[inline(always)]
    pub const fn set_RBDS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for FDCAN_RXESC {
    #[inline(always)]
    fn default() -> FDCAN_RXESC {
        FDCAN_RXESC(0)
    }
}
impl core::fmt::Debug for FDCAN_RXESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXESC")
            .field("F0DS", &self.F0DS())
            .field("F1DS", &self.F1DS())
            .field("RBDS", &self.RBDS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXESC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RXESC {{ F0DS: {=u8:?}, F1DS: {=u8:?}, RBDS: {=u8:?} }}",
            self.F0DS(),
            self.F1DS(),
            self.RBDS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF0A(pub u32);
impl FDCAN_RXF0A {
    #[doc = "Rx FIFO 0 Acknowledge Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F0AI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub const fn set_F0AI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for FDCAN_RXF0A {
    #[inline(always)]
    fn default() -> FDCAN_RXF0A {
        FDCAN_RXF0A(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF0A {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0A")
            .field("F0AI", &self.F0AI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF0A {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_RXF0A {{ F0AI: {=u8:?} }}", self.F0AI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF0C(pub u32);
impl FDCAN_RXF0C {
    #[doc = "Rx FIFO 0 Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn F0SA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub const fn set_F0SA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of Rx FIFO 0 elements"]
    #[must_use]
    #[inline(always)]
    pub const fn F0S(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of Rx FIFO 0 elements"]
    #[inline(always)]
    pub const fn set_F0S(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "FIFO 0 Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn F0WM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "FIFO 0 Watermark"]
    #[inline(always)]
    pub const fn set_F0WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 0 Operation Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn F0OM(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Operation Mode"]
    #[inline(always)]
    pub const fn set_F0OM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FDCAN_RXF0C {
    #[inline(always)]
    fn default() -> FDCAN_RXF0C {
        FDCAN_RXF0C(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF0C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0C")
            .field("F0SA", &self.F0SA())
            .field("F0S", &self.F0S())
            .field("F0WM", &self.F0WM())
            .field("F0OM", &self.F0OM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF0C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RXF0C {{ F0SA: {=u16:?}, F0S: {=u8:?}, F0WM: {=u8:?}, F0OM: {=bool:?} }}",
            self.F0SA(),
            self.F0S(),
            self.F0WM(),
            self.F0OM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF0S(pub u32);
impl FDCAN_RXF0S {
    #[doc = "Rx FIFO 0 Fill Level"]
    #[must_use]
    #[inline(always)]
    pub const fn F0FL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub const fn set_F0FL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 0 Get Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F0GI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub const fn set_F0GI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 0 Put Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F0PI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub const fn set_F0PI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 0 Full"]
    #[must_use]
    #[inline(always)]
    pub const fn F0F(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Full"]
    #[inline(always)]
    pub const fn set_F0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 0 Message Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn RF0L(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub const fn set_RF0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for FDCAN_RXF0S {
    #[inline(always)]
    fn default() -> FDCAN_RXF0S {
        FDCAN_RXF0S(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF0S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF0S")
            .field("F0FL", &self.F0FL())
            .field("F0GI", &self.F0GI())
            .field("F0PI", &self.F0PI())
            .field("F0F", &self.F0F())
            .field("RF0L", &self.RF0L())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF0S {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RXF0S {{ F0FL: {=u8:?}, F0GI: {=u8:?}, F0PI: {=u8:?}, F0F: {=bool:?}, RF0L: {=bool:?} }}",
            self.F0FL(),
            self.F0GI(),
            self.F0PI(),
            self.F0F(),
            self.RF0L()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF1A(pub u32);
impl FDCAN_RXF1A {
    #[doc = "Rx FIFO 1 Acknowledge Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F1AI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Acknowledge Index"]
    #[inline(always)]
    pub const fn set_F1AI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for FDCAN_RXF1A {
    #[inline(always)]
    fn default() -> FDCAN_RXF1A {
        FDCAN_RXF1A(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF1A {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1A")
            .field("F1AI", &self.F1AI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF1A {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_RXF1A {{ F1AI: {=u8:?} }}", self.F1AI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF1C(pub u32);
impl FDCAN_RXF1C {
    #[doc = "Rx FIFO 1 Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn F1SA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub const fn set_F1SA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of Rx FIFO 1 elements"]
    #[must_use]
    #[inline(always)]
    pub const fn F1S(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of Rx FIFO 1 elements"]
    #[inline(always)]
    pub const fn set_F1S(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 1 Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn F1WM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub const fn set_F1WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 1 Operation Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn F1OM(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 Operation Mode"]
    #[inline(always)]
    pub const fn set_F1OM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FDCAN_RXF1C {
    #[inline(always)]
    fn default() -> FDCAN_RXF1C {
        FDCAN_RXF1C(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF1C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1C")
            .field("F1SA", &self.F1SA())
            .field("F1S", &self.F1S())
            .field("F1WM", &self.F1WM())
            .field("F1OM", &self.F1OM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF1C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RXF1C {{ F1SA: {=u16:?}, F1S: {=u8:?}, F1WM: {=u8:?}, F1OM: {=bool:?} }}",
            self.F1SA(),
            self.F1S(),
            self.F1WM(),
            self.F1OM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_RXF1S(pub u32);
impl FDCAN_RXF1S {
    #[doc = "Rx FIFO 1 Fill Level"]
    #[must_use]
    #[inline(always)]
    pub const fn F1FL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub const fn set_F1FL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 1 Get Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F1GI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub const fn set_F1GI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 1 Put Index"]
    #[must_use]
    #[inline(always)]
    pub const fn F1PI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub const fn set_F1PI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 1 Full"]
    #[must_use]
    #[inline(always)]
    pub const fn F1F(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Full"]
    #[inline(always)]
    pub const fn set_F1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 1 Message Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn RF1L(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub const fn set_RF1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Debug Message Status"]
    #[must_use]
    #[inline(always)]
    pub const fn DMS(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Debug Message Status"]
    #[inline(always)]
    pub const fn set_DMS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for FDCAN_RXF1S {
    #[inline(always)]
    fn default() -> FDCAN_RXF1S {
        FDCAN_RXF1S(0)
    }
}
impl core::fmt::Debug for FDCAN_RXF1S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_RXF1S")
            .field("F1FL", &self.F1FL())
            .field("F1GI", &self.F1GI())
            .field("F1PI", &self.F1PI())
            .field("F1F", &self.F1F())
            .field("RF1L", &self.RF1L())
            .field("DMS", &self.DMS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_RXF1S {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_RXF1S {{ F1FL: {=u8:?}, F1GI: {=u8:?}, F1PI: {=u8:?}, F1F: {=bool:?}, RF1L: {=bool:?}, DMS: {=u8:?} }}",
            self.F1FL(),
            self.F1GI(),
            self.F1PI(),
            self.F1F(),
            self.RF1L(),
            self.DMS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_SIDFC(pub u32);
impl FDCAN_SIDFC {
    #[doc = "Filter List Standard Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn FLSSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter List Standard Start Address"]
    #[inline(always)]
    pub const fn set_FLSSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List Size Standard"]
    #[must_use]
    #[inline(always)]
    pub const fn LSS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List Size Standard"]
    #[inline(always)]
    pub const fn set_LSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for FDCAN_SIDFC {
    #[inline(always)]
    fn default() -> FDCAN_SIDFC {
        FDCAN_SIDFC(0)
    }
}
impl core::fmt::Debug for FDCAN_SIDFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_SIDFC")
            .field("FLSSA", &self.FLSSA())
            .field("LSS", &self.LSS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_SIDFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_SIDFC {{ FLSSA: {=u16:?}, LSS: {=u8:?} }}",
            self.FLSSA(),
            self.LSS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TDCR(pub u32);
impl FDCAN_TDCR {
    #[doc = "Transmitter Delay Compensation Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn TDCF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Filter"]
    #[inline(always)]
    pub const fn set_TDCF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transmitter Delay Compensation Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn TDCO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub const fn set_TDCO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for FDCAN_TDCR {
    #[inline(always)]
    fn default() -> FDCAN_TDCR {
        FDCAN_TDCR(0)
    }
}
impl core::fmt::Debug for FDCAN_TDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TDCR")
            .field("TDCF", &self.TDCF())
            .field("TDCO", &self.TDCO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TDCR {{ TDCF: {=u8:?}, TDCO: {=u8:?} }}",
            self.TDCF(),
            self.TDCO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TEST(pub u32);
impl FDCAN_TEST {
    #[doc = "Loop Back mode"]
    #[must_use]
    #[inline(always)]
    pub const fn LBCK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Loop Back mode"]
    #[inline(always)]
    pub const fn set_LBCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of Transmit Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn TX(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Control of Transmit Pin"]
    #[inline(always)]
    pub const fn set_TX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Receive Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn RX(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Pin"]
    #[inline(always)]
    pub const fn set_RX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for FDCAN_TEST {
    #[inline(always)]
    fn default() -> FDCAN_TEST {
        FDCAN_TEST(0)
    }
}
impl core::fmt::Debug for FDCAN_TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TEST")
            .field("LBCK", &self.LBCK())
            .field("TX", &self.TX())
            .field("RX", &self.RX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TEST {{ LBCK: {=bool:?}, TX: {=u8:?}, RX: {=bool:?} }}",
            self.LBCK(),
            self.TX(),
            self.RX()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TOCC(pub u32);
impl FDCAN_TOCC {
    #[doc = "Enable Timeout Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ETOC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timeout Counter"]
    #[inline(always)]
    pub const fn set_ETOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timeout Select"]
    #[must_use]
    #[inline(always)]
    pub const fn TOS(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Timeout Select"]
    #[inline(always)]
    pub const fn set_TOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Timeout Period"]
    #[must_use]
    #[inline(always)]
    pub const fn TOP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout Period"]
    #[inline(always)]
    pub const fn set_TOP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FDCAN_TOCC {
    #[inline(always)]
    fn default() -> FDCAN_TOCC {
        FDCAN_TOCC(0)
    }
}
impl core::fmt::Debug for FDCAN_TOCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCC")
            .field("ETOC", &self.ETOC())
            .field("TOS", &self.TOS())
            .field("TOP", &self.TOP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TOCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TOCC {{ ETOC: {=bool:?}, TOS: {=u8:?}, TOP: {=u16:?} }}",
            self.ETOC(),
            self.TOS(),
            self.TOP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TOCV(pub u32);
impl FDCAN_TOCV {
    #[doc = "Timeout Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn TOC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout Counter"]
    #[inline(always)]
    pub const fn set_TOC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FDCAN_TOCV {
    #[inline(always)]
    fn default() -> FDCAN_TOCV {
        FDCAN_TOCV(0)
    }
}
impl core::fmt::Debug for FDCAN_TOCV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCV")
            .field("TOC", &self.TOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TOCV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TOCV {{ TOC: {=u16:?} }}", self.TOC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TSCC(pub u32);
impl FDCAN_TSCC {
    #[doc = "Timestamp Select"]
    #[must_use]
    #[inline(always)]
    pub const fn TSS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Timestamp Select"]
    #[inline(always)]
    pub const fn set_TSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Timestamp Counter Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn TCP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp Counter Prescaler"]
    #[inline(always)]
    pub const fn set_TCP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FDCAN_TSCC {
    #[inline(always)]
    fn default() -> FDCAN_TSCC {
        FDCAN_TSCC(0)
    }
}
impl core::fmt::Debug for FDCAN_TSCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCC")
            .field("TSS", &self.TSS())
            .field("TCP", &self.TCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TSCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TSCC {{ TSS: {=u8:?}, TCP: {=u8:?} }}",
            self.TSS(),
            self.TCP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TSCV(pub u32);
impl FDCAN_TSCV {
    #[doc = "Timestamp Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn TSC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timestamp Counter"]
    #[inline(always)]
    pub const fn set_TSC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FDCAN_TSCV {
    #[inline(always)]
    fn default() -> FDCAN_TSCV {
        FDCAN_TSCV(0)
    }
}
impl core::fmt::Debug for FDCAN_TSCV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCV")
            .field("TSC", &self.TSC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TSCV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TSCV {{ TSC: {=u16:?} }}", self.TSC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBAR(pub u32);
impl FDCAN_TXBAR {
    #[doc = "Add Request"]
    #[must_use]
    #[inline(always)]
    pub const fn AR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Add Request"]
    #[inline(always)]
    pub const fn set_AR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBAR {
    #[inline(always)]
    fn default() -> FDCAN_TXBAR {
        FDCAN_TXBAR(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBAR")
            .field("AR", &self.AR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBAR {{ AR: {=u32:?} }}", self.AR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBC(pub u32);
impl FDCAN_TXBC {
    #[doc = "Tx Buffers Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn TBSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Tx Buffers Start Address"]
    #[inline(always)]
    pub const fn set_TBSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of Dedicated Transmit Buffers"]
    #[must_use]
    #[inline(always)]
    pub const fn NDTB(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub const fn set_NDTB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Transmit FIFO/Queue Size"]
    #[must_use]
    #[inline(always)]
    pub const fn TFQS(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub const fn set_TFQS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Tx FIFO/Queue Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn TFQM(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub const fn set_TFQM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for FDCAN_TXBC {
    #[inline(always)]
    fn default() -> FDCAN_TXBC {
        FDCAN_TXBC(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBC")
            .field("TBSA", &self.TBSA())
            .field("NDTB", &self.NDTB())
            .field("TFQS", &self.TFQS())
            .field("TFQM", &self.TFQM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TXBC {{ TBSA: {=u16:?}, NDTB: {=u8:?}, TFQS: {=u8:?}, TFQM: {=bool:?} }}",
            self.TBSA(),
            self.NDTB(),
            self.TFQS(),
            self.TFQM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBCF(pub u32);
impl FDCAN_TXBCF {
    #[doc = "Cancellation Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn CF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Finished"]
    #[inline(always)]
    pub const fn set_CF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBCF {
    #[inline(always)]
    fn default() -> FDCAN_TXBCF {
        FDCAN_TXBCF(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBCF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCF")
            .field("CF", &self.CF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBCF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBCF {{ CF: {=u32:?} }}", self.CF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBCIE(pub u32);
impl FDCAN_TXBCIE {
    #[doc = "Cancellation Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_CF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBCIE {
    #[inline(always)]
    fn default() -> FDCAN_TXBCIE {
        FDCAN_TXBCIE(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBCIE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCIE")
            .field("CF", &self.CF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBCIE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBCIE {{ CF: {=u32:?} }}", self.CF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBCR(pub u32);
impl FDCAN_TXBCR {
    #[doc = "Cancellation Request"]
    #[must_use]
    #[inline(always)]
    pub const fn CR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation Request"]
    #[inline(always)]
    pub const fn set_CR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBCR {
    #[inline(always)]
    fn default() -> FDCAN_TXBCR {
        FDCAN_TXBCR(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCR")
            .field("CR", &self.CR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBCR {{ CR: {=u32:?} }}", self.CR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBRP(pub u32);
impl FDCAN_TXBRP {
    #[doc = "Transmission Request Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn TRP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Request Pending"]
    #[inline(always)]
    pub const fn set_TRP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBRP {
    #[inline(always)]
    fn default() -> FDCAN_TXBRP {
        FDCAN_TXBRP(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBRP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBRP")
            .field("TRP", &self.TRP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBRP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBRP {{ TRP: {=u32:?} }}", self.TRP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBTIE(pub u32);
impl FDCAN_TXBTIE {
    #[doc = "Transmission Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Interrupt Enable"]
    #[inline(always)]
    pub const fn set_TIE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBTIE {
    #[inline(always)]
    fn default() -> FDCAN_TXBTIE {
        FDCAN_TXBTIE(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBTIE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTIE")
            .field("TIE", &self.TIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBTIE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBTIE {{ TIE: {=u32:?} }}", self.TIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXBTO(pub u32);
impl FDCAN_TXBTO {
    #[doc = "Transmission Occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn TO(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission Occurred"]
    #[inline(always)]
    pub const fn set_TO(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FDCAN_TXBTO {
    #[inline(always)]
    fn default() -> FDCAN_TXBTO {
        FDCAN_TXBTO(0)
    }
}
impl core::fmt::Debug for FDCAN_TXBTO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTO")
            .field("TO", &self.TO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXBTO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXBTO {{ TO: {=u32:?} }}", self.TO())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXEFA(pub u32);
impl FDCAN_TXEFA {
    #[doc = "Event FIFO Acknowledge Index"]
    #[must_use]
    #[inline(always)]
    pub const fn EFAI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub const fn set_EFAI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for FDCAN_TXEFA {
    #[inline(always)]
    fn default() -> FDCAN_TXEFA {
        FDCAN_TXEFA(0)
    }
}
impl core::fmt::Debug for FDCAN_TXEFA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFA")
            .field("EFAI", &self.EFAI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXEFA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXEFA {{ EFAI: {=u8:?} }}", self.EFAI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXEFC(pub u32);
impl FDCAN_TXEFC {
    #[doc = "Event FIFO Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn EFSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Event FIFO Start Address"]
    #[inline(always)]
    pub const fn set_EFSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Event FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn EFS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Size"]
    #[inline(always)]
    pub const fn set_EFS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn EFWM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Watermark"]
    #[inline(always)]
    pub const fn set_EFWM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for FDCAN_TXEFC {
    #[inline(always)]
    fn default() -> FDCAN_TXEFC {
        FDCAN_TXEFC(0)
    }
}
impl core::fmt::Debug for FDCAN_TXEFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFC")
            .field("EFSA", &self.EFSA())
            .field("EFS", &self.EFS())
            .field("EFWM", &self.EFWM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXEFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TXEFC {{ EFSA: {=u16:?}, EFS: {=u8:?}, EFWM: {=u8:?} }}",
            self.EFSA(),
            self.EFS(),
            self.EFWM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXEFS(pub u32);
impl FDCAN_TXEFS {
    #[doc = "Event FIFO Fill Level"]
    #[must_use]
    #[inline(always)]
    pub const fn EFFL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO Fill Level"]
    #[inline(always)]
    pub const fn set_EFFL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Event FIFO Get Index"]
    #[must_use]
    #[inline(always)]
    pub const fn EFGI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Get Index"]
    #[inline(always)]
    pub const fn set_EFGI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Event FIFO Put Index"]
    #[must_use]
    #[inline(always)]
    pub const fn EFPI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO Put Index"]
    #[inline(always)]
    pub const fn set_EFPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Event FIFO Full"]
    #[must_use]
    #[inline(always)]
    pub const fn EFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event FIFO Full"]
    #[inline(always)]
    pub const fn set_EFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tx Event FIFO Element Lost"]
    #[must_use]
    #[inline(always)]
    pub const fn TEFL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub const fn set_TEFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for FDCAN_TXEFS {
    #[inline(always)]
    fn default() -> FDCAN_TXEFS {
        FDCAN_TXEFS(0)
    }
}
impl core::fmt::Debug for FDCAN_TXEFS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFS")
            .field("EFFL", &self.EFFL())
            .field("EFGI", &self.EFGI())
            .field("EFPI", &self.EFPI())
            .field("EFF", &self.EFF())
            .field("TEFL", &self.TEFL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXEFS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TXEFS {{ EFFL: {=u8:?}, EFGI: {=u8:?}, EFPI: {=u8:?}, EFF: {=bool:?}, TEFL: {=bool:?} }}",
            self.EFFL(),
            self.EFGI(),
            self.EFPI(),
            self.EFF(),
            self.TEFL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXESC(pub u32);
impl FDCAN_TXESC {
    #[doc = "Tx Buffer Data Field Size"]
    #[must_use]
    #[inline(always)]
    pub const fn TBDS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tx Buffer Data Field Size"]
    #[inline(always)]
    pub const fn set_TBDS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for FDCAN_TXESC {
    #[inline(always)]
    fn default() -> FDCAN_TXESC {
        FDCAN_TXESC(0)
    }
}
impl core::fmt::Debug for FDCAN_TXESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXESC")
            .field("TBDS", &self.TBDS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXESC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_TXESC {{ TBDS: {=u8:?} }}", self.TBDS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_TXFQS(pub u32);
impl FDCAN_TXFQS {
    #[doc = "Tx FIFO Free Level"]
    #[must_use]
    #[inline(always)]
    pub const fn TFFL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Tx FIFO Free Level"]
    #[inline(always)]
    pub const fn set_TFFL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Tx FIFO Get Index"]
    #[must_use]
    #[inline(always)]
    pub const fn TFGI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO Get Index"]
    #[inline(always)]
    pub const fn set_TFGI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Tx FIFO/Queue Put Index"]
    #[must_use]
    #[inline(always)]
    pub const fn TFQPI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub const fn set_TFQPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Tx FIFO/Queue Full"]
    #[must_use]
    #[inline(always)]
    pub const fn TFQF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/Queue Full"]
    #[inline(always)]
    pub const fn set_TFQF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for FDCAN_TXFQS {
    #[inline(always)]
    fn default() -> FDCAN_TXFQS {
        FDCAN_TXFQS(0)
    }
}
impl core::fmt::Debug for FDCAN_TXFQS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXFQS")
            .field("TFFL", &self.TFFL())
            .field("TFGI", &self.TFGI())
            .field("TFQPI", &self.TFQPI())
            .field("TFQF", &self.TFQF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_TXFQS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_TXFQS {{ TFFL: {=u8:?}, TFGI: {=u8:?}, TFQPI: {=u8:?}, TFQF: {=bool:?} }}",
            self.TFFL(),
            self.TFGI(),
            self.TFQPI(),
            self.TFQF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_XIDAM(pub u32);
impl FDCAN_XIDAM {
    #[doc = "Extended ID Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn EIDM(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Extended ID Mask"]
    #[inline(always)]
    pub const fn set_EIDM(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for FDCAN_XIDAM {
    #[inline(always)]
    fn default() -> FDCAN_XIDAM {
        FDCAN_XIDAM(0)
    }
}
impl core::fmt::Debug for FDCAN_XIDAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_XIDAM")
            .field("EIDM", &self.EIDM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_XIDAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FDCAN_XIDAM {{ EIDM: {=u32:?} }}", self.EIDM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FDCAN_XIDFC(pub u32);
impl FDCAN_XIDFC {
    #[doc = "Filter List Standard Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn FLESA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter List Standard Start Address"]
    #[inline(always)]
    pub const fn set_FLESA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List Size Extended"]
    #[must_use]
    #[inline(always)]
    pub const fn LSE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List Size Extended"]
    #[inline(always)]
    pub const fn set_LSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for FDCAN_XIDFC {
    #[inline(always)]
    fn default() -> FDCAN_XIDFC {
        FDCAN_XIDFC(0)
    }
}
impl core::fmt::Debug for FDCAN_XIDFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_XIDFC")
            .field("FLESA", &self.FLESA())
            .field("LSE", &self.LSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FDCAN_XIDFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FDCAN_XIDFC {{ FLESA: {=u16:?}, LSE: {=u8:?} }}",
            self.FLESA(),
            self.LSE()
        )
    }
}

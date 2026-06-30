#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN {
    ptr: *mut u8,
}
unsafe impl Send for CAN {}
unsafe impl Sync for CAN {}
impl CAN {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<CAN_MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL1(self) -> crate::common::Reg<CAN_CTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMER(self) -> crate::common::Reg<CAN_TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn RXMGMASK(self) -> crate::common::Reg<CAN_RXMGMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn RX14MASK(self) -> crate::common::Reg<CAN_RX14MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn RX15MASK(self) -> crate::common::Reg<CAN_RX15MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn ECR(self) -> crate::common::Reg<CAN_ECR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn ESR1(self) -> crate::common::Reg<CAN_ESR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn IMASK2(self) -> crate::common::Reg<CAN_IMASK2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn IMASK1(self) -> crate::common::Reg<CAN_IMASK1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn IFLAG2(self) -> crate::common::Reg<CAN_IFLAG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn IFLAG1(self) -> crate::common::Reg<CAN_IFLAG1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL2(self) -> crate::common::Reg<CAN_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ESR2(self) -> crate::common::Reg<CAN_ESR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn CRCR(self) -> crate::common::Reg<CAN_CRCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn RXFGMASK(self) -> crate::common::Reg<CAN_RXFGMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn RXFIR(self) -> crate::common::Reg<CAN_RXFIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn CBT(self) -> crate::common::Reg<CAN_CBT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn IMASK3(self) -> crate::common::Reg<CAN_IMASK3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn IFLAG3(self) -> crate::common::Reg<CAN_IFLAG3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn MB_8B_CS(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 96usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 16usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CBT(pub u32);
impl CAN_CBT {
    #[doc = "Extended Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn EPSEG2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 2"]
    #[inline(always)]
    pub const fn set_EPSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Extended Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn EPSEG1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 2"]
    #[inline(always)]
    pub const fn set_EPSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Extended Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn EPROPSEG(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "Extended Phase Segment 1"]
    #[inline(always)]
    pub const fn set_EPROPSEG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "Extended Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn ERJW(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Propagation Segment"]
    #[inline(always)]
    pub const fn set_ERJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Extended Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn EPRESDIV(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Resync Jump Width"]
    #[inline(always)]
    pub const fn set_EPRESDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
    #[doc = "Extended Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn BTF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_BTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_CBT {
    #[inline(always)]
    fn default() -> CAN_CBT {
        CAN_CBT(0)
    }
}
impl core::fmt::Debug for CAN_CBT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CBT")
            .field("EPSEG2", &self.EPSEG2())
            .field("EPSEG1", &self.EPSEG1())
            .field("EPROPSEG", &self.EPROPSEG())
            .field("ERJW", &self.ERJW())
            .field("EPRESDIV", &self.EPRESDIV())
            .field("BTF", &self.BTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CBT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_CBT {{ EPSEG2: {=u8:?}, EPSEG1: {=u8:?}, EPROPSEG: {=u8:?}, ERJW: {=u8:?}, EPRESDIV: {=u16:?}, BTF: {=bool:?} }}",
            self.EPSEG2(),
            self.EPSEG1(),
            self.EPROPSEG(),
            self.ERJW(),
            self.EPRESDIV(),
            self.BTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CRCR(pub u32);
impl CAN_CRCR {
    #[doc = "Transmitted CRC value"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCRC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Transmitted CRC value"]
    #[inline(always)]
    pub const fn set_TXCRC(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Transmitted CRC value"]
    #[must_use]
    #[inline(always)]
    pub const fn MBCRC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitted CRC value"]
    #[inline(always)]
    pub const fn set_MBCRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for CAN_CRCR {
    #[inline(always)]
    fn default() -> CAN_CRCR {
        CAN_CRCR(0)
    }
}
impl core::fmt::Debug for CAN_CRCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CRCR")
            .field("TXCRC", &self.TXCRC())
            .field("MBCRC", &self.MBCRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CRCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_CRCR {{ TXCRC: {=u16:?}, MBCRC: {=u8:?} }}",
            self.TXCRC(),
            self.MBCRC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CS(pub u32);
impl CAN_CS {
    #[must_use]
    #[inline(always)]
    pub const fn DLC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DLC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn RTR(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_RTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn IDE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_IDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn SRR(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_SRR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn CODE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ESI(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ESI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BRS(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BRS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EDL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EDL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_CS {
    #[inline(always)]
    fn default() -> CAN_CS {
        CAN_CS(0)
    }
}
impl core::fmt::Debug for CAN_CS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CS")
            .field("DLC", &self.DLC())
            .field("RTR", &self.RTR())
            .field("IDE", &self.IDE())
            .field("SRR", &self.SRR())
            .field("CODE", &self.CODE())
            .field("ESI", &self.ESI())
            .field("BRS", &self.BRS())
            .field("EDL", &self.EDL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_CS {{ DLC: {=u8:?}, RTR: {=bool:?}, IDE: {=bool:?}, SRR: {=bool:?}, CODE: {=u8:?}, ESI: {=bool:?}, BRS: {=bool:?}, EDL: {=bool:?} }}",
            self.DLC(),
            self.RTR(),
            self.IDE(),
            self.SRR(),
            self.CODE(),
            self.ESI(),
            self.BRS(),
            self.EDL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CS_TIME(pub u32);
impl CAN_CS_TIME {
    #[must_use]
    #[inline(always)]
    pub const fn STAMP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_STAMP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CAN_CS_TIME {
    #[inline(always)]
    fn default() -> CAN_CS_TIME {
        CAN_CS_TIME(0)
    }
}
impl core::fmt::Debug for CAN_CS_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CS_TIME")
            .field("STAMP", &self.STAMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CS_TIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_CS_TIME {{ STAMP: {=u16:?} }}", self.STAMP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CTRL1(pub u32);
impl CAN_CTRL1 {
    #[doc = "Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn PROPSEG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Propagation Segment"]
    #[inline(always)]
    pub const fn set_PROPSEG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn LOM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Propagation Segment"]
    #[inline(always)]
    pub const fn set_LOM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LBUF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LBUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSYN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TSYN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOFFREC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOFFREC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SMP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RWRNMSK(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RWRNMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TWRNMSK(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TWRNMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LPB(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LPB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLKSRC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLKSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRMSK(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOFFMSK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOFFMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn PSEG2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 2"]
    #[inline(always)]
    pub const fn set_PSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn PSEG1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 2"]
    #[inline(always)]
    pub const fn set_PSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn RJW(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Phase Segment 1"]
    #[inline(always)]
    pub const fn set_RJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn PRESDIV(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Resync Jump Width"]
    #[inline(always)]
    pub const fn set_PRESDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CAN_CTRL1 {
    #[inline(always)]
    fn default() -> CAN_CTRL1 {
        CAN_CTRL1(0)
    }
}
impl core::fmt::Debug for CAN_CTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CTRL1")
            .field("PROPSEG", &self.PROPSEG())
            .field("LOM", &self.LOM())
            .field("LBUF", &self.LBUF())
            .field("TSYN", &self.TSYN())
            .field("BOFFREC", &self.BOFFREC())
            .field("SMP", &self.SMP())
            .field("RWRNMSK", &self.RWRNMSK())
            .field("TWRNMSK", &self.TWRNMSK())
            .field("LPB", &self.LPB())
            .field("CLKSRC", &self.CLKSRC())
            .field("ERRMSK", &self.ERRMSK())
            .field("BOFFMSK", &self.BOFFMSK())
            .field("PSEG2", &self.PSEG2())
            .field("PSEG1", &self.PSEG1())
            .field("RJW", &self.RJW())
            .field("PRESDIV", &self.PRESDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_CTRL1 {{ PROPSEG: {=u8:?}, LOM: {=bool:?}, LBUF: {=bool:?}, TSYN: {=bool:?}, BOFFREC: {=bool:?}, SMP: {=bool:?}, RWRNMSK: {=bool:?}, TWRNMSK: {=bool:?}, LPB: {=bool:?}, CLKSRC: {=bool:?}, ERRMSK: {=bool:?}, BOFFMSK: {=bool:?}, PSEG2: {=u8:?}, PSEG1: {=u8:?}, RJW: {=u8:?}, PRESDIV: {=u8:?} }}",
            self.PROPSEG(),
            self.LOM(),
            self.LBUF(),
            self.TSYN(),
            self.BOFFREC(),
            self.SMP(),
            self.RWRNMSK(),
            self.TWRNMSK(),
            self.LPB(),
            self.CLKSRC(),
            self.ERRMSK(),
            self.BOFFMSK(),
            self.PSEG2(),
            self.PSEG1(),
            self.RJW(),
            self.PRESDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CTRL2(pub u32);
impl CAN_CTRL2 {
    #[must_use]
    #[inline(always)]
    pub const fn TSTAMPCAP(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TSTAMPCAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MBTSBASE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MBTSBASE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EDFLTDIS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EDFLTDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISOCANFDEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISOCANFDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BTE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PREXCEN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PREXCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EACEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RRS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RRS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MRP(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MRP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TASD(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TASD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Transmission Arbitration Start Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn RFFN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmission Arbitration Start Delay"]
    #[inline(always)]
    pub const fn set_RFFN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Number of Legacy Receive FIFO Filters"]
    #[must_use]
    #[inline(always)]
    pub const fn WRMFRZ(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Number of Legacy Receive FIFO Filters"]
    #[inline(always)]
    pub const fn set_WRMFRZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECRWRE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECRWRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOFFDONEMSK(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOFFDONEMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CAN_CTRL2 {
    #[inline(always)]
    fn default() -> CAN_CTRL2 {
        CAN_CTRL2(0)
    }
}
impl core::fmt::Debug for CAN_CTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CTRL2")
            .field("TSTAMPCAP", &self.TSTAMPCAP())
            .field("MBTSBASE", &self.MBTSBASE())
            .field("EDFLTDIS", &self.EDFLTDIS())
            .field("ISOCANFDEN", &self.ISOCANFDEN())
            .field("BTE", &self.BTE())
            .field("PREXCEN", &self.PREXCEN())
            .field("EACEN", &self.EACEN())
            .field("RRS", &self.RRS())
            .field("MRP", &self.MRP())
            .field("TASD", &self.TASD())
            .field("RFFN", &self.RFFN())
            .field("WRMFRZ", &self.WRMFRZ())
            .field("ECRWRE", &self.ECRWRE())
            .field("BOFFDONEMSK", &self.BOFFDONEMSK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_CTRL2 {{ TSTAMPCAP: {=u8:?}, MBTSBASE: {=u8:?}, EDFLTDIS: {=bool:?}, ISOCANFDEN: {=bool:?}, BTE: {=bool:?}, PREXCEN: {=bool:?}, EACEN: {=bool:?}, RRS: {=bool:?}, MRP: {=bool:?}, TASD: {=u8:?}, RFFN: {=u8:?}, WRMFRZ: {=bool:?}, ECRWRE: {=bool:?}, BOFFDONEMSK: {=bool:?} }}",
            self.TSTAMPCAP(),
            self.MBTSBASE(),
            self.EDFLTDIS(),
            self.ISOCANFDEN(),
            self.BTE(),
            self.PREXCEN(),
            self.EACEN(),
            self.RRS(),
            self.MRP(),
            self.TASD(),
            self.RFFN(),
            self.WRMFRZ(),
            self.ECRWRE(),
            self.BOFFDONEMSK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CTRL2_ERRMSK(pub u32);
impl CAN_CTRL2_ERRMSK {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_CTRL2_ERRMSK {
    #[inline(always)]
    fn default() -> CAN_CTRL2_ERRMSK {
        CAN_CTRL2_ERRMSK(0)
    }
}
impl core::fmt::Debug for CAN_CTRL2_ERRMSK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CTRL2_ERRMSK")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CTRL2_ERRMSK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_CTRL2_ERRMSK {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_CTRL2_TIMER(pub u32);
impl CAN_CTRL2_TIMER {
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CAN_CTRL2_TIMER {
    #[inline(always)]
    fn default() -> CAN_CTRL2_TIMER {
        CAN_CTRL2_TIMER(0)
    }
}
impl core::fmt::Debug for CAN_CTRL2_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_CTRL2_TIMER")
            .field("SRC", &self.SRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_CTRL2_TIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_CTRL2_TIMER {{ SRC: {=bool:?} }}", self.SRC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ECR(pub u32);
impl CAN_ECR {
    #[doc = "Transmit Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn TXERRCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter"]
    #[inline(always)]
    pub const fn set_TXERRCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transmit Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn RXERRCNT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter"]
    #[inline(always)]
    pub const fn set_RXERRCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for CAN_ECR {
    #[inline(always)]
    fn default() -> CAN_ECR {
        CAN_ECR(0)
    }
}
impl core::fmt::Debug for CAN_ECR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ECR")
            .field("TXERRCNT", &self.TXERRCNT())
            .field("RXERRCNT", &self.RXERRCNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ECR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ECR {{ TXERRCNT: {=u8:?}, RXERRCNT: {=u8:?} }}",
            self.TXERRCNT(),
            self.RXERRCNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ECR_RXERRCNT(pub u32);
impl CAN_ECR_RXERRCNT {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CAN_ECR_RXERRCNT {
    #[inline(always)]
    fn default() -> CAN_ECR_RXERRCNT {
        CAN_ECR_RXERRCNT(0)
    }
}
impl core::fmt::Debug for CAN_ECR_RXERRCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ECR_RXERRCNT")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ECR_RXERRCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ECR_RXERRCNT {{ FAST: {=u8:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ECR_TXERRCNT(pub u32);
impl CAN_ECR_TXERRCNT {
    #[doc = "Receive Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Error Counter"]
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for CAN_ECR_TXERRCNT {
    #[inline(always)]
    fn default() -> CAN_ECR_TXERRCNT {
        CAN_ECR_TXERRCNT(0)
    }
}
impl core::fmt::Debug for CAN_ECR_TXERRCNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ECR_TXERRCNT")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ECR_TXERRCNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ECR_TXERRCNT {{ FAST: {=u8:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_EDCBT(pub u32);
impl CAN_EDCBT {
    #[doc = "Data Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Phase Segment 1"]
    #[inline(always)]
    pub const fn set_DTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Data Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Segment 1"]
    #[inline(always)]
    pub const fn set_DTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Data Phase Time Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn DRJW(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Time Segment 2"]
    #[inline(always)]
    pub const fn set_DRJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
}
impl Default for CAN_EDCBT {
    #[inline(always)]
    fn default() -> CAN_EDCBT {
        CAN_EDCBT(0)
    }
}
impl core::fmt::Debug for CAN_EDCBT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_EDCBT")
            .field("DTSEG1", &self.DTSEG1())
            .field("DTSEG2", &self.DTSEG2())
            .field("DRJW", &self.DRJW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_EDCBT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_EDCBT {{ DTSEG1: {=u8:?}, DTSEG2: {=u8:?}, DRJW: {=u8:?} }}",
            self.DTSEG1(),
            self.DTSEG2(),
            self.DRJW()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ENCBT(pub u32);
impl CAN_ENCBT {
    #[doc = "Nominal Time Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal Time Segment 1"]
    #[inline(always)]
    pub const fn set_NTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Nominal Time Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time Segment 1"]
    #[inline(always)]
    pub const fn set_NTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Nominal Time Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn NRJW(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time Segment 2"]
    #[inline(always)]
    pub const fn set_NRJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
    }
}
impl Default for CAN_ENCBT {
    #[inline(always)]
    fn default() -> CAN_ENCBT {
        CAN_ENCBT(0)
    }
}
impl core::fmt::Debug for CAN_ENCBT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ENCBT")
            .field("NTSEG1", &self.NTSEG1())
            .field("NTSEG2", &self.NTSEG2())
            .field("NRJW", &self.NRJW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ENCBT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ENCBT {{ NTSEG1: {=u8:?}, NTSEG2: {=u8:?}, NRJW: {=u8:?} }}",
            self.NTSEG1(),
            self.NTSEG2(),
            self.NRJW()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_EPRS(pub u32);
impl CAN_EPRS {
    #[must_use]
    #[inline(always)]
    pub const fn ENPRESDIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ENPRESDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Extended Nominal Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn EDPRESDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Nominal Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_EDPRESDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for CAN_EPRS {
    #[inline(always)]
    fn default() -> CAN_EPRS {
        CAN_EPRS(0)
    }
}
impl core::fmt::Debug for CAN_EPRS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_EPRS")
            .field("ENPRESDIV", &self.ENPRESDIV())
            .field("EDPRESDIV", &self.EDPRESDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_EPRS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_EPRS {{ ENPRESDIV: {=u16:?}, EDPRESDIV: {=u16:?} }}",
            self.ENPRESDIV(),
            self.EDPRESDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERFCR(pub u32);
impl CAN_ERFCR {
    #[doc = "Enhanced RX FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn ERFWM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Watermark"]
    #[inline(always)]
    pub const fn set_ERFWM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Enhanced RX FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn NFE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Watermark"]
    #[inline(always)]
    pub const fn set_NFE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn NEXIF(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements"]
    #[inline(always)]
    pub const fn set_NEXIF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Number of Extended ID Filter Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn DMALW(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Extended ID Filter Elements"]
    #[inline(always)]
    pub const fn set_DMALW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "DMA Last Word"]
    #[must_use]
    #[inline(always)]
    pub const fn ERFEN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Last Word"]
    #[inline(always)]
    pub const fn set_ERFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_ERFCR {
    #[inline(always)]
    fn default() -> CAN_ERFCR {
        CAN_ERFCR(0)
    }
}
impl core::fmt::Debug for CAN_ERFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERFCR")
            .field("ERFWM", &self.ERFWM())
            .field("NFE", &self.NFE())
            .field("NEXIF", &self.NEXIF())
            .field("DMALW", &self.DMALW())
            .field("ERFEN", &self.ERFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERFCR {{ ERFWM: {=u8:?}, NFE: {=u8:?}, NEXIF: {=u8:?}, DMALW: {=u8:?}, ERFEN: {=bool:?} }}",
            self.ERFWM(),
            self.NFE(),
            self.NEXIF(),
            self.DMALW(),
            self.ERFEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERFFEL(pub u32);
impl CAN_ERFFEL {
    #[doc = "Filter Element Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn FEL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Filter Element Bits"]
    #[inline(always)]
    pub const fn set_FEL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_ERFFEL {
    #[inline(always)]
    fn default() -> CAN_ERFFEL {
        CAN_ERFFEL(0)
    }
}
impl core::fmt::Debug for CAN_ERFFEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERFFEL")
            .field("FEL", &self.FEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERFFEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ERFFEL {{ FEL: {=u32:?} }}", self.FEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERFIER(pub u32);
impl CAN_ERFIER {
    #[must_use]
    #[inline(always)]
    pub const fn ERFDAIE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFDAIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFWMIIE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFWMIIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFOVFIE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFOVFIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFUFWIE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFUFWIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_ERFIER {
    #[inline(always)]
    fn default() -> CAN_ERFIER {
        CAN_ERFIER(0)
    }
}
impl core::fmt::Debug for CAN_ERFIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERFIER")
            .field("ERFDAIE", &self.ERFDAIE())
            .field("ERFWMIIE", &self.ERFWMIIE())
            .field("ERFOVFIE", &self.ERFOVFIE())
            .field("ERFUFWIE", &self.ERFUFWIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERFIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERFIER {{ ERFDAIE: {=bool:?}, ERFWMIIE: {=bool:?}, ERFOVFIE: {=bool:?}, ERFUFWIE: {=bool:?} }}",
            self.ERFDAIE(),
            self.ERFWMIIE(),
            self.ERFOVFIE(),
            self.ERFUFWIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERFSR(pub u32);
impl CAN_ERFSR {
    #[doc = "Enhanced RX FIFO Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn ERFEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Elements"]
    #[inline(always)]
    pub const fn set_ERFEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Enhanced RX FIFO Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn ERFF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Elements"]
    #[inline(always)]
    pub const fn set_ERFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFCLR(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFDA(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFDA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFWMI(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFWMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFOVF(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFOVF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERFUFW(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERFUFW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_ERFSR {
    #[inline(always)]
    fn default() -> CAN_ERFSR {
        CAN_ERFSR(0)
    }
}
impl core::fmt::Debug for CAN_ERFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERFSR")
            .field("ERFEL", &self.ERFEL())
            .field("ERFF", &self.ERFF())
            .field("ERFE", &self.ERFE())
            .field("ERFCLR", &self.ERFCLR())
            .field("ERFDA", &self.ERFDA())
            .field("ERFWMI", &self.ERFWMI())
            .field("ERFOVF", &self.ERFOVF())
            .field("ERFUFW", &self.ERFUFW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERFSR {{ ERFEL: {=u8:?}, ERFF: {=bool:?}, ERFE: {=bool:?}, ERFCLR: {=bool:?}, ERFDA: {=bool:?}, ERFWMI: {=bool:?}, ERFOVF: {=bool:?}, ERFUFW: {=bool:?} }}",
            self.ERFEL(),
            self.ERFF(),
            self.ERFE(),
            self.ERFCLR(),
            self.ERFDA(),
            self.ERFWMI(),
            self.ERFOVF(),
            self.ERFUFW()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERRIAR_INJADDR(pub u32);
impl CAN_ERRIAR_INJADDR {
    #[must_use]
    #[inline(always)]
    pub const fn L(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_L(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Error Injection Address Low"]
    #[must_use]
    #[inline(always)]
    pub const fn H(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x0fff;
        val as u16
    }
    #[doc = "Error Injection Address Low"]
    #[inline(always)]
    pub const fn set_H(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 2usize)) | (((val as u32) & 0x0fff) << 2usize);
    }
}
impl Default for CAN_ERRIAR_INJADDR {
    #[inline(always)]
    fn default() -> CAN_ERRIAR_INJADDR {
        CAN_ERRIAR_INJADDR(0)
    }
}
impl core::fmt::Debug for CAN_ERRIAR_INJADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERRIAR_INJADDR")
            .field("L", &self.L())
            .field("H", &self.H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERRIAR_INJADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERRIAR_INJADDR {{ L: {=u8:?}, H: {=u16:?} }}",
            self.L(),
            self.H()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERRIDPR(pub u32);
impl CAN_ERRIDPR {
    #[doc = "Data Flip Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn DFLIP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Flip Pattern"]
    #[inline(always)]
    pub const fn set_DFLIP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_ERRIDPR {
    #[inline(always)]
    fn default() -> CAN_ERRIDPR {
        CAN_ERRIDPR(0)
    }
}
impl core::fmt::Debug for CAN_ERRIDPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERRIDPR")
            .field("DFLIP", &self.DFLIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERRIDPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ERRIDPR {{ DFLIP: {=u32:?} }}", self.DFLIP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERRIPPR(pub u32);
impl CAN_ERRIPPR {
    #[must_use]
    #[inline(always)]
    pub const fn PFLIP0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PFLIP0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PFLIP1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PFLIP1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Parity Flip Pattern for Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn PFLIP2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parity Flip Pattern for Byte 1"]
    #[inline(always)]
    pub const fn set_PFLIP2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Parity Flip Pattern for Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn PFLIP3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Parity Flip Pattern for Byte 2"]
    #[inline(always)]
    pub const fn set_PFLIP3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for CAN_ERRIPPR {
    #[inline(always)]
    fn default() -> CAN_ERRIPPR {
        CAN_ERRIPPR(0)
    }
}
impl core::fmt::Debug for CAN_ERRIPPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERRIPPR")
            .field("PFLIP0", &self.PFLIP0())
            .field("PFLIP1", &self.PFLIP1())
            .field("PFLIP2", &self.PFLIP2())
            .field("PFLIP3", &self.PFLIP3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERRIPPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERRIPPR {{ PFLIP0: {=u8:?}, PFLIP1: {=u8:?}, PFLIP2: {=u8:?}, PFLIP3: {=u8:?} }}",
            self.PFLIP0(),
            self.PFLIP1(),
            self.PFLIP2(),
            self.PFLIP3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ERRSR(pub u32);
impl CAN_ERRSR {
    #[must_use]
    #[inline(always)]
    pub const fn CEIOF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CEIOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FANCEIOF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FANCEIOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HANCEIOF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HANCEIOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CEIF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CEIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FANCEIF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FANCEIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HANCEIF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HANCEIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CAN_ERRSR {
    #[inline(always)]
    fn default() -> CAN_ERRSR {
        CAN_ERRSR(0)
    }
}
impl core::fmt::Debug for CAN_ERRSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ERRSR")
            .field("CEIOF", &self.CEIOF())
            .field("FANCEIOF", &self.FANCEIOF())
            .field("HANCEIOF", &self.HANCEIOF())
            .field("CEIF", &self.CEIF())
            .field("FANCEIF", &self.FANCEIF())
            .field("HANCEIF", &self.HANCEIF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ERRSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ERRSR {{ CEIOF: {=bool:?}, FANCEIOF: {=bool:?}, HANCEIOF: {=bool:?}, CEIF: {=bool:?}, FANCEIF: {=bool:?}, HANCEIF: {=bool:?} }}",
            self.CEIOF(),
            self.FANCEIOF(),
            self.HANCEIOF(),
            self.CEIF(),
            self.FANCEIF(),
            self.HANCEIF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1(pub u32);
impl CAN_ESR1 {
    #[must_use]
    #[inline(always)]
    pub const fn WAKINT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAKINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRINT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOFFINT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOFFINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RX(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FLTCONF(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FLTCONF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TX(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IDLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXWRN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXWRN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXWRN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXWRN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STFERR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STFERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FRMERR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FRMERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CRCERR(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CRCERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACKERR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACKERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BIT0ERR(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BIT0ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BIT1ERR(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BIT1ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RWRNINT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RWRNINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TWRNINT(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TWRNINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SYNCH(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SYNCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOFFDONEINT(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BOFFDONEINT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERROVR(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERROVR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for CAN_ESR1 {
    #[inline(always)]
    fn default() -> CAN_ESR1 {
        CAN_ESR1(0)
    }
}
impl core::fmt::Debug for CAN_ESR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1")
            .field("WAKINT", &self.WAKINT())
            .field("ERRINT", &self.ERRINT())
            .field("BOFFINT", &self.BOFFINT())
            .field("RX", &self.RX())
            .field("FLTCONF", &self.FLTCONF())
            .field("TX", &self.TX())
            .field("IDLE", &self.IDLE())
            .field("RXWRN", &self.RXWRN())
            .field("TXWRN", &self.TXWRN())
            .field("STFERR", &self.STFERR())
            .field("FRMERR", &self.FRMERR())
            .field("CRCERR", &self.CRCERR())
            .field("ACKERR", &self.ACKERR())
            .field("BIT0ERR", &self.BIT0ERR())
            .field("BIT1ERR", &self.BIT1ERR())
            .field("RWRNINT", &self.RWRNINT())
            .field("TWRNINT", &self.TWRNINT())
            .field("SYNCH", &self.SYNCH())
            .field("BOFFDONEINT", &self.BOFFDONEINT())
            .field("ERROVR", &self.ERROVR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ESR1 {{ WAKINT: {=bool:?}, ERRINT: {=bool:?}, BOFFINT: {=bool:?}, RX: {=bool:?}, FLTCONF: {=u8:?}, TX: {=bool:?}, IDLE: {=bool:?}, RXWRN: {=bool:?}, TXWRN: {=bool:?}, STFERR: {=bool:?}, FRMERR: {=bool:?}, CRCERR: {=bool:?}, ACKERR: {=bool:?}, BIT0ERR: {=bool:?}, BIT1ERR: {=bool:?}, RWRNINT: {=bool:?}, TWRNINT: {=bool:?}, SYNCH: {=bool:?}, BOFFDONEINT: {=bool:?}, ERROVR: {=bool:?} }}",
            self.WAKINT(),
            self.ERRINT(),
            self.BOFFINT(),
            self.RX(),
            self.FLTCONF(),
            self.TX(),
            self.IDLE(),
            self.RXWRN(),
            self.TXWRN(),
            self.STFERR(),
            self.FRMERR(),
            self.CRCERR(),
            self.ACKERR(),
            self.BIT0ERR(),
            self.BIT1ERR(),
            self.RWRNINT(),
            self.TWRNINT(),
            self.SYNCH(),
            self.BOFFDONEINT(),
            self.ERROVR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_BIT0ERR(pub u32);
impl CAN_ESR1_BIT0ERR {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CAN_ESR1_BIT0ERR {
    #[inline(always)]
    fn default() -> CAN_ESR1_BIT0ERR {
        CAN_ESR1_BIT0ERR(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_BIT0ERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_BIT0ERR")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_BIT0ERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_BIT0ERR {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_BIT1ERR(pub u32);
impl CAN_ESR1_BIT1ERR {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_ESR1_BIT1ERR {
    #[inline(always)]
    fn default() -> CAN_ESR1_BIT1ERR {
        CAN_ESR1_BIT1ERR(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_BIT1ERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_BIT1ERR")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_BIT1ERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_BIT1ERR {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_CRCERR(pub u32);
impl CAN_ESR1_CRCERR {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for CAN_ESR1_CRCERR {
    #[inline(always)]
    fn default() -> CAN_ESR1_CRCERR {
        CAN_ESR1_CRCERR(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_CRCERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_CRCERR")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_CRCERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_CRCERR {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_ERRINT(pub u32);
impl CAN_ESR1_ERRINT {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for CAN_ESR1_ERRINT {
    #[inline(always)]
    fn default() -> CAN_ESR1_ERRINT {
        CAN_ESR1_ERRINT(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_ERRINT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_ERRINT")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_ERRINT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_ERRINT {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_FRMERR(pub u32);
impl CAN_ESR1_FRMERR {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for CAN_ESR1_FRMERR {
    #[inline(always)]
    fn default() -> CAN_ESR1_FRMERR {
        CAN_ESR1_FRMERR(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_FRMERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_FRMERR")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_FRMERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_FRMERR {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR1_STFERR(pub u32);
impl CAN_ESR1_STFERR {
    #[must_use]
    #[inline(always)]
    pub const fn FAST(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for CAN_ESR1_STFERR {
    #[inline(always)]
    fn default() -> CAN_ESR1_STFERR {
        CAN_ESR1_STFERR(0)
    }
}
impl core::fmt::Debug for CAN_ESR1_STFERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR1_STFERR")
            .field("FAST", &self.FAST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR1_STFERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_ESR1_STFERR {{ FAST: {=bool:?} }}", self.FAST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ESR2(pub u32);
impl CAN_ESR2 {
    #[must_use]
    #[inline(always)]
    pub const fn IMB(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IMB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VPS(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LPTM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_LPTM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for CAN_ESR2 {
    #[inline(always)]
    fn default() -> CAN_ESR2 {
        CAN_ESR2(0)
    }
}
impl core::fmt::Debug for CAN_ESR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ESR2")
            .field("IMB", &self.IMB())
            .field("VPS", &self.VPS())
            .field("LPTM", &self.LPTM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ESR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ESR2 {{ IMB: {=bool:?}, VPS: {=bool:?}, LPTM: {=u8:?} }}",
            self.IMB(),
            self.VPS(),
            self.LPTM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ETDC(pub u32);
impl CAN_ETDC {
    #[must_use]
    #[inline(always)]
    pub const fn ETDCVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ETDCVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ETDCFAIL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub const fn set_ETDCFAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ETDCOFF(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ETDCOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn TDMDIS(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub const fn set_TDMDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ETDCEN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ETDCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_ETDC {
    #[inline(always)]
    fn default() -> CAN_ETDC {
        CAN_ETDC(0)
    }
}
impl core::fmt::Debug for CAN_ETDC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ETDC")
            .field("ETDCVAL", &self.ETDCVAL())
            .field("ETDCFAIL", &self.ETDCFAIL())
            .field("ETDCOFF", &self.ETDCOFF())
            .field("TDMDIS", &self.TDMDIS())
            .field("ETDCEN", &self.ETDCEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ETDC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ETDC {{ ETDCVAL: {=u8:?}, ETDCFAIL: {=bool:?}, ETDCOFF: {=u8:?}, TDMDIS: {=bool:?}, ETDCEN: {=bool:?} }}",
            self.ETDCVAL(),
            self.ETDCFAIL(),
            self.ETDCOFF(),
            self.TDMDIS(),
            self.ETDCEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_FDCBT(pub u32);
impl CAN_FDCBT {
    #[doc = "Fast Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn FPSEG2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 2"]
    #[inline(always)]
    pub const fn set_FPSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Fast Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn FPSEG1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 2"]
    #[inline(always)]
    pub const fn set_FPSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Fast Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn FPROPSEG(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Fast Phase Segment 1"]
    #[inline(always)]
    pub const fn set_FPROPSEG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Fast Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn FRJW(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Propagation Segment"]
    #[inline(always)]
    pub const fn set_FRJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Fast Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn FPRESDIV(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Fast Resync Jump Width"]
    #[inline(always)]
    pub const fn set_FPRESDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for CAN_FDCBT {
    #[inline(always)]
    fn default() -> CAN_FDCBT {
        CAN_FDCBT(0)
    }
}
impl core::fmt::Debug for CAN_FDCBT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_FDCBT")
            .field("FPSEG2", &self.FPSEG2())
            .field("FPSEG1", &self.FPSEG1())
            .field("FPROPSEG", &self.FPROPSEG())
            .field("FRJW", &self.FRJW())
            .field("FPRESDIV", &self.FPRESDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_FDCBT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_FDCBT {{ FPSEG2: {=u8:?}, FPSEG1: {=u8:?}, FPROPSEG: {=u8:?}, FRJW: {=u8:?}, FPRESDIV: {=u16:?} }}",
            self.FPSEG2(),
            self.FPSEG1(),
            self.FPROPSEG(),
            self.FRJW(),
            self.FPRESDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_FDCRC_FD(pub u32);
impl CAN_FDCRC_FD {
    #[must_use]
    #[inline(always)]
    pub const fn TXCRC(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_TXCRC(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
    #[doc = "Extended Transmitted CRC value"]
    #[must_use]
    #[inline(always)]
    pub const fn MBCRC(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Extended Transmitted CRC value"]
    #[inline(always)]
    pub const fn set_MBCRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for CAN_FDCRC_FD {
    #[inline(always)]
    fn default() -> CAN_FDCRC_FD {
        CAN_FDCRC_FD(0)
    }
}
impl core::fmt::Debug for CAN_FDCRC_FD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_FDCRC_FD")
            .field("TXCRC", &self.TXCRC())
            .field("MBCRC", &self.MBCRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_FDCRC_FD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_FDCRC_FD {{ TXCRC: {=u32:?}, MBCRC: {=u8:?} }}",
            self.TXCRC(),
            self.MBCRC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_FDCTRL(pub u32);
impl CAN_FDCTRL {
    #[must_use]
    #[inline(always)]
    pub const fn TDCVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TDCVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Transceiver Delay Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn TDCOFF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub const fn set_TDCOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn TDCFAIL(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub const fn set_TDCFAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TDCEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TDCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MBDSR0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MBDSR0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MBDSR1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MBDSR1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MBDSR2(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MBDSR2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FDRATE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FDRATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_FDCTRL {
    #[inline(always)]
    fn default() -> CAN_FDCTRL {
        CAN_FDCTRL(0)
    }
}
impl core::fmt::Debug for CAN_FDCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_FDCTRL")
            .field("TDCVAL", &self.TDCVAL())
            .field("TDCOFF", &self.TDCOFF())
            .field("TDCFAIL", &self.TDCFAIL())
            .field("TDCEN", &self.TDCEN())
            .field("MBDSR0", &self.MBDSR0())
            .field("MBDSR1", &self.MBDSR1())
            .field("MBDSR2", &self.MBDSR2())
            .field("FDRATE", &self.FDRATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_FDCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_FDCTRL {{ TDCVAL: {=u8:?}, TDCOFF: {=u8:?}, TDCFAIL: {=bool:?}, TDCEN: {=bool:?}, MBDSR0: {=u8:?}, MBDSR1: {=u8:?}, MBDSR2: {=u8:?}, FDRATE: {=bool:?} }}",
            self.TDCVAL(),
            self.TDCOFF(),
            self.TDCFAIL(),
            self.TDCEN(),
            self.MBDSR0(),
            self.MBDSR1(),
            self.MBDSR2(),
            self.FDRATE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_HR_TIME_STAMP(pub u32);
impl CAN_HR_TIME_STAMP {
    #[doc = "High-Resolution Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn TS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "High-Resolution Timestamp"]
    #[inline(always)]
    pub const fn set_TS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_HR_TIME_STAMP {
    #[inline(always)]
    fn default() -> CAN_HR_TIME_STAMP {
        CAN_HR_TIME_STAMP(0)
    }
}
impl core::fmt::Debug for CAN_HR_TIME_STAMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_HR_TIME_STAMP")
            .field("TS", &self.TS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_HR_TIME_STAMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_HR_TIME_STAMP {{ TS: {=u32:?} }}", self.TS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_ID(pub u32);
impl CAN_ID {
    #[must_use]
    #[inline(always)]
    pub const fn EXT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_EXT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn STD(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_STD(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRIO(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PRIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for CAN_ID {
    #[inline(always)]
    fn default() -> CAN_ID {
        CAN_ID(0)
    }
}
impl core::fmt::Debug for CAN_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_ID")
            .field("EXT", &self.EXT())
            .field("STD", &self.STD())
            .field("PRIO", &self.PRIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_ID {{ EXT: {=u32:?}, STD: {=u16:?}, PRIO: {=u8:?} }}",
            self.EXT(),
            self.STD(),
            self.PRIO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IFLAG1(pub u32);
impl CAN_IFLAG1 {
    #[must_use]
    #[inline(always)]
    pub const fn BUF0I(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUF0I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BUF4TO1I(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BUF4TO1I(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Buffer MBi Interrupt or Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF5I(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer MBi Interrupt or Reserved"]
    #[inline(always)]
    pub const fn set_BUF5I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BUF6I(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUF6I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BUF7I(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUF7I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Buffer MBi Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF31TO8I(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Interrupt"]
    #[inline(always)]
    pub const fn set_BUF31TO8I(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for CAN_IFLAG1 {
    #[inline(always)]
    fn default() -> CAN_IFLAG1 {
        CAN_IFLAG1(0)
    }
}
impl core::fmt::Debug for CAN_IFLAG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IFLAG1")
            .field("BUF0I", &self.BUF0I())
            .field("BUF4TO1I", &self.BUF4TO1I())
            .field("BUF5I", &self.BUF5I())
            .field("BUF6I", &self.BUF6I())
            .field("BUF7I", &self.BUF7I())
            .field("BUF31TO8I", &self.BUF31TO8I())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IFLAG1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_IFLAG1 {{ BUF0I: {=bool:?}, BUF4TO1I: {=u8:?}, BUF5I: {=bool:?}, BUF6I: {=bool:?}, BUF7I: {=bool:?}, BUF31TO8I: {=u32:?} }}",
            self.BUF0I(),
            self.BUF4TO1I(),
            self.BUF5I(),
            self.BUF6I(),
            self.BUF7I(),
            self.BUF31TO8I()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IFLAG2(pub u32);
impl CAN_IFLAG2 {
    #[doc = "Buffer MBi Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF63TO32I(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Interrupt"]
    #[inline(always)]
    pub const fn set_BUF63TO32I(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_IFLAG2 {
    #[inline(always)]
    fn default() -> CAN_IFLAG2 {
        CAN_IFLAG2(0)
    }
}
impl core::fmt::Debug for CAN_IFLAG2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IFLAG2")
            .field("BUF63TO32I", &self.BUF63TO32I())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IFLAG2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_IFLAG2 {{ BUF63TO32I: {=u32:?} }}",
            self.BUF63TO32I()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IFLAG3(pub u32);
impl CAN_IFLAG3 {
    #[doc = "Buffer MBi Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF95TO64(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Interrupt"]
    #[inline(always)]
    pub const fn set_BUF95TO64(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_IFLAG3 {
    #[inline(always)]
    fn default() -> CAN_IFLAG3 {
        CAN_IFLAG3(0)
    }
}
impl core::fmt::Debug for CAN_IFLAG3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IFLAG3")
            .field("BUF95TO64", &self.BUF95TO64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IFLAG3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_IFLAG3 {{ BUF95TO64: {=u32:?} }}", self.BUF95TO64())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IMASK1(pub u32);
impl CAN_IMASK1 {
    #[doc = "Buffer MBi Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF31TO0M(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Mask"]
    #[inline(always)]
    pub const fn set_BUF31TO0M(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_IMASK1 {
    #[inline(always)]
    fn default() -> CAN_IMASK1 {
        CAN_IMASK1(0)
    }
}
impl core::fmt::Debug for CAN_IMASK1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IMASK1")
            .field("BUF31TO0M", &self.BUF31TO0M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IMASK1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_IMASK1 {{ BUF31TO0M: {=u32:?} }}", self.BUF31TO0M())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IMASK2(pub u32);
impl CAN_IMASK2 {
    #[doc = "Buffer MBi Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF63TO32M(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Mask"]
    #[inline(always)]
    pub const fn set_BUF63TO32M(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_IMASK2 {
    #[inline(always)]
    fn default() -> CAN_IMASK2 {
        CAN_IMASK2(0)
    }
}
impl core::fmt::Debug for CAN_IMASK2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IMASK2")
            .field("BUF63TO32M", &self.BUF63TO32M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IMASK2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_IMASK2 {{ BUF63TO32M: {=u32:?} }}",
            self.BUF63TO32M()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_IMASK3(pub u32);
impl CAN_IMASK3 {
    #[doc = "Buffer MBi Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn BUF95TO64M(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Mask"]
    #[inline(always)]
    pub const fn set_BUF95TO64M(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_IMASK3 {
    #[inline(always)]
    fn default() -> CAN_IMASK3 {
        CAN_IMASK3(0)
    }
}
impl core::fmt::Debug for CAN_IMASK3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_IMASK3")
            .field("BUF95TO64M", &self.BUF95TO64M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_IMASK3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_IMASK3 {{ BUF95TO64M: {=u32:?} }}",
            self.BUF95TO64M()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_MCR(pub u32);
impl CAN_MCR {
    #[must_use]
    #[inline(always)]
    pub const fn MAXMB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MAXMB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Number of the Last Message Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn IDAM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Number of the Last Message Buffer"]
    #[inline(always)]
    pub const fn set_IDAM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FDEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LPRIOEN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LPRIOEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IRMQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IRMQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SRXDIS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SRXDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOZE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAKSRC(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAKSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LPMACK(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LPMACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WRNEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WRNEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SLFWAK(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLFWAK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SUPV(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SUPV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FRZACK(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FRZACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SOFTRST(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAKMSK(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAKMSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOTRDY(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOTRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RFEN(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FRZ(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FRZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MDIS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_MCR {
    #[inline(always)]
    fn default() -> CAN_MCR {
        CAN_MCR(0)
    }
}
impl core::fmt::Debug for CAN_MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_MCR")
            .field("MAXMB", &self.MAXMB())
            .field("IDAM", &self.IDAM())
            .field("FDEN", &self.FDEN())
            .field("AEN", &self.AEN())
            .field("LPRIOEN", &self.LPRIOEN())
            .field("DMA", &self.DMA())
            .field("IRMQ", &self.IRMQ())
            .field("SRXDIS", &self.SRXDIS())
            .field("DOZE", &self.DOZE())
            .field("WAKSRC", &self.WAKSRC())
            .field("LPMACK", &self.LPMACK())
            .field("WRNEN", &self.WRNEN())
            .field("SLFWAK", &self.SLFWAK())
            .field("SUPV", &self.SUPV())
            .field("FRZACK", &self.FRZACK())
            .field("SOFTRST", &self.SOFTRST())
            .field("WAKMSK", &self.WAKMSK())
            .field("NOTRDY", &self.NOTRDY())
            .field("HALT", &self.HALT())
            .field("RFEN", &self.RFEN())
            .field("FRZ", &self.FRZ())
            .field("MDIS", &self.MDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_MCR {{ MAXMB: {=u8:?}, IDAM: {=u8:?}, FDEN: {=bool:?}, AEN: {=bool:?}, LPRIOEN: {=bool:?}, DMA: {=bool:?}, IRMQ: {=bool:?}, SRXDIS: {=bool:?}, DOZE: {=bool:?}, WAKSRC: {=bool:?}, LPMACK: {=bool:?}, WRNEN: {=bool:?}, SLFWAK: {=bool:?}, SUPV: {=bool:?}, FRZACK: {=bool:?}, SOFTRST: {=bool:?}, WAKMSK: {=bool:?}, NOTRDY: {=bool:?}, HALT: {=bool:?}, RFEN: {=bool:?}, FRZ: {=bool:?}, MDIS: {=bool:?} }}",
            self.MAXMB(),
            self.IDAM(),
            self.FDEN(),
            self.AEN(),
            self.LPRIOEN(),
            self.DMA(),
            self.IRMQ(),
            self.SRXDIS(),
            self.DOZE(),
            self.WAKSRC(),
            self.LPMACK(),
            self.WRNEN(),
            self.SLFWAK(),
            self.SUPV(),
            self.FRZACK(),
            self.SOFTRST(),
            self.WAKMSK(),
            self.NOTRDY(),
            self.HALT(),
            self.RFEN(),
            self.FRZ(),
            self.MDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_MECR(pub u32);
impl CAN_MECR {
    #[must_use]
    #[inline(always)]
    pub const fn NCEFAFRZ(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NCEFAFRZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECCDIS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECCDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RERRDIS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RERRDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EXTERRIE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EXTERRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FAERRIE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FAERRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HAERRIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HAERRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECRWRDIS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECRWRDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_MECR {
    #[inline(always)]
    fn default() -> CAN_MECR {
        CAN_MECR(0)
    }
}
impl core::fmt::Debug for CAN_MECR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_MECR")
            .field("NCEFAFRZ", &self.NCEFAFRZ())
            .field("ECCDIS", &self.ECCDIS())
            .field("RERRDIS", &self.RERRDIS())
            .field("EXTERRIE", &self.EXTERRIE())
            .field("FAERRIE", &self.FAERRIE())
            .field("HAERRIE", &self.HAERRIE())
            .field("ECRWRDIS", &self.ECRWRDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_MECR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_MECR {{ NCEFAFRZ: {=bool:?}, ECCDIS: {=bool:?}, RERRDIS: {=bool:?}, EXTERRIE: {=bool:?}, FAERRIE: {=bool:?}, HAERRIE: {=bool:?}, ECRWRDIS: {=bool:?} }}",
            self.NCEFAFRZ(),
            self.ECCDIS(),
            self.RERRDIS(),
            self.EXTERRIE(),
            self.FAERRIE(),
            self.HAERRIE(),
            self.ECRWRDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_MECR_CEI(pub u32);
impl CAN_MECR_CEI {
    #[must_use]
    #[inline(always)]
    pub const fn MSK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for CAN_MECR_CEI {
    #[inline(always)]
    fn default() -> CAN_MECR_CEI {
        CAN_MECR_CEI(0)
    }
}
impl core::fmt::Debug for CAN_MECR_CEI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_MECR_CEI")
            .field("MSK", &self.MSK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_MECR_CEI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_MECR_CEI {{ MSK: {=bool:?} }}", self.MSK())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_MECR_FANCEI(pub u32);
impl CAN_MECR_FANCEI {
    #[must_use]
    #[inline(always)]
    pub const fn MSK(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for CAN_MECR_FANCEI {
    #[inline(always)]
    fn default() -> CAN_MECR_FANCEI {
        CAN_MECR_FANCEI(0)
    }
}
impl core::fmt::Debug for CAN_MECR_FANCEI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_MECR_FANCEI")
            .field("MSK", &self.MSK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_MECR_FANCEI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_MECR_FANCEI {{ MSK: {=bool:?} }}", self.MSK())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_MECR_HANCEI(pub u32);
impl CAN_MECR_HANCEI {
    #[must_use]
    #[inline(always)]
    pub const fn MSK(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for CAN_MECR_HANCEI {
    #[inline(always)]
    fn default() -> CAN_MECR_HANCEI {
        CAN_MECR_HANCEI(0)
    }
}
impl core::fmt::Debug for CAN_MECR_HANCEI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_MECR_HANCEI")
            .field("MSK", &self.MSK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_MECR_HANCEI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_MECR_HANCEI {{ MSK: {=bool:?} }}", self.MSK())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RERRAR(pub u32);
impl CAN_RERRAR {
    #[doc = "Address Where Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRADDR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Address Where Error Detected"]
    #[inline(always)]
    pub const fn set_ERRADDR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Address Where Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn SAID(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Address Where Error Detected"]
    #[inline(always)]
    pub const fn set_SAID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "SAID"]
    #[must_use]
    #[inline(always)]
    pub const fn NCE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SAID"]
    #[inline(always)]
    pub const fn set_NCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for CAN_RERRAR {
    #[inline(always)]
    fn default() -> CAN_RERRAR {
        CAN_RERRAR(0)
    }
}
impl core::fmt::Debug for CAN_RERRAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RERRAR")
            .field("ERRADDR", &self.ERRADDR())
            .field("SAID", &self.SAID())
            .field("NCE", &self.NCE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RERRAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_RERRAR {{ ERRADDR: {=u16:?}, SAID: {=u8:?}, NCE: {=bool:?} }}",
            self.ERRADDR(),
            self.SAID(),
            self.NCE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RERRDR(pub u32);
impl CAN_RERRDR {
    #[must_use]
    #[inline(always)]
    pub const fn RDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_RDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RERRDR {
    #[inline(always)]
    fn default() -> CAN_RERRDR {
        CAN_RERRDR(0)
    }
}
impl core::fmt::Debug for CAN_RERRDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RERRDR")
            .field("RDATA", &self.RDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RERRDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RERRDR {{ RDATA: {=u32:?} }}", self.RDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RERRSYNR(pub u32);
impl CAN_RERRSYNR {
    #[must_use]
    #[inline(always)]
    pub const fn SYND0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SYND0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Error Syndrome for Byte 0 (Least Significant)"]
    #[must_use]
    #[inline(always)]
    pub const fn BE0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error Syndrome for Byte 0 (Least Significant)"]
    #[inline(always)]
    pub const fn set_BE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Syndrome for Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn SYND1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Error Syndrome for Byte 1"]
    #[inline(always)]
    pub const fn set_SYND1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Error Syndrome for Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn BE1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Syndrome for Byte 1"]
    #[inline(always)]
    pub const fn set_BE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Error Syndrome for Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn SYND2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Error Syndrome for Byte 2"]
    #[inline(always)]
    pub const fn set_SYND2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Error Syndrome for Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn BE2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Syndrome for Byte 2"]
    #[inline(always)]
    pub const fn set_BE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SYND3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SYND3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Error Syndrome for Byte 3 (Most Significant)"]
    #[must_use]
    #[inline(always)]
    pub const fn BE3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Syndrome for Byte 3 (Most Significant)"]
    #[inline(always)]
    pub const fn set_BE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CAN_RERRSYNR {
    #[inline(always)]
    fn default() -> CAN_RERRSYNR {
        CAN_RERRSYNR(0)
    }
}
impl core::fmt::Debug for CAN_RERRSYNR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RERRSYNR")
            .field("SYND0", &self.SYND0())
            .field("BE0", &self.BE0())
            .field("SYND1", &self.SYND1())
            .field("BE1", &self.BE1())
            .field("SYND2", &self.SYND2())
            .field("BE2", &self.BE2())
            .field("SYND3", &self.SYND3())
            .field("BE3", &self.BE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RERRSYNR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CAN_RERRSYNR {{ SYND0: {=u8:?}, BE0: {=bool:?}, SYND1: {=u8:?}, BE1: {=bool:?}, SYND2: {=u8:?}, BE2: {=bool:?}, SYND3: {=u8:?}, BE3: {=bool:?} }}",
            self.SYND0(),
            self.BE0(),
            self.SYND1(),
            self.BE1(),
            self.SYND2(),
            self.BE2(),
            self.SYND3(),
            self.BE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RX14MASK(pub u32);
impl CAN_RX14MASK {
    #[doc = "RX Buffer 14 Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn RX14M(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 14 Mask Bits"]
    #[inline(always)]
    pub const fn set_RX14M(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RX14MASK {
    #[inline(always)]
    fn default() -> CAN_RX14MASK {
        CAN_RX14MASK(0)
    }
}
impl core::fmt::Debug for CAN_RX14MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RX14MASK")
            .field("RX14M", &self.RX14M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RX14MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RX14MASK {{ RX14M: {=u32:?} }}", self.RX14M())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RX15MASK(pub u32);
impl CAN_RX15MASK {
    #[doc = "RX Buffer 15 Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn RX15M(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 15 Mask Bits"]
    #[inline(always)]
    pub const fn set_RX15M(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RX15MASK {
    #[inline(always)]
    fn default() -> CAN_RX15MASK {
        CAN_RX15MASK(0)
    }
}
impl core::fmt::Debug for CAN_RX15MASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RX15MASK")
            .field("RX15M", &self.RX15M())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RX15MASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RX15MASK {{ RX15M: {=u32:?} }}", self.RX15M())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RXFGMASK(pub u32);
impl CAN_RXFGMASK {
    #[doc = "Legacy RX FIFO Global Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn FGM(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Legacy RX FIFO Global Mask Bits"]
    #[inline(always)]
    pub const fn set_FGM(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RXFGMASK {
    #[inline(always)]
    fn default() -> CAN_RXFGMASK {
        CAN_RXFGMASK(0)
    }
}
impl core::fmt::Debug for CAN_RXFGMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RXFGMASK")
            .field("FGM", &self.FGM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RXFGMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RXFGMASK {{ FGM: {=u32:?} }}", self.FGM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RXFIR(pub u32);
impl CAN_RXFIR {
    #[must_use]
    #[inline(always)]
    pub const fn IDHIT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_IDHIT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for CAN_RXFIR {
    #[inline(always)]
    fn default() -> CAN_RXFIR {
        CAN_RXFIR(0)
    }
}
impl core::fmt::Debug for CAN_RXFIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RXFIR")
            .field("IDHIT", &self.IDHIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RXFIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RXFIR {{ IDHIT: {=u16:?} }}", self.IDHIT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RXIMR(pub u32);
impl CAN_RXIMR {
    #[doc = "Individual Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn MI(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Individual Mask Bits"]
    #[inline(always)]
    pub const fn set_MI(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RXIMR {
    #[inline(always)]
    fn default() -> CAN_RXIMR {
        CAN_RXIMR(0)
    }
}
impl core::fmt::Debug for CAN_RXIMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RXIMR").field("MI", &self.MI()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RXIMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RXIMR {{ MI: {=u32:?} }}", self.MI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_RXMGMASK(pub u32);
impl CAN_RXMGMASK {
    #[must_use]
    #[inline(always)]
    pub const fn MG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_MG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CAN_RXMGMASK {
    #[inline(always)]
    fn default() -> CAN_RXMGMASK {
        CAN_RXMGMASK(0)
    }
}
impl core::fmt::Debug for CAN_RXMGMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_RXMGMASK")
            .field("MG", &self.MG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_RXMGMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_RXMGMASK {{ MG: {=u32:?} }}", self.MG())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN_TIMER(pub u32);
impl CAN_TIMER {
    #[doc = "Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Value"]
    #[inline(always)]
    pub const fn set_TIMER(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CAN_TIMER {
    #[inline(always)]
    fn default() -> CAN_TIMER {
        CAN_TIMER(0)
    }
}
impl core::fmt::Debug for CAN_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAN_TIMER")
            .field("TIMER", &self.TIMER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAN_TIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAN_TIMER {{ TIMER: {=u16:?} }}", self.TIMER())
    }
}

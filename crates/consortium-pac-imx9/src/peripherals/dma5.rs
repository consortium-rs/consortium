#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5 {
    ptr: *mut u8,
}
unsafe impl Send for DMA5 {}
unsafe impl Sync for DMA5 {}
impl DMA5 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MP_CSR(self) -> crate::common::Reg<DMA5_MP_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_ES(self) -> crate::common::Reg<DMA5_MP_ES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_INT_LOW(self) -> crate::common::Reg<DMA5_MP_INT_LOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_INT_HIGH(self) -> crate::common::Reg<DMA5_MP_INT_HIGH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MP_HRS_LOW(self) -> crate::common::Reg<DMA5_MP_HRS_LOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_HRS_HIGH(self) -> crate::common::Reg<DMA5_MP_HRS_HIGH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_STOPCH(self) -> crate::common::Reg<DMA5_MP_STOPCH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_SSR_LOW(self) -> crate::common::Reg<DMA5_MP_SSR_LOW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_SSR_HIGH(self) -> crate::common::Reg<DMA5_MP_SSR_HIGH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn CH_GRPRI(self, n: usize) -> crate::common::Reg<DMA5_CH_GRPRI, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CH_MUX(self, n: usize) -> crate::common::Reg<DMA5_CH_MUX, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CH_PROT(self, n: usize) -> crate::common::Reg<DMA5_CH_PROT, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SLAST_SDA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0030usize) as _) }
    }
    #[inline(always)]
    pub const fn SLAST_SDA_HIGH(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0034usize) as _) }
    }
    #[inline(always)]
    pub const fn DADDR(self) -> crate::common::Reg<DMA5_DADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0038usize) as _) }
    }
    #[inline(always)]
    pub const fn DADDR_HIGH(self) -> crate::common::Reg<DMA5_DADDR_HIGH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_003cusize) as _) }
    }
    #[inline(always)]
    pub const fn DLAST_SGA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0040usize) as _) }
    }
    #[inline(always)]
    pub const fn DLAST_SGA_HIGH(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0044usize) as _) }
    }
    #[inline(always)]
    pub const fn DOFF(self) -> crate::common::Reg<DMA5_DOFF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0048usize) as _) }
    }
    #[inline(always)]
    pub const fn CSR(self) -> crate::common::Reg<DMA5_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_004cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_ATTR(pub u32);
impl DMA5_ATTR {
    #[must_use]
    #[inline(always)]
    pub const fn DSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn DMOD(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_DMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn SSIZE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_SSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMOD(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for DMA5_ATTR {
    #[inline(always)]
    fn default() -> DMA5_ATTR {
        DMA5_ATTR(0)
    }
}
impl core::fmt::Debug for DMA5_ATTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_ATTR")
            .field("DSIZE", &self.DSIZE())
            .field("DMOD", &self.DMOD())
            .field("SSIZE", &self.SSIZE())
            .field("SMOD", &self.SMOD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_ATTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_ATTR {{ DSIZE: {=u8:?}, DMOD: {=u8:?}, SSIZE: {=u8:?}, SMOD: {=u8:?} }}",
            self.DSIZE(),
            self.DMOD(),
            self.SSIZE(),
            self.SMOD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_BITER_ELINKNO(pub u32);
impl DMA5_BITER_ELINKNO {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn BITER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_BITER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ELINK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_ELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA5_BITER_ELINKNO {
    #[inline(always)]
    fn default() -> DMA5_BITER_ELINKNO {
        DMA5_BITER_ELINKNO(0)
    }
}
impl core::fmt::Debug for DMA5_BITER_ELINKNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_BITER_ELINKNO")
            .field("BITER", &self.BITER())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_BITER_ELINKNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_BITER_ELINKNO {{ BITER: {=u16:?}, ELINK: {=bool:?} }}",
            self.BITER(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_BITER_ELINKYES(pub u32);
impl DMA5_BITER_ELINKYES {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn BITER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_BITER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn LINKCH(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_LINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ELINK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA5_BITER_ELINKYES {
    #[inline(always)]
    fn default() -> DMA5_BITER_ELINKYES {
        DMA5_BITER_ELINKYES(0)
    }
}
impl core::fmt::Debug for DMA5_BITER_ELINKYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_BITER_ELINKYES")
            .field("BITER", &self.BITER())
            .field("LINKCH", &self.LINKCH())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_BITER_ELINKYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_BITER_ELINKYES {{ BITER: {=u16:?}, LINKCH: {=u8:?}, ELINK: {=bool:?} }}",
            self.BITER(),
            self.LINKCH(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_CSR(pub u32);
impl DMA5_CH_CSR {
    #[must_use]
    #[inline(always)]
    pub const fn ERQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EARQ(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EARQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EEI(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EEI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CX(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECX(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SWAP(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SWAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SIGNEXT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SIGNEXT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_CH_CSR {
    #[inline(always)]
    fn default() -> DMA5_CH_CSR {
        DMA5_CH_CSR(0)
    }
}
impl core::fmt::Debug for DMA5_CH_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_CSR")
            .field("ERQ", &self.ERQ())
            .field("EARQ", &self.EARQ())
            .field("EEI", &self.EEI())
            .field("CX", &self.CX())
            .field("ECX", &self.ECX())
            .field("SWAP", &self.SWAP())
            .field("SIGNEXT", &self.SIGNEXT())
            .field("DONE", &self.DONE())
            .field("ACTIVE", &self.ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_CSR {{ ERQ: {=bool:?}, EARQ: {=bool:?}, EEI: {=bool:?}, CX: {=bool:?}, ECX: {=bool:?}, SWAP: {=u8:?}, SIGNEXT: {=u8:?}, DONE: {=bool:?}, ACTIVE: {=bool:?} }}",
            self.ERQ(),
            self.EARQ(),
            self.EEI(),
            self.CX(),
            self.ECX(),
            self.SWAP(),
            self.SIGNEXT(),
            self.DONE(),
            self.ACTIVE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_ES(pub u32);
impl DMA5_CH_ES {
    #[must_use]
    #[inline(always)]
    pub const fn DBE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SBE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SGE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NCE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DAE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SOE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SAE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECX(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UCE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_CH_ES {
    #[inline(always)]
    fn default() -> DMA5_CH_ES {
        DMA5_CH_ES(0)
    }
}
impl core::fmt::Debug for DMA5_CH_ES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_ES")
            .field("DBE", &self.DBE())
            .field("SBE", &self.SBE())
            .field("SGE", &self.SGE())
            .field("NCE", &self.NCE())
            .field("DOE", &self.DOE())
            .field("DAE", &self.DAE())
            .field("SOE", &self.SOE())
            .field("SAE", &self.SAE())
            .field("ECX", &self.ECX())
            .field("UCE", &self.UCE())
            .field("ERR", &self.ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_ES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_ES {{ DBE: {=bool:?}, SBE: {=bool:?}, SGE: {=bool:?}, NCE: {=bool:?}, DOE: {=bool:?}, DAE: {=bool:?}, SOE: {=bool:?}, SAE: {=bool:?}, ECX: {=bool:?}, UCE: {=bool:?}, ERR: {=bool:?} }}",
            self.DBE(),
            self.SBE(),
            self.SGE(),
            self.NCE(),
            self.DOE(),
            self.DAE(),
            self.SOE(),
            self.SAE(),
            self.ECX(),
            self.UCE(),
            self.ERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_GRPRI(pub u32);
impl DMA5_CH_GRPRI {
    #[doc = "Arbitration Group Per Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn GRPRI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Arbitration Group Per Channel"]
    #[inline(always)]
    pub const fn set_GRPRI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for DMA5_CH_GRPRI {
    #[inline(always)]
    fn default() -> DMA5_CH_GRPRI {
        DMA5_CH_GRPRI(0)
    }
}
impl core::fmt::Debug for DMA5_CH_GRPRI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_GRPRI")
            .field("GRPRI", &self.GRPRI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_GRPRI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_CH_GRPRI {{ GRPRI: {=u8:?} }}", self.GRPRI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_INT(pub u32);
impl DMA5_CH_INT {
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for DMA5_CH_INT {
    #[inline(always)]
    fn default() -> DMA5_CH_INT {
        DMA5_CH_INT(0)
    }
}
impl core::fmt::Debug for DMA5_CH_INT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_INT")
            .field("INT", &self.INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_INT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_CH_INT {{ INT: {=bool:?} }}", self.INT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_MATTR(pub u32);
impl DMA5_CH_MATTR {
    #[doc = "Read Cache Attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn RCACHE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Cache Attributes"]
    #[inline(always)]
    pub const fn set_RCACHE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Read Cache Attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn WCACHE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Cache Attributes"]
    #[inline(always)]
    pub const fn set_WCACHE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Write Cache Attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn RDOMAINS(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Write Cache Attributes"]
    #[inline(always)]
    pub const fn set_RDOMAINS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDOMAINS(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_WDOMAINS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WSNOOPS(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_WSNOOPS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Write Snoop Transaction Type"]
    #[must_use]
    #[inline(always)]
    pub const fn WSENB(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write Snoop Transaction Type"]
    #[inline(always)]
    pub const fn set_WSENB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Stash Local Processor Thread"]
    #[must_use]
    #[inline(always)]
    pub const fn WSTHRD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Stash Local Processor Thread"]
    #[inline(always)]
    pub const fn set_WSTHRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Stash Local Processor Thread"]
    #[must_use]
    #[inline(always)]
    pub const fn WSCORE(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Stash Local Processor Thread"]
    #[inline(always)]
    pub const fn set_WSCORE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
}
impl Default for DMA5_CH_MATTR {
    #[inline(always)]
    fn default() -> DMA5_CH_MATTR {
        DMA5_CH_MATTR(0)
    }
}
impl core::fmt::Debug for DMA5_CH_MATTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_MATTR")
            .field("RCACHE", &self.RCACHE())
            .field("WCACHE", &self.WCACHE())
            .field("RDOMAINS", &self.RDOMAINS())
            .field("WDOMAINS", &self.WDOMAINS())
            .field("WSNOOPS", &self.WSNOOPS())
            .field("WSENB", &self.WSENB())
            .field("WSTHRD", &self.WSTHRD())
            .field("WSCORE", &self.WSCORE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_MATTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_MATTR {{ RCACHE: {=u8:?}, WCACHE: {=u8:?}, RDOMAINS: {=u8:?}, WDOMAINS: {=u8:?}, WSNOOPS: {=u8:?}, WSENB: {=bool:?}, WSTHRD: {=bool:?}, WSCORE: {=u8:?} }}",
            self.RCACHE(),
            self.WCACHE(),
            self.RDOMAINS(),
            self.WDOMAINS(),
            self.WSNOOPS(),
            self.WSENB(),
            self.WSTHRD(),
            self.WSCORE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_MUX(pub u32);
impl DMA5_CH_MUX {
    #[doc = "Service Request Source"]
    #[must_use]
    #[inline(always)]
    pub const fn SRC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Service Request Source"]
    #[inline(always)]
    pub const fn set_SRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for DMA5_CH_MUX {
    #[inline(always)]
    fn default() -> DMA5_CH_MUX {
        DMA5_CH_MUX(0)
    }
}
impl core::fmt::Debug for DMA5_CH_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_MUX")
            .field("SRC", &self.SRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_MUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_CH_MUX {{ SRC: {=u8:?} }}", self.SRC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_PRI(pub u32);
impl DMA5_CH_PRI {
    #[doc = "Arbitration Priority Level"]
    #[must_use]
    #[inline(always)]
    pub const fn APL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Arbitration Priority Level"]
    #[inline(always)]
    pub const fn set_APL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Arbitration Priority Level"]
    #[must_use]
    #[inline(always)]
    pub const fn DPA(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration Priority Level"]
    #[inline(always)]
    pub const fn set_DPA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECP(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_CH_PRI {
    #[inline(always)]
    fn default() -> DMA5_CH_PRI {
        DMA5_CH_PRI(0)
    }
}
impl core::fmt::Debug for DMA5_CH_PRI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_PRI")
            .field("APL", &self.APL())
            .field("DPA", &self.DPA())
            .field("ECP", &self.ECP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_PRI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_PRI {{ APL: {=u8:?}, DPA: {=bool:?}, ECP: {=bool:?} }}",
            self.APL(),
            self.DPA(),
            self.ECP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_PROT(pub u32);
impl DMA5_CH_PROT {
    #[must_use]
    #[inline(always)]
    pub const fn MID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INSTR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INSTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PAL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EMI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for DMA5_CH_PROT {
    #[inline(always)]
    fn default() -> DMA5_CH_PROT {
        DMA5_CH_PROT(0)
    }
}
impl core::fmt::Debug for DMA5_CH_PROT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_PROT")
            .field("MID", &self.MID())
            .field("INSTR", &self.INSTR())
            .field("SEC", &self.SEC())
            .field("PAL", &self.PAL())
            .field("EMI", &self.EMI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_PROT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_PROT {{ MID: {=u8:?}, INSTR: {=bool:?}, SEC: {=bool:?}, PAL: {=bool:?}, EMI: {=bool:?} }}",
            self.MID(),
            self.INSTR(),
            self.SEC(),
            self.PAL(),
            self.EMI()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CH_SBR(pub u32);
impl DMA5_CH_SBR {
    #[must_use]
    #[inline(always)]
    pub const fn MID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INSTR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INSTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PAL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EMI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ATTR(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ATTR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
    }
}
impl Default for DMA5_CH_SBR {
    #[inline(always)]
    fn default() -> DMA5_CH_SBR {
        DMA5_CH_SBR(0)
    }
}
impl core::fmt::Debug for DMA5_CH_SBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CH_SBR")
            .field("MID", &self.MID())
            .field("INSTR", &self.INSTR())
            .field("PAL", &self.PAL())
            .field("EMI", &self.EMI())
            .field("ATTR", &self.ATTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CH_SBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CH_SBR {{ MID: {=u8:?}, INSTR: {=bool:?}, PAL: {=bool:?}, EMI: {=bool:?}, ATTR: {=u8:?} }}",
            self.MID(),
            self.INSTR(),
            self.PAL(),
            self.EMI(),
            self.ATTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CITER_ELINKNO(pub u32);
impl DMA5_CITER_ELINKNO {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn CITER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_CITER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ELINK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_ELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA5_CITER_ELINKNO {
    #[inline(always)]
    fn default() -> DMA5_CITER_ELINKNO {
        DMA5_CITER_ELINKNO(0)
    }
}
impl core::fmt::Debug for DMA5_CITER_ELINKNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CITER_ELINKNO")
            .field("CITER", &self.CITER())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CITER_ELINKNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CITER_ELINKNO {{ CITER: {=u16:?}, ELINK: {=bool:?} }}",
            self.CITER(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CITER_ELINKYES(pub u32);
impl DMA5_CITER_ELINKYES {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn CITER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_CITER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn LINKCH(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_LINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ELINK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA5_CITER_ELINKYES {
    #[inline(always)]
    fn default() -> DMA5_CITER_ELINKYES {
        DMA5_CITER_ELINKYES(0)
    }
}
impl core::fmt::Debug for DMA5_CITER_ELINKYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CITER_ELINKYES")
            .field("CITER", &self.CITER())
            .field("LINKCH", &self.LINKCH())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CITER_ELINKYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CITER_ELINKYES {{ CITER: {=u16:?}, LINKCH: {=u8:?}, ELINK: {=bool:?} }}",
            self.CITER(),
            self.LINKCH(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_CSR(pub u32);
impl DMA5_CSR {
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INTMAJOR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INTMAJOR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INTHALF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INTHALF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DREQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ESG(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ESG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MAJORELINK(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MAJORELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EEOP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EEOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ESDA(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ESDA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MAJORLINKCH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MAJORLINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn TMC(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_TMC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for DMA5_CSR {
    #[inline(always)]
    fn default() -> DMA5_CSR {
        DMA5_CSR(0)
    }
}
impl core::fmt::Debug for DMA5_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_CSR")
            .field("START", &self.START())
            .field("INTMAJOR", &self.INTMAJOR())
            .field("INTHALF", &self.INTHALF())
            .field("DREQ", &self.DREQ())
            .field("ESG", &self.ESG())
            .field("MAJORELINK", &self.MAJORELINK())
            .field("EEOP", &self.EEOP())
            .field("ESDA", &self.ESDA())
            .field("MAJORLINKCH", &self.MAJORLINKCH())
            .field("TMC", &self.TMC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_CSR {{ START: {=bool:?}, INTMAJOR: {=bool:?}, INTHALF: {=bool:?}, DREQ: {=bool:?}, ESG: {=bool:?}, MAJORELINK: {=bool:?}, EEOP: {=bool:?}, ESDA: {=bool:?}, MAJORLINKCH: {=u8:?}, TMC: {=u8:?} }}",
            self.START(),
            self.INTMAJOR(),
            self.INTHALF(),
            self.DREQ(),
            self.ESG(),
            self.MAJORELINK(),
            self.EEOP(),
            self.ESDA(),
            self.MAJORLINKCH(),
            self.TMC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_DADDR(pub u32);
impl DMA5_DADDR {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn DADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_DADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_DADDR {
    #[inline(always)]
    fn default() -> DMA5_DADDR {
        DMA5_DADDR(0)
    }
}
impl core::fmt::Debug for DMA5_DADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_DADDR")
            .field("DADDR", &self.DADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_DADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_DADDR {{ DADDR: {=u32:?} }}", self.DADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_DADDR_HIGH(pub u32);
impl DMA5_DADDR_HIGH {
    #[must_use]
    #[inline(always)]
    pub const fn DADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_DADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_DADDR_HIGH {
    #[inline(always)]
    fn default() -> DMA5_DADDR_HIGH {
        DMA5_DADDR_HIGH(0)
    }
}
impl core::fmt::Debug for DMA5_DADDR_HIGH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_DADDR_HIGH")
            .field("DADDR", &self.DADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_DADDR_HIGH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_DADDR_HIGH {{ DADDR: {=u32:?} }}", self.DADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_DLAST_SGA_DLAST(pub u32);
impl DMA5_DLAST_SGA_DLAST {
    #[must_use]
    #[inline(always)]
    pub const fn SGA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SGA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_DLAST_SGA_DLAST {
    #[inline(always)]
    fn default() -> DMA5_DLAST_SGA_DLAST {
        DMA5_DLAST_SGA_DLAST(0)
    }
}
impl core::fmt::Debug for DMA5_DLAST_SGA_DLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_DLAST_SGA_DLAST")
            .field("SGA", &self.SGA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_DLAST_SGA_DLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_DLAST_SGA_DLAST {{ SGA: {=u32:?} }}", self.SGA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_DLAST_SGA_HIGH_DLAST(pub u32);
impl DMA5_DLAST_SGA_HIGH_DLAST {
    #[must_use]
    #[inline(always)]
    pub const fn SGA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SGA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_DLAST_SGA_HIGH_DLAST {
    #[inline(always)]
    fn default() -> DMA5_DLAST_SGA_HIGH_DLAST {
        DMA5_DLAST_SGA_HIGH_DLAST(0)
    }
}
impl core::fmt::Debug for DMA5_DLAST_SGA_HIGH_DLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_DLAST_SGA_HIGH_DLAST")
            .field("SGA", &self.SGA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_DLAST_SGA_HIGH_DLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_DLAST_SGA_HIGH_DLAST {{ SGA: {=u32:?} }}",
            self.SGA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_DOFF(pub u32);
impl DMA5_DOFF {
    #[must_use]
    #[inline(always)]
    pub const fn DOFF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMA5_DOFF {
    #[inline(always)]
    fn default() -> DMA5_DOFF {
        DMA5_DOFF(0)
    }
}
impl core::fmt::Debug for DMA5_DOFF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_DOFF")
            .field("DOFF", &self.DOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_DOFF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_DOFF {{ DOFF: {=u16:?} }}", self.DOFF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_CSR(pub u32);
impl DMA5_MP_CSR {
    #[must_use]
    #[inline(always)]
    pub const fn EDBG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EDBG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERCA(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERCA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GCPC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GCPC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HAE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GCLC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GCLC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GMRC(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GMRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "eDMA Version"]
    #[must_use]
    #[inline(always)]
    pub const fn VER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "eDMA Version"]
    #[inline(always)]
    pub const fn set_VER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_MP_CSR {
    #[inline(always)]
    fn default() -> DMA5_MP_CSR {
        DMA5_MP_CSR(0)
    }
}
impl core::fmt::Debug for DMA5_MP_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_CSR")
            .field("EDBG", &self.EDBG())
            .field("ERCA", &self.ERCA())
            .field("GCPC", &self.GCPC())
            .field("HAE", &self.HAE())
            .field("HALT", &self.HALT())
            .field("GCLC", &self.GCLC())
            .field("GMRC", &self.GMRC())
            .field("VER", &self.VER())
            .field("ACTIVE", &self.ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_MP_CSR {{ EDBG: {=bool:?}, ERCA: {=bool:?}, GCPC: {=bool:?}, HAE: {=bool:?}, HALT: {=bool:?}, GCLC: {=bool:?}, GMRC: {=bool:?}, VER: {=u8:?}, ACTIVE: {=bool:?} }}",
            self.EDBG(),
            self.ERCA(),
            self.GCPC(),
            self.HAE(),
            self.HALT(),
            self.GCLC(),
            self.GMRC(),
            self.VER(),
            self.ACTIVE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_CSR_ACTIVE(pub u32);
impl DMA5_MP_CSR_ACTIVE {
    #[doc = "eDMA Version"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "eDMA Version"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for DMA5_MP_CSR_ACTIVE {
    #[inline(always)]
    fn default() -> DMA5_MP_CSR_ACTIVE {
        DMA5_MP_CSR_ACTIVE(0)
    }
}
impl core::fmt::Debug for DMA5_MP_CSR_ACTIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_CSR_ACTIVE")
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_CSR_ACTIVE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_CSR_ACTIVE {{ ID: {=u8:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_ES(pub u32);
impl DMA5_MP_ES {
    #[must_use]
    #[inline(always)]
    pub const fn DBE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SBE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SBE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SGE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NCE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DAE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SOE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SAE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ECX(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UCE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRCHN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ERRCHN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VLD(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_MP_ES {
    #[inline(always)]
    fn default() -> DMA5_MP_ES {
        DMA5_MP_ES(0)
    }
}
impl core::fmt::Debug for DMA5_MP_ES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_ES")
            .field("DBE", &self.DBE())
            .field("SBE", &self.SBE())
            .field("SGE", &self.SGE())
            .field("NCE", &self.NCE())
            .field("DOE", &self.DOE())
            .field("DAE", &self.DAE())
            .field("SOE", &self.SOE())
            .field("SAE", &self.SAE())
            .field("ECX", &self.ECX())
            .field("UCE", &self.UCE())
            .field("ERRCHN", &self.ERRCHN())
            .field("VLD", &self.VLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_ES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_MP_ES {{ DBE: {=bool:?}, SBE: {=bool:?}, SGE: {=bool:?}, NCE: {=bool:?}, DOE: {=bool:?}, DAE: {=bool:?}, SOE: {=bool:?}, SAE: {=bool:?}, ECX: {=bool:?}, UCE: {=bool:?}, ERRCHN: {=u8:?}, VLD: {=bool:?} }}",
            self.DBE(),
            self.SBE(),
            self.SGE(),
            self.NCE(),
            self.DOE(),
            self.DAE(),
            self.SOE(),
            self.SAE(),
            self.ECX(),
            self.UCE(),
            self.ERRCHN(),
            self.VLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_HRS_HIGH(pub u32);
impl DMA5_MP_HRS_HIGH {
    #[doc = "Hardware Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn HRS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Hardware Request Status"]
    #[inline(always)]
    pub const fn set_HRS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_HRS_HIGH {
    #[inline(always)]
    fn default() -> DMA5_MP_HRS_HIGH {
        DMA5_MP_HRS_HIGH(0)
    }
}
impl core::fmt::Debug for DMA5_MP_HRS_HIGH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_HRS_HIGH")
            .field("HRS", &self.HRS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_HRS_HIGH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_HRS_HIGH {{ HRS: {=u32:?} }}", self.HRS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_HRS_LOW(pub u32);
impl DMA5_MP_HRS_LOW {
    #[doc = "Hardware Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn HRS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Hardware Request Status"]
    #[inline(always)]
    pub const fn set_HRS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_HRS_LOW {
    #[inline(always)]
    fn default() -> DMA5_MP_HRS_LOW {
        DMA5_MP_HRS_LOW(0)
    }
}
impl core::fmt::Debug for DMA5_MP_HRS_LOW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_HRS_LOW")
            .field("HRS", &self.HRS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_HRS_LOW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_HRS_LOW {{ HRS: {=u32:?} }}", self.HRS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_INT_HIGH(pub u32);
impl DMA5_MP_INT_HIGH {
    #[doc = "Interrupt Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Request Status"]
    #[inline(always)]
    pub const fn set_INT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_INT_HIGH {
    #[inline(always)]
    fn default() -> DMA5_MP_INT_HIGH {
        DMA5_MP_INT_HIGH(0)
    }
}
impl core::fmt::Debug for DMA5_MP_INT_HIGH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_INT_HIGH")
            .field("INT", &self.INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_INT_HIGH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_INT_HIGH {{ INT: {=u32:?} }}", self.INT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_INT_LOW(pub u32);
impl DMA5_MP_INT_LOW {
    #[doc = "Interrupt Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Request Status"]
    #[inline(always)]
    pub const fn set_INT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_INT_LOW {
    #[inline(always)]
    fn default() -> DMA5_MP_INT_LOW {
        DMA5_MP_INT_LOW(0)
    }
}
impl core::fmt::Debug for DMA5_MP_INT_LOW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_INT_LOW")
            .field("INT", &self.INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_INT_LOW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_INT_LOW {{ INT: {=u32:?} }}", self.INT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_SSR_HIGH(pub u32);
impl DMA5_MP_SSR_HIGH {
    #[doc = "Channel Stop Status"]
    #[must_use]
    #[inline(always)]
    pub const fn CSS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel Stop Status"]
    #[inline(always)]
    pub const fn set_CSS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_SSR_HIGH {
    #[inline(always)]
    fn default() -> DMA5_MP_SSR_HIGH {
        DMA5_MP_SSR_HIGH(0)
    }
}
impl core::fmt::Debug for DMA5_MP_SSR_HIGH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_SSR_HIGH")
            .field("CSS", &self.CSS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_SSR_HIGH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_SSR_HIGH {{ CSS: {=u32:?} }}", self.CSS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_SSR_LOW(pub u32);
impl DMA5_MP_SSR_LOW {
    #[doc = "Channel Stop Status"]
    #[must_use]
    #[inline(always)]
    pub const fn CSS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel Stop Status"]
    #[inline(always)]
    pub const fn set_CSS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_MP_SSR_LOW {
    #[inline(always)]
    fn default() -> DMA5_MP_SSR_LOW {
        DMA5_MP_SSR_LOW(0)
    }
}
impl core::fmt::Debug for DMA5_MP_SSR_LOW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_SSR_LOW")
            .field("CSS", &self.CSS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_SSR_LOW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_MP_SSR_LOW {{ CSS: {=u32:?} }}", self.CSS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_MP_STOPCH(pub u32);
impl DMA5_MP_STOPCH {
    #[must_use]
    #[inline(always)]
    pub const fn STOPCH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_STOPCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for DMA5_MP_STOPCH {
    #[inline(always)]
    fn default() -> DMA5_MP_STOPCH {
        DMA5_MP_STOPCH(0)
    }
}
impl core::fmt::Debug for DMA5_MP_STOPCH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_MP_STOPCH")
            .field("STOPCH", &self.STOPCH())
            .field("ERR", &self.ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_MP_STOPCH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_MP_STOPCH {{ STOPCH: {=u8:?}, ERR: {=bool:?} }}",
            self.STOPCH(),
            self.ERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_NBYTES_MLOFFNO(pub u32);
impl DMA5_NBYTES_MLOFFNO {
    #[must_use]
    #[inline(always)]
    pub const fn NBYTES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_NBYTES(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMLOE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMLOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMLOE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SMLOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_NBYTES_MLOFFNO {
    #[inline(always)]
    fn default() -> DMA5_NBYTES_MLOFFNO {
        DMA5_NBYTES_MLOFFNO(0)
    }
}
impl core::fmt::Debug for DMA5_NBYTES_MLOFFNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_NBYTES_MLOFFNO")
            .field("NBYTES", &self.NBYTES())
            .field("DMLOE", &self.DMLOE())
            .field("SMLOE", &self.SMLOE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_NBYTES_MLOFFNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_NBYTES_MLOFFNO {{ NBYTES: {=u32:?}, DMLOE: {=bool:?}, SMLOE: {=bool:?} }}",
            self.NBYTES(),
            self.DMLOE(),
            self.SMLOE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_NBYTES_MLOFFYES(pub u32);
impl DMA5_NBYTES_MLOFFYES {
    #[must_use]
    #[inline(always)]
    pub const fn NBYTES(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_NBYTES(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Minor Loop Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn MLOFF(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Minor Loop Offset"]
    #[inline(always)]
    pub const fn set_MLOFF(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Minor Loop Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn DMLOE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Minor Loop Offset"]
    #[inline(always)]
    pub const fn set_DMLOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SMLOE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SMLOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA5_NBYTES_MLOFFYES {
    #[inline(always)]
    fn default() -> DMA5_NBYTES_MLOFFYES {
        DMA5_NBYTES_MLOFFYES(0)
    }
}
impl core::fmt::Debug for DMA5_NBYTES_MLOFFYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_NBYTES_MLOFFYES")
            .field("NBYTES", &self.NBYTES())
            .field("MLOFF", &self.MLOFF())
            .field("DMLOE", &self.DMLOE())
            .field("SMLOE", &self.SMLOE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_NBYTES_MLOFFYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_NBYTES_MLOFFYES {{ NBYTES: {=u16:?}, MLOFF: {=u32:?}, DMLOE: {=bool:?}, SMLOE: {=bool:?} }}",
            self.NBYTES(),
            self.MLOFF(),
            self.DMLOE(),
            self.SMLOE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_SADDR(pub u32);
impl DMA5_SADDR {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn SADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_SADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_SADDR {
    #[inline(always)]
    fn default() -> DMA5_SADDR {
        DMA5_SADDR(0)
    }
}
impl core::fmt::Debug for DMA5_SADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_SADDR")
            .field("SADDR", &self.SADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_SADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_SADDR {{ SADDR: {=u32:?} }}", self.SADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_SADDR_HIGH(pub u32);
impl DMA5_SADDR_HIGH {
    #[must_use]
    #[inline(always)]
    pub const fn SADDR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SADDR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_SADDR_HIGH {
    #[inline(always)]
    fn default() -> DMA5_SADDR_HIGH {
        DMA5_SADDR_HIGH(0)
    }
}
impl core::fmt::Debug for DMA5_SADDR_HIGH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_SADDR_HIGH")
            .field("SADDR", &self.SADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_SADDR_HIGH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_SADDR_HIGH {{ SADDR: {=u32:?} }}", self.SADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_SLAST_SDA_HIGH_SLAST(pub u32);
impl DMA5_SLAST_SDA_HIGH_SLAST {
    #[must_use]
    #[inline(always)]
    pub const fn SDA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SDA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_SLAST_SDA_HIGH_SLAST {
    #[inline(always)]
    fn default() -> DMA5_SLAST_SDA_HIGH_SLAST {
        DMA5_SLAST_SDA_HIGH_SLAST(0)
    }
}
impl core::fmt::Debug for DMA5_SLAST_SDA_HIGH_SLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_SLAST_SDA_HIGH_SLAST")
            .field("SDA", &self.SDA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_SLAST_SDA_HIGH_SLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA5_SLAST_SDA_HIGH_SLAST {{ SDA: {=u32:?} }}",
            self.SDA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_SLAST_SDA_SLAST(pub u32);
impl DMA5_SLAST_SDA_SLAST {
    #[must_use]
    #[inline(always)]
    pub const fn SDA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SDA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DMA5_SLAST_SDA_SLAST {
    #[inline(always)]
    fn default() -> DMA5_SLAST_SDA_SLAST {
        DMA5_SLAST_SDA_SLAST(0)
    }
}
impl core::fmt::Debug for DMA5_SLAST_SDA_SLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_SLAST_SDA_SLAST")
            .field("SDA", &self.SDA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_SLAST_SDA_SLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_SLAST_SDA_SLAST {{ SDA: {=u32:?} }}", self.SDA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA5_SOFF(pub u32);
impl DMA5_SOFF {
    #[doc = "Source Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn SOFF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source Address Signed Offset"]
    #[inline(always)]
    pub const fn set_SOFF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DMA5_SOFF {
    #[inline(always)]
    fn default() -> DMA5_SOFF {
        DMA5_SOFF(0)
    }
}
impl core::fmt::Debug for DMA5_SOFF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA5_SOFF")
            .field("SOFF", &self.SOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA5_SOFF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA5_SOFF {{ SOFF: {=u16:?} }}", self.SOFF())
    }
}

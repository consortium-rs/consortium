#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA {
    ptr: *mut u8,
}
unsafe impl Send for DMA {}
unsafe impl Sync for DMA {}
impl DMA {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MP_CSR(self) -> crate::common::Reg<DMA_MP_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_ES(self) -> crate::common::Reg<DMA_MP_ES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_INT(self) -> crate::common::Reg<DMA_MP_INT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MP_HRS(self) -> crate::common::Reg<DMA_MP_HRS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CH_GRPRI(self, n: usize) -> crate::common::Reg<DMA_CH_GRPRI, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TCD_SLAST_SDA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_002cusize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DADDR(self) -> crate::common::Reg<DMA_TCD_DADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0030usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DOFF(self) -> crate::common::Reg<DMA_TCD_DOFF, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0034usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_DLAST_SGA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0038usize) as _) }
    }
    #[inline(always)]
    pub const fn TCD_CSR(self) -> crate::common::Reg<DMA_TCD_CSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_003cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_CSR(pub u32);
impl DMA_CH_CSR {
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
    pub const fn EBW(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EBW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn DONE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_DONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA_CH_CSR {
    #[inline(always)]
    fn default() -> DMA_CH_CSR {
        DMA_CH_CSR(0)
    }
}
impl core::fmt::Debug for DMA_CH_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_CSR")
            .field("ERQ", &self.ERQ())
            .field("EARQ", &self.EARQ())
            .field("EEI", &self.EEI())
            .field("EBW", &self.EBW())
            .field("DONE", &self.DONE())
            .field("ACTIVE", &self.ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_CH_CSR {{ ERQ: {=bool:?}, EARQ: {=bool:?}, EEI: {=bool:?}, EBW: {=bool:?}, DONE: {=bool:?}, ACTIVE: {=bool:?} }}",
            self.ERQ(),
            self.EARQ(),
            self.EEI(),
            self.EBW(),
            self.DONE(),
            self.ACTIVE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_ES(pub u32);
impl DMA_CH_ES {
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
    pub const fn ERR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA_CH_ES {
    #[inline(always)]
    fn default() -> DMA_CH_ES {
        DMA_CH_ES(0)
    }
}
impl core::fmt::Debug for DMA_CH_ES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_ES")
            .field("DBE", &self.DBE())
            .field("SBE", &self.SBE())
            .field("SGE", &self.SGE())
            .field("NCE", &self.NCE())
            .field("DOE", &self.DOE())
            .field("DAE", &self.DAE())
            .field("SOE", &self.SOE())
            .field("SAE", &self.SAE())
            .field("ERR", &self.ERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_ES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_CH_ES {{ DBE: {=bool:?}, SBE: {=bool:?}, SGE: {=bool:?}, NCE: {=bool:?}, DOE: {=bool:?}, DAE: {=bool:?}, SOE: {=bool:?}, SAE: {=bool:?}, ERR: {=bool:?} }}",
            self.DBE(),
            self.SBE(),
            self.SGE(),
            self.NCE(),
            self.DOE(),
            self.DAE(),
            self.SOE(),
            self.SAE(),
            self.ERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_GRPRI(pub u32);
impl DMA_CH_GRPRI {
    #[must_use]
    #[inline(always)]
    pub const fn GRPRI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GRPRI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for DMA_CH_GRPRI {
    #[inline(always)]
    fn default() -> DMA_CH_GRPRI {
        DMA_CH_GRPRI(0)
    }
}
impl core::fmt::Debug for DMA_CH_GRPRI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_GRPRI")
            .field("GRPRI", &self.GRPRI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_GRPRI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_CH_GRPRI {{ GRPRI: {=u8:?} }}", self.GRPRI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_INT(pub u32);
impl DMA_CH_INT {
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
impl Default for DMA_CH_INT {
    #[inline(always)]
    fn default() -> DMA_CH_INT {
        DMA_CH_INT(0)
    }
}
impl core::fmt::Debug for DMA_CH_INT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_INT")
            .field("INT", &self.INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_INT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_CH_INT {{ INT: {=bool:?} }}", self.INT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_PRI(pub u32);
impl DMA_CH_PRI {
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
impl Default for DMA_CH_PRI {
    #[inline(always)]
    fn default() -> DMA_CH_PRI {
        DMA_CH_PRI(0)
    }
}
impl core::fmt::Debug for DMA_CH_PRI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_PRI")
            .field("APL", &self.APL())
            .field("DPA", &self.DPA())
            .field("ECP", &self.ECP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_PRI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_CH_PRI {{ APL: {=u8:?}, DPA: {=bool:?}, ECP: {=bool:?} }}",
            self.APL(),
            self.DPA(),
            self.ECP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH_SBR(pub u32);
impl DMA_CH_SBR {
    #[doc = "Initiator ID"]
    #[must_use]
    #[inline(always)]
    pub const fn MID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Initiator ID"]
    #[inline(always)]
    pub const fn set_MID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Initiator ID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Initiator ID"]
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
    #[doc = "Attribute Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x3f;
        val as u8
    }
    #[doc = "Attribute Output"]
    #[inline(always)]
    pub const fn set_ATTR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
    }
}
impl Default for DMA_CH_SBR {
    #[inline(always)]
    fn default() -> DMA_CH_SBR {
        DMA_CH_SBR(0)
    }
}
impl core::fmt::Debug for DMA_CH_SBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CH_SBR")
            .field("MID", &self.MID())
            .field("SEC", &self.SEC())
            .field("PAL", &self.PAL())
            .field("EMI", &self.EMI())
            .field("ATTR", &self.ATTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_CH_SBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_CH_SBR {{ MID: {=u8:?}, SEC: {=bool:?}, PAL: {=bool:?}, EMI: {=bool:?}, ATTR: {=u8:?} }}",
            self.MID(),
            self.SEC(),
            self.PAL(),
            self.EMI(),
            self.ATTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MP_CSR(pub u32);
impl DMA_MP_CSR {
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
    pub const fn CX(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Active Channel ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Active Channel ID"]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA_MP_CSR {
    #[inline(always)]
    fn default() -> DMA_MP_CSR {
        DMA_MP_CSR(0)
    }
}
impl core::fmt::Debug for DMA_MP_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MP_CSR")
            .field("EDBG", &self.EDBG())
            .field("ERCA", &self.ERCA())
            .field("HAE", &self.HAE())
            .field("HALT", &self.HALT())
            .field("GCLC", &self.GCLC())
            .field("GMRC", &self.GMRC())
            .field("ECX", &self.ECX())
            .field("CX", &self.CX())
            .field("ACTIVE", &self.ACTIVE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MP_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_MP_CSR {{ EDBG: {=bool:?}, ERCA: {=bool:?}, HAE: {=bool:?}, HALT: {=bool:?}, GCLC: {=bool:?}, GMRC: {=bool:?}, ECX: {=bool:?}, CX: {=bool:?}, ACTIVE: {=bool:?} }}",
            self.EDBG(),
            self.ERCA(),
            self.HAE(),
            self.HALT(),
            self.GCLC(),
            self.GMRC(),
            self.ECX(),
            self.CX(),
            self.ACTIVE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MP_CSR_ACTIVE(pub u32);
impl DMA_MP_CSR_ACTIVE {
    #[doc = "Active Channel ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Active Channel ID"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for DMA_MP_CSR_ACTIVE {
    #[inline(always)]
    fn default() -> DMA_MP_CSR_ACTIVE {
        DMA_MP_CSR_ACTIVE(0)
    }
}
impl core::fmt::Debug for DMA_MP_CSR_ACTIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MP_CSR_ACTIVE")
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MP_CSR_ACTIVE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_MP_CSR_ACTIVE {{ ID: {=u8:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MP_ES(pub u32);
impl DMA_MP_ES {
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
    pub const fn ERRCHN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ERRCHN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn VLD(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_VLD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DMA_MP_ES {
    #[inline(always)]
    fn default() -> DMA_MP_ES {
        DMA_MP_ES(0)
    }
}
impl core::fmt::Debug for DMA_MP_ES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MP_ES")
            .field("DBE", &self.DBE())
            .field("SBE", &self.SBE())
            .field("SGE", &self.SGE())
            .field("NCE", &self.NCE())
            .field("DOE", &self.DOE())
            .field("DAE", &self.DAE())
            .field("SOE", &self.SOE())
            .field("SAE", &self.SAE())
            .field("ECX", &self.ECX())
            .field("ERRCHN", &self.ERRCHN())
            .field("VLD", &self.VLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MP_ES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_MP_ES {{ DBE: {=bool:?}, SBE: {=bool:?}, SGE: {=bool:?}, NCE: {=bool:?}, DOE: {=bool:?}, DAE: {=bool:?}, SOE: {=bool:?}, SAE: {=bool:?}, ECX: {=bool:?}, ERRCHN: {=u8:?}, VLD: {=bool:?} }}",
            self.DBE(),
            self.SBE(),
            self.SGE(),
            self.NCE(),
            self.DOE(),
            self.DAE(),
            self.SOE(),
            self.SAE(),
            self.ECX(),
            self.ERRCHN(),
            self.VLD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MP_HRS(pub u32);
impl DMA_MP_HRS {
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
impl Default for DMA_MP_HRS {
    #[inline(always)]
    fn default() -> DMA_MP_HRS {
        DMA_MP_HRS(0)
    }
}
impl core::fmt::Debug for DMA_MP_HRS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MP_HRS")
            .field("HRS", &self.HRS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MP_HRS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_MP_HRS {{ HRS: {=u32:?} }}", self.HRS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MP_INT(pub u32);
impl DMA_MP_INT {
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
impl Default for DMA_MP_INT {
    #[inline(always)]
    fn default() -> DMA_MP_INT {
        DMA_MP_INT(0)
    }
}
impl core::fmt::Debug for DMA_MP_INT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MP_INT")
            .field("INT", &self.INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MP_INT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_MP_INT {{ INT: {=u32:?} }}", self.INT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_ATTR(pub u32);
impl DMA_TCD_ATTR {
    #[doc = "Destination Data Transfer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination Data Transfer Size"]
    #[inline(always)]
    pub const fn set_DSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Destination Data Transfer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn DMOD(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Data Transfer Size"]
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
impl Default for DMA_TCD_ATTR {
    #[inline(always)]
    fn default() -> DMA_TCD_ATTR {
        DMA_TCD_ATTR(0)
    }
}
impl core::fmt::Debug for DMA_TCD_ATTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_ATTR")
            .field("DSIZE", &self.DSIZE())
            .field("DMOD", &self.DMOD())
            .field("SSIZE", &self.SSIZE())
            .field("SMOD", &self.SMOD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_ATTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_ATTR {{ DSIZE: {=u8:?}, DMOD: {=u8:?}, SSIZE: {=u8:?}, SMOD: {=u8:?} }}",
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
pub struct DMA_TCD_BITER_ELINKNO(pub u32);
impl DMA_TCD_BITER_ELINKNO {
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
impl Default for DMA_TCD_BITER_ELINKNO {
    #[inline(always)]
    fn default() -> DMA_TCD_BITER_ELINKNO {
        DMA_TCD_BITER_ELINKNO(0)
    }
}
impl core::fmt::Debug for DMA_TCD_BITER_ELINKNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_BITER_ELINKNO")
            .field("BITER", &self.BITER())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_BITER_ELINKNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_BITER_ELINKNO {{ BITER: {=u16:?}, ELINK: {=bool:?} }}",
            self.BITER(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_BITER_ELINKYES(pub u32);
impl DMA_TCD_BITER_ELINKYES {
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
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn LINKCH(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_LINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn ELINK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_ELINK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_TCD_BITER_ELINKYES {
    #[inline(always)]
    fn default() -> DMA_TCD_BITER_ELINKYES {
        DMA_TCD_BITER_ELINKYES(0)
    }
}
impl core::fmt::Debug for DMA_TCD_BITER_ELINKYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_BITER_ELINKYES")
            .field("BITER", &self.BITER())
            .field("LINKCH", &self.LINKCH())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_BITER_ELINKYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_BITER_ELINKYES {{ BITER: {=u16:?}, LINKCH: {=u8:?}, ELINK: {=bool:?} }}",
            self.BITER(),
            self.LINKCH(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_CITER_ELINKNO(pub u32);
impl DMA_TCD_CITER_ELINKNO {
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
impl Default for DMA_TCD_CITER_ELINKNO {
    #[inline(always)]
    fn default() -> DMA_TCD_CITER_ELINKNO {
        DMA_TCD_CITER_ELINKNO(0)
    }
}
impl core::fmt::Debug for DMA_TCD_CITER_ELINKNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_CITER_ELINKNO")
            .field("CITER", &self.CITER())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_CITER_ELINKNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_CITER_ELINKNO {{ CITER: {=u16:?}, ELINK: {=bool:?} }}",
            self.CITER(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_CITER_ELINKYES(pub u32);
impl DMA_TCD_CITER_ELINKYES {
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
    #[must_use]
    #[inline(always)]
    pub const fn LINKCH(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_LINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
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
impl Default for DMA_TCD_CITER_ELINKYES {
    #[inline(always)]
    fn default() -> DMA_TCD_CITER_ELINKYES {
        DMA_TCD_CITER_ELINKYES(0)
    }
}
impl core::fmt::Debug for DMA_TCD_CITER_ELINKYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_CITER_ELINKYES")
            .field("CITER", &self.CITER())
            .field("LINKCH", &self.LINKCH())
            .field("ELINK", &self.ELINK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_CITER_ELINKYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_CITER_ELINKYES {{ CITER: {=u16:?}, LINKCH: {=u8:?}, ELINK: {=bool:?} }}",
            self.CITER(),
            self.LINKCH(),
            self.ELINK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_CSR(pub u32);
impl DMA_TCD_CSR {
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
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MAJORLINKCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn BWC(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_BWC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for DMA_TCD_CSR {
    #[inline(always)]
    fn default() -> DMA_TCD_CSR {
        DMA_TCD_CSR(0)
    }
}
impl core::fmt::Debug for DMA_TCD_CSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_CSR")
            .field("START", &self.START())
            .field("INTMAJOR", &self.INTMAJOR())
            .field("INTHALF", &self.INTHALF())
            .field("DREQ", &self.DREQ())
            .field("ESG", &self.ESG())
            .field("MAJORELINK", &self.MAJORELINK())
            .field("EEOP", &self.EEOP())
            .field("ESDA", &self.ESDA())
            .field("MAJORLINKCH", &self.MAJORLINKCH())
            .field("BWC", &self.BWC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_CSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_CSR {{ START: {=bool:?}, INTMAJOR: {=bool:?}, INTHALF: {=bool:?}, DREQ: {=bool:?}, ESG: {=bool:?}, MAJORELINK: {=bool:?}, EEOP: {=bool:?}, ESDA: {=bool:?}, MAJORLINKCH: {=u8:?}, BWC: {=u8:?} }}",
            self.START(),
            self.INTMAJOR(),
            self.INTHALF(),
            self.DREQ(),
            self.ESG(),
            self.MAJORELINK(),
            self.EEOP(),
            self.ESDA(),
            self.MAJORLINKCH(),
            self.BWC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_DADDR(pub u32);
impl DMA_TCD_DADDR {
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
impl Default for DMA_TCD_DADDR {
    #[inline(always)]
    fn default() -> DMA_TCD_DADDR {
        DMA_TCD_DADDR(0)
    }
}
impl core::fmt::Debug for DMA_TCD_DADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_DADDR")
            .field("DADDR", &self.DADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_DADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_DADDR {{ DADDR: {=u32:?} }}", self.DADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_DLAST_SGA_DLAST(pub u32);
impl DMA_TCD_DLAST_SGA_DLAST {
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
impl Default for DMA_TCD_DLAST_SGA_DLAST {
    #[inline(always)]
    fn default() -> DMA_TCD_DLAST_SGA_DLAST {
        DMA_TCD_DLAST_SGA_DLAST(0)
    }
}
impl core::fmt::Debug for DMA_TCD_DLAST_SGA_DLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_DLAST_SGA_DLAST")
            .field("SGA", &self.SGA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_DLAST_SGA_DLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_DLAST_SGA_DLAST {{ SGA: {=u32:?} }}", self.SGA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_DOFF(pub u32);
impl DMA_TCD_DOFF {
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
impl Default for DMA_TCD_DOFF {
    #[inline(always)]
    fn default() -> DMA_TCD_DOFF {
        DMA_TCD_DOFF(0)
    }
}
impl core::fmt::Debug for DMA_TCD_DOFF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_DOFF")
            .field("DOFF", &self.DOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_DOFF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_DOFF {{ DOFF: {=u16:?} }}", self.DOFF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_NBYTES_MLOFFNO(pub u32);
impl DMA_TCD_NBYTES_MLOFFNO {
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
impl Default for DMA_TCD_NBYTES_MLOFFNO {
    #[inline(always)]
    fn default() -> DMA_TCD_NBYTES_MLOFFNO {
        DMA_TCD_NBYTES_MLOFFNO(0)
    }
}
impl core::fmt::Debug for DMA_TCD_NBYTES_MLOFFNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_NBYTES_MLOFFNO")
            .field("NBYTES", &self.NBYTES())
            .field("DMLOE", &self.DMLOE())
            .field("SMLOE", &self.SMLOE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_NBYTES_MLOFFNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_NBYTES_MLOFFNO {{ NBYTES: {=u32:?}, DMLOE: {=bool:?}, SMLOE: {=bool:?} }}",
            self.NBYTES(),
            self.DMLOE(),
            self.SMLOE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_NBYTES_MLOFFYES(pub u32);
impl DMA_TCD_NBYTES_MLOFFYES {
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
impl Default for DMA_TCD_NBYTES_MLOFFYES {
    #[inline(always)]
    fn default() -> DMA_TCD_NBYTES_MLOFFYES {
        DMA_TCD_NBYTES_MLOFFYES(0)
    }
}
impl core::fmt::Debug for DMA_TCD_NBYTES_MLOFFYES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_NBYTES_MLOFFYES")
            .field("NBYTES", &self.NBYTES())
            .field("MLOFF", &self.MLOFF())
            .field("DMLOE", &self.DMLOE())
            .field("SMLOE", &self.SMLOE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_NBYTES_MLOFFYES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_TCD_NBYTES_MLOFFYES {{ NBYTES: {=u16:?}, MLOFF: {=u32:?}, DMLOE: {=bool:?}, SMLOE: {=bool:?} }}",
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
pub struct DMA_TCD_SADDR(pub u32);
impl DMA_TCD_SADDR {
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
impl Default for DMA_TCD_SADDR {
    #[inline(always)]
    fn default() -> DMA_TCD_SADDR {
        DMA_TCD_SADDR(0)
    }
}
impl core::fmt::Debug for DMA_TCD_SADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_SADDR")
            .field("SADDR", &self.SADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_SADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_SADDR {{ SADDR: {=u32:?} }}", self.SADDR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_SLAST_SDA_SLAST(pub u32);
impl DMA_TCD_SLAST_SDA_SLAST {
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
impl Default for DMA_TCD_SLAST_SDA_SLAST {
    #[inline(always)]
    fn default() -> DMA_TCD_SLAST_SDA_SLAST {
        DMA_TCD_SLAST_SDA_SLAST(0)
    }
}
impl core::fmt::Debug for DMA_TCD_SLAST_SDA_SLAST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_SLAST_SDA_SLAST")
            .field("SDA", &self.SDA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_SLAST_SDA_SLAST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_SLAST_SDA_SLAST {{ SDA: {=u32:?} }}", self.SDA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_TCD_SOFF(pub u32);
impl DMA_TCD_SOFF {
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
impl Default for DMA_TCD_SOFF {
    #[inline(always)]
    fn default() -> DMA_TCD_SOFF {
        DMA_TCD_SOFF(0)
    }
}
impl core::fmt::Debug for DMA_TCD_SOFF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_TCD_SOFF")
            .field("SOFF", &self.SOFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_TCD_SOFF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA_TCD_SOFF {{ SOFF: {=u16:?} }}", self.SOFF())
    }
}

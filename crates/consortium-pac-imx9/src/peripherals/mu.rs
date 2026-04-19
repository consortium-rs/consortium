#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU {
    ptr: *mut u8,
}
unsafe impl Send for MU {}
unsafe impl Sync for MU {}
impl MU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VER(self) -> crate::common::Reg<MU_VER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PAR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<MU_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<MU_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CCR0(self) -> crate::common::Reg<MU_CCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CIER0(self) -> crate::common::Reg<MU_CIER0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CSSR0(self) -> crate::common::Reg<MU_CSSR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn CSR0(self) -> crate::common::Reg<MU_CSR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn FCR(self) -> crate::common::Reg<MU_FCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn FSR(self) -> crate::common::Reg<MU_FSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn GIER(self) -> crate::common::Reg<MU_GIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn GCR(self) -> crate::common::Reg<MU_GCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn GSR(self) -> crate::common::Reg<MU_GSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn TCR(self) -> crate::common::Reg<MU_TCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn TSR(self) -> crate::common::Reg<MU_TSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn RCR(self) -> crate::common::Reg<MU_RCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn RSR(self) -> crate::common::Reg<MU_RSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn TR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn RR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_CCR0(pub u32);
impl MU_CCR0 {
    #[must_use]
    #[inline(always)]
    pub const fn NMI(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NMI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HRM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSTH(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSTH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BOOT(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BOOT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for MU_CCR0 {
    #[inline(always)]
    fn default() -> MU_CCR0 {
        MU_CCR0(0)
    }
}
impl core::fmt::Debug for MU_CCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_CCR0")
            .field("NMI", &self.NMI())
            .field("HR", &self.HR())
            .field("HRM", &self.HRM())
            .field("RSTH", &self.RSTH())
            .field("BOOT", &self.BOOT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_CCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_CCR0 {{ NMI: {=bool:?}, HR: {=bool:?}, HRM: {=bool:?}, RSTH: {=bool:?}, BOOT: {=u8:?} }}",
            self.NMI(),
            self.HR(),
            self.HRM(),
            self.RSTH(),
            self.BOOT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_CIER0(pub u32);
impl MU_CIER0 {
    #[must_use]
    #[inline(always)]
    pub const fn HRIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RUNIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RUNIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RAIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RAIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HALTIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HALTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAITIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAITIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOPIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for MU_CIER0 {
    #[inline(always)]
    fn default() -> MU_CIER0 {
        MU_CIER0(0)
    }
}
impl core::fmt::Debug for MU_CIER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_CIER0")
            .field("HRIE", &self.HRIE())
            .field("RUNIE", &self.RUNIE())
            .field("RAIE", &self.RAIE())
            .field("HALTIE", &self.HALTIE())
            .field("WAITIE", &self.WAITIE())
            .field("STOPIE", &self.STOPIE())
            .field("PDIE", &self.PDIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_CIER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_CIER0 {{ HRIE: {=bool:?}, RUNIE: {=bool:?}, RAIE: {=bool:?}, HALTIE: {=bool:?}, WAITIE: {=bool:?}, STOPIE: {=bool:?}, PDIE: {=bool:?} }}",
            self.HRIE(),
            self.RUNIE(),
            self.RAIE(),
            self.HALTIE(),
            self.WAITIE(),
            self.STOPIE(),
            self.PDIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_CR(pub u32);
impl MU_CR {
    #[must_use]
    #[inline(always)]
    pub const fn MUR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MUR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MURIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MURIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MU_CR {
    #[inline(always)]
    fn default() -> MU_CR {
        MU_CR(0)
    }
}
impl core::fmt::Debug for MU_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_CR")
            .field("MUR", &self.MUR())
            .field("MURIE", &self.MURIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_CR {{ MUR: {=bool:?}, MURIE: {=bool:?} }}",
            self.MUR(),
            self.MURIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_CSR0(pub u32);
impl MU_CSR0 {
    #[must_use]
    #[inline(always)]
    pub const fn HRIP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRIP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RUN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RAIP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RAIP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAIT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for MU_CSR0 {
    #[inline(always)]
    fn default() -> MU_CSR0 {
        MU_CSR0(0)
    }
}
impl core::fmt::Debug for MU_CSR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_CSR0")
            .field("HRIP", &self.HRIP())
            .field("RUN", &self.RUN())
            .field("RAIP", &self.RAIP())
            .field("HALT", &self.HALT())
            .field("WAIT", &self.WAIT())
            .field("STOP", &self.STOP())
            .field("PD", &self.PD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_CSR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_CSR0 {{ HRIP: {=bool:?}, RUN: {=bool:?}, RAIP: {=bool:?}, HALT: {=bool:?}, WAIT: {=bool:?}, STOP: {=bool:?}, PD: {=bool:?} }}",
            self.HRIP(),
            self.RUN(),
            self.RAIP(),
            self.HALT(),
            self.WAIT(),
            self.STOP(),
            self.PD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_CSSR0(pub u32);
impl MU_CSSR0 {
    #[must_use]
    #[inline(always)]
    pub const fn NMIC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NMIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HRIP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRIP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RUN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RAIP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RAIP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HALT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HALT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WAIT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for MU_CSSR0 {
    #[inline(always)]
    fn default() -> MU_CSSR0 {
        MU_CSSR0(0)
    }
}
impl core::fmt::Debug for MU_CSSR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_CSSR0")
            .field("NMIC", &self.NMIC())
            .field("HRIP", &self.HRIP())
            .field("RUN", &self.RUN())
            .field("RAIP", &self.RAIP())
            .field("HALT", &self.HALT())
            .field("WAIT", &self.WAIT())
            .field("STOP", &self.STOP())
            .field("PD", &self.PD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_CSSR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_CSSR0 {{ NMIC: {=bool:?}, HRIP: {=bool:?}, RUN: {=bool:?}, RAIP: {=bool:?}, HALT: {=bool:?}, WAIT: {=bool:?}, STOP: {=bool:?}, PD: {=bool:?} }}",
            self.NMIC(),
            self.HRIP(),
            self.RUN(),
            self.RAIP(),
            self.HALT(),
            self.WAIT(),
            self.STOP(),
            self.PD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_FCR(pub u32);
impl MU_FCR {
    #[must_use]
    #[inline(always)]
    pub const fn F0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn F1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn F2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for MU_FCR {
    #[inline(always)]
    fn default() -> MU_FCR {
        MU_FCR(0)
    }
}
impl core::fmt::Debug for MU_FCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_FCR")
            .field("F0", &self.F0())
            .field("F1", &self.F1())
            .field("F2", &self.F2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_FCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_FCR {{ F0: {=bool:?}, F1: {=bool:?}, F2: {=bool:?} }}",
            self.F0(),
            self.F1(),
            self.F2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_FSR(pub u32);
impl MU_FSR {
    #[must_use]
    #[inline(always)]
    pub const fn F0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn F1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn F2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_F2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for MU_FSR {
    #[inline(always)]
    fn default() -> MU_FSR {
        MU_FSR(0)
    }
}
impl core::fmt::Debug for MU_FSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_FSR")
            .field("F0", &self.F0())
            .field("F1", &self.F1())
            .field("F2", &self.F2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_FSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_FSR {{ F0: {=bool:?}, F1: {=bool:?}, F2: {=bool:?} }}",
            self.F0(),
            self.F1(),
            self.F2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_GCR(pub u32);
impl MU_GCR {
    #[must_use]
    #[inline(always)]
    pub const fn GIR0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIR1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIR2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIR3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIR3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_GCR {
    #[inline(always)]
    fn default() -> MU_GCR {
        MU_GCR(0)
    }
}
impl core::fmt::Debug for MU_GCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_GCR")
            .field("GIR0", &self.GIR0())
            .field("GIR1", &self.GIR1())
            .field("GIR2", &self.GIR2())
            .field("GIR3", &self.GIR3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_GCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_GCR {{ GIR0: {=bool:?}, GIR1: {=bool:?}, GIR2: {=bool:?}, GIR3: {=bool:?} }}",
            self.GIR0(),
            self.GIR1(),
            self.GIR2(),
            self.GIR3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_GIER(pub u32);
impl MU_GIER {
    #[must_use]
    #[inline(always)]
    pub const fn GIE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_GIER {
    #[inline(always)]
    fn default() -> MU_GIER {
        MU_GIER(0)
    }
}
impl core::fmt::Debug for MU_GIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_GIER")
            .field("GIE0", &self.GIE0())
            .field("GIE1", &self.GIE1())
            .field("GIE2", &self.GIE2())
            .field("GIE3", &self.GIE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_GIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_GIER {{ GIE0: {=bool:?}, GIE1: {=bool:?}, GIE2: {=bool:?}, GIE3: {=bool:?} }}",
            self.GIE0(),
            self.GIE1(),
            self.GIE2(),
            self.GIE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_GSR(pub u32);
impl MU_GSR {
    #[must_use]
    #[inline(always)]
    pub const fn GIP0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIP1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIP2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIP2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIP3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIP3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_GSR {
    #[inline(always)]
    fn default() -> MU_GSR {
        MU_GSR(0)
    }
}
impl core::fmt::Debug for MU_GSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_GSR")
            .field("GIP0", &self.GIP0())
            .field("GIP1", &self.GIP1())
            .field("GIP2", &self.GIP2())
            .field("GIP3", &self.GIP3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_GSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_GSR {{ GIP0: {=bool:?}, GIP1: {=bool:?}, GIP2: {=bool:?}, GIP3: {=bool:?} }}",
            self.GIP0(),
            self.GIP1(),
            self.GIP2(),
            self.GIP3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_PAR_FLAG(pub u32);
impl MU_PAR_FLAG {
    #[doc = "General-Purpose Interrupt Request Number"]
    #[must_use]
    #[inline(always)]
    pub const fn WIDTH(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "General-Purpose Interrupt Request Number"]
    #[inline(always)]
    pub const fn set_WIDTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for MU_PAR_FLAG {
    #[inline(always)]
    fn default() -> MU_PAR_FLAG {
        MU_PAR_FLAG(0)
    }
}
impl core::fmt::Debug for MU_PAR_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_PAR_FLAG")
            .field("WIDTH", &self.WIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_PAR_FLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_PAR_FLAG {{ WIDTH: {=u8:?} }}", self.WIDTH())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_PAR_GIR(pub u32);
impl MU_PAR_GIR {
    #[doc = "Receive Register Number"]
    #[must_use]
    #[inline(always)]
    pub const fn NUM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Register Number"]
    #[inline(always)]
    pub const fn set_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for MU_PAR_GIR {
    #[inline(always)]
    fn default() -> MU_PAR_GIR {
        MU_PAR_GIR(0)
    }
}
impl core::fmt::Debug for MU_PAR_GIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_PAR_GIR")
            .field("NUM", &self.NUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_PAR_GIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_PAR_GIR {{ NUM: {=u8:?} }}", self.NUM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_PAR_RR(pub u32);
impl MU_PAR_RR {
    #[doc = "Transmit Register Number"]
    #[must_use]
    #[inline(always)]
    pub const fn NUM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Register Number"]
    #[inline(always)]
    pub const fn set_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for MU_PAR_RR {
    #[inline(always)]
    fn default() -> MU_PAR_RR {
        MU_PAR_RR(0)
    }
}
impl core::fmt::Debug for MU_PAR_RR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_PAR_RR")
            .field("NUM", &self.NUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_PAR_RR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_PAR_RR {{ NUM: {=u8:?} }}", self.NUM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_PAR_TR(pub u32);
impl MU_PAR_TR {
    #[doc = "Transmit Register Number"]
    #[must_use]
    #[inline(always)]
    pub const fn NUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Register Number"]
    #[inline(always)]
    pub const fn set_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for MU_PAR_TR {
    #[inline(always)]
    fn default() -> MU_PAR_TR {
        MU_PAR_TR(0)
    }
}
impl core::fmt::Debug for MU_PAR_TR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_PAR_TR")
            .field("NUM", &self.NUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_PAR_TR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_PAR_TR {{ NUM: {=u8:?} }}", self.NUM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_RCR(pub u32);
impl MU_RCR {
    #[must_use]
    #[inline(always)]
    pub const fn RIE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RIE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RIE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_RCR {
    #[inline(always)]
    fn default() -> MU_RCR {
        MU_RCR(0)
    }
}
impl core::fmt::Debug for MU_RCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_RCR")
            .field("RIE0", &self.RIE0())
            .field("RIE1", &self.RIE1())
            .field("RIE2", &self.RIE2())
            .field("RIE3", &self.RIE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_RCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_RCR {{ RIE0: {=bool:?}, RIE1: {=bool:?}, RIE2: {=bool:?}, RIE3: {=bool:?} }}",
            self.RIE0(),
            self.RIE1(),
            self.RIE2(),
            self.RIE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_RR_RR(pub u32);
impl MU_RR_RR {
    #[doc = "MUB Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MUB Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MU_RR_RR {
    #[inline(always)]
    fn default() -> MU_RR_RR {
        MU_RR_RR(0)
    }
}
impl core::fmt::Debug for MU_RR_RR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_RR_RR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_RR_RR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_RR_RR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_RSR(pub u32);
impl MU_RSR {
    #[must_use]
    #[inline(always)]
    pub const fn RF0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RF0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RF1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RF1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RF2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RF2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RF3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RF3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_RSR {
    #[inline(always)]
    fn default() -> MU_RSR {
        MU_RSR(0)
    }
}
impl core::fmt::Debug for MU_RSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_RSR")
            .field("RF0", &self.RF0())
            .field("RF1", &self.RF1())
            .field("RF2", &self.RF2())
            .field("RF3", &self.RF3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_RSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_RSR {{ RF0: {=bool:?}, RF1: {=bool:?}, RF2: {=bool:?}, RF3: {=bool:?} }}",
            self.RF0(),
            self.RF1(),
            self.RF2(),
            self.RF3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_SR(pub u32);
impl MU_SR {
    #[must_use]
    #[inline(always)]
    pub const fn MURS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MURS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MURIP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MURIP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FUP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIRP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIRP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TEP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RFP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RFP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CEP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for MU_SR {
    #[inline(always)]
    fn default() -> MU_SR {
        MU_SR(0)
    }
}
impl core::fmt::Debug for MU_SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_SR")
            .field("MURS", &self.MURS())
            .field("MURIP", &self.MURIP())
            .field("EP", &self.EP())
            .field("FUP", &self.FUP())
            .field("GIRP", &self.GIRP())
            .field("TEP", &self.TEP())
            .field("RFP", &self.RFP())
            .field("CEP", &self.CEP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_SR {{ MURS: {=bool:?}, MURIP: {=bool:?}, EP: {=bool:?}, FUP: {=bool:?}, GIRP: {=bool:?}, TEP: {=bool:?}, RFP: {=bool:?}, CEP: {=bool:?} }}",
            self.MURS(),
            self.MURIP(),
            self.EP(),
            self.FUP(),
            self.GIRP(),
            self.TEP(),
            self.RFP(),
            self.CEP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_TCR(pub u32);
impl MU_TCR {
    #[must_use]
    #[inline(always)]
    pub const fn TIE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_TCR {
    #[inline(always)]
    fn default() -> MU_TCR {
        MU_TCR(0)
    }
}
impl core::fmt::Debug for MU_TCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_TCR")
            .field("TIE0", &self.TIE0())
            .field("TIE1", &self.TIE1())
            .field("TIE2", &self.TIE2())
            .field("TIE3", &self.TIE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_TCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_TCR {{ TIE0: {=bool:?}, TIE1: {=bool:?}, TIE2: {=bool:?}, TIE3: {=bool:?} }}",
            self.TIE0(),
            self.TIE1(),
            self.TIE2(),
            self.TIE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_TR_TR(pub u32);
impl MU_TR_TR {
    #[doc = "MUB Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MUB Transmit Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MU_TR_TR {
    #[inline(always)]
    fn default() -> MU_TR_TR {
        MU_TR_TR(0)
    }
}
impl core::fmt::Debug for MU_TR_TR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_TR_TR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_TR_TR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MU_TR_TR {{ DATA: {=u32:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_TSR(pub u32);
impl MU_TSR {
    #[must_use]
    #[inline(always)]
    pub const fn TE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MU_TSR {
    #[inline(always)]
    fn default() -> MU_TSR {
        MU_TSR(0)
    }
}
impl core::fmt::Debug for MU_TSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_TSR")
            .field("TE0", &self.TE0())
            .field("TE1", &self.TE1())
            .field("TE2", &self.TE2())
            .field("TE3", &self.TE3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_TSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_TSR {{ TE0: {=bool:?}, TE1: {=bool:?}, TE2: {=bool:?}, TE3: {=bool:?} }}",
            self.TE0(),
            self.TE1(),
            self.TE2(),
            self.TE3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MU_VER(pub u32);
impl MU_VER {
    #[doc = "Feature Set Number"]
    #[must_use]
    #[inline(always)]
    pub const fn FEATURE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Set Number"]
    #[inline(always)]
    pub const fn set_FEATURE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feature Set Number"]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Feature Set Number"]
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
impl Default for MU_VER {
    #[inline(always)]
    fn default() -> MU_VER {
        MU_VER(0)
    }
}
impl core::fmt::Debug for MU_VER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MU_VER")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MU_VER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MU_VER {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
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

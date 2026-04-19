#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C {
    ptr: *mut u8,
}
unsafe impl Send for LPI2C {}
unsafe impl Sync for LPI2C {}
impl LPI2C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<LPI2C_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<LPI2C_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<LPI2C_MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<LPI2C_MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MIER(self) -> crate::common::Reg<LPI2C_MIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn MDER(self) -> crate::common::Reg<LPI2C_MDER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR0(self) -> crate::common::Reg<LPI2C_MCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR1(self) -> crate::common::Reg<LPI2C_MCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR2(self) -> crate::common::Reg<LPI2C_MCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn MCFGR3(self) -> crate::common::Reg<LPI2C_MCFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn MDMR(self) -> crate::common::Reg<LPI2C_MDMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn MCCR0(self) -> crate::common::Reg<LPI2C_MCCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn MCCR1(self) -> crate::common::Reg<LPI2C_MCCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn MFCR(self) -> crate::common::Reg<LPI2C_MFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn MFSR(self) -> crate::common::Reg<LPI2C_MFSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn MTDR(self) -> crate::common::Reg<LPI2C_MTDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDR(self) -> crate::common::Reg<LPI2C_MRDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDROR(self) -> crate::common::Reg<LPI2C_MRDROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn SCR(self) -> crate::common::Reg<LPI2C_SCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn SSR(self) -> crate::common::Reg<LPI2C_SSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn SIER(self) -> crate::common::Reg<LPI2C_SIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn SDER(self) -> crate::common::Reg<LPI2C_SDER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR0(self) -> crate::common::Reg<LPI2C_SCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR1(self) -> crate::common::Reg<LPI2C_SCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn SCFGR2(self) -> crate::common::Reg<LPI2C_SCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn SAMR(self) -> crate::common::Reg<LPI2C_SAMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn SASR(self) -> crate::common::Reg<LPI2C_SASR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn STAR(self) -> crate::common::Reg<LPI2C_STAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn STDR(self) -> crate::common::Reg<LPI2C_STDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDR(self) -> crate::common::Reg<LPI2C_SRDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDROR(self) -> crate::common::Reg<LPI2C_SRDROR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn MTCBR(self, n: usize) -> crate::common::Reg<LPI2C_MTCBR, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn MTDBR(self, n: usize) -> crate::common::Reg<LPI2C_MTDBR, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCCR0(pub u32);
impl LPI2C_MCCR0 {
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKLO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_CLKLO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKHI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_CLKHI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Clock High Period"]
    #[must_use]
    #[inline(always)]
    pub const fn SETHOLD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_SETHOLD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Setup Hold Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_DATAVD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for LPI2C_MCCR0 {
    #[inline(always)]
    fn default() -> LPI2C_MCCR0 {
        LPI2C_MCCR0(0)
    }
}
impl core::fmt::Debug for LPI2C_MCCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCCR0")
            .field("CLKLO", &self.CLKLO())
            .field("CLKHI", &self.CLKHI())
            .field("SETHOLD", &self.SETHOLD())
            .field("DATAVD", &self.DATAVD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCCR0 {{ CLKLO: {=u8:?}, CLKHI: {=u8:?}, SETHOLD: {=u8:?}, DATAVD: {=u8:?} }}",
            self.CLKLO(),
            self.CLKHI(),
            self.SETHOLD(),
            self.DATAVD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCCR1(pub u32);
impl LPI2C_MCCR1 {
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKLO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_CLKLO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Clock Low Period"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKHI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Low Period"]
    #[inline(always)]
    pub const fn set_CLKHI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Clock High Period"]
    #[must_use]
    #[inline(always)]
    pub const fn SETHOLD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock High Period"]
    #[inline(always)]
    pub const fn set_SETHOLD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Setup Hold Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Setup Hold Delay"]
    #[inline(always)]
    pub const fn set_DATAVD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for LPI2C_MCCR1 {
    #[inline(always)]
    fn default() -> LPI2C_MCCR1 {
        LPI2C_MCCR1(0)
    }
}
impl core::fmt::Debug for LPI2C_MCCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCCR1")
            .field("CLKLO", &self.CLKLO())
            .field("CLKHI", &self.CLKHI())
            .field("SETHOLD", &self.SETHOLD())
            .field("DATAVD", &self.DATAVD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCCR1 {{ CLKLO: {=u8:?}, CLKHI: {=u8:?}, SETHOLD: {=u8:?}, DATAVD: {=u8:?} }}",
            self.CLKLO(),
            self.CLKHI(),
            self.SETHOLD(),
            self.DATAVD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCFGR0(pub u32);
impl LPI2C_MCFGR0 {
    #[must_use]
    #[inline(always)]
    pub const fn HREN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HRPOL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HRSEL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HRSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
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
    #[must_use]
    #[inline(always)]
    pub const fn RELAX(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RELAX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ABORT(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ABORT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for LPI2C_MCFGR0 {
    #[inline(always)]
    fn default() -> LPI2C_MCFGR0 {
        LPI2C_MCFGR0(0)
    }
}
impl core::fmt::Debug for LPI2C_MCFGR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCFGR0")
            .field("HREN", &self.HREN())
            .field("HRPOL", &self.HRPOL())
            .field("HRSEL", &self.HRSEL())
            .field("CIRFIFO", &self.CIRFIFO())
            .field("RDMO", &self.RDMO())
            .field("RELAX", &self.RELAX())
            .field("ABORT", &self.ABORT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCFGR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCFGR0 {{ HREN: {=bool:?}, HRPOL: {=bool:?}, HRSEL: {=bool:?}, CIRFIFO: {=bool:?}, RDMO: {=bool:?}, RELAX: {=bool:?}, ABORT: {=bool:?} }}",
            self.HREN(),
            self.HRPOL(),
            self.HRSEL(),
            self.CIRFIFO(),
            self.RDMO(),
            self.RELAX(),
            self.ABORT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCFGR1(pub u32);
impl LPI2C_MCFGR1 {
    #[must_use]
    #[inline(always)]
    pub const fn PRESCALE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PRESCALE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AUTOSTOP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AUTOSTOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IGNACK(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IGNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMECFG(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIMECFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOPCFG(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOPCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STARTCFG(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STARTCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PINCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for LPI2C_MCFGR1 {
    #[inline(always)]
    fn default() -> LPI2C_MCFGR1 {
        LPI2C_MCFGR1(0)
    }
}
impl core::fmt::Debug for LPI2C_MCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCFGR1")
            .field("PRESCALE", &self.PRESCALE())
            .field("AUTOSTOP", &self.AUTOSTOP())
            .field("IGNACK", &self.IGNACK())
            .field("TIMECFG", &self.TIMECFG())
            .field("STOPCFG", &self.STOPCFG())
            .field("STARTCFG", &self.STARTCFG())
            .field("MATCFG", &self.MATCFG())
            .field("PINCFG", &self.PINCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCFGR1 {{ PRESCALE: {=u8:?}, AUTOSTOP: {=bool:?}, IGNACK: {=bool:?}, TIMECFG: {=bool:?}, STOPCFG: {=bool:?}, STARTCFG: {=bool:?}, MATCFG: {=u8:?}, PINCFG: {=u8:?} }}",
            self.PRESCALE(),
            self.AUTOSTOP(),
            self.IGNACK(),
            self.TIMECFG(),
            self.STOPCFG(),
            self.STARTCFG(),
            self.MATCFG(),
            self.PINCFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCFGR2(pub u32);
impl LPI2C_MCFGR2 {
    #[doc = "Bus Idle Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn BUSIDLE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bus Idle Timeout"]
    #[inline(always)]
    pub const fn set_BUSIDLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Bus Idle Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn FILTSCL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Bus Idle Timeout"]
    #[inline(always)]
    pub const fn set_FILTSCL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[must_use]
    #[inline(always)]
    pub const fn FILTSDA(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_FILTSDA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for LPI2C_MCFGR2 {
    #[inline(always)]
    fn default() -> LPI2C_MCFGR2 {
        LPI2C_MCFGR2(0)
    }
}
impl core::fmt::Debug for LPI2C_MCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCFGR2")
            .field("BUSIDLE", &self.BUSIDLE())
            .field("FILTSCL", &self.FILTSCL())
            .field("FILTSDA", &self.FILTSDA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCFGR2 {{ BUSIDLE: {=u16:?}, FILTSCL: {=u8:?}, FILTSDA: {=u8:?} }}",
            self.BUSIDLE(),
            self.FILTSCL(),
            self.FILTSDA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCFGR3(pub u32);
impl LPI2C_MCFGR3 {
    #[doc = "Pin Low Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn PINLOW(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pin Low Timeout"]
    #[inline(always)]
    pub const fn set_PINLOW(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for LPI2C_MCFGR3 {
    #[inline(always)]
    fn default() -> LPI2C_MCFGR3 {
        LPI2C_MCFGR3(0)
    }
}
impl core::fmt::Debug for LPI2C_MCFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCFGR3")
            .field("PINLOW", &self.PINLOW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MCFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPI2C_MCFGR3 {{ PINLOW: {=u16:?} }}", self.PINLOW())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MCR(pub u32);
impl LPI2C_MCR {
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
impl Default for LPI2C_MCR {
    #[inline(always)]
    fn default() -> LPI2C_MCR {
        LPI2C_MCR(0)
    }
}
impl core::fmt::Debug for LPI2C_MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MCR")
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
impl defmt::Format for LPI2C_MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MCR {{ MEN: {=bool:?}, RST: {=bool:?}, DOZEN: {=bool:?}, DBGEN: {=bool:?}, RTF: {=bool:?}, RRF: {=bool:?} }}",
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
pub struct LPI2C_MDER(pub u32);
impl LPI2C_MDER {
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
}
impl Default for LPI2C_MDER {
    #[inline(always)]
    fn default() -> LPI2C_MDER {
        LPI2C_MDER(0)
    }
}
impl core::fmt::Debug for LPI2C_MDER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MDER")
            .field("TDDE", &self.TDDE())
            .field("RDDE", &self.RDDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MDER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MDER {{ TDDE: {=bool:?}, RDDE: {=bool:?} }}",
            self.TDDE(),
            self.RDDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MDMR(pub u32);
impl LPI2C_MDMR {
    #[doc = "Match 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCH0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_MATCH0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Match 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCH1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_MATCH1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for LPI2C_MDMR {
    #[inline(always)]
    fn default() -> LPI2C_MDMR {
        LPI2C_MDMR(0)
    }
}
impl core::fmt::Debug for LPI2C_MDMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MDMR")
            .field("MATCH0", &self.MATCH0())
            .field("MATCH1", &self.MATCH1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MDMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MDMR {{ MATCH0: {=u8:?}, MATCH1: {=u8:?} }}",
            self.MATCH0(),
            self.MATCH1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MFCR(pub u32);
impl LPI2C_MFCR {
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn TXWATER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_TXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn RXWATER(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_RXWATER(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for LPI2C_MFCR {
    #[inline(always)]
    fn default() -> LPI2C_MFCR {
        LPI2C_MFCR(0)
    }
}
impl core::fmt::Debug for LPI2C_MFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MFCR")
            .field("TXWATER", &self.TXWATER())
            .field("RXWATER", &self.RXWATER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MFCR {{ TXWATER: {=u8:?}, RXWATER: {=u8:?} }}",
            self.TXWATER(),
            self.RXWATER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MFSR(pub u32);
impl LPI2C_MFSR {
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCOUNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_TXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCOUNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_RXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for LPI2C_MFSR {
    #[inline(always)]
    fn default() -> LPI2C_MFSR {
        LPI2C_MFSR(0)
    }
}
impl core::fmt::Debug for LPI2C_MFSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MFSR")
            .field("TXCOUNT", &self.TXCOUNT())
            .field("RXCOUNT", &self.RXCOUNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MFSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MFSR {{ TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?} }}",
            self.TXCOUNT(),
            self.RXCOUNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MIER(pub u32);
impl LPI2C_MIER {
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
    pub const fn EPIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NDIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ALIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ALIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FEIE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PLTIE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PLTIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMIE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for LPI2C_MIER {
    #[inline(always)]
    fn default() -> LPI2C_MIER {
        LPI2C_MIER(0)
    }
}
impl core::fmt::Debug for LPI2C_MIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MIER")
            .field("TDIE", &self.TDIE())
            .field("RDIE", &self.RDIE())
            .field("EPIE", &self.EPIE())
            .field("SDIE", &self.SDIE())
            .field("NDIE", &self.NDIE())
            .field("ALIE", &self.ALIE())
            .field("FEIE", &self.FEIE())
            .field("PLTIE", &self.PLTIE())
            .field("DMIE", &self.DMIE())
            .field("STIE", &self.STIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MIER {{ TDIE: {=bool:?}, RDIE: {=bool:?}, EPIE: {=bool:?}, SDIE: {=bool:?}, NDIE: {=bool:?}, ALIE: {=bool:?}, FEIE: {=bool:?}, PLTIE: {=bool:?}, DMIE: {=bool:?}, STIE: {=bool:?} }}",
            self.TDIE(),
            self.RDIE(),
            self.EPIE(),
            self.SDIE(),
            self.NDIE(),
            self.ALIE(),
            self.FEIE(),
            self.PLTIE(),
            self.DMIE(),
            self.STIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MRDR(pub u32);
impl LPI2C_MRDR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for LPI2C_MRDR {
    #[inline(always)]
    fn default() -> LPI2C_MRDR {
        LPI2C_MRDR(0)
    }
}
impl core::fmt::Debug for LPI2C_MRDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MRDR")
            .field("DATA", &self.DATA())
            .field("RXEMPTY", &self.RXEMPTY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MRDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MRDR {{ DATA: {=u8:?}, RXEMPTY: {=bool:?} }}",
            self.DATA(),
            self.RXEMPTY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MRDROR(pub u32);
impl LPI2C_MRDROR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for LPI2C_MRDROR {
    #[inline(always)]
    fn default() -> LPI2C_MRDROR {
        LPI2C_MRDROR(0)
    }
}
impl core::fmt::Debug for LPI2C_MRDROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MRDROR")
            .field("DATA", &self.DATA())
            .field("RXEMPTY", &self.RXEMPTY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MRDROR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MRDROR {{ DATA: {=u8:?}, RXEMPTY: {=bool:?} }}",
            self.DATA(),
            self.RXEMPTY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MSR(pub u32);
impl LPI2C_MSR {
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
    pub const fn EPF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NDF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ALF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ALF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FEF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PLTF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PLTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
    #[must_use]
    #[inline(always)]
    pub const fn BBF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LPI2C_MSR {
    #[inline(always)]
    fn default() -> LPI2C_MSR {
        LPI2C_MSR(0)
    }
}
impl core::fmt::Debug for LPI2C_MSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MSR")
            .field("TDF", &self.TDF())
            .field("RDF", &self.RDF())
            .field("EPF", &self.EPF())
            .field("SDF", &self.SDF())
            .field("NDF", &self.NDF())
            .field("ALF", &self.ALF())
            .field("FEF", &self.FEF())
            .field("PLTF", &self.PLTF())
            .field("DMF", &self.DMF())
            .field("STF", &self.STF())
            .field("MBF", &self.MBF())
            .field("BBF", &self.BBF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MSR {{ TDF: {=bool:?}, RDF: {=bool:?}, EPF: {=bool:?}, SDF: {=bool:?}, NDF: {=bool:?}, ALF: {=bool:?}, FEF: {=bool:?}, PLTF: {=bool:?}, DMF: {=bool:?}, STF: {=bool:?}, MBF: {=bool:?}, BBF: {=bool:?} }}",
            self.TDF(),
            self.RDF(),
            self.EPF(),
            self.SDF(),
            self.NDF(),
            self.ALF(),
            self.FEF(),
            self.PLTF(),
            self.DMF(),
            self.STF(),
            self.MBF(),
            self.BBF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MTCBR(pub u32);
impl LPI2C_MTCBR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for LPI2C_MTCBR {
    #[inline(always)]
    fn default() -> LPI2C_MTCBR {
        LPI2C_MTCBR(0)
    }
}
impl core::fmt::Debug for LPI2C_MTCBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MTCBR")
            .field("DATA", &self.DATA())
            .field("CMD", &self.CMD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MTCBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MTCBR {{ DATA: {=u8:?}, CMD: {=u8:?} }}",
            self.DATA(),
            self.CMD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MTDBR(pub u32);
impl LPI2C_MTDBR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for LPI2C_MTDBR {
    #[inline(always)]
    fn default() -> LPI2C_MTDBR {
        LPI2C_MTDBR(0)
    }
}
impl core::fmt::Debug for LPI2C_MTDBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MTDBR")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .field("DATA2", &self.DATA2())
            .field("DATA3", &self.DATA3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MTDBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MTDBR {{ DATA0: {=u8:?}, DATA1: {=u8:?}, DATA2: {=u8:?}, DATA3: {=u8:?} }}",
            self.DATA0(),
            self.DATA1(),
            self.DATA2(),
            self.DATA3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_MTDR(pub u32);
impl LPI2C_MTDR {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn CMD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_CMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for LPI2C_MTDR {
    #[inline(always)]
    fn default() -> LPI2C_MTDR {
        LPI2C_MTDR(0)
    }
}
impl core::fmt::Debug for LPI2C_MTDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_MTDR")
            .field("DATA", &self.DATA())
            .field("CMD", &self.CMD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_MTDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_MTDR {{ DATA: {=u8:?}, CMD: {=u8:?} }}",
            self.DATA(),
            self.CMD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_PARAM(pub u32);
impl LPI2C_PARAM {
    #[must_use]
    #[inline(always)]
    pub const fn MTXFIFO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MTXFIFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Controller Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn MRXFIFO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Controller Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_MRXFIFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for LPI2C_PARAM {
    #[inline(always)]
    fn default() -> LPI2C_PARAM {
        LPI2C_PARAM(0)
    }
}
impl core::fmt::Debug for LPI2C_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_PARAM")
            .field("MTXFIFO", &self.MTXFIFO())
            .field("MRXFIFO", &self.MRXFIFO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_PARAM {{ MTXFIFO: {=u8:?}, MRXFIFO: {=u8:?} }}",
            self.MTXFIFO(),
            self.MRXFIFO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SAMR(pub u32);
impl LPI2C_SAMR {
    #[doc = "Address 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 0 Value"]
    #[inline(always)]
    pub const fn set_ADDR0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Address 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR1(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x03ff;
        val as u16
    }
    #[doc = "Address 0 Value"]
    #[inline(always)]
    pub const fn set_ADDR1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 17usize)) | (((val as u32) & 0x03ff) << 17usize);
    }
}
impl Default for LPI2C_SAMR {
    #[inline(always)]
    fn default() -> LPI2C_SAMR {
        LPI2C_SAMR(0)
    }
}
impl core::fmt::Debug for LPI2C_SAMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SAMR")
            .field("ADDR0", &self.ADDR0())
            .field("ADDR1", &self.ADDR1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SAMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SAMR {{ ADDR0: {=u16:?}, ADDR1: {=u16:?} }}",
            self.ADDR0(),
            self.ADDR1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SASR(pub u32);
impl LPI2C_SASR {
    #[doc = "Received Address"]
    #[must_use]
    #[inline(always)]
    pub const fn RADDR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_RADDR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Received Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ANV(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_ANV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for LPI2C_SASR {
    #[inline(always)]
    fn default() -> LPI2C_SASR {
        LPI2C_SASR(0)
    }
}
impl core::fmt::Debug for LPI2C_SASR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SASR")
            .field("RADDR", &self.RADDR())
            .field("ANV", &self.ANV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SASR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SASR {{ RADDR: {=u16:?}, ANV: {=bool:?} }}",
            self.RADDR(),
            self.ANV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SCFGR0(pub u32);
impl LPI2C_SCFGR0 {
    #[must_use]
    #[inline(always)]
    pub const fn RDREQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RDACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RDACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for LPI2C_SCFGR0 {
    #[inline(always)]
    fn default() -> LPI2C_SCFGR0 {
        LPI2C_SCFGR0(0)
    }
}
impl core::fmt::Debug for LPI2C_SCFGR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SCFGR0")
            .field("RDREQ", &self.RDREQ())
            .field("RDACK", &self.RDACK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SCFGR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SCFGR0 {{ RDREQ: {=bool:?}, RDACK: {=bool:?} }}",
            self.RDREQ(),
            self.RDACK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SCFGR1(pub u32);
impl LPI2C_SCFGR1 {
    #[must_use]
    #[inline(always)]
    pub const fn ADRSTALL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ADRSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXSTALL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXDSTALL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXDSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACKSTALL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACKSTALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXNACK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GCEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SAEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXCFG(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXCFG(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IGNACK(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IGNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HSMEN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ADDRCFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ADDRCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXALL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXALL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSCFG(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDCFG(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDCFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for LPI2C_SCFGR1 {
    #[inline(always)]
    fn default() -> LPI2C_SCFGR1 {
        LPI2C_SCFGR1(0)
    }
}
impl core::fmt::Debug for LPI2C_SCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SCFGR1")
            .field("ADRSTALL", &self.ADRSTALL())
            .field("RXSTALL", &self.RXSTALL())
            .field("TXDSTALL", &self.TXDSTALL())
            .field("ACKSTALL", &self.ACKSTALL())
            .field("RXNACK", &self.RXNACK())
            .field("GCEN", &self.GCEN())
            .field("SAEN", &self.SAEN())
            .field("TXCFG", &self.TXCFG())
            .field("RXCFG", &self.RXCFG())
            .field("IGNACK", &self.IGNACK())
            .field("HSMEN", &self.HSMEN())
            .field("ADDRCFG", &self.ADDRCFG())
            .field("RXALL", &self.RXALL())
            .field("RSCFG", &self.RSCFG())
            .field("SDCFG", &self.SDCFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SCFGR1 {{ ADRSTALL: {=bool:?}, RXSTALL: {=bool:?}, TXDSTALL: {=bool:?}, ACKSTALL: {=bool:?}, RXNACK: {=bool:?}, GCEN: {=bool:?}, SAEN: {=bool:?}, TXCFG: {=bool:?}, RXCFG: {=bool:?}, IGNACK: {=bool:?}, HSMEN: {=bool:?}, ADDRCFG: {=u8:?}, RXALL: {=bool:?}, RSCFG: {=bool:?}, SDCFG: {=bool:?} }}",
            self.ADRSTALL(),
            self.RXSTALL(),
            self.TXDSTALL(),
            self.ACKSTALL(),
            self.RXNACK(),
            self.GCEN(),
            self.SAEN(),
            self.TXCFG(),
            self.RXCFG(),
            self.IGNACK(),
            self.HSMEN(),
            self.ADDRCFG(),
            self.RXALL(),
            self.RSCFG(),
            self.SDCFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SCFGR2(pub u32);
impl LPI2C_SCFGR2 {
    #[doc = "Clock Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn CLKHOLD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock Hold Time"]
    #[inline(always)]
    pub const fn set_CLKHOLD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Clock Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn DATAVD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock Hold Time"]
    #[inline(always)]
    pub const fn set_DATAVD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Data Valid Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn FILTSCL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Valid Delay"]
    #[inline(always)]
    pub const fn set_FILTSCL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Glitch Filter SCL"]
    #[must_use]
    #[inline(always)]
    pub const fn FILTSDA(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Glitch Filter SCL"]
    #[inline(always)]
    pub const fn set_FILTSDA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for LPI2C_SCFGR2 {
    #[inline(always)]
    fn default() -> LPI2C_SCFGR2 {
        LPI2C_SCFGR2(0)
    }
}
impl core::fmt::Debug for LPI2C_SCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SCFGR2")
            .field("CLKHOLD", &self.CLKHOLD())
            .field("DATAVD", &self.DATAVD())
            .field("FILTSCL", &self.FILTSCL())
            .field("FILTSDA", &self.FILTSDA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SCFGR2 {{ CLKHOLD: {=u8:?}, DATAVD: {=u8:?}, FILTSCL: {=u8:?}, FILTSDA: {=u8:?} }}",
            self.CLKHOLD(),
            self.DATAVD(),
            self.FILTSCL(),
            self.FILTSDA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SCR(pub u32);
impl LPI2C_SCR {
    #[must_use]
    #[inline(always)]
    pub const fn SEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEN(&mut self, val: bool) {
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
    pub const fn FILTEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FILTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FILTDZ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FILTDZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
impl Default for LPI2C_SCR {
    #[inline(always)]
    fn default() -> LPI2C_SCR {
        LPI2C_SCR(0)
    }
}
impl core::fmt::Debug for LPI2C_SCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SCR")
            .field("SEN", &self.SEN())
            .field("RST", &self.RST())
            .field("FILTEN", &self.FILTEN())
            .field("FILTDZ", &self.FILTDZ())
            .field("RTF", &self.RTF())
            .field("RRF", &self.RRF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SCR {{ SEN: {=bool:?}, RST: {=bool:?}, FILTEN: {=bool:?}, FILTDZ: {=bool:?}, RTF: {=bool:?}, RRF: {=bool:?} }}",
            self.SEN(),
            self.RST(),
            self.FILTEN(),
            self.FILTDZ(),
            self.RTF(),
            self.RRF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SDER(pub u32);
impl LPI2C_SDER {
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
    pub const fn AVDE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AVDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSDE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDDE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for LPI2C_SDER {
    #[inline(always)]
    fn default() -> LPI2C_SDER {
        LPI2C_SDER(0)
    }
}
impl core::fmt::Debug for LPI2C_SDER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SDER")
            .field("TDDE", &self.TDDE())
            .field("RDDE", &self.RDDE())
            .field("AVDE", &self.AVDE())
            .field("RSDE", &self.RSDE())
            .field("SDDE", &self.SDDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SDER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SDER {{ TDDE: {=bool:?}, RDDE: {=bool:?}, AVDE: {=bool:?}, RSDE: {=bool:?}, SDDE: {=bool:?} }}",
            self.TDDE(),
            self.RDDE(),
            self.AVDE(),
            self.RSDE(),
            self.SDDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SIER(pub u32);
impl LPI2C_SIER {
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
    pub const fn AVIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AVIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TAIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TAIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BEIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FEIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AM0IE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AM0IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AM1IE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AM1IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GCIE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GCIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SARIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SARIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for LPI2C_SIER {
    #[inline(always)]
    fn default() -> LPI2C_SIER {
        LPI2C_SIER(0)
    }
}
impl core::fmt::Debug for LPI2C_SIER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SIER")
            .field("TDIE", &self.TDIE())
            .field("RDIE", &self.RDIE())
            .field("AVIE", &self.AVIE())
            .field("TAIE", &self.TAIE())
            .field("RSIE", &self.RSIE())
            .field("SDIE", &self.SDIE())
            .field("BEIE", &self.BEIE())
            .field("FEIE", &self.FEIE())
            .field("AM0IE", &self.AM0IE())
            .field("AM1IE", &self.AM1IE())
            .field("GCIE", &self.GCIE())
            .field("SARIE", &self.SARIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SIER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SIER {{ TDIE: {=bool:?}, RDIE: {=bool:?}, AVIE: {=bool:?}, TAIE: {=bool:?}, RSIE: {=bool:?}, SDIE: {=bool:?}, BEIE: {=bool:?}, FEIE: {=bool:?}, AM0IE: {=bool:?}, AM1IE: {=bool:?}, GCIE: {=bool:?}, SARIE: {=bool:?} }}",
            self.TDIE(),
            self.RDIE(),
            self.AVIE(),
            self.TAIE(),
            self.RSIE(),
            self.SDIE(),
            self.BEIE(),
            self.FEIE(),
            self.AM0IE(),
            self.AM1IE(),
            self.GCIE(),
            self.SARIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SRDR(pub u32);
impl LPI2C_SRDR {
    #[doc = "Received Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Received Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Received Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RADDR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Received Data"]
    #[inline(always)]
    pub const fn set_RADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Received Address"]
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SOF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for LPI2C_SRDR {
    #[inline(always)]
    fn default() -> LPI2C_SRDR {
        LPI2C_SRDR(0)
    }
}
impl core::fmt::Debug for LPI2C_SRDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SRDR")
            .field("DATA", &self.DATA())
            .field("RADDR", &self.RADDR())
            .field("RXEMPTY", &self.RXEMPTY())
            .field("SOF", &self.SOF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SRDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SRDR {{ DATA: {=u8:?}, RADDR: {=u8:?}, RXEMPTY: {=bool:?}, SOF: {=bool:?} }}",
            self.DATA(),
            self.RADDR(),
            self.RXEMPTY(),
            self.SOF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SRDROR(pub u32);
impl LPI2C_SRDROR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RADDR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_RADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Received Address"]
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Received Address"]
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SOF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SOF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for LPI2C_SRDROR {
    #[inline(always)]
    fn default() -> LPI2C_SRDROR {
        LPI2C_SRDROR(0)
    }
}
impl core::fmt::Debug for LPI2C_SRDROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SRDROR")
            .field("DATA", &self.DATA())
            .field("RADDR", &self.RADDR())
            .field("RXEMPTY", &self.RXEMPTY())
            .field("SOF", &self.SOF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SRDROR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SRDROR {{ DATA: {=u8:?}, RADDR: {=u8:?}, RXEMPTY: {=bool:?}, SOF: {=bool:?} }}",
            self.DATA(),
            self.RADDR(),
            self.RXEMPTY(),
            self.SOF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_SSR(pub u32);
impl LPI2C_SSR {
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
    pub const fn AVF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AVF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TAF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RSF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RSF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SDF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BEF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FEF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FEF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AM0F(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AM0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AM1F(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AM1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GCF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SARF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SARF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SBF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BBF(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BBF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LPI2C_SSR {
    #[inline(always)]
    fn default() -> LPI2C_SSR {
        LPI2C_SSR(0)
    }
}
impl core::fmt::Debug for LPI2C_SSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_SSR")
            .field("TDF", &self.TDF())
            .field("RDF", &self.RDF())
            .field("AVF", &self.AVF())
            .field("TAF", &self.TAF())
            .field("RSF", &self.RSF())
            .field("SDF", &self.SDF())
            .field("BEF", &self.BEF())
            .field("FEF", &self.FEF())
            .field("AM0F", &self.AM0F())
            .field("AM1F", &self.AM1F())
            .field("GCF", &self.GCF())
            .field("SARF", &self.SARF())
            .field("SBF", &self.SBF())
            .field("BBF", &self.BBF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_SSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_SSR {{ TDF: {=bool:?}, RDF: {=bool:?}, AVF: {=bool:?}, TAF: {=bool:?}, RSF: {=bool:?}, SDF: {=bool:?}, BEF: {=bool:?}, FEF: {=bool:?}, AM0F: {=bool:?}, AM1F: {=bool:?}, GCF: {=bool:?}, SARF: {=bool:?}, SBF: {=bool:?}, BBF: {=bool:?} }}",
            self.TDF(),
            self.RDF(),
            self.AVF(),
            self.TAF(),
            self.RSF(),
            self.SDF(),
            self.BEF(),
            self.FEF(),
            self.AM0F(),
            self.AM1F(),
            self.GCF(),
            self.SARF(),
            self.SBF(),
            self.BBF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_STAR(pub u32);
impl LPI2C_STAR {
    #[must_use]
    #[inline(always)]
    pub const fn TXNACK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for LPI2C_STAR {
    #[inline(always)]
    fn default() -> LPI2C_STAR {
        LPI2C_STAR(0)
    }
}
impl core::fmt::Debug for LPI2C_STAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_STAR")
            .field("TXNACK", &self.TXNACK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_STAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPI2C_STAR {{ TXNACK: {=bool:?} }}", self.TXNACK())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_STDR(pub u32);
impl LPI2C_STDR {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for LPI2C_STDR {
    #[inline(always)]
    fn default() -> LPI2C_STDR {
        LPI2C_STDR(0)
    }
}
impl core::fmt::Debug for LPI2C_STDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_STDR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_STDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LPI2C_STDR {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPI2C_VERID(pub u32);
impl LPI2C_VERID {
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
impl Default for LPI2C_VERID {
    #[inline(always)]
    fn default() -> LPI2C_VERID {
        LPI2C_VERID(0)
    }
}
impl core::fmt::Debug for LPI2C_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPI2C_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPI2C_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
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

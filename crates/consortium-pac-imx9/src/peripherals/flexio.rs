#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO {
    ptr: *mut u8,
}
unsafe impl Send for FLEXIO {}
unsafe impl Sync for FLEXIO {}
impl FLEXIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<FLEXIO_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<FLEXIO_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<FLEXIO_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn PIN(self) -> crate::common::Reg<FLEXIO_PIN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSTAT(self) -> crate::common::Reg<FLEXIO_SHIFTSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTERR(self) -> crate::common::Reg<FLEXIO_SHIFTERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMSTAT(self) -> crate::common::Reg<FLEXIO_TIMSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSIEN(self) -> crate::common::Reg<FLEXIO_SHIFTSIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTEIEN(self) -> crate::common::Reg<FLEXIO_SHIFTEIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMIEN(self) -> crate::common::Reg<FLEXIO_TIMIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSDEN(self) -> crate::common::Reg<FLEXIO_SHIFTSDEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn TIMERSDEN(self) -> crate::common::Reg<FLEXIO_TIMERSDEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTSTATE(self) -> crate::common::Reg<FLEXIO_SHIFTSTATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn TRGSTAT(self) -> crate::common::Reg<FLEXIO_TRGSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn TRIGIEN(self) -> crate::common::Reg<FLEXIO_TRIGIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINSTAT(self) -> crate::common::Reg<FLEXIO_PINSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PINIEN(self) -> crate::common::Reg<FLEXIO_PINIEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn PINREN(self) -> crate::common::Reg<FLEXIO_PINREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn PINFEN(self) -> crate::common::Reg<FLEXIO_PINFEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTD(self) -> crate::common::Reg<FLEXIO_PINOUTD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTE(self) -> crate::common::Reg<FLEXIO_PINOUTE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTDIS(self) -> crate::common::Reg<FLEXIO_PINOUTDIS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTCLR(self) -> crate::common::Reg<FLEXIO_PINOUTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTSET(self) -> crate::common::Reg<FLEXIO_PINOUTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn PINOUTTOG(self) -> crate::common::Reg<FLEXIO_PINOUTTOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCTL(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTCTL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn SHIFTCFG(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTCFG, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUF(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUF, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBIS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFBIS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBYS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFBYS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFBBS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFBBS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TIMCTL(self, n: usize) -> crate::common::Reg<FLEXIO_TIMCTL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TIMCFG(self, n: usize) -> crate::common::Reg<FLEXIO_TIMCFG, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TIMCMP(self, n: usize) -> crate::common::Reg<FLEXIO_TIMCMP, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNBS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFNBS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHWS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFHWS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFNIS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFNIS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFOES(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFOES, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFEOS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFEOS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn SHIFTBUFHBS(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXIO_SHIFTBUFHBS, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_CTRL(pub u32);
impl FLEXIO_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn FLEXEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLEXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SWRST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SWRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FASTACC(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FASTACC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DBGE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DBGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOZEN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FLEXIO_CTRL {
    #[inline(always)]
    fn default() -> FLEXIO_CTRL {
        FLEXIO_CTRL(0)
    }
}
impl core::fmt::Debug for FLEXIO_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_CTRL")
            .field("FLEXEN", &self.FLEXEN())
            .field("SWRST", &self.SWRST())
            .field("FASTACC", &self.FASTACC())
            .field("DBGE", &self.DBGE())
            .field("DOZEN", &self.DOZEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_CTRL {{ FLEXEN: {=bool:?}, SWRST: {=bool:?}, FASTACC: {=bool:?}, DBGE: {=bool:?}, DOZEN: {=bool:?} }}",
            self.FLEXEN(),
            self.SWRST(),
            self.FASTACC(),
            self.DBGE(),
            self.DOZEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PARAM(pub u32);
impl FLEXIO_PARAM {
    #[doc = "Shifter Number"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn set_SHIFTER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Shifter Number"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMER(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn set_TIMER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Timer Number"]
    #[must_use]
    #[inline(always)]
    pub const fn PIN(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Number"]
    #[inline(always)]
    pub const fn set_PIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Pin Number"]
    #[must_use]
    #[inline(always)]
    pub const fn TRIGGER(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number"]
    #[inline(always)]
    pub const fn set_TRIGGER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLEXIO_PARAM {
    #[inline(always)]
    fn default() -> FLEXIO_PARAM {
        FLEXIO_PARAM(0)
    }
}
impl core::fmt::Debug for FLEXIO_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PARAM")
            .field("SHIFTER", &self.SHIFTER())
            .field("TIMER", &self.TIMER())
            .field("PIN", &self.PIN())
            .field("TRIGGER", &self.TRIGGER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_PARAM {{ SHIFTER: {=u8:?}, TIMER: {=u8:?}, PIN: {=u8:?}, TRIGGER: {=u8:?} }}",
            self.SHIFTER(),
            self.TIMER(),
            self.PIN(),
            self.TRIGGER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PIN(pub u32);
impl FLEXIO_PIN {
    #[doc = "Pin Data Input"]
    #[must_use]
    #[inline(always)]
    pub const fn PDI(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Data Input"]
    #[inline(always)]
    pub const fn set_PDI(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PIN {
    #[inline(always)]
    fn default() -> FLEXIO_PIN {
        FLEXIO_PIN(0)
    }
}
impl core::fmt::Debug for FLEXIO_PIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PIN")
            .field("PDI", &self.PDI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PIN {{ PDI: {=u32:?} }}", self.PDI())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINFEN(pub u32);
impl FLEXIO_PINFEN {
    #[doc = "Pin Falling Edge"]
    #[must_use]
    #[inline(always)]
    pub const fn PFE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Falling Edge"]
    #[inline(always)]
    pub const fn set_PFE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINFEN {
    #[inline(always)]
    fn default() -> FLEXIO_PINFEN {
        FLEXIO_PINFEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINFEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINFEN")
            .field("PFE", &self.PFE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINFEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINFEN {{ PFE: {=u32:?} }}", self.PFE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINIEN(pub u32);
impl FLEXIO_PINIEN {
    #[doc = "Pin Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PSIE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_PSIE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINIEN {
    #[inline(always)]
    fn default() -> FLEXIO_PINIEN {
        FLEXIO_PINIEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINIEN")
            .field("PSIE", &self.PSIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINIEN {{ PSIE: {=u32:?} }}", self.PSIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTCLR(pub u32);
impl FLEXIO_PINOUTCLR {
    #[doc = "Output Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTCLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Clear"]
    #[inline(always)]
    pub const fn set_OUTCLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTCLR {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTCLR {
        FLEXIO_PINOUTCLR(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTCLR")
            .field("OUTCLR", &self.OUTCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTCLR {{ OUTCLR: {=u32:?} }}", self.OUTCLR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTD(pub u32);
impl FLEXIO_PINOUTD {
    #[doc = "Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data"]
    #[inline(always)]
    pub const fn set_OUTD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTD {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTD {
        FLEXIO_PINOUTD(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTD")
            .field("OUTD", &self.OUTD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTD {{ OUTD: {=u32:?} }}", self.OUTD())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTDIS(pub u32);
impl FLEXIO_PINOUTDIS {
    #[doc = "Output Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTDIS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Disable"]
    #[inline(always)]
    pub const fn set_OUTDIS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTDIS {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTDIS {
        FLEXIO_PINOUTDIS(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTDIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTDIS")
            .field("OUTDIS", &self.OUTDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTDIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTDIS {{ OUTDIS: {=u32:?} }}", self.OUTDIS())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTE(pub u32);
impl FLEXIO_PINOUTE {
    #[doc = "Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub const fn set_OUTE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTE {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTE {
        FLEXIO_PINOUTE(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTE")
            .field("OUTE", &self.OUTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTE {{ OUTE: {=u32:?} }}", self.OUTE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTSET(pub u32);
impl FLEXIO_PINOUTSET {
    #[doc = "Output Set"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Set"]
    #[inline(always)]
    pub const fn set_OUTSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTSET {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTSET {
        FLEXIO_PINOUTSET(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTSET")
            .field("OUTSET", &self.OUTSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTSET {{ OUTSET: {=u32:?} }}", self.OUTSET())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINOUTTOG(pub u32);
impl FLEXIO_PINOUTTOG {
    #[doc = "Output Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn OUTTOG(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Toggle"]
    #[inline(always)]
    pub const fn set_OUTTOG(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINOUTTOG {
    #[inline(always)]
    fn default() -> FLEXIO_PINOUTTOG {
        FLEXIO_PINOUTTOG(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINOUTTOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINOUTTOG")
            .field("OUTTOG", &self.OUTTOG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINOUTTOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINOUTTOG {{ OUTTOG: {=u32:?} }}", self.OUTTOG())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINREN(pub u32);
impl FLEXIO_PINREN {
    #[doc = "Pin Rising Edge"]
    #[must_use]
    #[inline(always)]
    pub const fn PRE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Rising Edge"]
    #[inline(always)]
    pub const fn set_PRE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINREN {
    #[inline(always)]
    fn default() -> FLEXIO_PINREN {
        FLEXIO_PINREN(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINREN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINREN")
            .field("PRE", &self.PRE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINREN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINREN {{ PRE: {=u32:?} }}", self.PRE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_PINSTAT(pub u32);
impl FLEXIO_PINSTAT {
    #[must_use]
    #[inline(always)]
    pub const fn PSF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_PSF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_PINSTAT {
    #[inline(always)]
    fn default() -> FLEXIO_PINSTAT {
        FLEXIO_PINSTAT(0)
    }
}
impl core::fmt::Debug for FLEXIO_PINSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_PINSTAT")
            .field("PSF", &self.PSF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_PINSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_PINSTAT {{ PSF: {=u32:?} }}", self.PSF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUF(pub u32);
impl FLEXIO_SHIFTBUF {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUF {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUF {
        FLEXIO_SHIFTBUF(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUF")
            .field("SHIFTBUF", &self.SHIFTBUF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUF {{ SHIFTBUF: {=u32:?} }}",
            self.SHIFTBUF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFBBS(pub u32);
impl FLEXIO_SHIFTBUFBBS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFBBS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFBBS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFBBS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFBBS {
        FLEXIO_SHIFTBUFBBS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFBBS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFBBS")
            .field("SHIFTBUFBBS", &self.SHIFTBUFBBS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFBBS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFBBS {{ SHIFTBUFBBS: {=u32:?} }}",
            self.SHIFTBUFBBS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFBIS(pub u32);
impl FLEXIO_SHIFTBUFBIS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFBIS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFBIS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFBIS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFBIS {
        FLEXIO_SHIFTBUFBIS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFBIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFBIS")
            .field("SHIFTBUFBIS", &self.SHIFTBUFBIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFBIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFBIS {{ SHIFTBUFBIS: {=u32:?} }}",
            self.SHIFTBUFBIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFBYS(pub u32);
impl FLEXIO_SHIFTBUFBYS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFBYS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFBYS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFBYS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFBYS {
        FLEXIO_SHIFTBUFBYS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFBYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFBYS")
            .field("SHIFTBUFBYS", &self.SHIFTBUFBYS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFBYS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFBYS {{ SHIFTBUFBYS: {=u32:?} }}",
            self.SHIFTBUFBYS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFEOS(pub u32);
impl FLEXIO_SHIFTBUFEOS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFEOS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFEOS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFEOS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFEOS {
        FLEXIO_SHIFTBUFEOS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFEOS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFEOS")
            .field("SHIFTBUFEOS", &self.SHIFTBUFEOS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFEOS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFEOS {{ SHIFTBUFEOS: {=u32:?} }}",
            self.SHIFTBUFEOS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFHBS(pub u32);
impl FLEXIO_SHIFTBUFHBS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFHBS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFHBS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFHBS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFHBS {
        FLEXIO_SHIFTBUFHBS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFHBS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFHBS")
            .field("SHIFTBUFHBS", &self.SHIFTBUFHBS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFHBS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFHBS {{ SHIFTBUFHBS: {=u32:?} }}",
            self.SHIFTBUFHBS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFHWS(pub u32);
impl FLEXIO_SHIFTBUFHWS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFHWS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFHWS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFHWS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFHWS {
        FLEXIO_SHIFTBUFHWS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFHWS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFHWS")
            .field("SHIFTBUFHWS", &self.SHIFTBUFHWS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFHWS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFHWS {{ SHIFTBUFHWS: {=u32:?} }}",
            self.SHIFTBUFHWS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFNBS(pub u32);
impl FLEXIO_SHIFTBUFNBS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFNBS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFNBS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFNBS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFNBS {
        FLEXIO_SHIFTBUFNBS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFNBS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFNBS")
            .field("SHIFTBUFNBS", &self.SHIFTBUFNBS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFNBS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFNBS {{ SHIFTBUFNBS: {=u32:?} }}",
            self.SHIFTBUFNBS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFNIS(pub u32);
impl FLEXIO_SHIFTBUFNIS {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFNIS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFNIS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFNIS {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFNIS {
        FLEXIO_SHIFTBUFNIS(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFNIS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFNIS")
            .field("SHIFTBUFNIS", &self.SHIFTBUFNIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFNIS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFNIS {{ SHIFTBUFNIS: {=u32:?} }}",
            self.SHIFTBUFNIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTBUFOES(pub u32);
impl FLEXIO_SHIFTBUFOES {
    #[doc = "Shift Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFTBUFOES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer"]
    #[inline(always)]
    pub const fn set_SHIFTBUFOES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTBUFOES {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTBUFOES {
        FLEXIO_SHIFTBUFOES(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTBUFOES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTBUFOES")
            .field("SHIFTBUFOES", &self.SHIFTBUFOES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTBUFOES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTBUFOES {{ SHIFTBUFOES: {=u32:?} }}",
            self.SHIFTBUFOES()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTCFG(pub u32);
impl FLEXIO_SHIFTCFG {
    #[must_use]
    #[inline(always)]
    pub const fn SSTART(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SSTART(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SSTOP(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SSTOP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INSRC(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LATST(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LATST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SSIZE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SSIZE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Parallel Width"]
    #[must_use]
    #[inline(always)]
    pub const fn PWIDTH(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parallel Width"]
    #[inline(always)]
    pub const fn set_PWIDTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for FLEXIO_SHIFTCFG {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTCFG {
        FLEXIO_SHIFTCFG(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTCFG")
            .field("SSTART", &self.SSTART())
            .field("SSTOP", &self.SSTOP())
            .field("INSRC", &self.INSRC())
            .field("LATST", &self.LATST())
            .field("SSIZE", &self.SSIZE())
            .field("PWIDTH", &self.PWIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTCFG {{ SSTART: {=u8:?}, SSTOP: {=u8:?}, INSRC: {=bool:?}, LATST: {=bool:?}, SSIZE: {=bool:?}, PWIDTH: {=u8:?} }}",
            self.SSTART(),
            self.SSTOP(),
            self.INSRC(),
            self.LATST(),
            self.SSIZE(),
            self.PWIDTH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTCTL(pub u32);
impl FLEXIO_SHIFTCTL {
    #[must_use]
    #[inline(always)]
    pub const fn SMOD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PINPOL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PINPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Shifter Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn PINSEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn set_PINSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Shifter Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn PINCFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn set_PINCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMPOL(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIMPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Timer Select"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMSEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Timer Select"]
    #[inline(always)]
    pub const fn set_TIMSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for FLEXIO_SHIFTCTL {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTCTL {
        FLEXIO_SHIFTCTL(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTCTL")
            .field("SMOD", &self.SMOD())
            .field("PINPOL", &self.PINPOL())
            .field("PINSEL", &self.PINSEL())
            .field("PINCFG", &self.PINCFG())
            .field("TIMPOL", &self.TIMPOL())
            .field("TIMSEL", &self.TIMSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_SHIFTCTL {{ SMOD: {=u8:?}, PINPOL: {=bool:?}, PINSEL: {=u8:?}, PINCFG: {=u8:?}, TIMPOL: {=bool:?}, TIMSEL: {=u8:?} }}",
            self.SMOD(),
            self.PINPOL(),
            self.PINSEL(),
            self.PINCFG(),
            self.TIMPOL(),
            self.TIMSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTEIEN(pub u32);
impl FLEXIO_SHIFTEIEN {
    #[doc = "Shifter Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEIE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_SEIE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTEIEN {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTEIEN {
        FLEXIO_SHIFTEIEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTEIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTEIEN")
            .field("SEIE", &self.SEIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTEIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTEIEN {{ SEIE: {=u8:?} }}", self.SEIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTERR(pub u32);
impl FLEXIO_SHIFTERR {
    #[must_use]
    #[inline(always)]
    pub const fn SEF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SEF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTERR {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTERR {
        FLEXIO_SHIFTERR(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTERR")
            .field("SEF", &self.SEF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTERR {{ SEF: {=u8:?} }}", self.SEF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTSDEN(pub u32);
impl FLEXIO_SHIFTSDEN {
    #[doc = "Shifter Status DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSDE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn set_SSDE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTSDEN {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTSDEN {
        FLEXIO_SHIFTSDEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTSDEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTSDEN")
            .field("SSDE", &self.SSDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTSDEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTSDEN {{ SSDE: {=u8:?} }}", self.SSDE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTSIEN(pub u32);
impl FLEXIO_SHIFTSIEN {
    #[doc = "Shifter Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSIE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_SSIE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTSIEN {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTSIEN {
        FLEXIO_SHIFTSIEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTSIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTSIEN")
            .field("SSIE", &self.SSIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTSIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTSIEN {{ SSIE: {=u8:?} }}", self.SSIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTSTAT(pub u32);
impl FLEXIO_SHIFTSTAT {
    #[must_use]
    #[inline(always)]
    pub const fn SSF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SSF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTSTAT {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTSTAT {
        FLEXIO_SHIFTSTAT(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTSTAT")
            .field("SSF", &self.SSF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTSTAT {{ SSF: {=u8:?} }}", self.SSF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_SHIFTSTATE(pub u32);
impl FLEXIO_SHIFTSTATE {
    #[doc = "Current State Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current State Pointer"]
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for FLEXIO_SHIFTSTATE {
    #[inline(always)]
    fn default() -> FLEXIO_SHIFTSTATE {
        FLEXIO_SHIFTSTATE(0)
    }
}
impl core::fmt::Debug for FLEXIO_SHIFTSTATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_SHIFTSTATE")
            .field("STATE", &self.STATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_SHIFTSTATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_SHIFTSTATE {{ STATE: {=u8:?} }}", self.STATE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMCFG(pub u32);
impl FLEXIO_TIMCFG {
    #[must_use]
    #[inline(always)]
    pub const fn TSTART(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSTOP(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TSTOP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMENA(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMENA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMDIS(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMDIS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMRST(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMRST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMDEC(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMDEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMOUT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for FLEXIO_TIMCFG {
    #[inline(always)]
    fn default() -> FLEXIO_TIMCFG {
        FLEXIO_TIMCFG(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMCFG")
            .field("TSTART", &self.TSTART())
            .field("TSTOP", &self.TSTOP())
            .field("TIMENA", &self.TIMENA())
            .field("TIMDIS", &self.TIMDIS())
            .field("TIMRST", &self.TIMRST())
            .field("TIMDEC", &self.TIMDEC())
            .field("TIMOUT", &self.TIMOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_TIMCFG {{ TSTART: {=bool:?}, TSTOP: {=u8:?}, TIMENA: {=u8:?}, TIMDIS: {=u8:?}, TIMRST: {=u8:?}, TIMDEC: {=u8:?}, TIMOUT: {=u8:?} }}",
            self.TSTART(),
            self.TSTOP(),
            self.TIMENA(),
            self.TIMDIS(),
            self.TIMRST(),
            self.TIMDEC(),
            self.TIMOUT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMCMP(pub u32);
impl FLEXIO_TIMCMP {
    #[doc = "Timer Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn CMP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Compare Value"]
    #[inline(always)]
    pub const fn set_CMP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FLEXIO_TIMCMP {
    #[inline(always)]
    fn default() -> FLEXIO_TIMCMP {
        FLEXIO_TIMCMP(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMCMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMCMP")
            .field("CMP", &self.CMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMCMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TIMCMP {{ CMP: {=u16:?} }}", self.CMP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMCTL(pub u32);
impl FLEXIO_TIMCTL {
    #[must_use]
    #[inline(always)]
    pub const fn TIMOD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ONETIM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ONETIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PININS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PININS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PINPOL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PINPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn PINSEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_PINSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Timer Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn PINCFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_PINCFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGSRC(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRGSRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGPOL(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRGPOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn TRGSEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_TRGSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for FLEXIO_TIMCTL {
    #[inline(always)]
    fn default() -> FLEXIO_TIMCTL {
        FLEXIO_TIMCTL(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMCTL")
            .field("TIMOD", &self.TIMOD())
            .field("ONETIM", &self.ONETIM())
            .field("PININS", &self.PININS())
            .field("PINPOL", &self.PINPOL())
            .field("PINSEL", &self.PINSEL())
            .field("PINCFG", &self.PINCFG())
            .field("TRGSRC", &self.TRGSRC())
            .field("TRGPOL", &self.TRGPOL())
            .field("TRGSEL", &self.TRGSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_TIMCTL {{ TIMOD: {=u8:?}, ONETIM: {=bool:?}, PININS: {=bool:?}, PINPOL: {=bool:?}, PINSEL: {=u8:?}, PINCFG: {=u8:?}, TRGSRC: {=bool:?}, TRGPOL: {=bool:?}, TRGSEL: {=u8:?} }}",
            self.TIMOD(),
            self.ONETIM(),
            self.PININS(),
            self.PINPOL(),
            self.PINSEL(),
            self.PINCFG(),
            self.TRGSRC(),
            self.TRGPOL(),
            self.TRGSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMERSDEN(pub u32);
impl FLEXIO_TIMERSDEN {
    #[doc = "Timer Status DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TSDE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status DMA Enable"]
    #[inline(always)]
    pub const fn set_TSDE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_TIMERSDEN {
    #[inline(always)]
    fn default() -> FLEXIO_TIMERSDEN {
        FLEXIO_TIMERSDEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMERSDEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMERSDEN")
            .field("TSDE", &self.TSDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMERSDEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TIMERSDEN {{ TSDE: {=u8:?} }}", self.TSDE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMIEN(pub u32);
impl FLEXIO_TIMIEN {
    #[doc = "Timer Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TEIE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_TEIE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_TIMIEN {
    #[inline(always)]
    fn default() -> FLEXIO_TIMIEN {
        FLEXIO_TIMIEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMIEN")
            .field("TEIE", &self.TEIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TIMIEN {{ TEIE: {=u8:?} }}", self.TEIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TIMSTAT(pub u32);
impl FLEXIO_TIMSTAT {
    #[must_use]
    #[inline(always)]
    pub const fn TSF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TSF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FLEXIO_TIMSTAT {
    #[inline(always)]
    fn default() -> FLEXIO_TIMSTAT {
        FLEXIO_TIMSTAT(0)
    }
}
impl core::fmt::Debug for FLEXIO_TIMSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TIMSTAT")
            .field("TSF", &self.TSF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TIMSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TIMSTAT {{ TSF: {=u8:?} }}", self.TSF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TRGSTAT(pub u32);
impl FLEXIO_TRGSTAT {
    #[must_use]
    #[inline(always)]
    pub const fn ETSF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ETSF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for FLEXIO_TRGSTAT {
    #[inline(always)]
    fn default() -> FLEXIO_TRGSTAT {
        FLEXIO_TRGSTAT(0)
    }
}
impl core::fmt::Debug for FLEXIO_TRGSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TRGSTAT")
            .field("ETSF", &self.ETSF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TRGSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TRGSTAT {{ ETSF: {=u8:?} }}", self.ETSF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_TRIGIEN(pub u32);
impl FLEXIO_TRIGIEN {
    #[must_use]
    #[inline(always)]
    pub const fn TRIE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TRIE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for FLEXIO_TRIGIEN {
    #[inline(always)]
    fn default() -> FLEXIO_TRIGIEN {
        FLEXIO_TRIGIEN(0)
    }
}
impl core::fmt::Debug for FLEXIO_TRIGIEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_TRIGIEN")
            .field("TRIE", &self.TRIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_TRIGIEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXIO_TRIGIEN {{ TRIE: {=u8:?} }}", self.TRIE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXIO_VERID(pub u32);
impl FLEXIO_VERID {
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
impl Default for FLEXIO_VERID {
    #[inline(always)]
    fn default() -> FLEXIO_VERID {
        FLEXIO_VERID(0)
    }
}
impl core::fmt::Debug for FLEXIO_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXIO_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXIO_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXIO_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}

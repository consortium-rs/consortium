#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO {
    ptr: *mut u8,
}
unsafe impl Send for RGPIO {}
unsafe impl Sync for RGPIO {}
impl RGPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<RGPIO_VERID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<RGPIO_PARAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<RGPIO_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn PCNS(self) -> crate::common::Reg<RGPIO_PCNS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ICNS(self) -> crate::common::Reg<RGPIO_ICNS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn PCNP(self) -> crate::common::Reg<RGPIO_PCNP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn ICNP(self) -> crate::common::Reg<RGPIO_ICNP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn PDOR(self) -> crate::common::Reg<RGPIO_PDOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn PSOR(self) -> crate::common::Reg<RGPIO_PSOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn PCOR(self) -> crate::common::Reg<RGPIO_PCOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn PTOR(self) -> crate::common::Reg<RGPIO_PTOR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn PDIR(self) -> crate::common::Reg<RGPIO_PDIR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn PDDR(self) -> crate::common::Reg<RGPIO_PDDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn PIDR(self) -> crate::common::Reg<RGPIO_PIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn PDR(self, n: usize) -> crate::common::Reg<RGPIO_PDR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 1usize) as _) }
    }
    #[inline(always)]
    pub const fn GICLR(self) -> crate::common::Reg<RGPIO_GICLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn GICHR(self) -> crate::common::Reg<RGPIO_GICHR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn ISFR(self, n: usize) -> crate::common::Reg<RGPIO_ISFR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_GICHR(pub u32);
impl RGPIO_GICHR {
    #[must_use]
    #[inline(always)]
    pub const fn GIWE16(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE17(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE18(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE19(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE20(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE21(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE22(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE24(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE25(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE26(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE27(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE28(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE29(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE30(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE31(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn GIWD(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_GIWD(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RGPIO_GICHR {
    #[inline(always)]
    fn default() -> RGPIO_GICHR {
        RGPIO_GICHR(0)
    }
}
impl core::fmt::Debug for RGPIO_GICHR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_GICHR")
            .field("GIWE16", &self.GIWE16())
            .field("GIWE17", &self.GIWE17())
            .field("GIWE18", &self.GIWE18())
            .field("GIWE19", &self.GIWE19())
            .field("GIWE20", &self.GIWE20())
            .field("GIWE21", &self.GIWE21())
            .field("GIWE22", &self.GIWE22())
            .field("GIWE23", &self.GIWE23())
            .field("GIWE24", &self.GIWE24())
            .field("GIWE25", &self.GIWE25())
            .field("GIWE26", &self.GIWE26())
            .field("GIWE27", &self.GIWE27())
            .field("GIWE28", &self.GIWE28())
            .field("GIWE29", &self.GIWE29())
            .field("GIWE30", &self.GIWE30())
            .field("GIWE31", &self.GIWE31())
            .field("GIWD", &self.GIWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_GICHR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_GICHR {{ GIWE16: {=bool:?}, GIWE17: {=bool:?}, GIWE18: {=bool:?}, GIWE19: {=bool:?}, GIWE20: {=bool:?}, GIWE21: {=bool:?}, GIWE22: {=bool:?}, GIWE23: {=bool:?}, GIWE24: {=bool:?}, GIWE25: {=bool:?}, GIWE26: {=bool:?}, GIWE27: {=bool:?}, GIWE28: {=bool:?}, GIWE29: {=bool:?}, GIWE30: {=bool:?}, GIWE31: {=bool:?}, GIWD: {=u16:?} }}",
            self.GIWE16(),
            self.GIWE17(),
            self.GIWE18(),
            self.GIWE19(),
            self.GIWE20(),
            self.GIWE21(),
            self.GIWE22(),
            self.GIWE23(),
            self.GIWE24(),
            self.GIWE25(),
            self.GIWE26(),
            self.GIWE27(),
            self.GIWE28(),
            self.GIWE29(),
            self.GIWE30(),
            self.GIWE31(),
            self.GIWD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_GICLR(pub u32);
impl RGPIO_GICLR {
    #[must_use]
    #[inline(always)]
    pub const fn GIWE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GIWE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_GIWE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Global Interrupt Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn GIWD(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data"]
    #[inline(always)]
    pub const fn set_GIWD(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for RGPIO_GICLR {
    #[inline(always)]
    fn default() -> RGPIO_GICLR {
        RGPIO_GICLR(0)
    }
}
impl core::fmt::Debug for RGPIO_GICLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_GICLR")
            .field("GIWE0", &self.GIWE0())
            .field("GIWE1", &self.GIWE1())
            .field("GIWE2", &self.GIWE2())
            .field("GIWE3", &self.GIWE3())
            .field("GIWE4", &self.GIWE4())
            .field("GIWE5", &self.GIWE5())
            .field("GIWE6", &self.GIWE6())
            .field("GIWE7", &self.GIWE7())
            .field("GIWE8", &self.GIWE8())
            .field("GIWE9", &self.GIWE9())
            .field("GIWE10", &self.GIWE10())
            .field("GIWE11", &self.GIWE11())
            .field("GIWE12", &self.GIWE12())
            .field("GIWE13", &self.GIWE13())
            .field("GIWE14", &self.GIWE14())
            .field("GIWE15", &self.GIWE15())
            .field("GIWD", &self.GIWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_GICLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_GICLR {{ GIWE0: {=bool:?}, GIWE1: {=bool:?}, GIWE2: {=bool:?}, GIWE3: {=bool:?}, GIWE4: {=bool:?}, GIWE5: {=bool:?}, GIWE6: {=bool:?}, GIWE7: {=bool:?}, GIWE8: {=bool:?}, GIWE9: {=bool:?}, GIWE10: {=bool:?}, GIWE11: {=bool:?}, GIWE12: {=bool:?}, GIWE13: {=bool:?}, GIWE14: {=bool:?}, GIWE15: {=bool:?}, GIWD: {=u16:?} }}",
            self.GIWE0(),
            self.GIWE1(),
            self.GIWE2(),
            self.GIWE3(),
            self.GIWE4(),
            self.GIWE5(),
            self.GIWE6(),
            self.GIWE7(),
            self.GIWE8(),
            self.GIWE9(),
            self.GIWE10(),
            self.GIWE11(),
            self.GIWE12(),
            self.GIWE13(),
            self.GIWE14(),
            self.GIWE15(),
            self.GIWD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_ICNP(pub u32);
impl RGPIO_ICNP {
    #[must_use]
    #[inline(always)]
    pub const fn NPE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RGPIO_ICNP {
    #[inline(always)]
    fn default() -> RGPIO_ICNP {
        RGPIO_ICNP(0)
    }
}
impl core::fmt::Debug for RGPIO_ICNP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_ICNP")
            .field("NPE0", &self.NPE0())
            .field("NPE1", &self.NPE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_ICNP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_ICNP {{ NPE0: {=bool:?}, NPE1: {=bool:?} }}",
            self.NPE0(),
            self.NPE1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_ICNS(pub u32);
impl RGPIO_ICNS {
    #[must_use]
    #[inline(always)]
    pub const fn NSE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RGPIO_ICNS {
    #[inline(always)]
    fn default() -> RGPIO_ICNS {
        RGPIO_ICNS(0)
    }
}
impl core::fmt::Debug for RGPIO_ICNS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_ICNS")
            .field("NSE0", &self.NSE0())
            .field("NSE1", &self.NSE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_ICNS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_ICNS {{ NSE0: {=bool:?}, NSE1: {=bool:?} }}",
            self.NSE0(),
            self.NSE1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_ICR(pub u32);
impl RGPIO_ICR {
    #[must_use]
    #[inline(always)]
    pub const fn IRQC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IRQC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IRQS(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IRQS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LK(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RGPIO_ICR {
    #[inline(always)]
    fn default() -> RGPIO_ICR {
        RGPIO_ICR(0)
    }
}
impl core::fmt::Debug for RGPIO_ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_ICR")
            .field("IRQC", &self.IRQC())
            .field("IRQS", &self.IRQS())
            .field("LK", &self.LK())
            .field("ISF", &self.ISF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_ICR {{ IRQC: {=u8:?}, IRQS: {=bool:?}, LK: {=bool:?}, ISF: {=bool:?} }}",
            self.IRQC(),
            self.IRQS(),
            self.LK(),
            self.ISF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_ISFR(pub u32);
impl RGPIO_ISFR {
    #[must_use]
    #[inline(always)]
    pub const fn ISF0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISF31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ISF31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_ISFR {
    #[inline(always)]
    fn default() -> RGPIO_ISFR {
        RGPIO_ISFR(0)
    }
}
impl core::fmt::Debug for RGPIO_ISFR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_ISFR")
            .field("ISF0", &self.ISF0())
            .field("ISF1", &self.ISF1())
            .field("ISF2", &self.ISF2())
            .field("ISF3", &self.ISF3())
            .field("ISF4", &self.ISF4())
            .field("ISF5", &self.ISF5())
            .field("ISF6", &self.ISF6())
            .field("ISF7", &self.ISF7())
            .field("ISF8", &self.ISF8())
            .field("ISF9", &self.ISF9())
            .field("ISF10", &self.ISF10())
            .field("ISF11", &self.ISF11())
            .field("ISF12", &self.ISF12())
            .field("ISF13", &self.ISF13())
            .field("ISF14", &self.ISF14())
            .field("ISF15", &self.ISF15())
            .field("ISF16", &self.ISF16())
            .field("ISF17", &self.ISF17())
            .field("ISF18", &self.ISF18())
            .field("ISF19", &self.ISF19())
            .field("ISF20", &self.ISF20())
            .field("ISF21", &self.ISF21())
            .field("ISF22", &self.ISF22())
            .field("ISF23", &self.ISF23())
            .field("ISF24", &self.ISF24())
            .field("ISF25", &self.ISF25())
            .field("ISF26", &self.ISF26())
            .field("ISF27", &self.ISF27())
            .field("ISF28", &self.ISF28())
            .field("ISF29", &self.ISF29())
            .field("ISF30", &self.ISF30())
            .field("ISF31", &self.ISF31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_ISFR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_ISFR {{ ISF0: {=bool:?}, ISF1: {=bool:?}, ISF2: {=bool:?}, ISF3: {=bool:?}, ISF4: {=bool:?}, ISF5: {=bool:?}, ISF6: {=bool:?}, ISF7: {=bool:?}, ISF8: {=bool:?}, ISF9: {=bool:?}, ISF10: {=bool:?}, ISF11: {=bool:?}, ISF12: {=bool:?}, ISF13: {=bool:?}, ISF14: {=bool:?}, ISF15: {=bool:?}, ISF16: {=bool:?}, ISF17: {=bool:?}, ISF18: {=bool:?}, ISF19: {=bool:?}, ISF20: {=bool:?}, ISF21: {=bool:?}, ISF22: {=bool:?}, ISF23: {=bool:?}, ISF24: {=bool:?}, ISF25: {=bool:?}, ISF26: {=bool:?}, ISF27: {=bool:?}, ISF28: {=bool:?}, ISF29: {=bool:?}, ISF30: {=bool:?}, ISF31: {=bool:?} }}",
            self.ISF0(),
            self.ISF1(),
            self.ISF2(),
            self.ISF3(),
            self.ISF4(),
            self.ISF5(),
            self.ISF6(),
            self.ISF7(),
            self.ISF8(),
            self.ISF9(),
            self.ISF10(),
            self.ISF11(),
            self.ISF12(),
            self.ISF13(),
            self.ISF14(),
            self.ISF15(),
            self.ISF16(),
            self.ISF17(),
            self.ISF18(),
            self.ISF19(),
            self.ISF20(),
            self.ISF21(),
            self.ISF22(),
            self.ISF23(),
            self.ISF24(),
            self.ISF25(),
            self.ISF26(),
            self.ISF27(),
            self.ISF28(),
            self.ISF29(),
            self.ISF30(),
            self.ISF31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_LOCK(pub u32);
impl RGPIO_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn PCNS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PCNS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ICNS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ICNS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PCNP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PCNP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ICNP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ICNP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RGPIO_LOCK {
    #[inline(always)]
    fn default() -> RGPIO_LOCK {
        RGPIO_LOCK(0)
    }
}
impl core::fmt::Debug for RGPIO_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_LOCK")
            .field("PCNS", &self.PCNS())
            .field("ICNS", &self.ICNS())
            .field("PCNP", &self.PCNP())
            .field("ICNP", &self.ICNP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_LOCK {{ PCNS: {=bool:?}, ICNS: {=bool:?}, PCNP: {=bool:?}, ICNP: {=bool:?} }}",
            self.PCNS(),
            self.ICNS(),
            self.PCNP(),
            self.ICNP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PARAM(pub u32);
impl RGPIO_PARAM {
    #[doc = "Interrupt Number"]
    #[must_use]
    #[inline(always)]
    pub const fn IRQNUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt Number"]
    #[inline(always)]
    pub const fn set_IRQNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for RGPIO_PARAM {
    #[inline(always)]
    fn default() -> RGPIO_PARAM {
        RGPIO_PARAM(0)
    }
}
impl core::fmt::Debug for RGPIO_PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PARAM")
            .field("IRQNUM", &self.IRQNUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RGPIO_PARAM {{ IRQNUM: {=u8:?} }}", self.IRQNUM())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PCNP(pub u32);
impl RGPIO_PCNP {
    #[must_use]
    #[inline(always)]
    pub const fn NPE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NPE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NPE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PCNP {
    #[inline(always)]
    fn default() -> RGPIO_PCNP {
        RGPIO_PCNP(0)
    }
}
impl core::fmt::Debug for RGPIO_PCNP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PCNP")
            .field("NPE0", &self.NPE0())
            .field("NPE1", &self.NPE1())
            .field("NPE2", &self.NPE2())
            .field("NPE3", &self.NPE3())
            .field("NPE4", &self.NPE4())
            .field("NPE5", &self.NPE5())
            .field("NPE6", &self.NPE6())
            .field("NPE7", &self.NPE7())
            .field("NPE8", &self.NPE8())
            .field("NPE9", &self.NPE9())
            .field("NPE10", &self.NPE10())
            .field("NPE11", &self.NPE11())
            .field("NPE12", &self.NPE12())
            .field("NPE13", &self.NPE13())
            .field("NPE14", &self.NPE14())
            .field("NPE15", &self.NPE15())
            .field("NPE16", &self.NPE16())
            .field("NPE17", &self.NPE17())
            .field("NPE18", &self.NPE18())
            .field("NPE19", &self.NPE19())
            .field("NPE20", &self.NPE20())
            .field("NPE21", &self.NPE21())
            .field("NPE22", &self.NPE22())
            .field("NPE23", &self.NPE23())
            .field("NPE24", &self.NPE24())
            .field("NPE25", &self.NPE25())
            .field("NPE26", &self.NPE26())
            .field("NPE27", &self.NPE27())
            .field("NPE28", &self.NPE28())
            .field("NPE29", &self.NPE29())
            .field("NPE30", &self.NPE30())
            .field("NPE31", &self.NPE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PCNP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PCNP {{ NPE0: {=bool:?}, NPE1: {=bool:?}, NPE2: {=bool:?}, NPE3: {=bool:?}, NPE4: {=bool:?}, NPE5: {=bool:?}, NPE6: {=bool:?}, NPE7: {=bool:?}, NPE8: {=bool:?}, NPE9: {=bool:?}, NPE10: {=bool:?}, NPE11: {=bool:?}, NPE12: {=bool:?}, NPE13: {=bool:?}, NPE14: {=bool:?}, NPE15: {=bool:?}, NPE16: {=bool:?}, NPE17: {=bool:?}, NPE18: {=bool:?}, NPE19: {=bool:?}, NPE20: {=bool:?}, NPE21: {=bool:?}, NPE22: {=bool:?}, NPE23: {=bool:?}, NPE24: {=bool:?}, NPE25: {=bool:?}, NPE26: {=bool:?}, NPE27: {=bool:?}, NPE28: {=bool:?}, NPE29: {=bool:?}, NPE30: {=bool:?}, NPE31: {=bool:?} }}",
            self.NPE0(),
            self.NPE1(),
            self.NPE2(),
            self.NPE3(),
            self.NPE4(),
            self.NPE5(),
            self.NPE6(),
            self.NPE7(),
            self.NPE8(),
            self.NPE9(),
            self.NPE10(),
            self.NPE11(),
            self.NPE12(),
            self.NPE13(),
            self.NPE14(),
            self.NPE15(),
            self.NPE16(),
            self.NPE17(),
            self.NPE18(),
            self.NPE19(),
            self.NPE20(),
            self.NPE21(),
            self.NPE22(),
            self.NPE23(),
            self.NPE24(),
            self.NPE25(),
            self.NPE26(),
            self.NPE27(),
            self.NPE28(),
            self.NPE29(),
            self.NPE30(),
            self.NPE31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PCNS(pub u32);
impl RGPIO_PCNS {
    #[must_use]
    #[inline(always)]
    pub const fn NSE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSE31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSE31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PCNS {
    #[inline(always)]
    fn default() -> RGPIO_PCNS {
        RGPIO_PCNS(0)
    }
}
impl core::fmt::Debug for RGPIO_PCNS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PCNS")
            .field("NSE0", &self.NSE0())
            .field("NSE1", &self.NSE1())
            .field("NSE2", &self.NSE2())
            .field("NSE3", &self.NSE3())
            .field("NSE4", &self.NSE4())
            .field("NSE5", &self.NSE5())
            .field("NSE6", &self.NSE6())
            .field("NSE7", &self.NSE7())
            .field("NSE8", &self.NSE8())
            .field("NSE9", &self.NSE9())
            .field("NSE10", &self.NSE10())
            .field("NSE11", &self.NSE11())
            .field("NSE12", &self.NSE12())
            .field("NSE13", &self.NSE13())
            .field("NSE14", &self.NSE14())
            .field("NSE15", &self.NSE15())
            .field("NSE16", &self.NSE16())
            .field("NSE17", &self.NSE17())
            .field("NSE18", &self.NSE18())
            .field("NSE19", &self.NSE19())
            .field("NSE20", &self.NSE20())
            .field("NSE21", &self.NSE21())
            .field("NSE22", &self.NSE22())
            .field("NSE23", &self.NSE23())
            .field("NSE24", &self.NSE24())
            .field("NSE25", &self.NSE25())
            .field("NSE26", &self.NSE26())
            .field("NSE27", &self.NSE27())
            .field("NSE28", &self.NSE28())
            .field("NSE29", &self.NSE29())
            .field("NSE30", &self.NSE30())
            .field("NSE31", &self.NSE31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PCNS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PCNS {{ NSE0: {=bool:?}, NSE1: {=bool:?}, NSE2: {=bool:?}, NSE3: {=bool:?}, NSE4: {=bool:?}, NSE5: {=bool:?}, NSE6: {=bool:?}, NSE7: {=bool:?}, NSE8: {=bool:?}, NSE9: {=bool:?}, NSE10: {=bool:?}, NSE11: {=bool:?}, NSE12: {=bool:?}, NSE13: {=bool:?}, NSE14: {=bool:?}, NSE15: {=bool:?}, NSE16: {=bool:?}, NSE17: {=bool:?}, NSE18: {=bool:?}, NSE19: {=bool:?}, NSE20: {=bool:?}, NSE21: {=bool:?}, NSE22: {=bool:?}, NSE23: {=bool:?}, NSE24: {=bool:?}, NSE25: {=bool:?}, NSE26: {=bool:?}, NSE27: {=bool:?}, NSE28: {=bool:?}, NSE29: {=bool:?}, NSE30: {=bool:?}, NSE31: {=bool:?} }}",
            self.NSE0(),
            self.NSE1(),
            self.NSE2(),
            self.NSE3(),
            self.NSE4(),
            self.NSE5(),
            self.NSE6(),
            self.NSE7(),
            self.NSE8(),
            self.NSE9(),
            self.NSE10(),
            self.NSE11(),
            self.NSE12(),
            self.NSE13(),
            self.NSE14(),
            self.NSE15(),
            self.NSE16(),
            self.NSE17(),
            self.NSE18(),
            self.NSE19(),
            self.NSE20(),
            self.NSE21(),
            self.NSE22(),
            self.NSE23(),
            self.NSE24(),
            self.NSE25(),
            self.NSE26(),
            self.NSE27(),
            self.NSE28(),
            self.NSE29(),
            self.NSE30(),
            self.NSE31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PCOR(pub u32);
impl RGPIO_PCOR {
    #[must_use]
    #[inline(always)]
    pub const fn PTCO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTCO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTCO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PCOR {
    #[inline(always)]
    fn default() -> RGPIO_PCOR {
        RGPIO_PCOR(0)
    }
}
impl core::fmt::Debug for RGPIO_PCOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PCOR")
            .field("PTCO0", &self.PTCO0())
            .field("PTCO1", &self.PTCO1())
            .field("PTCO2", &self.PTCO2())
            .field("PTCO3", &self.PTCO3())
            .field("PTCO4", &self.PTCO4())
            .field("PTCO5", &self.PTCO5())
            .field("PTCO6", &self.PTCO6())
            .field("PTCO7", &self.PTCO7())
            .field("PTCO8", &self.PTCO8())
            .field("PTCO9", &self.PTCO9())
            .field("PTCO10", &self.PTCO10())
            .field("PTCO11", &self.PTCO11())
            .field("PTCO12", &self.PTCO12())
            .field("PTCO13", &self.PTCO13())
            .field("PTCO14", &self.PTCO14())
            .field("PTCO15", &self.PTCO15())
            .field("PTCO16", &self.PTCO16())
            .field("PTCO17", &self.PTCO17())
            .field("PTCO18", &self.PTCO18())
            .field("PTCO19", &self.PTCO19())
            .field("PTCO20", &self.PTCO20())
            .field("PTCO21", &self.PTCO21())
            .field("PTCO22", &self.PTCO22())
            .field("PTCO23", &self.PTCO23())
            .field("PTCO24", &self.PTCO24())
            .field("PTCO25", &self.PTCO25())
            .field("PTCO26", &self.PTCO26())
            .field("PTCO27", &self.PTCO27())
            .field("PTCO28", &self.PTCO28())
            .field("PTCO29", &self.PTCO29())
            .field("PTCO30", &self.PTCO30())
            .field("PTCO31", &self.PTCO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PCOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PCOR {{ PTCO0: {=bool:?}, PTCO1: {=bool:?}, PTCO2: {=bool:?}, PTCO3: {=bool:?}, PTCO4: {=bool:?}, PTCO5: {=bool:?}, PTCO6: {=bool:?}, PTCO7: {=bool:?}, PTCO8: {=bool:?}, PTCO9: {=bool:?}, PTCO10: {=bool:?}, PTCO11: {=bool:?}, PTCO12: {=bool:?}, PTCO13: {=bool:?}, PTCO14: {=bool:?}, PTCO15: {=bool:?}, PTCO16: {=bool:?}, PTCO17: {=bool:?}, PTCO18: {=bool:?}, PTCO19: {=bool:?}, PTCO20: {=bool:?}, PTCO21: {=bool:?}, PTCO22: {=bool:?}, PTCO23: {=bool:?}, PTCO24: {=bool:?}, PTCO25: {=bool:?}, PTCO26: {=bool:?}, PTCO27: {=bool:?}, PTCO28: {=bool:?}, PTCO29: {=bool:?}, PTCO30: {=bool:?}, PTCO31: {=bool:?} }}",
            self.PTCO0(),
            self.PTCO1(),
            self.PTCO2(),
            self.PTCO3(),
            self.PTCO4(),
            self.PTCO5(),
            self.PTCO6(),
            self.PTCO7(),
            self.PTCO8(),
            self.PTCO9(),
            self.PTCO10(),
            self.PTCO11(),
            self.PTCO12(),
            self.PTCO13(),
            self.PTCO14(),
            self.PTCO15(),
            self.PTCO16(),
            self.PTCO17(),
            self.PTCO18(),
            self.PTCO19(),
            self.PTCO20(),
            self.PTCO21(),
            self.PTCO22(),
            self.PTCO23(),
            self.PTCO24(),
            self.PTCO25(),
            self.PTCO26(),
            self.PTCO27(),
            self.PTCO28(),
            self.PTCO29(),
            self.PTCO30(),
            self.PTCO31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PDDR(pub u32);
impl RGPIO_PDDR {
    #[must_use]
    #[inline(always)]
    pub const fn PDD0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDD31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDD31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PDDR {
    #[inline(always)]
    fn default() -> RGPIO_PDDR {
        RGPIO_PDDR(0)
    }
}
impl core::fmt::Debug for RGPIO_PDDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PDDR")
            .field("PDD0", &self.PDD0())
            .field("PDD1", &self.PDD1())
            .field("PDD2", &self.PDD2())
            .field("PDD3", &self.PDD3())
            .field("PDD4", &self.PDD4())
            .field("PDD5", &self.PDD5())
            .field("PDD6", &self.PDD6())
            .field("PDD7", &self.PDD7())
            .field("PDD8", &self.PDD8())
            .field("PDD9", &self.PDD9())
            .field("PDD10", &self.PDD10())
            .field("PDD11", &self.PDD11())
            .field("PDD12", &self.PDD12())
            .field("PDD13", &self.PDD13())
            .field("PDD14", &self.PDD14())
            .field("PDD15", &self.PDD15())
            .field("PDD16", &self.PDD16())
            .field("PDD17", &self.PDD17())
            .field("PDD18", &self.PDD18())
            .field("PDD19", &self.PDD19())
            .field("PDD20", &self.PDD20())
            .field("PDD21", &self.PDD21())
            .field("PDD22", &self.PDD22())
            .field("PDD23", &self.PDD23())
            .field("PDD24", &self.PDD24())
            .field("PDD25", &self.PDD25())
            .field("PDD26", &self.PDD26())
            .field("PDD27", &self.PDD27())
            .field("PDD28", &self.PDD28())
            .field("PDD29", &self.PDD29())
            .field("PDD30", &self.PDD30())
            .field("PDD31", &self.PDD31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PDDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PDDR {{ PDD0: {=bool:?}, PDD1: {=bool:?}, PDD2: {=bool:?}, PDD3: {=bool:?}, PDD4: {=bool:?}, PDD5: {=bool:?}, PDD6: {=bool:?}, PDD7: {=bool:?}, PDD8: {=bool:?}, PDD9: {=bool:?}, PDD10: {=bool:?}, PDD11: {=bool:?}, PDD12: {=bool:?}, PDD13: {=bool:?}, PDD14: {=bool:?}, PDD15: {=bool:?}, PDD16: {=bool:?}, PDD17: {=bool:?}, PDD18: {=bool:?}, PDD19: {=bool:?}, PDD20: {=bool:?}, PDD21: {=bool:?}, PDD22: {=bool:?}, PDD23: {=bool:?}, PDD24: {=bool:?}, PDD25: {=bool:?}, PDD26: {=bool:?}, PDD27: {=bool:?}, PDD28: {=bool:?}, PDD29: {=bool:?}, PDD30: {=bool:?}, PDD31: {=bool:?} }}",
            self.PDD0(),
            self.PDD1(),
            self.PDD2(),
            self.PDD3(),
            self.PDD4(),
            self.PDD5(),
            self.PDD6(),
            self.PDD7(),
            self.PDD8(),
            self.PDD9(),
            self.PDD10(),
            self.PDD11(),
            self.PDD12(),
            self.PDD13(),
            self.PDD14(),
            self.PDD15(),
            self.PDD16(),
            self.PDD17(),
            self.PDD18(),
            self.PDD19(),
            self.PDD20(),
            self.PDD21(),
            self.PDD22(),
            self.PDD23(),
            self.PDD24(),
            self.PDD25(),
            self.PDD26(),
            self.PDD27(),
            self.PDD28(),
            self.PDD29(),
            self.PDD30(),
            self.PDD31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PDIR(pub u32);
impl RGPIO_PDIR {
    #[must_use]
    #[inline(always)]
    pub const fn PDI0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDI31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDI31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PDIR {
    #[inline(always)]
    fn default() -> RGPIO_PDIR {
        RGPIO_PDIR(0)
    }
}
impl core::fmt::Debug for RGPIO_PDIR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PDIR")
            .field("PDI0", &self.PDI0())
            .field("PDI1", &self.PDI1())
            .field("PDI2", &self.PDI2())
            .field("PDI3", &self.PDI3())
            .field("PDI4", &self.PDI4())
            .field("PDI5", &self.PDI5())
            .field("PDI6", &self.PDI6())
            .field("PDI7", &self.PDI7())
            .field("PDI8", &self.PDI8())
            .field("PDI9", &self.PDI9())
            .field("PDI10", &self.PDI10())
            .field("PDI11", &self.PDI11())
            .field("PDI12", &self.PDI12())
            .field("PDI13", &self.PDI13())
            .field("PDI14", &self.PDI14())
            .field("PDI15", &self.PDI15())
            .field("PDI16", &self.PDI16())
            .field("PDI17", &self.PDI17())
            .field("PDI18", &self.PDI18())
            .field("PDI19", &self.PDI19())
            .field("PDI20", &self.PDI20())
            .field("PDI21", &self.PDI21())
            .field("PDI22", &self.PDI22())
            .field("PDI23", &self.PDI23())
            .field("PDI24", &self.PDI24())
            .field("PDI25", &self.PDI25())
            .field("PDI26", &self.PDI26())
            .field("PDI27", &self.PDI27())
            .field("PDI28", &self.PDI28())
            .field("PDI29", &self.PDI29())
            .field("PDI30", &self.PDI30())
            .field("PDI31", &self.PDI31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PDIR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PDIR {{ PDI0: {=bool:?}, PDI1: {=bool:?}, PDI2: {=bool:?}, PDI3: {=bool:?}, PDI4: {=bool:?}, PDI5: {=bool:?}, PDI6: {=bool:?}, PDI7: {=bool:?}, PDI8: {=bool:?}, PDI9: {=bool:?}, PDI10: {=bool:?}, PDI11: {=bool:?}, PDI12: {=bool:?}, PDI13: {=bool:?}, PDI14: {=bool:?}, PDI15: {=bool:?}, PDI16: {=bool:?}, PDI17: {=bool:?}, PDI18: {=bool:?}, PDI19: {=bool:?}, PDI20: {=bool:?}, PDI21: {=bool:?}, PDI22: {=bool:?}, PDI23: {=bool:?}, PDI24: {=bool:?}, PDI25: {=bool:?}, PDI26: {=bool:?}, PDI27: {=bool:?}, PDI28: {=bool:?}, PDI29: {=bool:?}, PDI30: {=bool:?}, PDI31: {=bool:?} }}",
            self.PDI0(),
            self.PDI1(),
            self.PDI2(),
            self.PDI3(),
            self.PDI4(),
            self.PDI5(),
            self.PDI6(),
            self.PDI7(),
            self.PDI8(),
            self.PDI9(),
            self.PDI10(),
            self.PDI11(),
            self.PDI12(),
            self.PDI13(),
            self.PDI14(),
            self.PDI15(),
            self.PDI16(),
            self.PDI17(),
            self.PDI18(),
            self.PDI19(),
            self.PDI20(),
            self.PDI21(),
            self.PDI22(),
            self.PDI23(),
            self.PDI24(),
            self.PDI25(),
            self.PDI26(),
            self.PDI27(),
            self.PDI28(),
            self.PDI29(),
            self.PDI30(),
            self.PDI31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PDOR(pub u32);
impl RGPIO_PDOR {
    #[must_use]
    #[inline(always)]
    pub const fn PDO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PDO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PDO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PDOR {
    #[inline(always)]
    fn default() -> RGPIO_PDOR {
        RGPIO_PDOR(0)
    }
}
impl core::fmt::Debug for RGPIO_PDOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PDOR")
            .field("PDO0", &self.PDO0())
            .field("PDO1", &self.PDO1())
            .field("PDO2", &self.PDO2())
            .field("PDO3", &self.PDO3())
            .field("PDO4", &self.PDO4())
            .field("PDO5", &self.PDO5())
            .field("PDO6", &self.PDO6())
            .field("PDO7", &self.PDO7())
            .field("PDO8", &self.PDO8())
            .field("PDO9", &self.PDO9())
            .field("PDO10", &self.PDO10())
            .field("PDO11", &self.PDO11())
            .field("PDO12", &self.PDO12())
            .field("PDO13", &self.PDO13())
            .field("PDO14", &self.PDO14())
            .field("PDO15", &self.PDO15())
            .field("PDO16", &self.PDO16())
            .field("PDO17", &self.PDO17())
            .field("PDO18", &self.PDO18())
            .field("PDO19", &self.PDO19())
            .field("PDO20", &self.PDO20())
            .field("PDO21", &self.PDO21())
            .field("PDO22", &self.PDO22())
            .field("PDO23", &self.PDO23())
            .field("PDO24", &self.PDO24())
            .field("PDO25", &self.PDO25())
            .field("PDO26", &self.PDO26())
            .field("PDO27", &self.PDO27())
            .field("PDO28", &self.PDO28())
            .field("PDO29", &self.PDO29())
            .field("PDO30", &self.PDO30())
            .field("PDO31", &self.PDO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PDOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PDOR {{ PDO0: {=bool:?}, PDO1: {=bool:?}, PDO2: {=bool:?}, PDO3: {=bool:?}, PDO4: {=bool:?}, PDO5: {=bool:?}, PDO6: {=bool:?}, PDO7: {=bool:?}, PDO8: {=bool:?}, PDO9: {=bool:?}, PDO10: {=bool:?}, PDO11: {=bool:?}, PDO12: {=bool:?}, PDO13: {=bool:?}, PDO14: {=bool:?}, PDO15: {=bool:?}, PDO16: {=bool:?}, PDO17: {=bool:?}, PDO18: {=bool:?}, PDO19: {=bool:?}, PDO20: {=bool:?}, PDO21: {=bool:?}, PDO22: {=bool:?}, PDO23: {=bool:?}, PDO24: {=bool:?}, PDO25: {=bool:?}, PDO26: {=bool:?}, PDO27: {=bool:?}, PDO28: {=bool:?}, PDO29: {=bool:?}, PDO30: {=bool:?}, PDO31: {=bool:?} }}",
            self.PDO0(),
            self.PDO1(),
            self.PDO2(),
            self.PDO3(),
            self.PDO4(),
            self.PDO5(),
            self.PDO6(),
            self.PDO7(),
            self.PDO8(),
            self.PDO9(),
            self.PDO10(),
            self.PDO11(),
            self.PDO12(),
            self.PDO13(),
            self.PDO14(),
            self.PDO15(),
            self.PDO16(),
            self.PDO17(),
            self.PDO18(),
            self.PDO19(),
            self.PDO20(),
            self.PDO21(),
            self.PDO22(),
            self.PDO23(),
            self.PDO24(),
            self.PDO25(),
            self.PDO26(),
            self.PDO27(),
            self.PDO28(),
            self.PDO29(),
            self.PDO30(),
            self.PDO31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PDR(pub u32);
impl RGPIO_PDR {
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RGPIO_PDR {
    #[inline(always)]
    fn default() -> RGPIO_PDR {
        RGPIO_PDR(0)
    }
}
impl core::fmt::Debug for RGPIO_PDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PDR").field("PD", &self.PD()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RGPIO_PDR {{ PD: {=bool:?} }}", self.PD())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PIDR(pub u32);
impl RGPIO_PIDR {
    #[must_use]
    #[inline(always)]
    pub const fn PID0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PID31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PID31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PIDR {
    #[inline(always)]
    fn default() -> RGPIO_PIDR {
        RGPIO_PIDR(0)
    }
}
impl core::fmt::Debug for RGPIO_PIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PIDR")
            .field("PID0", &self.PID0())
            .field("PID1", &self.PID1())
            .field("PID2", &self.PID2())
            .field("PID3", &self.PID3())
            .field("PID4", &self.PID4())
            .field("PID5", &self.PID5())
            .field("PID6", &self.PID6())
            .field("PID7", &self.PID7())
            .field("PID8", &self.PID8())
            .field("PID9", &self.PID9())
            .field("PID10", &self.PID10())
            .field("PID11", &self.PID11())
            .field("PID12", &self.PID12())
            .field("PID13", &self.PID13())
            .field("PID14", &self.PID14())
            .field("PID15", &self.PID15())
            .field("PID16", &self.PID16())
            .field("PID17", &self.PID17())
            .field("PID18", &self.PID18())
            .field("PID19", &self.PID19())
            .field("PID20", &self.PID20())
            .field("PID21", &self.PID21())
            .field("PID22", &self.PID22())
            .field("PID23", &self.PID23())
            .field("PID24", &self.PID24())
            .field("PID25", &self.PID25())
            .field("PID26", &self.PID26())
            .field("PID27", &self.PID27())
            .field("PID28", &self.PID28())
            .field("PID29", &self.PID29())
            .field("PID30", &self.PID30())
            .field("PID31", &self.PID31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PIDR {{ PID0: {=bool:?}, PID1: {=bool:?}, PID2: {=bool:?}, PID3: {=bool:?}, PID4: {=bool:?}, PID5: {=bool:?}, PID6: {=bool:?}, PID7: {=bool:?}, PID8: {=bool:?}, PID9: {=bool:?}, PID10: {=bool:?}, PID11: {=bool:?}, PID12: {=bool:?}, PID13: {=bool:?}, PID14: {=bool:?}, PID15: {=bool:?}, PID16: {=bool:?}, PID17: {=bool:?}, PID18: {=bool:?}, PID19: {=bool:?}, PID20: {=bool:?}, PID21: {=bool:?}, PID22: {=bool:?}, PID23: {=bool:?}, PID24: {=bool:?}, PID25: {=bool:?}, PID26: {=bool:?}, PID27: {=bool:?}, PID28: {=bool:?}, PID29: {=bool:?}, PID30: {=bool:?}, PID31: {=bool:?} }}",
            self.PID0(),
            self.PID1(),
            self.PID2(),
            self.PID3(),
            self.PID4(),
            self.PID5(),
            self.PID6(),
            self.PID7(),
            self.PID8(),
            self.PID9(),
            self.PID10(),
            self.PID11(),
            self.PID12(),
            self.PID13(),
            self.PID14(),
            self.PID15(),
            self.PID16(),
            self.PID17(),
            self.PID18(),
            self.PID19(),
            self.PID20(),
            self.PID21(),
            self.PID22(),
            self.PID23(),
            self.PID24(),
            self.PID25(),
            self.PID26(),
            self.PID27(),
            self.PID28(),
            self.PID29(),
            self.PID30(),
            self.PID31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PSOR(pub u32);
impl RGPIO_PSOR {
    #[must_use]
    #[inline(always)]
    pub const fn PTSO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTSO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTSO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PSOR {
    #[inline(always)]
    fn default() -> RGPIO_PSOR {
        RGPIO_PSOR(0)
    }
}
impl core::fmt::Debug for RGPIO_PSOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PSOR")
            .field("PTSO0", &self.PTSO0())
            .field("PTSO1", &self.PTSO1())
            .field("PTSO2", &self.PTSO2())
            .field("PTSO3", &self.PTSO3())
            .field("PTSO4", &self.PTSO4())
            .field("PTSO5", &self.PTSO5())
            .field("PTSO6", &self.PTSO6())
            .field("PTSO7", &self.PTSO7())
            .field("PTSO8", &self.PTSO8())
            .field("PTSO9", &self.PTSO9())
            .field("PTSO10", &self.PTSO10())
            .field("PTSO11", &self.PTSO11())
            .field("PTSO12", &self.PTSO12())
            .field("PTSO13", &self.PTSO13())
            .field("PTSO14", &self.PTSO14())
            .field("PTSO15", &self.PTSO15())
            .field("PTSO16", &self.PTSO16())
            .field("PTSO17", &self.PTSO17())
            .field("PTSO18", &self.PTSO18())
            .field("PTSO19", &self.PTSO19())
            .field("PTSO20", &self.PTSO20())
            .field("PTSO21", &self.PTSO21())
            .field("PTSO22", &self.PTSO22())
            .field("PTSO23", &self.PTSO23())
            .field("PTSO24", &self.PTSO24())
            .field("PTSO25", &self.PTSO25())
            .field("PTSO26", &self.PTSO26())
            .field("PTSO27", &self.PTSO27())
            .field("PTSO28", &self.PTSO28())
            .field("PTSO29", &self.PTSO29())
            .field("PTSO30", &self.PTSO30())
            .field("PTSO31", &self.PTSO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PSOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PSOR {{ PTSO0: {=bool:?}, PTSO1: {=bool:?}, PTSO2: {=bool:?}, PTSO3: {=bool:?}, PTSO4: {=bool:?}, PTSO5: {=bool:?}, PTSO6: {=bool:?}, PTSO7: {=bool:?}, PTSO8: {=bool:?}, PTSO9: {=bool:?}, PTSO10: {=bool:?}, PTSO11: {=bool:?}, PTSO12: {=bool:?}, PTSO13: {=bool:?}, PTSO14: {=bool:?}, PTSO15: {=bool:?}, PTSO16: {=bool:?}, PTSO17: {=bool:?}, PTSO18: {=bool:?}, PTSO19: {=bool:?}, PTSO20: {=bool:?}, PTSO21: {=bool:?}, PTSO22: {=bool:?}, PTSO23: {=bool:?}, PTSO24: {=bool:?}, PTSO25: {=bool:?}, PTSO26: {=bool:?}, PTSO27: {=bool:?}, PTSO28: {=bool:?}, PTSO29: {=bool:?}, PTSO30: {=bool:?}, PTSO31: {=bool:?} }}",
            self.PTSO0(),
            self.PTSO1(),
            self.PTSO2(),
            self.PTSO3(),
            self.PTSO4(),
            self.PTSO5(),
            self.PTSO6(),
            self.PTSO7(),
            self.PTSO8(),
            self.PTSO9(),
            self.PTSO10(),
            self.PTSO11(),
            self.PTSO12(),
            self.PTSO13(),
            self.PTSO14(),
            self.PTSO15(),
            self.PTSO16(),
            self.PTSO17(),
            self.PTSO18(),
            self.PTSO19(),
            self.PTSO20(),
            self.PTSO21(),
            self.PTSO22(),
            self.PTSO23(),
            self.PTSO24(),
            self.PTSO25(),
            self.PTSO26(),
            self.PTSO27(),
            self.PTSO28(),
            self.PTSO29(),
            self.PTSO30(),
            self.PTSO31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_PTOR(pub u32);
impl RGPIO_PTOR {
    #[must_use]
    #[inline(always)]
    pub const fn PTTO0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PTTO31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PTTO31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RGPIO_PTOR {
    #[inline(always)]
    fn default() -> RGPIO_PTOR {
        RGPIO_PTOR(0)
    }
}
impl core::fmt::Debug for RGPIO_PTOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_PTOR")
            .field("PTTO0", &self.PTTO0())
            .field("PTTO1", &self.PTTO1())
            .field("PTTO2", &self.PTTO2())
            .field("PTTO3", &self.PTTO3())
            .field("PTTO4", &self.PTTO4())
            .field("PTTO5", &self.PTTO5())
            .field("PTTO6", &self.PTTO6())
            .field("PTTO7", &self.PTTO7())
            .field("PTTO8", &self.PTTO8())
            .field("PTTO9", &self.PTTO9())
            .field("PTTO10", &self.PTTO10())
            .field("PTTO11", &self.PTTO11())
            .field("PTTO12", &self.PTTO12())
            .field("PTTO13", &self.PTTO13())
            .field("PTTO14", &self.PTTO14())
            .field("PTTO15", &self.PTTO15())
            .field("PTTO16", &self.PTTO16())
            .field("PTTO17", &self.PTTO17())
            .field("PTTO18", &self.PTTO18())
            .field("PTTO19", &self.PTTO19())
            .field("PTTO20", &self.PTTO20())
            .field("PTTO21", &self.PTTO21())
            .field("PTTO22", &self.PTTO22())
            .field("PTTO23", &self.PTTO23())
            .field("PTTO24", &self.PTTO24())
            .field("PTTO25", &self.PTTO25())
            .field("PTTO26", &self.PTTO26())
            .field("PTTO27", &self.PTTO27())
            .field("PTTO28", &self.PTTO28())
            .field("PTTO29", &self.PTTO29())
            .field("PTTO30", &self.PTTO30())
            .field("PTTO31", &self.PTTO31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_PTOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_PTOR {{ PTTO0: {=bool:?}, PTTO1: {=bool:?}, PTTO2: {=bool:?}, PTTO3: {=bool:?}, PTTO4: {=bool:?}, PTTO5: {=bool:?}, PTTO6: {=bool:?}, PTTO7: {=bool:?}, PTTO8: {=bool:?}, PTTO9: {=bool:?}, PTTO10: {=bool:?}, PTTO11: {=bool:?}, PTTO12: {=bool:?}, PTTO13: {=bool:?}, PTTO14: {=bool:?}, PTTO15: {=bool:?}, PTTO16: {=bool:?}, PTTO17: {=bool:?}, PTTO18: {=bool:?}, PTTO19: {=bool:?}, PTTO20: {=bool:?}, PTTO21: {=bool:?}, PTTO22: {=bool:?}, PTTO23: {=bool:?}, PTTO24: {=bool:?}, PTTO25: {=bool:?}, PTTO26: {=bool:?}, PTTO27: {=bool:?}, PTTO28: {=bool:?}, PTTO29: {=bool:?}, PTTO30: {=bool:?}, PTTO31: {=bool:?} }}",
            self.PTTO0(),
            self.PTTO1(),
            self.PTTO2(),
            self.PTTO3(),
            self.PTTO4(),
            self.PTTO5(),
            self.PTTO6(),
            self.PTTO7(),
            self.PTTO8(),
            self.PTTO9(),
            self.PTTO10(),
            self.PTTO11(),
            self.PTTO12(),
            self.PTTO13(),
            self.PTTO14(),
            self.PTTO15(),
            self.PTTO16(),
            self.PTTO17(),
            self.PTTO18(),
            self.PTTO19(),
            self.PTTO20(),
            self.PTTO21(),
            self.PTTO22(),
            self.PTTO23(),
            self.PTTO24(),
            self.PTTO25(),
            self.PTTO26(),
            self.PTTO27(),
            self.PTTO28(),
            self.PTTO29(),
            self.PTTO30(),
            self.PTTO31()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RGPIO_VERID(pub u32);
impl RGPIO_VERID {
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
impl Default for RGPIO_VERID {
    #[inline(always)]
    fn default() -> RGPIO_VERID {
        RGPIO_VERID(0)
    }
}
impl core::fmt::Debug for RGPIO_VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGPIO_VERID")
            .field("FEATURE", &self.FEATURE())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RGPIO_VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RGPIO_VERID {{ FEATURE: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.FEATURE(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}

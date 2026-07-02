#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C {
    ptr: *mut u8,
}
unsafe impl Send for I3C {}
unsafe impl Sync for I3C {}
impl I3C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCONFIG(self) -> crate::common::Reg<I3C_MCONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn SCONFIG(self) -> crate::common::Reg<I3C_SCONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SSTATUS(self) -> crate::common::Reg<I3C_SSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn SCTRL(self) -> crate::common::Reg<I3C_SCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SINTSET(self) -> crate::common::Reg<I3C_SINTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn SINTCLR(self) -> crate::common::Reg<I3C_SINTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn SINTMASKED(self) -> crate::common::Reg<I3C_SINTMASKED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn SERRWARN(self) -> crate::common::Reg<I3C_SERRWARN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn SDMACTRL(self) -> crate::common::Reg<I3C_SDMACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn SDATACTRL(self) -> crate::common::Reg<I3C_SDATACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAB(self) -> crate::common::Reg<I3C_SWDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATABE(self) -> crate::common::Reg<I3C_SWDATABE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAH(self) -> crate::common::Reg<I3C_SWDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAHE(self) -> crate::common::Reg<I3C_SWDATAHE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn SRDATAB(self) -> crate::common::Reg<I3C_SRDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn SRDATAH(self) -> crate::common::Reg<I3C_SRDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAB1(self) -> crate::common::Reg<I3C_SWDATAB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn SWDATAH1(self) -> crate::common::Reg<I3C_SWDATAH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn SCAPABILITIES2(self) -> crate::common::Reg<I3C_SCAPABILITIES2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn SCAPABILITIES(self) -> crate::common::Reg<I3C_SCAPABILITIES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn SMAXLIMITS(self) -> crate::common::Reg<I3C_SMAXLIMITS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn SIDPARTNO(self) -> crate::common::Reg<I3C_SIDPARTNO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn SIDEXT(self) -> crate::common::Reg<I3C_SIDEXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn SVENDORID(self) -> crate::common::Reg<I3C_SVENDORID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn STCCLOCK(self) -> crate::common::Reg<I3C_STCCLOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn SMSGMAPADDR(self) -> crate::common::Reg<I3C_SMSGMAPADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn MCONFIG_EXT(self) -> crate::common::Reg<I3C_MCONFIG_EXT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn MCTRL(self) -> crate::common::Reg<I3C_MCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn MSTATUS(self) -> crate::common::Reg<I3C_MSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn MIBIRULES(self) -> crate::common::Reg<I3C_MIBIRULES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn MINTSET(self) -> crate::common::Reg<I3C_MINTSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn MINTCLR(self) -> crate::common::Reg<I3C_MINTCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn MINTMASKED(self) -> crate::common::Reg<I3C_MINTMASKED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn MERRWARN(self) -> crate::common::Reg<I3C_MERRWARN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn MDMACTRL(self) -> crate::common::Reg<I3C_MDMACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn MDATACTRL(self) -> crate::common::Reg<I3C_MDATACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAB(self) -> crate::common::Reg<I3C_MWDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATABE(self) -> crate::common::Reg<I3C_MWDATABE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAH(self) -> crate::common::Reg<I3C_MWDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAHE(self) -> crate::common::Reg<I3C_MWDATAHE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn MRDATAB(self) -> crate::common::Reg<I3C_MRDATAB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRDATAH(self) -> crate::common::Reg<I3C_MRDATAH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAB1(self) -> crate::common::Reg<I3C_MWDATAB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn MWDATAH1(self) -> crate::common::Reg<I3C_MWDATAH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_SDR_CONTROL(
        self,
    ) -> crate::common::Reg<I3C_MWMSG_SDR_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_SDR_DATA(self) -> crate::common::Reg<I3C_MWMSG_SDR_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MRMSG_SDR(self) -> crate::common::Reg<I3C_MRMSG_SDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_CONTROL(
        self,
    ) -> crate::common::Reg<I3C_MWMSG_DDR_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_CONTROL2(
        self,
    ) -> crate::common::Reg<I3C_MWMSG_DDR_CONTROL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MWMSG_DDR_DATA(self) -> crate::common::Reg<I3C_MWMSG_DDR_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MRMSG_DDR(self) -> crate::common::Reg<I3C_MRMSG_DDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn MDYNADDR(self) -> crate::common::Reg<I3C_MDYNADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRSTACTTIME(self) -> crate::common::Reg<I3C_SRSTACTTIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn SCCCMASK(self) -> crate::common::Reg<I3C_SCCCMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn SERRWARNMASK(self) -> crate::common::Reg<I3C_SERRWARNMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn SMAPCTRL0(self) -> crate::common::Reg<I3C_SMAPCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn SMAPCTRL1(self) -> crate::common::Reg<I3C_SMAPCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn IBIEXT1(self) -> crate::common::Reg<I3C_IBIEXT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn IBIEXT2(self) -> crate::common::Reg<I3C_IBIEXT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn SELFRESET(self) -> crate::common::Reg<I3C_SELFRESET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_IBIEXT1(pub u32);
impl I3C_IBIEXT1 {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn MAX(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_MAX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Maximum"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum"]
    #[inline(always)]
    pub const fn set_EXT1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 1"]
    #[inline(always)]
    pub const fn set_EXT2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 2"]
    #[inline(always)]
    pub const fn set_EXT3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_IBIEXT1 {
    #[inline(always)]
    fn default() -> I3C_IBIEXT1 {
        I3C_IBIEXT1(0)
    }
}
impl core::fmt::Debug for I3C_IBIEXT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_IBIEXT1")
            .field("CNT", &self.CNT())
            .field("MAX", &self.MAX())
            .field("EXT1", &self.EXT1())
            .field("EXT2", &self.EXT2())
            .field("EXT3", &self.EXT3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_IBIEXT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_IBIEXT1 {{ CNT: {=u8:?}, MAX: {=u8:?}, EXT1: {=u8:?}, EXT2: {=u8:?}, EXT3: {=u8:?} }}",
            self.CNT(),
            self.MAX(),
            self.EXT1(),
            self.EXT2(),
            self.EXT3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_IBIEXT2(pub u32);
impl I3C_IBIEXT2 {
    #[doc = "Extra Byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 4"]
    #[inline(always)]
    pub const fn set_EXT4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Extra Byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 4"]
    #[inline(always)]
    pub const fn set_EXT5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 5"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 5"]
    #[inline(always)]
    pub const fn set_EXT6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 6"]
    #[must_use]
    #[inline(always)]
    pub const fn EXT7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 6"]
    #[inline(always)]
    pub const fn set_EXT7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_IBIEXT2 {
    #[inline(always)]
    fn default() -> I3C_IBIEXT2 {
        I3C_IBIEXT2(0)
    }
}
impl core::fmt::Debug for I3C_IBIEXT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_IBIEXT2")
            .field("EXT4", &self.EXT4())
            .field("EXT5", &self.EXT5())
            .field("EXT6", &self.EXT6())
            .field("EXT7", &self.EXT7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_IBIEXT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_IBIEXT2 {{ EXT4: {=u8:?}, EXT5: {=u8:?}, EXT6: {=u8:?}, EXT7: {=u8:?} }}",
            self.EXT4(),
            self.EXT5(),
            self.EXT6(),
            self.EXT7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCONFIG(pub u32);
impl I3C_MCONFIG {
    #[must_use]
    #[inline(always)]
    pub const fn MSTENA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MSTENA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DISTO(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HKEEP(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_HKEEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ODSTOP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ODSTOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Push-Pull Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn PPBAUD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Baud Rate"]
    #[inline(always)]
    pub const fn set_PPBAUD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Push-Pull Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn PPLOW(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Baud Rate"]
    #[inline(always)]
    pub const fn set_PPLOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Push-Pull Low"]
    #[must_use]
    #[inline(always)]
    pub const fn ODBAUD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Push-Pull Low"]
    #[inline(always)]
    pub const fn set_ODBAUD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Open-drain Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn ODHPP(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Open-drain Baud Rate"]
    #[inline(always)]
    pub const fn set_ODHPP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Skew"]
    #[must_use]
    #[inline(always)]
    pub const fn SKEW(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Skew"]
    #[inline(always)]
    pub const fn set_SKEW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "Skew"]
    #[must_use]
    #[inline(always)]
    pub const fn I2CBAUD(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Skew"]
    #[inline(always)]
    pub const fn set_I2CBAUD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for I3C_MCONFIG {
    #[inline(always)]
    fn default() -> I3C_MCONFIG {
        I3C_MCONFIG(0)
    }
}
impl core::fmt::Debug for I3C_MCONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCONFIG")
            .field("MSTENA", &self.MSTENA())
            .field("DISTO", &self.DISTO())
            .field("HKEEP", &self.HKEEP())
            .field("ODSTOP", &self.ODSTOP())
            .field("PPBAUD", &self.PPBAUD())
            .field("PPLOW", &self.PPLOW())
            .field("ODBAUD", &self.ODBAUD())
            .field("ODHPP", &self.ODHPP())
            .field("SKEW", &self.SKEW())
            .field("I2CBAUD", &self.I2CBAUD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCONFIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MCONFIG {{ MSTENA: {=u8:?}, DISTO: {=bool:?}, HKEEP: {=u8:?}, ODSTOP: {=bool:?}, PPBAUD: {=u8:?}, PPLOW: {=u8:?}, ODBAUD: {=u8:?}, ODHPP: {=bool:?}, SKEW: {=u8:?}, I2CBAUD: {=u8:?} }}",
            self.MSTENA(),
            self.DISTO(),
            self.HKEEP(),
            self.ODSTOP(),
            self.PPBAUD(),
            self.PPLOW(),
            self.ODBAUD(),
            self.ODHPP(),
            self.SKEW(),
            self.I2CBAUD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCONFIG_EXT(pub u32);
impl I3C_MCONFIG_EXT {
    #[doc = "I2C Baud Low"]
    #[must_use]
    #[inline(always)]
    pub const fn I2CBLOW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "I2C Baud Low"]
    #[inline(always)]
    pub const fn set_I2CBLOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "I2C Baud Low"]
    #[must_use]
    #[inline(always)]
    pub const fn I2CHS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Baud Low"]
    #[inline(always)]
    pub const fn set_I2CHS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for I3C_MCONFIG_EXT {
    #[inline(always)]
    fn default() -> I3C_MCONFIG_EXT {
        I3C_MCONFIG_EXT(0)
    }
}
impl core::fmt::Debug for I3C_MCONFIG_EXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCONFIG_EXT")
            .field("I2CBLOW", &self.I2CBLOW())
            .field("I2CHS", &self.I2CHS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCONFIG_EXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MCONFIG_EXT {{ I2CBLOW: {=u8:?}, I2CHS: {=bool:?} }}",
            self.I2CBLOW(),
            self.I2CHS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCONFIG_EXT_I2C(pub u32);
impl I3C_MCONFIG_EXT_I2C {
    #[doc = "I2C HS"]
    #[must_use]
    #[inline(always)]
    pub const fn A10B(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "I2C HS"]
    #[inline(always)]
    pub const fn set_A10B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C_A10BEXT"]
    #[must_use]
    #[inline(always)]
    pub const fn A10BEXT(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "I2C_A10BEXT"]
    #[inline(always)]
    pub const fn set_A10BEXT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
}
impl Default for I3C_MCONFIG_EXT_I2C {
    #[inline(always)]
    fn default() -> I3C_MCONFIG_EXT_I2C {
        I3C_MCONFIG_EXT_I2C(0)
    }
}
impl core::fmt::Debug for I3C_MCONFIG_EXT_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCONFIG_EXT_I2C")
            .field("A10B", &self.A10B())
            .field("A10BEXT", &self.A10BEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCONFIG_EXT_I2C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MCONFIG_EXT_I2C {{ A10B: {=bool:?}, A10BEXT: {=u8:?} }}",
            self.A10B(),
            self.A10BEXT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCONFIG_EXT_I3C_CAS(pub u32);
impl I3C_MCONFIG_EXT_I3C_CAS {
    #[doc = "I2C_A10BEXT"]
    #[must_use]
    #[inline(always)]
    pub const fn DEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "I2C_A10BEXT"]
    #[inline(always)]
    pub const fn set_DEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for I3C_MCONFIG_EXT_I3C_CAS {
    #[inline(always)]
    fn default() -> I3C_MCONFIG_EXT_I3C_CAS {
        I3C_MCONFIG_EXT_I3C_CAS(0)
    }
}
impl core::fmt::Debug for I3C_MCONFIG_EXT_I3C_CAS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCONFIG_EXT_I3C_CAS")
            .field("DEL", &self.DEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCONFIG_EXT_I3C_CAS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MCONFIG_EXT_I3C_CAS {{ DEL: {=u8:?} }}", self.DEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCONFIG_EXT_I3C_CASR(pub u32);
impl I3C_MCONFIG_EXT_I3C_CASR {
    #[must_use]
    #[inline(always)]
    pub const fn DEL(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for I3C_MCONFIG_EXT_I3C_CASR {
    #[inline(always)]
    fn default() -> I3C_MCONFIG_EXT_I3C_CASR {
        I3C_MCONFIG_EXT_I3C_CASR(0)
    }
}
impl core::fmt::Debug for I3C_MCONFIG_EXT_I3C_CASR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCONFIG_EXT_I3C_CASR")
            .field("DEL", &self.DEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCONFIG_EXT_I3C_CASR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MCONFIG_EXT_I3C_CASR {{ DEL: {=u8:?} }}", self.DEL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MCTRL(pub u32);
impl I3C_MCTRL {
    #[must_use]
    #[inline(always)]
    pub const fn REQUEST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_REQUEST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIRESP(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IBIRESP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn RDTERM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_RDTERM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for I3C_MCTRL {
    #[inline(always)]
    fn default() -> I3C_MCTRL {
        I3C_MCTRL(0)
    }
}
impl core::fmt::Debug for I3C_MCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MCTRL")
            .field("REQUEST", &self.REQUEST())
            .field("TYPE", &self.TYPE())
            .field("IBIRESP", &self.IBIRESP())
            .field("DIR", &self.DIR())
            .field("ADDR", &self.ADDR())
            .field("RDTERM", &self.RDTERM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MCTRL {{ REQUEST: {=u8:?}, TYPE: {=u8:?}, IBIRESP: {=u8:?}, DIR: {=bool:?}, ADDR: {=u8:?}, RDTERM: {=u8:?} }}",
            self.REQUEST(),
            self.TYPE(),
            self.IBIRESP(),
            self.DIR(),
            self.ADDR(),
            self.RDTERM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MDATACTRL(pub u32);
impl I3C_MDATACTRL {
    #[must_use]
    #[inline(always)]
    pub const fn FLUSHTB(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLUSHTB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FLUSHFB(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLUSHFB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UNLOCK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UNLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXTRIG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TXTRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXTRIG(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXTRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Transmit Entry Count"]
    #[must_use]
    #[inline(always)]
    pub const fn TXCOUNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Entry Count"]
    #[inline(always)]
    pub const fn set_TXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transmit Entry Count"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCOUNT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Entry Count"]
    #[inline(always)]
    pub const fn set_RXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Receive Entry Count"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFULL(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Entry Count"]
    #[inline(always)]
    pub const fn set_TXFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_MDATACTRL {
    #[inline(always)]
    fn default() -> I3C_MDATACTRL {
        I3C_MDATACTRL(0)
    }
}
impl core::fmt::Debug for I3C_MDATACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MDATACTRL")
            .field("FLUSHTB", &self.FLUSHTB())
            .field("FLUSHFB", &self.FLUSHFB())
            .field("UNLOCK", &self.UNLOCK())
            .field("TXTRIG", &self.TXTRIG())
            .field("RXTRIG", &self.RXTRIG())
            .field("TXCOUNT", &self.TXCOUNT())
            .field("RXCOUNT", &self.RXCOUNT())
            .field("TXFULL", &self.TXFULL())
            .field("RXEMPTY", &self.RXEMPTY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MDATACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MDATACTRL {{ FLUSHTB: {=bool:?}, FLUSHFB: {=bool:?}, UNLOCK: {=bool:?}, TXTRIG: {=u8:?}, RXTRIG: {=u8:?}, TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?}, TXFULL: {=bool:?}, RXEMPTY: {=bool:?} }}",
            self.FLUSHTB(),
            self.FLUSHFB(),
            self.UNLOCK(),
            self.TXTRIG(),
            self.RXTRIG(),
            self.TXCOUNT(),
            self.RXCOUNT(),
            self.TXFULL(),
            self.RXEMPTY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MDMACTRL(pub u32);
impl I3C_MDMACTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DMAFB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMAFB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMATB(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMATB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMAWIDTH(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMAWIDTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for I3C_MDMACTRL {
    #[inline(always)]
    fn default() -> I3C_MDMACTRL {
        I3C_MDMACTRL(0)
    }
}
impl core::fmt::Debug for I3C_MDMACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MDMACTRL")
            .field("DMAFB", &self.DMAFB())
            .field("DMATB", &self.DMATB())
            .field("DMAWIDTH", &self.DMAWIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MDMACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MDMACTRL {{ DMAFB: {=u8:?}, DMATB: {=u8:?}, DMAWIDTH: {=u8:?} }}",
            self.DMAFB(),
            self.DMATB(),
            self.DMAWIDTH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MDYNADDR(pub u32);
impl I3C_MDYNADDR {
    #[must_use]
    #[inline(always)]
    pub const fn DAVALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DAVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn DADDR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_DADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for I3C_MDYNADDR {
    #[inline(always)]
    fn default() -> I3C_MDYNADDR {
        I3C_MDYNADDR(0)
    }
}
impl core::fmt::Debug for I3C_MDYNADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MDYNADDR")
            .field("DAVALID", &self.DAVALID())
            .field("DADDR", &self.DADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MDYNADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MDYNADDR {{ DAVALID: {=bool:?}, DADDR: {=u8:?} }}",
            self.DAVALID(),
            self.DADDR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MERRWARN(pub u32);
impl I3C_MERRWARN {
    #[must_use]
    #[inline(always)]
    pub const fn URUN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_URUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WRABT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WRABT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TERM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TERM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HPAR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HPAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HCRC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OREAD(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OWRITE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OWRITE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSGERR(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSGERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INVREQ(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INVREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for I3C_MERRWARN {
    #[inline(always)]
    fn default() -> I3C_MERRWARN {
        I3C_MERRWARN(0)
    }
}
impl core::fmt::Debug for I3C_MERRWARN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MERRWARN")
            .field("URUN", &self.URUN())
            .field("NACK", &self.NACK())
            .field("WRABT", &self.WRABT())
            .field("TERM", &self.TERM())
            .field("HPAR", &self.HPAR())
            .field("HCRC", &self.HCRC())
            .field("OREAD", &self.OREAD())
            .field("OWRITE", &self.OWRITE())
            .field("MSGERR", &self.MSGERR())
            .field("INVREQ", &self.INVREQ())
            .field("TIMEOUT", &self.TIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MERRWARN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MERRWARN {{ URUN: {=bool:?}, NACK: {=bool:?}, WRABT: {=bool:?}, TERM: {=bool:?}, HPAR: {=bool:?}, HCRC: {=bool:?}, OREAD: {=bool:?}, OWRITE: {=bool:?}, MSGERR: {=bool:?}, INVREQ: {=bool:?}, TIMEOUT: {=bool:?} }}",
            self.URUN(),
            self.NACK(),
            self.WRABT(),
            self.TERM(),
            self.HPAR(),
            self.HCRC(),
            self.OREAD(),
            self.OWRITE(),
            self.MSGERR(),
            self.INVREQ(),
            self.TIMEOUT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MIBIRULES(pub u32);
impl I3C_MIBIRULES {
    #[doc = "ADDR0"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR0"]
    #[inline(always)]
    pub const fn set_ADDR0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "ADDR0"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR0"]
    #[inline(always)]
    pub const fn set_ADDR1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "ADDR1"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR1"]
    #[inline(always)]
    pub const fn set_ADDR2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "ADDR2"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR2"]
    #[inline(always)]
    pub const fn set_ADDR3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "ADDR3"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR3"]
    #[inline(always)]
    pub const fn set_ADDR4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "ADDR4"]
    #[must_use]
    #[inline(always)]
    pub const fn MSB0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ADDR4"]
    #[inline(always)]
    pub const fn set_MSB0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOBYTE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOBYTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_MIBIRULES {
    #[inline(always)]
    fn default() -> I3C_MIBIRULES {
        I3C_MIBIRULES(0)
    }
}
impl core::fmt::Debug for I3C_MIBIRULES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MIBIRULES")
            .field("ADDR0", &self.ADDR0())
            .field("ADDR1", &self.ADDR1())
            .field("ADDR2", &self.ADDR2())
            .field("ADDR3", &self.ADDR3())
            .field("ADDR4", &self.ADDR4())
            .field("MSB0", &self.MSB0())
            .field("NOBYTE", &self.NOBYTE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MIBIRULES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MIBIRULES {{ ADDR0: {=u8:?}, ADDR1: {=u8:?}, ADDR2: {=u8:?}, ADDR3: {=u8:?}, ADDR4: {=u8:?}, MSB0: {=bool:?}, NOBYTE: {=bool:?} }}",
            self.ADDR0(),
            self.ADDR1(),
            self.ADDR2(),
            self.ADDR3(),
            self.ADDR4(),
            self.MSB0(),
            self.NOBYTE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MINTCLR(pub u32);
impl I3C_MINTCLR {
    #[must_use]
    #[inline(always)]
    pub const fn SLVSTART(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MCTRLDONE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MCTRLDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMPLETE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMPLETE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIWON(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIWON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOWMASTER(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOWMASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_MINTCLR {
    #[inline(always)]
    fn default() -> I3C_MINTCLR {
        I3C_MINTCLR(0)
    }
}
impl core::fmt::Debug for I3C_MINTCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MINTCLR")
            .field("SLVSTART", &self.SLVSTART())
            .field("MCTRLDONE", &self.MCTRLDONE())
            .field("COMPLETE", &self.COMPLETE())
            .field("RXPEND", &self.RXPEND())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("IBIWON", &self.IBIWON())
            .field("ERRWARN", &self.ERRWARN())
            .field("NOWMASTER", &self.NOWMASTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MINTCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MINTCLR {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}",
            self.SLVSTART(),
            self.MCTRLDONE(),
            self.COMPLETE(),
            self.RXPEND(),
            self.TXNOTFULL(),
            self.IBIWON(),
            self.ERRWARN(),
            self.NOWMASTER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MINTMASKED(pub u32);
impl I3C_MINTMASKED {
    #[must_use]
    #[inline(always)]
    pub const fn SLVSTART(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MCTRLDONE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MCTRLDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMPLETE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMPLETE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIWON(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIWON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOWMASTER(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOWMASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_MINTMASKED {
    #[inline(always)]
    fn default() -> I3C_MINTMASKED {
        I3C_MINTMASKED(0)
    }
}
impl core::fmt::Debug for I3C_MINTMASKED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MINTMASKED")
            .field("SLVSTART", &self.SLVSTART())
            .field("MCTRLDONE", &self.MCTRLDONE())
            .field("COMPLETE", &self.COMPLETE())
            .field("RXPEND", &self.RXPEND())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("IBIWON", &self.IBIWON())
            .field("ERRWARN", &self.ERRWARN())
            .field("NOWMASTER", &self.NOWMASTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MINTMASKED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MINTMASKED {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}",
            self.SLVSTART(),
            self.MCTRLDONE(),
            self.COMPLETE(),
            self.RXPEND(),
            self.TXNOTFULL(),
            self.IBIWON(),
            self.ERRWARN(),
            self.NOWMASTER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MINTSET(pub u32);
impl I3C_MINTSET {
    #[must_use]
    #[inline(always)]
    pub const fn SLVSTART(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MCTRLDONE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MCTRLDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMPLETE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMPLETE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Pending Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Pending Interrupt Enable"]
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIWON(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIWON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOWMASTER(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOWMASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_MINTSET {
    #[inline(always)]
    fn default() -> I3C_MINTSET {
        I3C_MINTSET(0)
    }
}
impl core::fmt::Debug for I3C_MINTSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MINTSET")
            .field("SLVSTART", &self.SLVSTART())
            .field("MCTRLDONE", &self.MCTRLDONE())
            .field("COMPLETE", &self.COMPLETE())
            .field("RXPEND", &self.RXPEND())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("IBIWON", &self.IBIWON())
            .field("ERRWARN", &self.ERRWARN())
            .field("NOWMASTER", &self.NOWMASTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MINTSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MINTSET {{ SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?} }}",
            self.SLVSTART(),
            self.MCTRLDONE(),
            self.COMPLETE(),
            self.RXPEND(),
            self.TXNOTFULL(),
            self.IBIWON(),
            self.ERRWARN(),
            self.NOWMASTER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MRDATAB(pub u32);
impl I3C_MRDATAB {
    #[doc = "Value"]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value"]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_MRDATAB {
    #[inline(always)]
    fn default() -> I3C_MRDATAB {
        I3C_MRDATAB(0)
    }
}
impl core::fmt::Debug for I3C_MRDATAB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MRDATAB")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MRDATAB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MRDATAB {{ VALUE: {=u8:?} }}", self.VALUE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MRDATAH(pub u32);
impl I3C_MRDATAH {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn LSB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_LSB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn MSB(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_MSB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_MRDATAH {
    #[inline(always)]
    fn default() -> I3C_MRDATAH {
        I3C_MRDATAH(0)
    }
}
impl core::fmt::Debug for I3C_MRDATAH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MRDATAH")
            .field("LSB", &self.LSB())
            .field("MSB", &self.MSB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MRDATAH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MRDATAH {{ LSB: {=u8:?}, MSB: {=u8:?} }}",
            self.LSB(),
            self.MSB()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MRMSG_DDR(pub u32);
impl I3C_MRMSG_DDR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MRMSG_DDR {
    #[inline(always)]
    fn default() -> I3C_MRMSG_DDR {
        I3C_MRMSG_DDR(0)
    }
}
impl core::fmt::Debug for I3C_MRMSG_DDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MRMSG_DDR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MRMSG_DDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MRMSG_DDR {{ DATA: {=u16:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MRMSG_SDR(pub u32);
impl I3C_MRMSG_SDR {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MRMSG_SDR {
    #[inline(always)]
    fn default() -> I3C_MRMSG_SDR {
        I3C_MRMSG_SDR(0)
    }
}
impl core::fmt::Debug for I3C_MRMSG_SDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MRMSG_SDR")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MRMSG_SDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MRMSG_SDR {{ DATA: {=u16:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MSTATUS(pub u32);
impl I3C_MSTATUS {
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BETWEEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BETWEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NACKED(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NACKED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBITYPE(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IBITYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SLVSTART(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MCTRLDONE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MCTRLDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn COMPLETE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_COMPLETE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIWON(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIWON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NOWMASTER(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NOWMASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "IBI Address"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIADDR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "IBI Address"]
    #[inline(always)]
    pub const fn set_IBIADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for I3C_MSTATUS {
    #[inline(always)]
    fn default() -> I3C_MSTATUS {
        I3C_MSTATUS(0)
    }
}
impl core::fmt::Debug for I3C_MSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MSTATUS")
            .field("STATE", &self.STATE())
            .field("BETWEEN", &self.BETWEEN())
            .field("NACKED", &self.NACKED())
            .field("IBITYPE", &self.IBITYPE())
            .field("SLVSTART", &self.SLVSTART())
            .field("MCTRLDONE", &self.MCTRLDONE())
            .field("COMPLETE", &self.COMPLETE())
            .field("RXPEND", &self.RXPEND())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("IBIWON", &self.IBIWON())
            .field("ERRWARN", &self.ERRWARN())
            .field("NOWMASTER", &self.NOWMASTER())
            .field("IBIADDR", &self.IBIADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MSTATUS {{ STATE: {=u8:?}, BETWEEN: {=bool:?}, NACKED: {=bool:?}, IBITYPE: {=u8:?}, SLVSTART: {=bool:?}, MCTRLDONE: {=bool:?}, COMPLETE: {=bool:?}, RXPEND: {=bool:?}, TXNOTFULL: {=bool:?}, IBIWON: {=bool:?}, ERRWARN: {=bool:?}, NOWMASTER: {=bool:?}, IBIADDR: {=u8:?} }}",
            self.STATE(),
            self.BETWEEN(),
            self.NACKED(),
            self.IBITYPE(),
            self.SLVSTART(),
            self.MCTRLDONE(),
            self.COMPLETE(),
            self.RXPEND(),
            self.TXNOTFULL(),
            self.IBIWON(),
            self.ERRWARN(),
            self.NOWMASTER(),
            self.IBIADDR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAB(pub u32);
impl I3C_MWDATAB {
    #[doc = "Data Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte"]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Data Byte"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for I3C_MWDATAB {
    #[inline(always)]
    fn default() -> I3C_MWDATAB {
        I3C_MWDATAB(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAB")
            .field("VALUE", &self.VALUE())
            .field("END", &self.END())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWDATAB {{ VALUE: {=u8:?}, END: {=bool:?} }}",
            self.VALUE(),
            self.END()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAB1(pub u32);
impl I3C_MWDATAB1 {
    #[doc = "Value"]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value"]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_MWDATAB1 {
    #[inline(always)]
    fn default() -> I3C_MWDATAB1 {
        I3C_MWDATAB1(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAB1")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAB1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MWDATAB1 {{ VALUE: {=u8:?} }}", self.VALUE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATABE(pub u32);
impl I3C_MWDATABE {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_MWDATABE {
    #[inline(always)]
    fn default() -> I3C_MWDATABE {
        I3C_MWDATABE(0)
    }
}
impl core::fmt::Debug for I3C_MWDATABE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATABE")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATABE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MWDATABE {{ VALUE: {=u8:?} }}", self.VALUE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAB_END(pub u32);
impl I3C_MWDATAB_END {
    #[must_use]
    #[inline(always)]
    pub const fn ALSO(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ALSO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3C_MWDATAB_END {
    #[inline(always)]
    fn default() -> I3C_MWDATAB_END {
        I3C_MWDATAB_END(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAB_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAB_END")
            .field("ALSO", &self.ALSO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAB_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MWDATAB_END {{ ALSO: {=bool:?} }}", self.ALSO())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAH(pub u32);
impl I3C_MWDATAH {
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Data Byte 1"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3C_MWDATAH {
    #[inline(always)]
    fn default() -> I3C_MWDATAH {
        I3C_MWDATAH(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAH")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .field("END", &self.END())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWDATAH {{ DATA0: {=u8:?}, DATA1: {=u8:?}, END: {=bool:?} }}",
            self.DATA0(),
            self.DATA1(),
            self.END()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAH1(pub u32);
impl I3C_MWDATAH1 {
    #[doc = "Value"]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value"]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MWDATAH1 {
    #[inline(always)]
    fn default() -> I3C_MWDATAH1 {
        I3C_MWDATAH1(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAH1")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_MWDATAH1 {{ VALUE: {=u16:?} }}", self.VALUE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWDATAHE(pub u32);
impl I3C_MWDATAHE {
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_MWDATAHE {
    #[inline(always)]
    fn default() -> I3C_MWDATAHE {
        I3C_MWDATAHE(0)
    }
}
impl core::fmt::Debug for I3C_MWDATAHE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWDATAHE")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWDATAHE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWDATAHE {{ DATA0: {=u8:?}, DATA1: {=u8:?} }}",
            self.DATA0(),
            self.DATA1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWMSG_DDR_CONTROL(pub u32);
impl I3C_MWMSG_DDR_CONTROL {
    #[doc = "Address Command"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRCMD(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Address Command"]
    #[inline(always)]
    pub const fn set_ADDRCMD(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MWMSG_DDR_CONTROL {
    #[inline(always)]
    fn default() -> I3C_MWMSG_DDR_CONTROL {
        I3C_MWMSG_DDR_CONTROL(0)
    }
}
impl core::fmt::Debug for I3C_MWMSG_DDR_CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWMSG_DDR_CONTROL")
            .field("ADDRCMD", &self.ADDRCMD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWMSG_DDR_CONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWMSG_DDR_CONTROL {{ ADDRCMD: {=u16:?} }}",
            self.ADDRCMD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWMSG_DDR_CONTROL2(pub u32);
impl I3C_MWMSG_DDR_CONTROL2 {
    #[doc = "Length of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Length of Message"]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Length of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Length of Message"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for I3C_MWMSG_DDR_CONTROL2 {
    #[inline(always)]
    fn default() -> I3C_MWMSG_DDR_CONTROL2 {
        I3C_MWMSG_DDR_CONTROL2(0)
    }
}
impl core::fmt::Debug for I3C_MWMSG_DDR_CONTROL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWMSG_DDR_CONTROL2")
            .field("LEN", &self.LEN())
            .field("END", &self.END())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWMSG_DDR_CONTROL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWMSG_DDR_CONTROL2 {{ LEN: {=u16:?}, END: {=bool:?} }}",
            self.LEN(),
            self.END()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWMSG_DDR_DATA(pub u32);
impl I3C_MWMSG_DDR_DATA {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA16B(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA16B(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MWMSG_DDR_DATA {
    #[inline(always)]
    fn default() -> I3C_MWMSG_DDR_DATA {
        I3C_MWMSG_DDR_DATA(0)
    }
}
impl core::fmt::Debug for I3C_MWMSG_DDR_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWMSG_DDR_DATA")
            .field("DATA16B", &self.DATA16B())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWMSG_DDR_DATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWMSG_DDR_DATA {{ DATA16B: {=u16:?} }}",
            self.DATA16B()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWMSG_SDR_CONTROL(pub u32);
impl I3C_MWMSG_SDR_CONTROL {
    #[must_use]
    #[inline(always)]
    pub const fn DIR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DIR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn I2C(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_I2C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn LEN(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_LEN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for I3C_MWMSG_SDR_CONTROL {
    #[inline(always)]
    fn default() -> I3C_MWMSG_SDR_CONTROL {
        I3C_MWMSG_SDR_CONTROL(0)
    }
}
impl core::fmt::Debug for I3C_MWMSG_SDR_CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWMSG_SDR_CONTROL")
            .field("DIR", &self.DIR())
            .field("ADDR", &self.ADDR())
            .field("END", &self.END())
            .field("I2C", &self.I2C())
            .field("LEN", &self.LEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWMSG_SDR_CONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWMSG_SDR_CONTROL {{ DIR: {=bool:?}, ADDR: {=u8:?}, END: {=bool:?}, I2C: {=bool:?}, LEN: {=u8:?} }}",
            self.DIR(),
            self.ADDR(),
            self.END(),
            self.I2C(),
            self.LEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_MWMSG_SDR_DATA(pub u32);
impl I3C_MWMSG_SDR_DATA {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA16B(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA16B(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_MWMSG_SDR_DATA {
    #[inline(always)]
    fn default() -> I3C_MWMSG_SDR_DATA {
        I3C_MWMSG_SDR_DATA(0)
    }
}
impl core::fmt::Debug for I3C_MWMSG_SDR_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_MWMSG_SDR_DATA")
            .field("DATA16B", &self.DATA16B())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_MWMSG_SDR_DATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_MWMSG_SDR_DATA {{ DATA16B: {=u16:?} }}",
            self.DATA16B()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCAPABILITIES(pub u32);
impl I3C_SCAPABILITIES {
    #[must_use]
    #[inline(always)]
    pub const fn IDENA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IDENA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IDREG(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IDREG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HDRSUPP(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_HDRSUPP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SADDR(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CCCHANDLE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CCCHANDLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMECTRL(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIMECTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EXTFIFO(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_EXTFIFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FIFOTX(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FIFOTX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FIFORX(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_FIFORX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INT(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_SCAPABILITIES {
    #[inline(always)]
    fn default() -> I3C_SCAPABILITIES {
        I3C_SCAPABILITIES(0)
    }
}
impl core::fmt::Debug for I3C_SCAPABILITIES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCAPABILITIES")
            .field("IDENA", &self.IDENA())
            .field("IDREG", &self.IDREG())
            .field("HDRSUPP", &self.HDRSUPP())
            .field("MASTER", &self.MASTER())
            .field("SADDR", &self.SADDR())
            .field("CCCHANDLE", &self.CCCHANDLE())
            .field("TIMECTRL", &self.TIMECTRL())
            .field("EXTFIFO", &self.EXTFIFO())
            .field("FIFOTX", &self.FIFOTX())
            .field("FIFORX", &self.FIFORX())
            .field("INT", &self.INT())
            .field("DMA", &self.DMA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCAPABILITIES {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SCAPABILITIES {{ IDENA: {=u8:?}, IDREG: {=u8:?}, HDRSUPP: {=u8:?}, MASTER: {=bool:?}, SADDR: {=u8:?}, CCCHANDLE: {=u8:?}, TIMECTRL: {=bool:?}, EXTFIFO: {=u8:?}, FIFOTX: {=u8:?}, FIFORX: {=u8:?}, INT: {=bool:?}, DMA: {=bool:?} }}",
            self.IDENA(),
            self.IDREG(),
            self.HDRSUPP(),
            self.MASTER(),
            self.SADDR(),
            self.CCCHANDLE(),
            self.TIMECTRL(),
            self.EXTFIFO(),
            self.FIFOTX(),
            self.FIFORX(),
            self.INT(),
            self.DMA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCAPABILITIES2(pub u32);
impl I3C_SCAPABILITIES2 {
    #[doc = "Map Count"]
    #[must_use]
    #[inline(always)]
    pub const fn MAPCNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Map Count"]
    #[inline(always)]
    pub const fn set_MAPCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Map Count"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C10B(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Map Count"]
    #[inline(always)]
    pub const fn set_I2C10B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn I2CDEVID(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_I2CDEVID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIEXT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIEXT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIXREG(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIXREG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SLVRST(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn GROUP(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_GROUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AASA(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AASA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SSTSUB(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SSTSUB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SSTWR(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SSTWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for I3C_SCAPABILITIES2 {
    #[inline(always)]
    fn default() -> I3C_SCAPABILITIES2 {
        I3C_SCAPABILITIES2(0)
    }
}
impl core::fmt::Debug for I3C_SCAPABILITIES2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCAPABILITIES2")
            .field("MAPCNT", &self.MAPCNT())
            .field("I2C10B", &self.I2C10B())
            .field("I2CDEVID", &self.I2CDEVID())
            .field("IBIEXT", &self.IBIEXT())
            .field("IBIXREG", &self.IBIXREG())
            .field("SLVRST", &self.SLVRST())
            .field("GROUP", &self.GROUP())
            .field("AASA", &self.AASA())
            .field("SSTSUB", &self.SSTSUB())
            .field("SSTWR", &self.SSTWR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCAPABILITIES2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SCAPABILITIES2 {{ MAPCNT: {=u8:?}, I2C10B: {=bool:?}, I2CDEVID: {=bool:?}, IBIEXT: {=bool:?}, IBIXREG: {=bool:?}, SLVRST: {=bool:?}, GROUP: {=u8:?}, AASA: {=bool:?}, SSTSUB: {=bool:?}, SSTWR: {=bool:?} }}",
            self.MAPCNT(),
            self.I2C10B(),
            self.I2CDEVID(),
            self.IBIEXT(),
            self.IBIXREG(),
            self.SLVRST(),
            self.GROUP(),
            self.AASA(),
            self.SSTSUB(),
            self.SSTWR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCAPABILITIES_IBI_MR(pub u32);
impl I3C_SCAPABILITIES_IBI_MR {
    #[must_use]
    #[inline(always)]
    pub const fn HJ(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_HJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for I3C_SCAPABILITIES_IBI_MR {
    #[inline(always)]
    fn default() -> I3C_SCAPABILITIES_IBI_MR {
        I3C_SCAPABILITIES_IBI_MR(0)
    }
}
impl core::fmt::Debug for I3C_SCAPABILITIES_IBI_MR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCAPABILITIES_IBI_MR")
            .field("HJ", &self.HJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCAPABILITIES_IBI_MR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SCAPABILITIES_IBI_MR {{ HJ: {=u8:?} }}", self.HJ())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCCCMASK(pub u32);
impl I3C_SCCCMASK {
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BASEBX(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BASEBX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BASEDX(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BASEDX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MEXTB(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MEXTB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MEXTD(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MEXTD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VENDB(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VENDB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VENDD(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VENDD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for I3C_SCCCMASK {
    #[inline(always)]
    fn default() -> I3C_SCCCMASK {
        I3C_SCCCMASK(0)
    }
}
impl core::fmt::Debug for I3C_SCCCMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCCCMASK")
            .field("BASE", &self.BASE())
            .field("BASEBX", &self.BASEBX())
            .field("BASEDX", &self.BASEDX())
            .field("MEXTB", &self.MEXTB())
            .field("MEXTD", &self.MEXTD())
            .field("VENDB", &self.VENDB())
            .field("VENDD", &self.VENDD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCCCMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SCCCMASK {{ BASE: {=bool:?}, BASEBX: {=bool:?}, BASEDX: {=bool:?}, MEXTB: {=bool:?}, MEXTD: {=bool:?}, VENDB: {=bool:?}, VENDD: {=bool:?} }}",
            self.BASE(),
            self.BASEBX(),
            self.BASEDX(),
            self.MEXTB(),
            self.MEXTD(),
            self.VENDB(),
            self.VENDD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCONFIG(pub u32);
impl I3C_SCONFIG {
    #[must_use]
    #[inline(always)]
    pub const fn SLVENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MATCHSS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MATCHSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S0IGNORE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S0IGNORE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HDROK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HDROK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OFFLINE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OFFLINE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Available Match"]
    #[must_use]
    #[inline(always)]
    pub const fn BAMATCH(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Bus Available Match"]
    #[inline(always)]
    pub const fn set_BAMATCH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Bus Available Match"]
    #[must_use]
    #[inline(always)]
    pub const fn SADDR(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Bus Available Match"]
    #[inline(always)]
    pub const fn set_SADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for I3C_SCONFIG {
    #[inline(always)]
    fn default() -> I3C_SCONFIG {
        I3C_SCONFIG(0)
    }
}
impl core::fmt::Debug for I3C_SCONFIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCONFIG")
            .field("SLVENA", &self.SLVENA())
            .field("NACK", &self.NACK())
            .field("MATCHSS", &self.MATCHSS())
            .field("S0IGNORE", &self.S0IGNORE())
            .field("HDROK", &self.HDROK())
            .field("OFFLINE", &self.OFFLINE())
            .field("BAMATCH", &self.BAMATCH())
            .field("SADDR", &self.SADDR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCONFIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SCONFIG {{ SLVENA: {=bool:?}, NACK: {=bool:?}, MATCHSS: {=bool:?}, S0IGNORE: {=bool:?}, HDROK: {=bool:?}, OFFLINE: {=bool:?}, BAMATCH: {=u8:?}, SADDR: {=u8:?} }}",
            self.SLVENA(),
            self.NACK(),
            self.MATCHSS(),
            self.S0IGNORE(),
            self.HDROK(),
            self.OFFLINE(),
            self.BAMATCH(),
            self.SADDR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SCTRL(pub u32);
impl I3C_SCTRL {
    #[must_use]
    #[inline(always)]
    pub const fn EVENT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_EVENT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EXTDATA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EXTDATA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Map Index"]
    #[must_use]
    #[inline(always)]
    pub const fn MAPIDX(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Map Index"]
    #[inline(always)]
    pub const fn set_MAPIDX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Map Index"]
    #[must_use]
    #[inline(always)]
    pub const fn IBIDATA(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Map Index"]
    #[inline(always)]
    pub const fn set_IBIDATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "In-Band Interrupt Data"]
    #[must_use]
    #[inline(always)]
    pub const fn PENDINT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "In-Band Interrupt Data"]
    #[inline(always)]
    pub const fn set_PENDINT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Pending Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTSTATE(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Pending Interrupt"]
    #[inline(always)]
    pub const fn set_ACTSTATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Activity State of Target"]
    #[must_use]
    #[inline(always)]
    pub const fn VENDINFO(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Activity State of Target"]
    #[inline(always)]
    pub const fn set_VENDINFO(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_SCTRL {
    #[inline(always)]
    fn default() -> I3C_SCTRL {
        I3C_SCTRL(0)
    }
}
impl core::fmt::Debug for I3C_SCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SCTRL")
            .field("EVENT", &self.EVENT())
            .field("EXTDATA", &self.EXTDATA())
            .field("MAPIDX", &self.MAPIDX())
            .field("IBIDATA", &self.IBIDATA())
            .field("PENDINT", &self.PENDINT())
            .field("ACTSTATE", &self.ACTSTATE())
            .field("VENDINFO", &self.VENDINFO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SCTRL {{ EVENT: {=u8:?}, EXTDATA: {=bool:?}, MAPIDX: {=bool:?}, IBIDATA: {=u8:?}, PENDINT: {=u8:?}, ACTSTATE: {=u8:?}, VENDINFO: {=u8:?} }}",
            self.EVENT(),
            self.EXTDATA(),
            self.MAPIDX(),
            self.IBIDATA(),
            self.PENDINT(),
            self.ACTSTATE(),
            self.VENDINFO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SDATACTRL(pub u32);
impl I3C_SDATACTRL {
    #[must_use]
    #[inline(always)]
    pub const fn FLUSHTB(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLUSHTB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn FLUSHFB(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_FLUSHFB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UNLOCK(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UNLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXTRIG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TXTRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXTRIG(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXTRIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXCOUNT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Count of Entries in Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn RXCOUNT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of Entries in Transmit"]
    #[inline(always)]
    pub const fn set_RXCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Count of Entries in Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn TXFULL(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Count of Entries in Receive"]
    #[inline(always)]
    pub const fn set_TXFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXEMPTY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXEMPTY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3C_SDATACTRL {
    #[inline(always)]
    fn default() -> I3C_SDATACTRL {
        I3C_SDATACTRL(0)
    }
}
impl core::fmt::Debug for I3C_SDATACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SDATACTRL")
            .field("FLUSHTB", &self.FLUSHTB())
            .field("FLUSHFB", &self.FLUSHFB())
            .field("UNLOCK", &self.UNLOCK())
            .field("TXTRIG", &self.TXTRIG())
            .field("RXTRIG", &self.RXTRIG())
            .field("TXCOUNT", &self.TXCOUNT())
            .field("RXCOUNT", &self.RXCOUNT())
            .field("TXFULL", &self.TXFULL())
            .field("RXEMPTY", &self.RXEMPTY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SDATACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SDATACTRL {{ FLUSHTB: {=bool:?}, FLUSHFB: {=bool:?}, UNLOCK: {=bool:?}, TXTRIG: {=u8:?}, RXTRIG: {=u8:?}, TXCOUNT: {=u8:?}, RXCOUNT: {=u8:?}, TXFULL: {=bool:?}, RXEMPTY: {=bool:?} }}",
            self.FLUSHTB(),
            self.FLUSHFB(),
            self.UNLOCK(),
            self.TXTRIG(),
            self.RXTRIG(),
            self.TXCOUNT(),
            self.RXCOUNT(),
            self.TXFULL(),
            self.RXEMPTY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SDMACTRL(pub u32);
impl I3C_SDMACTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DMAFB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMAFB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMATB(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMATB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMAWIDTH(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DMAWIDTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for I3C_SDMACTRL {
    #[inline(always)]
    fn default() -> I3C_SDMACTRL {
        I3C_SDMACTRL(0)
    }
}
impl core::fmt::Debug for I3C_SDMACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SDMACTRL")
            .field("DMAFB", &self.DMAFB())
            .field("DMATB", &self.DMATB())
            .field("DMAWIDTH", &self.DMAWIDTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SDMACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SDMACTRL {{ DMAFB: {=u8:?}, DMATB: {=u8:?}, DMAWIDTH: {=u8:?} }}",
            self.DMAFB(),
            self.DMATB(),
            self.DMAWIDTH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SELFRESET(pub u32);
impl I3C_SELFRESET {
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Key"]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Key"]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for I3C_SELFRESET {
    #[inline(always)]
    fn default() -> I3C_SELFRESET {
        I3C_SELFRESET(0)
    }
}
impl core::fmt::Debug for I3C_SELFRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SELFRESET")
            .field("RST", &self.RST())
            .field("KEY", &self.KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SELFRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SELFRESET {{ RST: {=bool:?}, KEY: {=u32:?} }}",
            self.RST(),
            self.KEY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SERRWARN(pub u32);
impl I3C_SERRWARN {
    #[must_use]
    #[inline(always)]
    pub const fn ORUN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ORUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn URUN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_URUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn URUNNACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_URUNNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TERM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TERM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INVSTART(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SPAR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SPAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HPAR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HPAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HCRC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S0S1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S0S1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OREAD(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OWRITE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OWRITE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for I3C_SERRWARN {
    #[inline(always)]
    fn default() -> I3C_SERRWARN {
        I3C_SERRWARN(0)
    }
}
impl core::fmt::Debug for I3C_SERRWARN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SERRWARN")
            .field("ORUN", &self.ORUN())
            .field("URUN", &self.URUN())
            .field("URUNNACK", &self.URUNNACK())
            .field("TERM", &self.TERM())
            .field("INVSTART", &self.INVSTART())
            .field("SPAR", &self.SPAR())
            .field("HPAR", &self.HPAR())
            .field("HCRC", &self.HCRC())
            .field("S0S1", &self.S0S1())
            .field("OREAD", &self.OREAD())
            .field("OWRITE", &self.OWRITE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SERRWARN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SERRWARN {{ ORUN: {=bool:?}, URUN: {=bool:?}, URUNNACK: {=bool:?}, TERM: {=bool:?}, INVSTART: {=bool:?}, SPAR: {=bool:?}, HPAR: {=bool:?}, HCRC: {=bool:?}, S0S1: {=bool:?}, OREAD: {=bool:?}, OWRITE: {=bool:?} }}",
            self.ORUN(),
            self.URUN(),
            self.URUNNACK(),
            self.TERM(),
            self.INVSTART(),
            self.SPAR(),
            self.HPAR(),
            self.HCRC(),
            self.S0S1(),
            self.OREAD(),
            self.OWRITE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SERRWARNMASK(pub u32);
impl I3C_SERRWARNMASK {
    #[must_use]
    #[inline(always)]
    pub const fn ORUN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ORUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn URUN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_URUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn URUNNACK(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_URUNNACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TERM(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TERM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn INVSTART(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_INVSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SPAR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SPAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HPAR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HPAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HCRC(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HCRC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S0S1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S0S1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for I3C_SERRWARNMASK {
    #[inline(always)]
    fn default() -> I3C_SERRWARNMASK {
        I3C_SERRWARNMASK(0)
    }
}
impl core::fmt::Debug for I3C_SERRWARNMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SERRWARNMASK")
            .field("ORUN", &self.ORUN())
            .field("URUN", &self.URUN())
            .field("URUNNACK", &self.URUNNACK())
            .field("TERM", &self.TERM())
            .field("INVSTART", &self.INVSTART())
            .field("SPAR", &self.SPAR())
            .field("HPAR", &self.HPAR())
            .field("HCRC", &self.HCRC())
            .field("S0S1", &self.S0S1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SERRWARNMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SERRWARNMASK {{ ORUN: {=bool:?}, URUN: {=bool:?}, URUNNACK: {=bool:?}, TERM: {=bool:?}, INVSTART: {=bool:?}, SPAR: {=bool:?}, HPAR: {=bool:?}, HCRC: {=bool:?}, S0S1: {=bool:?} }}",
            self.ORUN(),
            self.URUN(),
            self.URUNNACK(),
            self.TERM(),
            self.INVSTART(),
            self.SPAR(),
            self.HPAR(),
            self.HCRC(),
            self.S0S1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SIDEXT(pub u32);
impl I3C_SIDEXT {
    #[doc = "Device Characteristic Register"]
    #[must_use]
    #[inline(always)]
    pub const fn DCR(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Device Characteristic Register"]
    #[inline(always)]
    pub const fn set_DCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Device Characteristic Register"]
    #[must_use]
    #[inline(always)]
    pub const fn BCR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Device Characteristic Register"]
    #[inline(always)]
    pub const fn set_BCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for I3C_SIDEXT {
    #[inline(always)]
    fn default() -> I3C_SIDEXT {
        I3C_SIDEXT(0)
    }
}
impl core::fmt::Debug for I3C_SIDEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SIDEXT")
            .field("DCR", &self.DCR())
            .field("BCR", &self.BCR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SIDEXT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SIDEXT {{ DCR: {=u8:?}, BCR: {=u8:?} }}",
            self.DCR(),
            self.BCR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SIDPARTNO(pub u32);
impl I3C_SIDPARTNO {
    #[doc = "Part Number"]
    #[must_use]
    #[inline(always)]
    pub const fn PARTNO(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Part Number"]
    #[inline(always)]
    pub const fn set_PARTNO(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for I3C_SIDPARTNO {
    #[inline(always)]
    fn default() -> I3C_SIDPARTNO {
        I3C_SIDPARTNO(0)
    }
}
impl core::fmt::Debug for I3C_SIDPARTNO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SIDPARTNO")
            .field("PARTNO", &self.PARTNO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SIDPARTNO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SIDPARTNO {{ PARTNO: {=u32:?} }}", self.PARTNO())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SINTCLR(pub u32);
impl I3C_SINTCLR {
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "START Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Matched Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Matched Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "STOP Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn TXSEND(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_TXSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TXSEND Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn DACHG(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_DACHG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DACHG Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn CCC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_CCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CCC Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRMATCHED(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_DDRMATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CHANDLED(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CHANDLED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CHANDLED Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EVENT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_EVENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "EVENT Interrupt Enable Clear Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SLVRST(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Enable Clear Flag"]
    #[inline(always)]
    pub const fn set_SLVRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_SINTCLR {
    #[inline(always)]
    fn default() -> I3C_SINTCLR {
        I3C_SINTCLR(0)
    }
}
impl core::fmt::Debug for I3C_SINTCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SINTCLR")
            .field("START", &self.START())
            .field("MATCHED", &self.MATCHED())
            .field("STOP", &self.STOP())
            .field("RXPEND", &self.RXPEND())
            .field("TXSEND", &self.TXSEND())
            .field("DACHG", &self.DACHG())
            .field("CCC", &self.CCC())
            .field("ERRWARN", &self.ERRWARN())
            .field("DDRMATCHED", &self.DDRMATCHED())
            .field("CHANDLED", &self.CHANDLED())
            .field("EVENT", &self.EVENT())
            .field("SLVRST", &self.SLVRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SINTCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SINTCLR {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?}, SLVRST: {=bool:?} }}",
            self.START(),
            self.MATCHED(),
            self.STOP(),
            self.RXPEND(),
            self.TXSEND(),
            self.DACHG(),
            self.CCC(),
            self.ERRWARN(),
            self.DDRMATCHED(),
            self.CHANDLED(),
            self.EVENT(),
            self.SLVRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SINTMASKED(pub u32);
impl I3C_SINTMASKED {
    #[doc = "START Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Mask"]
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "START Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Mask"]
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "MATCHED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "MATCHED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "STOP Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Mask"]
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn TXSEND(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_TXSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TXSEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn DACHG(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_DACHG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DACHG Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn CCC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Mask"]
    #[inline(always)]
    pub const fn set_CCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CCC Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRMATCHED(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[inline(always)]
    pub const fn set_DDRMATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn CHANDLED(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_CHANDLED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CHANDLED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn EVENT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_EVENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "EVENT Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn SLVRST(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Mask"]
    #[inline(always)]
    pub const fn set_SLVRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_SINTMASKED {
    #[inline(always)]
    fn default() -> I3C_SINTMASKED {
        I3C_SINTMASKED(0)
    }
}
impl core::fmt::Debug for I3C_SINTMASKED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SINTMASKED")
            .field("START", &self.START())
            .field("MATCHED", &self.MATCHED())
            .field("STOP", &self.STOP())
            .field("RXPEND", &self.RXPEND())
            .field("TXSEND", &self.TXSEND())
            .field("DACHG", &self.DACHG())
            .field("CCC", &self.CCC())
            .field("ERRWARN", &self.ERRWARN())
            .field("DDRMATCHED", &self.DDRMATCHED())
            .field("CHANDLED", &self.CHANDLED())
            .field("EVENT", &self.EVENT())
            .field("SLVRST", &self.SLVRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SINTMASKED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SINTMASKED {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?}, SLVRST: {=bool:?} }}",
            self.START(),
            self.MATCHED(),
            self.STOP(),
            self.RXPEND(),
            self.TXSEND(),
            self.DACHG(),
            self.CCC(),
            self.ERRWARN(),
            self.DDRMATCHED(),
            self.CHANDLED(),
            self.EVENT(),
            self.SLVRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SINTSET(pub u32);
impl I3C_SINTSET {
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXPEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXSEND(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DACHG(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DACHG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CCC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DDRMATCHED(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DDRMATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CHANDLED(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CHANDLED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EVENT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EVENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SLVRST(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLVRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for I3C_SINTSET {
    #[inline(always)]
    fn default() -> I3C_SINTSET {
        I3C_SINTSET(0)
    }
}
impl core::fmt::Debug for I3C_SINTSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SINTSET")
            .field("START", &self.START())
            .field("MATCHED", &self.MATCHED())
            .field("STOP", &self.STOP())
            .field("RXPEND", &self.RXPEND())
            .field("TXSEND", &self.TXSEND())
            .field("DACHG", &self.DACHG())
            .field("CCC", &self.CCC())
            .field("ERRWARN", &self.ERRWARN())
            .field("DDRMATCHED", &self.DDRMATCHED())
            .field("CHANDLED", &self.CHANDLED())
            .field("EVENT", &self.EVENT())
            .field("SLVRST", &self.SLVRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SINTSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SINTSET {{ START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, RXPEND: {=bool:?}, TXSEND: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, DDRMATCHED: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?}, SLVRST: {=bool:?} }}",
            self.START(),
            self.MATCHED(),
            self.STOP(),
            self.RXPEND(),
            self.TXSEND(),
            self.DACHG(),
            self.CCC(),
            self.ERRWARN(),
            self.DDRMATCHED(),
            self.CHANDLED(),
            self.EVENT(),
            self.SLVRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SMAPCTRL0(pub u32);
impl I3C_SMAPCTRL0 {
    #[must_use]
    #[inline(always)]
    pub const fn ENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn DA(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_DA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn CAUSE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_CAUSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for I3C_SMAPCTRL0 {
    #[inline(always)]
    fn default() -> I3C_SMAPCTRL0 {
        I3C_SMAPCTRL0(0)
    }
}
impl core::fmt::Debug for I3C_SMAPCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SMAPCTRL0")
            .field("ENA", &self.ENA())
            .field("DA", &self.DA())
            .field("CAUSE", &self.CAUSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SMAPCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SMAPCTRL0 {{ ENA: {=bool:?}, DA: {=u8:?}, CAUSE: {=u8:?} }}",
            self.ENA(),
            self.DA(),
            self.CAUSE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SMAPCTRL1(pub u32);
impl I3C_SMAPCTRL1 {
    #[must_use]
    #[inline(always)]
    pub const fn ENA(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_ADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn MAPSA(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_MAPSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SA10B(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SA10B(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Static Address 10-Bit Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn NACK(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Static Address 10-Bit Extension"]
    #[inline(always)]
    pub const fn set_NACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AUTO(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AUTO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DCR"]
    #[must_use]
    #[inline(always)]
    pub const fn DCR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "DCR"]
    #[inline(always)]
    pub const fn set_DCR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for I3C_SMAPCTRL1 {
    #[inline(always)]
    fn default() -> I3C_SMAPCTRL1 {
        I3C_SMAPCTRL1(0)
    }
}
impl core::fmt::Debug for I3C_SMAPCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SMAPCTRL1")
            .field("ENA", &self.ENA())
            .field("ADDR", &self.ADDR())
            .field("MAPSA", &self.MAPSA())
            .field("SA10B", &self.SA10B())
            .field("NACK", &self.NACK())
            .field("AUTO", &self.AUTO())
            .field("DCR", &self.DCR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SMAPCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SMAPCTRL1 {{ ENA: {=bool:?}, ADDR: {=u8:?}, MAPSA: {=bool:?}, SA10B: {=u8:?}, NACK: {=bool:?}, AUTO: {=bool:?}, DCR: {=u8:?} }}",
            self.ENA(),
            self.ADDR(),
            self.MAPSA(),
            self.SA10B(),
            self.NACK(),
            self.AUTO(),
            self.DCR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SMAXLIMITS(pub u32);
impl I3C_SMAXLIMITS {
    #[doc = "Maximum Read Length"]
    #[must_use]
    #[inline(always)]
    pub const fn MAXRD(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Read Length"]
    #[inline(always)]
    pub const fn set_MAXRD(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Maximum Read Length"]
    #[must_use]
    #[inline(always)]
    pub const fn MAXWR(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Read Length"]
    #[inline(always)]
    pub const fn set_MAXWR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for I3C_SMAXLIMITS {
    #[inline(always)]
    fn default() -> I3C_SMAXLIMITS {
        I3C_SMAXLIMITS(0)
    }
}
impl core::fmt::Debug for I3C_SMAXLIMITS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SMAXLIMITS")
            .field("MAXRD", &self.MAXRD())
            .field("MAXWR", &self.MAXWR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SMAXLIMITS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SMAXLIMITS {{ MAXRD: {=u16:?}, MAXWR: {=u16:?} }}",
            self.MAXRD(),
            self.MAXWR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SMSGMAPADDR(pub u32);
impl I3C_SMSGMAPADDR {
    #[doc = "Matched Address Index"]
    #[must_use]
    #[inline(always)]
    pub const fn MAPLAST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Address Index"]
    #[inline(always)]
    pub const fn set_MAPLAST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Matched Address Index"]
    #[must_use]
    #[inline(always)]
    pub const fn LASTSTATIC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Matched Address Index"]
    #[inline(always)]
    pub const fn set_LASTSTATIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MAPLASTM1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MAPLASTM1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Matched Previous Index 2"]
    #[must_use]
    #[inline(always)]
    pub const fn MAPLASTM2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Previous Index 2"]
    #[inline(always)]
    pub const fn set_MAPLASTM2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for I3C_SMSGMAPADDR {
    #[inline(always)]
    fn default() -> I3C_SMSGMAPADDR {
        I3C_SMSGMAPADDR(0)
    }
}
impl core::fmt::Debug for I3C_SMSGMAPADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SMSGMAPADDR")
            .field("MAPLAST", &self.MAPLAST())
            .field("LASTSTATIC", &self.LASTSTATIC())
            .field("MAPLASTM1", &self.MAPLASTM1())
            .field("MAPLASTM2", &self.MAPLASTM2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SMSGMAPADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SMSGMAPADDR {{ MAPLAST: {=u8:?}, LASTSTATIC: {=bool:?}, MAPLASTM1: {=u8:?}, MAPLASTM2: {=u8:?} }}",
            self.MAPLAST(),
            self.LASTSTATIC(),
            self.MAPLASTM1(),
            self.MAPLASTM2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SRDATAB(pub u32);
impl I3C_SRDATAB {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for I3C_SRDATAB {
    #[inline(always)]
    fn default() -> I3C_SRDATAB {
        I3C_SRDATAB(0)
    }
}
impl core::fmt::Debug for I3C_SRDATAB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SRDATAB")
            .field("DATA0", &self.DATA0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SRDATAB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SRDATAB {{ DATA0: {=u8:?} }}", self.DATA0())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SRDATAH(pub u32);
impl I3C_SRDATAH {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn LSB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_LSB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn MSB(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_MSB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_SRDATAH {
    #[inline(always)]
    fn default() -> I3C_SRDATAH {
        I3C_SRDATAH(0)
    }
}
impl core::fmt::Debug for I3C_SRDATAH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SRDATAH")
            .field("LSB", &self.LSB())
            .field("MSB", &self.MSB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SRDATAH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SRDATAH {{ LSB: {=u8:?}, MSB: {=u8:?} }}",
            self.LSB(),
            self.MSB()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SRSTACTTIME(pub u32);
impl I3C_SRSTACTTIME {
    #[must_use]
    #[inline(always)]
    pub const fn PERRSTTIM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PERRSTTIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SYSRSTTIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SYSRSTTIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_SRSTACTTIME {
    #[inline(always)]
    fn default() -> I3C_SRSTACTTIME {
        I3C_SRSTACTTIME(0)
    }
}
impl core::fmt::Debug for I3C_SRSTACTTIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SRSTACTTIME")
            .field("PERRSTTIM", &self.PERRSTTIM())
            .field("SYSRSTTIM", &self.SYSRSTTIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SRSTACTTIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SRSTACTTIME {{ PERRSTTIM: {=u8:?}, SYSRSTTIM: {=u8:?} }}",
            self.PERRSTTIM(),
            self.SYSRSTTIM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SSTATUS(pub u32);
impl I3C_SSTATUS {
    #[must_use]
    #[inline(always)]
    pub const fn STNOTSTOP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STNOTSTOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STMSG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STMSG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STCCCH(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STCCCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STREQRD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STREQRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STREQWR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STREQWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STDAA(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STDAA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STHDR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STHDR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn START(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_START(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MATCHED(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MATCHED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXNOTFULL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXNOTFULL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DACHG(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DACHG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CCC(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn ERRWARN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning"]
    #[inline(always)]
    pub const fn set_ERRWARN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Error Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn HDRMATCH(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning"]
    #[inline(always)]
    pub const fn set_HDRMATCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CHANDLED(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CHANDLED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EVENT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EVENT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Target Reset Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SLVRST(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Target Reset Flag"]
    #[inline(always)]
    pub const fn set_SLVRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Target Reset Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EVDET(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Target Reset Flag"]
    #[inline(always)]
    pub const fn set_EVDET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IBIDIS(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IBIDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MRDIS(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MRDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HJDIS(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HJDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACTSTATE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ACTSTATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TIMECTRL(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TIMECTRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for I3C_SSTATUS {
    #[inline(always)]
    fn default() -> I3C_SSTATUS {
        I3C_SSTATUS(0)
    }
}
impl core::fmt::Debug for I3C_SSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SSTATUS")
            .field("STNOTSTOP", &self.STNOTSTOP())
            .field("STMSG", &self.STMSG())
            .field("STCCCH", &self.STCCCH())
            .field("STREQRD", &self.STREQRD())
            .field("STREQWR", &self.STREQWR())
            .field("STDAA", &self.STDAA())
            .field("STHDR", &self.STHDR())
            .field("START", &self.START())
            .field("MATCHED", &self.MATCHED())
            .field("STOP", &self.STOP())
            .field("TXNOTFULL", &self.TXNOTFULL())
            .field("DACHG", &self.DACHG())
            .field("CCC", &self.CCC())
            .field("ERRWARN", &self.ERRWARN())
            .field("HDRMATCH", &self.HDRMATCH())
            .field("CHANDLED", &self.CHANDLED())
            .field("EVENT", &self.EVENT())
            .field("SLVRST", &self.SLVRST())
            .field("EVDET", &self.EVDET())
            .field("IBIDIS", &self.IBIDIS())
            .field("MRDIS", &self.MRDIS())
            .field("HJDIS", &self.HJDIS())
            .field("ACTSTATE", &self.ACTSTATE())
            .field("TIMECTRL", &self.TIMECTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SSTATUS {{ STNOTSTOP: {=bool:?}, STMSG: {=bool:?}, STCCCH: {=bool:?}, STREQRD: {=bool:?}, STREQWR: {=bool:?}, STDAA: {=bool:?}, STHDR: {=bool:?}, START: {=bool:?}, MATCHED: {=bool:?}, STOP: {=bool:?}, TXNOTFULL: {=bool:?}, DACHG: {=bool:?}, CCC: {=bool:?}, ERRWARN: {=bool:?}, HDRMATCH: {=bool:?}, CHANDLED: {=bool:?}, EVENT: {=bool:?}, SLVRST: {=bool:?}, EVDET: {=u8:?}, IBIDIS: {=bool:?}, MRDIS: {=bool:?}, HJDIS: {=bool:?}, ACTSTATE: {=u8:?}, TIMECTRL: {=u8:?} }}",
            self.STNOTSTOP(),
            self.STMSG(),
            self.STCCCH(),
            self.STREQRD(),
            self.STREQWR(),
            self.STDAA(),
            self.STHDR(),
            self.START(),
            self.MATCHED(),
            self.STOP(),
            self.TXNOTFULL(),
            self.DACHG(),
            self.CCC(),
            self.ERRWARN(),
            self.HDRMATCH(),
            self.CHANDLED(),
            self.EVENT(),
            self.SLVRST(),
            self.EVDET(),
            self.IBIDIS(),
            self.MRDIS(),
            self.HJDIS(),
            self.ACTSTATE(),
            self.TIMECTRL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SSTATUS_RX(pub u32);
impl I3C_SSTATUS_RX {
    #[must_use]
    #[inline(always)]
    pub const fn PEND(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for I3C_SSTATUS_RX {
    #[inline(always)]
    fn default() -> I3C_SSTATUS_RX {
        I3C_SSTATUS_RX(0)
    }
}
impl core::fmt::Debug for I3C_SSTATUS_RX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SSTATUS_RX")
            .field("PEND", &self.PEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SSTATUS_RX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SSTATUS_RX {{ PEND: {=bool:?} }}", self.PEND())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_STCCLOCK(pub u32);
impl I3C_STCCLOCK {
    #[doc = "Clock Accuracy"]
    #[must_use]
    #[inline(always)]
    pub const fn ACCURACY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Accuracy"]
    #[inline(always)]
    pub const fn set_ACCURACY(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Accuracy"]
    #[must_use]
    #[inline(always)]
    pub const fn FREQ(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Accuracy"]
    #[inline(always)]
    pub const fn set_FREQ(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_STCCLOCK {
    #[inline(always)]
    fn default() -> I3C_STCCLOCK {
        I3C_STCCLOCK(0)
    }
}
impl core::fmt::Debug for I3C_STCCLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_STCCLOCK")
            .field("ACCURACY", &self.ACCURACY())
            .field("FREQ", &self.FREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_STCCLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_STCCLOCK {{ ACCURACY: {=u8:?}, FREQ: {=u8:?} }}",
            self.ACCURACY(),
            self.FREQ()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SVENDORID(pub u32);
impl I3C_SVENDORID {
    #[doc = "Vendor ID"]
    #[must_use]
    #[inline(always)]
    pub const fn VID(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Vendor ID"]
    #[inline(always)]
    pub const fn set_VID(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for I3C_SVENDORID {
    #[inline(always)]
    fn default() -> I3C_SVENDORID {
        I3C_SVENDORID(0)
    }
}
impl core::fmt::Debug for I3C_SVENDORID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SVENDORID")
            .field("VID", &self.VID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SVENDORID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SVENDORID {{ VID: {=u16:?} }}", self.VID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAB(pub u32);
impl I3C_SWDATAB {
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
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for I3C_SWDATAB {
    #[inline(always)]
    fn default() -> I3C_SWDATAB {
        I3C_SWDATAB(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAB")
            .field("DATA", &self.DATA())
            .field("END", &self.END())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SWDATAB {{ DATA: {=u8:?}, END: {=bool:?} }}",
            self.DATA(),
            self.END()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAB1(pub u32);
impl I3C_SWDATAB1 {
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
}
impl Default for I3C_SWDATAB1 {
    #[inline(always)]
    fn default() -> I3C_SWDATAB1 {
        I3C_SWDATAB1(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAB1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAB1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SWDATAB1 {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATABE(pub u32);
impl I3C_SWDATABE {
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
}
impl Default for I3C_SWDATABE {
    #[inline(always)]
    fn default() -> I3C_SWDATABE {
        I3C_SWDATABE(0)
    }
}
impl core::fmt::Debug for I3C_SWDATABE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATABE")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATABE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SWDATABE {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAB_END(pub u32);
impl I3C_SWDATAB_END {
    #[must_use]
    #[inline(always)]
    pub const fn ALSO(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ALSO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3C_SWDATAB_END {
    #[inline(always)]
    fn default() -> I3C_SWDATAB_END {
        I3C_SWDATAB_END(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAB_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAB_END")
            .field("ALSO", &self.ALSO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAB_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SWDATAB_END {{ ALSO: {=bool:?} }}", self.ALSO())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAH(pub u32);
impl I3C_SWDATAH {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data 1"]
    #[must_use]
    #[inline(always)]
    pub const fn END(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Data 1"]
    #[inline(always)]
    pub const fn set_END(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for I3C_SWDATAH {
    #[inline(always)]
    fn default() -> I3C_SWDATAH {
        I3C_SWDATAH(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAH")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .field("END", &self.END())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SWDATAH {{ DATA0: {=u8:?}, DATA1: {=u8:?}, END: {=bool:?} }}",
            self.DATA0(),
            self.DATA1(),
            self.END()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAH1(pub u32);
impl I3C_SWDATAH1 {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for I3C_SWDATAH1 {
    #[inline(always)]
    fn default() -> I3C_SWDATAH1 {
        I3C_SWDATAH1(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAH1")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3C_SWDATAH1 {{ DATA: {=u16:?} }}", self.DATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3C_SWDATAHE(pub u32);
impl I3C_SWDATAHE {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for I3C_SWDATAHE {
    #[inline(always)]
    fn default() -> I3C_SWDATAHE {
        I3C_SWDATAHE(0)
    }
}
impl core::fmt::Debug for I3C_SWDATAHE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_SWDATAHE")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3C_SWDATAHE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3C_SWDATAHE {{ DATA0: {=u8:?}, DATA1: {=u8:?} }}",
            self.DATA0(),
            self.DATA1()
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI {
    ptr: *mut u8,
}
unsafe impl Send for FLEXSPI {}
unsafe impl Sync for FLEXSPI {}
impl FLEXSPI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCR0(self) -> crate::common::Reg<FLEXSPI_MCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR1(self) -> crate::common::Reg<FLEXSPI_MCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR2(self) -> crate::common::Reg<FLEXSPI_MCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCR(self) -> crate::common::Reg<FLEXSPI_AHBCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<FLEXSPI_INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INTR(self) -> crate::common::Reg<FLEXSPI_INTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn LUTKEY(self) -> crate::common::Reg<FLEXSPI_LUTKEY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn LUTCR(self) -> crate::common::Reg<FLEXSPI_LUTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBRXBUFCR0(
        self,
        n: usize,
    ) -> crate::common::Reg<FLEXSPI_AHBRXBUFCR0, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR0(self, n: usize) -> crate::common::Reg<FLEXSPI_FLSHCR0, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR1(self, n: usize) -> crate::common::Reg<FLEXSPI_FLSHCR1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR2(self, n: usize) -> crate::common::Reg<FLEXSPI_FLSHCR2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR4(self) -> crate::common::Reg<FLEXSPI_FLSHCR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR0(self) -> crate::common::Reg<FLEXSPI_IPCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR1(self) -> crate::common::Reg<FLEXSPI_IPCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR2(self) -> crate::common::Reg<FLEXSPI_IPCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCMD(self) -> crate::common::Reg<FLEXSPI_IPCMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn DLPR(self) -> crate::common::Reg<FLEXSPI_DLPR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPRXFCR(self) -> crate::common::Reg<FLEXSPI_IPRXFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPTXFCR(self) -> crate::common::Reg<FLEXSPI_IPTXFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn DLLCR(self, n: usize) -> crate::common::Reg<FLEXSPI_DLLCR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn STS0(self) -> crate::common::Reg<FLEXSPI_STS0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn STS1(self) -> crate::common::Reg<FLEXSPI_STS1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn STS2(self) -> crate::common::Reg<FLEXSPI_STS2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBSPNDSTS(self) -> crate::common::Reg<FLEXSPI_AHBSPNDSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn IPRXFSTS(self) -> crate::common::Reg<FLEXSPI_IPRXFSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPTXFSTS(self) -> crate::common::Reg<FLEXSPI_IPTXFSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn RFDR(self, n: usize) -> crate::common::Reg<FLEXSPI_RFDR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn TFDR(self, n: usize) -> crate::common::Reg<FLEXSPI_TFDR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LUT(self, n: usize) -> crate::common::Reg<FLEXSPI_LUT, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn IPSNSZSTART0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZEND0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZSTART1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZEND1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONEND0_END(pub u32);
impl FLEXSPI_AHBBUFREGIONEND0_END {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONEND0_END {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONEND0_END {
        FLEXSPI_AHBBUFREGIONEND0_END(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONEND0_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONEND0_END")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONEND0_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONEND0_END {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONEND1_END(pub u32);
impl FLEXSPI_AHBBUFREGIONEND1_END {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONEND1_END {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONEND1_END {
        FLEXSPI_AHBBUFREGIONEND1_END(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONEND1_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONEND1_END")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONEND1_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONEND1_END {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONEND2_END(pub u32);
impl FLEXSPI_AHBBUFREGIONEND2_END {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONEND2_END {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONEND2_END {
        FLEXSPI_AHBBUFREGIONEND2_END(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONEND2_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONEND2_END")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONEND2_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONEND2_END {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONEND3_END(pub u32);
impl FLEXSPI_AHBBUFREGIONEND3_END {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONEND3_END {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONEND3_END {
        FLEXSPI_AHBBUFREGIONEND3_END(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONEND3_END {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONEND3_END")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONEND3_END {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONEND3_END {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONSTART0_START(pub u32);
impl FLEXSPI_AHBBUFREGIONSTART0_START {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONSTART0_START {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONSTART0_START {
        FLEXSPI_AHBBUFREGIONSTART0_START(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONSTART0_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONSTART0_START")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONSTART0_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONSTART0_START {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONSTART1_START(pub u32);
impl FLEXSPI_AHBBUFREGIONSTART1_START {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONSTART1_START {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONSTART1_START {
        FLEXSPI_AHBBUFREGIONSTART1_START(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONSTART1_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONSTART1_START")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONSTART1_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONSTART1_START {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONSTART2_START(pub u32);
impl FLEXSPI_AHBBUFREGIONSTART2_START {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONSTART2_START {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONSTART2_START {
        FLEXSPI_AHBBUFREGIONSTART2_START(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONSTART2_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONSTART2_START")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONSTART2_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONSTART2_START {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBBUFREGIONSTART3_START(pub u32);
impl FLEXSPI_AHBBUFREGIONSTART3_START {
    #[must_use]
    #[inline(always)]
    pub const fn ADDRESS(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_ADDRESS(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_AHBBUFREGIONSTART3_START {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBBUFREGIONSTART3_START {
        FLEXSPI_AHBBUFREGIONSTART3_START(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBBUFREGIONSTART3_START {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBBUFREGIONSTART3_START")
            .field("ADDRESS", &self.ADDRESS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBBUFREGIONSTART3_START {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBBUFREGIONSTART3_START {{ ADDRESS: {=u32:?} }}",
            self.ADDRESS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBCR(pub u32);
impl FLEXSPI_AHBCR {
    #[must_use]
    #[inline(always)]
    pub const fn CLRAHBTXBUF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRAHBTXBUF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CACHABLEEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CACHABLEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BUFFERABLEEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUFFERABLEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PREFETCHEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PREFETCHEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn READADDROPT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_READADDROPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEDISABLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RESUMEDISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn READSZALIGN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_READSZALIGN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ALIGNMENT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ALIGNMENT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AFLASHBASE(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_AFLASHBASE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for FLEXSPI_AHBCR {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBCR {
        FLEXSPI_AHBCR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBCR")
            .field("CLRAHBTXBUF", &self.CLRAHBTXBUF())
            .field("CACHABLEEN", &self.CACHABLEEN())
            .field("BUFFERABLEEN", &self.BUFFERABLEEN())
            .field("PREFETCHEN", &self.PREFETCHEN())
            .field("READADDROPT", &self.READADDROPT())
            .field("RESUMEDISABLE", &self.RESUMEDISABLE())
            .field("READSZALIGN", &self.READSZALIGN())
            .field("ALIGNMENT", &self.ALIGNMENT())
            .field("AFLASHBASE", &self.AFLASHBASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBCR {{ CLRAHBTXBUF: {=bool:?}, CACHABLEEN: {=bool:?}, BUFFERABLEEN: {=bool:?}, PREFETCHEN: {=bool:?}, READADDROPT: {=bool:?}, RESUMEDISABLE: {=bool:?}, READSZALIGN: {=bool:?}, ALIGNMENT: {=u8:?}, AFLASHBASE: {=u8:?} }}",
            self.CLRAHBTXBUF(),
            self.CACHABLEEN(),
            self.BUFFERABLEEN(),
            self.PREFETCHEN(),
            self.READADDROPT(),
            self.RESUMEDISABLE(),
            self.READSZALIGN(),
            self.ALIGNMENT(),
            self.AFLASHBASE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBRXBUFCR0(pub u32);
impl FLEXSPI_AHBRXBUFCR0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn BUFSZ(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_BUFSZ(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn MSTRID(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_MSTRID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIORITY(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_PRIORITY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn REGIONEN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REGIONEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PREFETCHEN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PREFETCHEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FLEXSPI_AHBRXBUFCR0 {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBRXBUFCR0 {
        FLEXSPI_AHBRXBUFCR0(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBRXBUFCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBRXBUFCR0")
            .field("BUFSZ", &self.BUFSZ())
            .field("MSTRID", &self.MSTRID())
            .field("PRIORITY", &self.PRIORITY())
            .field("REGIONEN", &self.REGIONEN())
            .field("PREFETCHEN", &self.PREFETCHEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBRXBUFCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBRXBUFCR0 {{ BUFSZ: {=u16:?}, MSTRID: {=u8:?}, PRIORITY: {=u8:?}, REGIONEN: {=bool:?}, PREFETCHEN: {=bool:?} }}",
            self.BUFSZ(),
            self.MSTRID(),
            self.PRIORITY(),
            self.REGIONEN(),
            self.PREFETCHEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_AHBSPNDSTS(pub u32);
impl FLEXSPI_AHBSPNDSTS {
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BUFID(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BUFID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Data Left"]
    #[must_use]
    #[inline(always)]
    pub const fn DATLFT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Left"]
    #[inline(always)]
    pub const fn set_DATLFT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLEXSPI_AHBSPNDSTS {
    #[inline(always)]
    fn default() -> FLEXSPI_AHBSPNDSTS {
        FLEXSPI_AHBSPNDSTS(0)
    }
}
impl core::fmt::Debug for FLEXSPI_AHBSPNDSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_AHBSPNDSTS")
            .field("ACTIVE", &self.ACTIVE())
            .field("BUFID", &self.BUFID())
            .field("DATLFT", &self.DATLFT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_AHBSPNDSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_AHBSPNDSTS {{ ACTIVE: {=bool:?}, BUFID: {=u8:?}, DATLFT: {=u16:?} }}",
            self.ACTIVE(),
            self.BUFID(),
            self.DATLFT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_DLLCR(pub u32);
impl FLEXSPI_DLLCR {
    #[must_use]
    #[inline(always)]
    pub const fn DLLEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DLLRESET(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DLLRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Target Delay Line"]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDLYTARGET(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Target Delay Line"]
    #[inline(always)]
    pub const fn set_SLVDLYTARGET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Target Delay Line"]
    #[must_use]
    #[inline(always)]
    pub const fn OVRDEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Target Delay Line"]
    #[inline(always)]
    pub const fn set_OVRDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OVRDVAL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_OVRDVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[must_use]
    #[inline(always)]
    pub const fn REFPHASEGAP(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[inline(always)]
    pub const fn set_REFPHASEGAP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
}
impl Default for FLEXSPI_DLLCR {
    #[inline(always)]
    fn default() -> FLEXSPI_DLLCR {
        FLEXSPI_DLLCR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_DLLCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_DLLCR")
            .field("DLLEN", &self.DLLEN())
            .field("DLLRESET", &self.DLLRESET())
            .field("SLVDLYTARGET", &self.SLVDLYTARGET())
            .field("OVRDEN", &self.OVRDEN())
            .field("OVRDVAL", &self.OVRDVAL())
            .field("REFPHASEGAP", &self.REFPHASEGAP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_DLLCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_DLLCR {{ DLLEN: {=bool:?}, DLLRESET: {=bool:?}, SLVDLYTARGET: {=u8:?}, OVRDEN: {=bool:?}, OVRDVAL: {=u8:?}, REFPHASEGAP: {=u8:?} }}",
            self.DLLEN(),
            self.DLLRESET(),
            self.SLVDLYTARGET(),
            self.OVRDEN(),
            self.OVRDVAL(),
            self.REFPHASEGAP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_DLPR(pub u32);
impl FLEXSPI_DLPR {
    #[doc = "Data Learning Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn DLP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Learning Pattern"]
    #[inline(always)]
    pub const fn set_DLP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXSPI_DLPR {
    #[inline(always)]
    fn default() -> FLEXSPI_DLPR {
        FLEXSPI_DLPR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_DLPR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_DLPR")
            .field("DLP", &self.DLP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_DLPR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_DLPR {{ DLP: {=u32:?} }}", self.DLP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_FLSHCR0(pub u32);
impl FLEXSPI_FLSHCR0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn FLSHSZ(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_FLSHSZ(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn ADDRSHIFT(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_ADDRSHIFT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for FLEXSPI_FLSHCR0 {
    #[inline(always)]
    fn default() -> FLEXSPI_FLSHCR0 {
        FLEXSPI_FLSHCR0(0)
    }
}
impl core::fmt::Debug for FLEXSPI_FLSHCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_FLSHCR0")
            .field("FLSHSZ", &self.FLSHSZ())
            .field("ADDRSHIFT", &self.ADDRSHIFT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_FLSHCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_FLSHCR0 {{ FLSHSZ: {=u32:?}, ADDRSHIFT: {=bool:?} }}",
            self.FLSHSZ(),
            self.ADDRSHIFT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_FLSHCR1(pub u32);
impl FLEXSPI_FLSHCR1 {
    #[doc = "Serial Flash CS Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn TCSS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Setup Time"]
    #[inline(always)]
    pub const fn set_TCSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn TCSH(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Setup Time"]
    #[inline(always)]
    pub const fn set_TCSH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn WA(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[inline(always)]
    pub const fn set_WA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size"]
    #[must_use]
    #[inline(always)]
    pub const fn CAS(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size"]
    #[inline(always)]
    pub const fn set_CAS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Column Address Size"]
    #[must_use]
    #[inline(always)]
    pub const fn CSINTERVALUNIT(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Column Address Size"]
    #[inline(always)]
    pub const fn set_CSINTERVALUNIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Chip Select Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn CSINTERVAL(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip Select Interval"]
    #[inline(always)]
    pub const fn set_CSINTERVAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLEXSPI_FLSHCR1 {
    #[inline(always)]
    fn default() -> FLEXSPI_FLSHCR1 {
        FLEXSPI_FLSHCR1(0)
    }
}
impl core::fmt::Debug for FLEXSPI_FLSHCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_FLSHCR1")
            .field("TCSS", &self.TCSS())
            .field("TCSH", &self.TCSH())
            .field("WA", &self.WA())
            .field("CAS", &self.CAS())
            .field("CSINTERVALUNIT", &self.CSINTERVALUNIT())
            .field("CSINTERVAL", &self.CSINTERVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_FLSHCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_FLSHCR1 {{ TCSS: {=u8:?}, TCSH: {=u8:?}, WA: {=bool:?}, CAS: {=u8:?}, CSINTERVALUNIT: {=bool:?}, CSINTERVAL: {=u16:?} }}",
            self.TCSS(),
            self.TCSH(),
            self.WA(),
            self.CAS(),
            self.CSINTERVALUNIT(),
            self.CSINTERVAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_FLSHCR2(pub u32);
impl FLEXSPI_FLSHCR2 {
    #[must_use]
    #[inline(always)]
    pub const fn ARDSEQID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ARDSEQID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ARDSEQNUM(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ARDSEQNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AWRSEQID(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_AWRSEQID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AWRSEQNUM(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_AWRSEQNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "AHB Write Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn AWRWAIT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "AHB Write Wait"]
    #[inline(always)]
    pub const fn set_AWRWAIT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AHB Write Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn AWRWAITUNIT(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Write Wait"]
    #[inline(always)]
    pub const fn set_AWRWAITUNIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLRINSTRPTR(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRINSTRPTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FLEXSPI_FLSHCR2 {
    #[inline(always)]
    fn default() -> FLEXSPI_FLSHCR2 {
        FLEXSPI_FLSHCR2(0)
    }
}
impl core::fmt::Debug for FLEXSPI_FLSHCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_FLSHCR2")
            .field("ARDSEQID", &self.ARDSEQID())
            .field("ARDSEQNUM", &self.ARDSEQNUM())
            .field("AWRSEQID", &self.AWRSEQID())
            .field("AWRSEQNUM", &self.AWRSEQNUM())
            .field("AWRWAIT", &self.AWRWAIT())
            .field("AWRWAITUNIT", &self.AWRWAITUNIT())
            .field("CLRINSTRPTR", &self.CLRINSTRPTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_FLSHCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_FLSHCR2 {{ ARDSEQID: {=u8:?}, ARDSEQNUM: {=u8:?}, AWRSEQID: {=u8:?}, AWRSEQNUM: {=u8:?}, AWRWAIT: {=u16:?}, AWRWAITUNIT: {=u8:?}, CLRINSTRPTR: {=bool:?} }}",
            self.ARDSEQID(),
            self.ARDSEQNUM(),
            self.AWRSEQID(),
            self.AWRSEQNUM(),
            self.AWRWAIT(),
            self.AWRWAITUNIT(),
            self.CLRINSTRPTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_FLSHCR4(pub u32);
impl FLEXSPI_FLSHCR4 {
    #[must_use]
    #[inline(always)]
    pub const fn WMOPT1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WMOPT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WMOPT2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WMOPT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WMENA(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WMENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for FLEXSPI_FLSHCR4 {
    #[inline(always)]
    fn default() -> FLEXSPI_FLSHCR4 {
        FLEXSPI_FLSHCR4(0)
    }
}
impl core::fmt::Debug for FLEXSPI_FLSHCR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_FLSHCR4")
            .field("WMOPT1", &self.WMOPT1())
            .field("WMOPT2", &self.WMOPT2())
            .field("WMENA", &self.WMENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_FLSHCR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_FLSHCR4 {{ WMOPT1: {=bool:?}, WMOPT2: {=bool:?}, WMENA: {=bool:?} }}",
            self.WMOPT1(),
            self.WMOPT2(),
            self.WMENA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_INTEN(pub u32);
impl FLEXSPI_INTEN {
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDDONEEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDDONEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDGEEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDGEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDGEEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBCMDGEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDERREN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDERREN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBCMDERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPRXWAEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPRXWAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPTXWEEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPTXWEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DATALEARNFAILEN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DATALEARNFAILEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SCKSTOPBYRDEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SCKSTOPBYRDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SCKSTOPBYWREN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SCKSTOPBYWREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBBUSERROREN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBBUSERROREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SEQTIMEOUTEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEQTIMEOUTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn KEYDONEEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_KEYDONEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn KEYERROREN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_KEYERROREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDSECUREVIOEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDSECUREVIOEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FLEXSPI_INTEN {
    #[inline(always)]
    fn default() -> FLEXSPI_INTEN {
        FLEXSPI_INTEN(0)
    }
}
impl core::fmt::Debug for FLEXSPI_INTEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_INTEN")
            .field("IPCMDDONEEN", &self.IPCMDDONEEN())
            .field("IPCMDGEEN", &self.IPCMDGEEN())
            .field("AHBCMDGEEN", &self.AHBCMDGEEN())
            .field("IPCMDERREN", &self.IPCMDERREN())
            .field("AHBCMDERREN", &self.AHBCMDERREN())
            .field("IPRXWAEN", &self.IPRXWAEN())
            .field("IPTXWEEN", &self.IPTXWEEN())
            .field("DATALEARNFAILEN", &self.DATALEARNFAILEN())
            .field("SCKSTOPBYRDEN", &self.SCKSTOPBYRDEN())
            .field("SCKSTOPBYWREN", &self.SCKSTOPBYWREN())
            .field("AHBBUSERROREN", &self.AHBBUSERROREN())
            .field("SEQTIMEOUTEN", &self.SEQTIMEOUTEN())
            .field("KEYDONEEN", &self.KEYDONEEN())
            .field("KEYERROREN", &self.KEYERROREN())
            .field("IPCMDSECUREVIOEN", &self.IPCMDSECUREVIOEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_INTEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_INTEN {{ IPCMDDONEEN: {=bool:?}, IPCMDGEEN: {=bool:?}, AHBCMDGEEN: {=bool:?}, IPCMDERREN: {=bool:?}, AHBCMDERREN: {=bool:?}, IPRXWAEN: {=bool:?}, IPTXWEEN: {=bool:?}, DATALEARNFAILEN: {=bool:?}, SCKSTOPBYRDEN: {=bool:?}, SCKSTOPBYWREN: {=bool:?}, AHBBUSERROREN: {=bool:?}, SEQTIMEOUTEN: {=bool:?}, KEYDONEEN: {=bool:?}, KEYERROREN: {=bool:?}, IPCMDSECUREVIOEN: {=bool:?} }}",
            self.IPCMDDONEEN(),
            self.IPCMDGEEN(),
            self.AHBCMDGEEN(),
            self.IPCMDERREN(),
            self.AHBCMDERREN(),
            self.IPRXWAEN(),
            self.IPTXWEEN(),
            self.DATALEARNFAILEN(),
            self.SCKSTOPBYRDEN(),
            self.SCKSTOPBYWREN(),
            self.AHBBUSERROREN(),
            self.SEQTIMEOUTEN(),
            self.KEYDONEEN(),
            self.KEYERROREN(),
            self.IPCMDSECUREVIOEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_INTR(pub u32);
impl FLEXSPI_INTR {
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDDONE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDGE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDGE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBCMDGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDERR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBCMDERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPRXWA(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPRXWA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPTXWE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPTXWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DATALEARNFAIL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DATALEARNFAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SCKSTOPBYRD(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SCKSTOPBYRD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SCKSTOPBYWR(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SCKSTOPBYWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBBUSERROR(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AHBBUSERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SEQTIMEOUT(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEQTIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn KEYDONE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_KEYDONE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OTFAD key blob processing done interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYERROR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD key blob processing done interrupt."]
    #[inline(always)]
    pub const fn set_KEYERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDSECUREVIO(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPCMDSECUREVIO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FLEXSPI_INTR {
    #[inline(always)]
    fn default() -> FLEXSPI_INTR {
        FLEXSPI_INTR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_INTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_INTR")
            .field("IPCMDDONE", &self.IPCMDDONE())
            .field("IPCMDGE", &self.IPCMDGE())
            .field("AHBCMDGE", &self.AHBCMDGE())
            .field("IPCMDERR", &self.IPCMDERR())
            .field("AHBCMDERR", &self.AHBCMDERR())
            .field("IPRXWA", &self.IPRXWA())
            .field("IPTXWE", &self.IPTXWE())
            .field("DATALEARNFAIL", &self.DATALEARNFAIL())
            .field("SCKSTOPBYRD", &self.SCKSTOPBYRD())
            .field("SCKSTOPBYWR", &self.SCKSTOPBYWR())
            .field("AHBBUSERROR", &self.AHBBUSERROR())
            .field("SEQTIMEOUT", &self.SEQTIMEOUT())
            .field("KEYDONE", &self.KEYDONE())
            .field("KEYERROR", &self.KEYERROR())
            .field("IPCMDSECUREVIO", &self.IPCMDSECUREVIO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_INTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_INTR {{ IPCMDDONE: {=bool:?}, IPCMDGE: {=bool:?}, AHBCMDGE: {=bool:?}, IPCMDERR: {=bool:?}, AHBCMDERR: {=bool:?}, IPRXWA: {=bool:?}, IPTXWE: {=bool:?}, DATALEARNFAIL: {=bool:?}, SCKSTOPBYRD: {=bool:?}, SCKSTOPBYWR: {=bool:?}, AHBBUSERROR: {=bool:?}, SEQTIMEOUT: {=bool:?}, KEYDONE: {=bool:?}, KEYERROR: {=bool:?}, IPCMDSECUREVIO: {=bool:?} }}",
            self.IPCMDDONE(),
            self.IPCMDGE(),
            self.AHBCMDGE(),
            self.IPCMDERR(),
            self.AHBCMDERR(),
            self.IPRXWA(),
            self.IPTXWE(),
            self.DATALEARNFAIL(),
            self.SCKSTOPBYRD(),
            self.SCKSTOPBYWR(),
            self.AHBBUSERROR(),
            self.SEQTIMEOUT(),
            self.KEYDONE(),
            self.KEYERROR(),
            self.IPCMDSECUREVIO()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPCMD(pub u32);
impl FLEXSPI_IPCMD {
    #[must_use]
    #[inline(always)]
    pub const fn TRG(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for FLEXSPI_IPCMD {
    #[inline(always)]
    fn default() -> FLEXSPI_IPCMD {
        FLEXSPI_IPCMD(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPCMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPCMD")
            .field("TRG", &self.TRG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPCMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_IPCMD {{ TRG: {=bool:?} }}", self.TRG())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPCR0(pub u32);
impl FLEXSPI_IPCR0 {
    #[doc = "Serial Flash Address"]
    #[must_use]
    #[inline(always)]
    pub const fn SFAR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Serial Flash Address"]
    #[inline(always)]
    pub const fn set_SFAR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXSPI_IPCR0 {
    #[inline(always)]
    fn default() -> FLEXSPI_IPCR0 {
        FLEXSPI_IPCR0(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPCR0")
            .field("SFAR", &self.SFAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_IPCR0 {{ SFAR: {=u32:?} }}", self.SFAR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPCR1(pub u32);
impl FLEXSPI_IPCR1 {
    #[must_use]
    #[inline(always)]
    pub const fn IDATSZ(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_IDATSZ(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ISEQID(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ISEQID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn ISEQNUM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn set_ISEQNUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for FLEXSPI_IPCR1 {
    #[inline(always)]
    fn default() -> FLEXSPI_IPCR1 {
        FLEXSPI_IPCR1(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPCR1")
            .field("IDATSZ", &self.IDATSZ())
            .field("ISEQID", &self.ISEQID())
            .field("ISEQNUM", &self.ISEQNUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPCR1 {{ IDATSZ: {=u16:?}, ISEQID: {=u8:?}, ISEQNUM: {=u8:?} }}",
            self.IDATSZ(),
            self.ISEQID(),
            self.ISEQNUM()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPCR2(pub u32);
impl FLEXSPI_IPCR2 {
    #[must_use]
    #[inline(always)]
    pub const fn IPBLKAHBREQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPBLKAHBREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPBLKAHBACK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPBLKAHBACK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPBLKALLAHB(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IPBLKALLAHB(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for FLEXSPI_IPCR2 {
    #[inline(always)]
    fn default() -> FLEXSPI_IPCR2 {
        FLEXSPI_IPCR2(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPCR2")
            .field("IPBLKAHBREQ", &self.IPBLKAHBREQ())
            .field("IPBLKAHBACK", &self.IPBLKAHBACK())
            .field("IPBLKALLAHB", &self.IPBLKALLAHB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPCR2 {{ IPBLKAHBREQ: {=bool:?}, IPBLKAHBACK: {=bool:?}, IPBLKALLAHB: {=bool:?} }}",
            self.IPBLKAHBREQ(),
            self.IPBLKAHBACK(),
            self.IPBLKALLAHB()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPRXFCR(pub u32);
impl FLEXSPI_IPRXFCR {
    #[must_use]
    #[inline(always)]
    pub const fn CLRIPRXF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRIPRXF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXDMAEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_RXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXWMRK(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXWMRK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
}
impl Default for FLEXSPI_IPRXFCR {
    #[inline(always)]
    fn default() -> FLEXSPI_IPRXFCR {
        FLEXSPI_IPRXFCR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPRXFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPRXFCR")
            .field("CLRIPRXF", &self.CLRIPRXF())
            .field("RXDMAEN", &self.RXDMAEN())
            .field("RXWMRK", &self.RXWMRK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPRXFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPRXFCR {{ CLRIPRXF: {=bool:?}, RXDMAEN: {=bool:?}, RXWMRK: {=u8:?} }}",
            self.CLRIPRXF(),
            self.RXDMAEN(),
            self.RXWMRK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPRXFSTS(pub u32);
impl FLEXSPI_IPRXFSTS {
    #[doc = "Fill Level of IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn FILL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_FILL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Fill Level of IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn RDCNTR(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fill Level of IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_RDCNTR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLEXSPI_IPRXFSTS {
    #[inline(always)]
    fn default() -> FLEXSPI_IPRXFSTS {
        FLEXSPI_IPRXFSTS(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPRXFSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPRXFSTS")
            .field("FILL", &self.FILL())
            .field("RDCNTR", &self.RDCNTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPRXFSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPRXFSTS {{ FILL: {=u8:?}, RDCNTR: {=u16:?} }}",
            self.FILL(),
            self.RDCNTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPSNSZEND0_end(pub u32);
impl FLEXSPI_IPSNSZEND0_end {
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_IPSNSZEND0_end {
    #[inline(always)]
    fn default() -> FLEXSPI_IPSNSZEND0_end {
        FLEXSPI_IPSNSZEND0_end(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPSNSZEND0_end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPSNSZEND0_end")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPSNSZEND0_end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPSNSZEND0_end {{ address: {=u32:?} }}",
            self.address()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPSNSZEND1_end(pub u32);
impl FLEXSPI_IPSNSZEND1_end {
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_IPSNSZEND1_end {
    #[inline(always)]
    fn default() -> FLEXSPI_IPSNSZEND1_end {
        FLEXSPI_IPSNSZEND1_end(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPSNSZEND1_end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPSNSZEND1_end")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPSNSZEND1_end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPSNSZEND1_end {{ address: {=u32:?} }}",
            self.address()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPSNSZSTART0_start(pub u32);
impl FLEXSPI_IPSNSZSTART0_start {
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_IPSNSZSTART0_start {
    #[inline(always)]
    fn default() -> FLEXSPI_IPSNSZSTART0_start {
        FLEXSPI_IPSNSZSTART0_start(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPSNSZSTART0_start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPSNSZSTART0_start")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPSNSZSTART0_start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPSNSZSTART0_start {{ address: {=u32:?} }}",
            self.address()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPSNSZSTART1_start(pub u32);
impl FLEXSPI_IPSNSZSTART1_start {
    #[must_use]
    #[inline(always)]
    pub const fn address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for FLEXSPI_IPSNSZSTART1_start {
    #[inline(always)]
    fn default() -> FLEXSPI_IPSNSZSTART1_start {
        FLEXSPI_IPSNSZSTART1_start(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPSNSZSTART1_start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPSNSZSTART1_start")
            .field("address", &self.address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPSNSZSTART1_start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPSNSZSTART1_start {{ address: {=u32:?} }}",
            self.address()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPTXFCR(pub u32);
impl FLEXSPI_IPTXFCR {
    #[must_use]
    #[inline(always)]
    pub const fn CLRIPTXF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRIPTXF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TXDMAEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TXDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn TXWMRK(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit Watermark Level"]
    #[inline(always)]
    pub const fn set_TXWMRK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for FLEXSPI_IPTXFCR {
    #[inline(always)]
    fn default() -> FLEXSPI_IPTXFCR {
        FLEXSPI_IPTXFCR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPTXFCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPTXFCR")
            .field("CLRIPTXF", &self.CLRIPTXF())
            .field("TXDMAEN", &self.TXDMAEN())
            .field("TXWMRK", &self.TXWMRK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPTXFCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPTXFCR {{ CLRIPTXF: {=bool:?}, TXDMAEN: {=bool:?}, TXWMRK: {=u8:?} }}",
            self.CLRIPTXF(),
            self.TXDMAEN(),
            self.TXWMRK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_IPTXFSTS(pub u32);
impl FLEXSPI_IPTXFSTS {
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn FILL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_FILL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn WRCNTR(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_WRCNTR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLEXSPI_IPTXFSTS {
    #[inline(always)]
    fn default() -> FLEXSPI_IPTXFSTS {
        FLEXSPI_IPTXFSTS(0)
    }
}
impl core::fmt::Debug for FLEXSPI_IPTXFSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_IPTXFSTS")
            .field("FILL", &self.FILL())
            .field("WRCNTR", &self.WRCNTR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_IPTXFSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_IPTXFSTS {{ FILL: {=u8:?}, WRCNTR: {=u16:?} }}",
            self.FILL(),
            self.WRCNTR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_LUT(pub u32);
impl FLEXSPI_LUT {
    #[doc = "OPERAND0"]
    #[must_use]
    #[inline(always)]
    pub const fn OPERAND0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub const fn set_OPERAND0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NUM_PADS0"]
    #[must_use]
    #[inline(always)]
    pub const fn OPCODE0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "NUM_PADS0"]
    #[inline(always)]
    pub const fn set_OPCODE0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "OPCODE"]
    #[must_use]
    #[inline(always)]
    pub const fn OPERAND1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPCODE"]
    #[inline(always)]
    pub const fn set_OPERAND1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NUM_PADS1"]
    #[must_use]
    #[inline(always)]
    pub const fn OPCODE1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "NUM_PADS1"]
    #[inline(always)]
    pub const fn set_OPCODE1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for FLEXSPI_LUT {
    #[inline(always)]
    fn default() -> FLEXSPI_LUT {
        FLEXSPI_LUT(0)
    }
}
impl core::fmt::Debug for FLEXSPI_LUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_LUT")
            .field("OPERAND0", &self.OPERAND0())
            .field("OPCODE0", &self.OPCODE0())
            .field("OPERAND1", &self.OPERAND1())
            .field("OPCODE1", &self.OPCODE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_LUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_LUT {{ OPERAND0: {=u8:?}, OPCODE0: {=u8:?}, OPERAND1: {=u8:?}, OPCODE1: {=u8:?} }}",
            self.OPERAND0(),
            self.OPCODE0(),
            self.OPERAND1(),
            self.OPCODE1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_LUTCR(pub u32);
impl FLEXSPI_LUTCR {
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn UNLOCK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_UNLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PROTECT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PROTECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for FLEXSPI_LUTCR {
    #[inline(always)]
    fn default() -> FLEXSPI_LUTCR {
        FLEXSPI_LUTCR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_LUTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_LUTCR")
            .field("LOCK", &self.LOCK())
            .field("UNLOCK", &self.UNLOCK())
            .field("PROTECT", &self.PROTECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_LUTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_LUTCR {{ LOCK: {=bool:?}, UNLOCK: {=bool:?}, PROTECT: {=bool:?} }}",
            self.LOCK(),
            self.UNLOCK(),
            self.PROTECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_LUTKEY(pub u32);
impl FLEXSPI_LUTKEY {
    #[doc = "LUT Key"]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LUT Key"]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXSPI_LUTKEY {
    #[inline(always)]
    fn default() -> FLEXSPI_LUTKEY {
        FLEXSPI_LUTKEY(0)
    }
}
impl core::fmt::Debug for FLEXSPI_LUTKEY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_LUTKEY")
            .field("KEY", &self.KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_LUTKEY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_LUTKEY {{ KEY: {=u32:?} }}", self.KEY())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_LUT_NUM(pub u32);
impl FLEXSPI_LUT_NUM {
    #[doc = "OPERAND0"]
    #[must_use]
    #[inline(always)]
    pub const fn PADS0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub const fn set_PADS0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "OPERAND1"]
    #[must_use]
    #[inline(always)]
    pub const fn PADS1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "OPERAND1"]
    #[inline(always)]
    pub const fn set_PADS1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for FLEXSPI_LUT_NUM {
    #[inline(always)]
    fn default() -> FLEXSPI_LUT_NUM {
        FLEXSPI_LUT_NUM(0)
    }
}
impl core::fmt::Debug for FLEXSPI_LUT_NUM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_LUT_NUM")
            .field("PADS0", &self.PADS0())
            .field("PADS1", &self.PADS1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_LUT_NUM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_LUT_NUM {{ PADS0: {=u8:?}, PADS1: {=u8:?} }}",
            self.PADS0(),
            self.PADS1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_MCR0(pub u32);
impl FLEXSPI_MCR0 {
    #[must_use]
    #[inline(always)]
    pub const fn SWRESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SWRESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MDIS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn RXCLKSRC(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_RXCLKSRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ARDFEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ARDFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ATDFEN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ATDFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SERCLKDIV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_SERCLKDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn HSEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_HSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOZEEN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOZEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SCKFREERUNEN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SCKFREERUNEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LEARNEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LEARNEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn IPGRANTWAIT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_IPGRANTWAIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AHBGRANTWAIT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_AHBGRANTWAIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLEXSPI_MCR0 {
    #[inline(always)]
    fn default() -> FLEXSPI_MCR0 {
        FLEXSPI_MCR0(0)
    }
}
impl core::fmt::Debug for FLEXSPI_MCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_MCR0")
            .field("SWRESET", &self.SWRESET())
            .field("MDIS", &self.MDIS())
            .field("RXCLKSRC", &self.RXCLKSRC())
            .field("ARDFEN", &self.ARDFEN())
            .field("ATDFEN", &self.ATDFEN())
            .field("SERCLKDIV", &self.SERCLKDIV())
            .field("HSEN", &self.HSEN())
            .field("DOZEEN", &self.DOZEEN())
            .field("SCKFREERUNEN", &self.SCKFREERUNEN())
            .field("LEARNEN", &self.LEARNEN())
            .field("IPGRANTWAIT", &self.IPGRANTWAIT())
            .field("AHBGRANTWAIT", &self.AHBGRANTWAIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_MCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_MCR0 {{ SWRESET: {=bool:?}, MDIS: {=bool:?}, RXCLKSRC: {=u8:?}, ARDFEN: {=bool:?}, ATDFEN: {=bool:?}, SERCLKDIV: {=u8:?}, HSEN: {=bool:?}, DOZEEN: {=bool:?}, SCKFREERUNEN: {=bool:?}, LEARNEN: {=bool:?}, IPGRANTWAIT: {=u8:?}, AHBGRANTWAIT: {=u8:?} }}",
            self.SWRESET(),
            self.MDIS(),
            self.RXCLKSRC(),
            self.ARDFEN(),
            self.ATDFEN(),
            self.SERCLKDIV(),
            self.HSEN(),
            self.DOZEEN(),
            self.SCKFREERUNEN(),
            self.LEARNEN(),
            self.IPGRANTWAIT(),
            self.AHBGRANTWAIT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_MCR1(pub u32);
impl FLEXSPI_MCR1 {
    #[doc = "AHB Bus Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn AHBBUSWAIT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Bus Wait"]
    #[inline(always)]
    pub const fn set_AHBBUSWAIT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "AHB Bus Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn SEQWAIT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Bus Wait"]
    #[inline(always)]
    pub const fn set_SEQWAIT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FLEXSPI_MCR1 {
    #[inline(always)]
    fn default() -> FLEXSPI_MCR1 {
        FLEXSPI_MCR1(0)
    }
}
impl core::fmt::Debug for FLEXSPI_MCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_MCR1")
            .field("AHBBUSWAIT", &self.AHBBUSWAIT())
            .field("SEQWAIT", &self.SEQWAIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_MCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_MCR1 {{ AHBBUSWAIT: {=u16:?}, SEQWAIT: {=u16:?} }}",
            self.AHBBUSWAIT(),
            self.SEQWAIT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_MCR2(pub u32);
impl FLEXSPI_MCR2 {
    #[must_use]
    #[inline(always)]
    pub const fn CLRAHBBUFOPT(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRAHBBUFOPT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLRLEARNPHASE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLRLEARNPHASE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SAMEDEVICEEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SAMEDEVICEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Resume Wait Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEWAIT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Resume Wait Duration"]
    #[inline(always)]
    pub const fn set_RESUMEWAIT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for FLEXSPI_MCR2 {
    #[inline(always)]
    fn default() -> FLEXSPI_MCR2 {
        FLEXSPI_MCR2(0)
    }
}
impl core::fmt::Debug for FLEXSPI_MCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_MCR2")
            .field("CLRAHBBUFOPT", &self.CLRAHBBUFOPT())
            .field("CLRLEARNPHASE", &self.CLRLEARNPHASE())
            .field("SAMEDEVICEEN", &self.SAMEDEVICEEN())
            .field("RESUMEWAIT", &self.RESUMEWAIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_MCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_MCR2 {{ CLRAHBBUFOPT: {=bool:?}, CLRLEARNPHASE: {=bool:?}, SAMEDEVICEEN: {=bool:?}, RESUMEWAIT: {=u8:?} }}",
            self.CLRAHBBUFOPT(),
            self.CLRLEARNPHASE(),
            self.SAMEDEVICEEN(),
            self.RESUMEWAIT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_RFDR(pub u32);
impl FLEXSPI_RFDR {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_RXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXSPI_RFDR {
    #[inline(always)]
    fn default() -> FLEXSPI_RFDR {
        FLEXSPI_RFDR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_RFDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_RFDR")
            .field("RXDATA", &self.RXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_RFDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_RFDR {{ RXDATA: {=u32:?} }}", self.RXDATA())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_STS0(pub u32);
impl FLEXSPI_STS0 {
    #[must_use]
    #[inline(always)]
    pub const fn SEQIDLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEQIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ARBIDLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ARBIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ARBCMDSRC(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ARBCMDSRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DATALEARNPHASEA(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DATALEARNPHASEA(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for FLEXSPI_STS0 {
    #[inline(always)]
    fn default() -> FLEXSPI_STS0 {
        FLEXSPI_STS0(0)
    }
}
impl core::fmt::Debug for FLEXSPI_STS0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_STS0")
            .field("SEQIDLE", &self.SEQIDLE())
            .field("ARBIDLE", &self.ARBIDLE())
            .field("ARBCMDSRC", &self.ARBCMDSRC())
            .field("DATALEARNPHASEA", &self.DATALEARNPHASEA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_STS0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_STS0 {{ SEQIDLE: {=bool:?}, ARBIDLE: {=bool:?}, ARBCMDSRC: {=u8:?}, DATALEARNPHASEA: {=u8:?} }}",
            self.SEQIDLE(),
            self.ARBIDLE(),
            self.ARBCMDSRC(),
            self.DATALEARNPHASEA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_STS1(pub u32);
impl FLEXSPI_STS1 {
    #[doc = "AHB Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDERRID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Command Error ID"]
    #[inline(always)]
    pub const fn set_AHBCMDERRID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "AHB Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn AHBCMDERRCODE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Command Error ID"]
    #[inline(always)]
    pub const fn set_AHBCMDERRCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "IP Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDERRID(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "IP Command Error ID"]
    #[inline(always)]
    pub const fn set_IPCMDERRID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "IP Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCMDERRCODE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IP Command Error ID"]
    #[inline(always)]
    pub const fn set_IPCMDERRCODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for FLEXSPI_STS1 {
    #[inline(always)]
    fn default() -> FLEXSPI_STS1 {
        FLEXSPI_STS1(0)
    }
}
impl core::fmt::Debug for FLEXSPI_STS1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_STS1")
            .field("AHBCMDERRID", &self.AHBCMDERRID())
            .field("AHBCMDERRCODE", &self.AHBCMDERRCODE())
            .field("IPCMDERRID", &self.IPCMDERRID())
            .field("IPCMDERRCODE", &self.IPCMDERRCODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_STS1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_STS1 {{ AHBCMDERRID: {=u8:?}, AHBCMDERRCODE: {=u8:?}, IPCMDERRID: {=u8:?}, IPCMDERRCODE: {=u8:?} }}",
            self.AHBCMDERRID(),
            self.AHBCMDERRCODE(),
            self.IPCMDERRID(),
            self.IPCMDERRCODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_STS2(pub u32);
impl FLEXSPI_STS2 {
    #[must_use]
    #[inline(always)]
    pub const fn ASLVLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ASLVLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AREFLOCK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AREFLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ASLVSEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ASLVSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AREFSEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_AREFSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BSLVLOCK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BSLVLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BREFLOCK(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BREFLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BSLVSEL(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BSLVSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BREFSEL(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BREFSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for FLEXSPI_STS2 {
    #[inline(always)]
    fn default() -> FLEXSPI_STS2 {
        FLEXSPI_STS2(0)
    }
}
impl core::fmt::Debug for FLEXSPI_STS2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_STS2")
            .field("ASLVLOCK", &self.ASLVLOCK())
            .field("AREFLOCK", &self.AREFLOCK())
            .field("ASLVSEL", &self.ASLVSEL())
            .field("AREFSEL", &self.AREFSEL())
            .field("BSLVLOCK", &self.BSLVLOCK())
            .field("BREFLOCK", &self.BREFLOCK())
            .field("BSLVSEL", &self.BSLVSEL())
            .field("BREFSEL", &self.BREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_STS2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLEXSPI_STS2 {{ ASLVLOCK: {=bool:?}, AREFLOCK: {=bool:?}, ASLVSEL: {=u8:?}, AREFSEL: {=u8:?}, BSLVLOCK: {=bool:?}, BREFLOCK: {=bool:?}, BSLVSEL: {=u8:?}, BREFSEL: {=u8:?} }}",
            self.ASLVLOCK(),
            self.AREFLOCK(),
            self.ASLVSEL(),
            self.AREFSEL(),
            self.BSLVLOCK(),
            self.BREFLOCK(),
            self.BSLVSEL(),
            self.BREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI_TFDR(pub u32);
impl FLEXSPI_TFDR {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn TXDATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_TXDATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLEXSPI_TFDR {
    #[inline(always)]
    fn default() -> FLEXSPI_TFDR {
        FLEXSPI_TFDR(0)
    }
}
impl core::fmt::Debug for FLEXSPI_TFDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXSPI_TFDR")
            .field("TXDATA", &self.TXDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLEXSPI_TFDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLEXSPI_TFDR {{ TXDATA: {=u32:?} }}", self.TXDATA())
    }
}

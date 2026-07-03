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
    #[doc = "DMA secure configuration register"]
    #[inline(always)]
    pub const fn SECCFGR(self) -> crate::common::Reg<DMA_SECCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA privileged configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(self) -> crate::common::Reg<DMA_PRIVCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA configuration lock register"]
    #[inline(always)]
    pub const fn RCFGLOCKR(self) -> crate::common::Reg<DMA_RCFGLOCKR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA non secure masked interrupt status register"]
    #[inline(always)]
    pub const fn MISR(self) -> crate::common::Reg<DMA_MISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DMA secure masked interrupt status register"]
    #[inline(always)]
    pub const fn SMISR(self) -> crate::common::Reg<DMA_SMISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 13 register"]
    #[inline(always)]
    pub const fn HWCFGR13(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 12 register"]
    #[inline(always)]
    pub const fn HWCFGR12(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 11 register"]
    #[inline(always)]
    pub const fn HWCFGR11(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc8usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 10 register"]
    #[inline(always)]
    pub const fn HWCFGR10(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fccusize) as _) }
    }
    #[doc = "HPDMA hardware configuration 9 register"]
    #[inline(always)]
    pub const fn HWCFGR9(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 8 register"]
    #[inline(always)]
    pub const fn HWCFGR8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 7 register"]
    #[inline(always)]
    pub const fn HWCFGR7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 6 register"]
    #[inline(always)]
    pub const fn HWCFGR6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "HPDMA hardware configuration 5 register"]
    #[inline(always)]
    pub const fn HWCFGR5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 4 register"]
    #[inline(always)]
    pub const fn HWCFGR4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 3 register"]
    #[inline(always)]
    pub const fn HWCFGR3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "HPDMA hardware configuration 2 register"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "HPDMA hardware configuration 1 register"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "HPDMA version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "HPDMA identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "HPDMA size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_MISR(pub u32);
impl DMA_MISR {
    #[doc = "Masked Interrupt State of Non-Secure Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 0"]
    #[inline(always)]
    pub const fn set_MIS0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 1"]
    #[inline(always)]
    pub const fn set_MIS1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 2"]
    #[inline(always)]
    pub const fn set_MIS2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 3"]
    #[inline(always)]
    pub const fn set_MIS3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 4"]
    #[inline(always)]
    pub const fn set_MIS4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 5"]
    #[inline(always)]
    pub const fn set_MIS5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 6"]
    #[inline(always)]
    pub const fn set_MIS6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 7"]
    #[inline(always)]
    pub const fn set_MIS7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 8"]
    #[inline(always)]
    pub const fn set_MIS8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 9"]
    #[inline(always)]
    pub const fn set_MIS9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 10"]
    #[inline(always)]
    pub const fn set_MIS10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 11"]
    #[inline(always)]
    pub const fn set_MIS11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 12"]
    #[inline(always)]
    pub const fn set_MIS12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 13"]
    #[inline(always)]
    pub const fn set_MIS13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Non-Secure Channel 14"]
    #[inline(always)]
    pub const fn set_MIS14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0x00008000"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0x00008000"]
    #[inline(always)]
    pub const fn set_MIS15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_MISR {
    #[inline(always)]
    fn default() -> DMA_MISR {
        DMA_MISR(0)
    }
}
impl core::fmt::Debug for DMA_MISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MISR")
            .field("MIS0", &self.MIS0())
            .field("MIS1", &self.MIS1())
            .field("MIS2", &self.MIS2())
            .field("MIS3", &self.MIS3())
            .field("MIS4", &self.MIS4())
            .field("MIS5", &self.MIS5())
            .field("MIS6", &self.MIS6())
            .field("MIS7", &self.MIS7())
            .field("MIS8", &self.MIS8())
            .field("MIS9", &self.MIS9())
            .field("MIS10", &self.MIS10())
            .field("MIS11", &self.MIS11())
            .field("MIS12", &self.MIS12())
            .field("MIS13", &self.MIS13())
            .field("MIS14", &self.MIS14())
            .field("MIS15", &self.MIS15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_MISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_MISR {{ MIS0: {=bool:?}, MIS1: {=bool:?}, MIS2: {=bool:?}, MIS3: {=bool:?}, MIS4: {=bool:?}, MIS5: {=bool:?}, MIS6: {=bool:?}, MIS7: {=bool:?}, MIS8: {=bool:?}, MIS9: {=bool:?}, MIS10: {=bool:?}, MIS11: {=bool:?}, MIS12: {=bool:?}, MIS13: {=bool:?}, MIS14: {=bool:?}, MIS15: {=bool:?} }}",
            self.MIS0(),
            self.MIS1(),
            self.MIS2(),
            self.MIS3(),
            self.MIS4(),
            self.MIS5(),
            self.MIS6(),
            self.MIS7(),
            self.MIS8(),
            self.MIS9(),
            self.MIS10(),
            self.MIS11(),
            self.MIS12(),
            self.MIS13(),
            self.MIS14(),
            self.MIS15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_PRIVCFGR(pub u32);
impl DMA_PRIVCFGR {
    #[doc = "Privileged State of Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 0"]
    #[inline(always)]
    pub const fn set_PRIV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Privileged State of Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 1"]
    #[inline(always)]
    pub const fn set_PRIV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Privileged State of Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 2"]
    #[inline(always)]
    pub const fn set_PRIV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Privileged State of Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 3"]
    #[inline(always)]
    pub const fn set_PRIV3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Privileged State of Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 4"]
    #[inline(always)]
    pub const fn set_PRIV4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Privileged State of Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 5"]
    #[inline(always)]
    pub const fn set_PRIV5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Privileged State of Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 6"]
    #[inline(always)]
    pub const fn set_PRIV6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Privileged State of Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 7"]
    #[inline(always)]
    pub const fn set_PRIV7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Privileged State of Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 8"]
    #[inline(always)]
    pub const fn set_PRIV8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Privileged State of Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 9"]
    #[inline(always)]
    pub const fn set_PRIV9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Privileged State of Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 10"]
    #[inline(always)]
    pub const fn set_PRIV10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Privileged State of Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 11"]
    #[inline(always)]
    pub const fn set_PRIV11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Privileged State of Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 12"]
    #[inline(always)]
    pub const fn set_PRIV12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Privileged State of Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 13"]
    #[inline(always)]
    pub const fn set_PRIV13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Privileged State of Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 14"]
    #[inline(always)]
    pub const fn set_PRIV14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Privileged State of Channel 15"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged State of Channel 15"]
    #[inline(always)]
    pub const fn set_PRIV15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_PRIVCFGR {
    #[inline(always)]
    fn default() -> DMA_PRIVCFGR {
        DMA_PRIVCFGR(0)
    }
}
impl core::fmt::Debug for DMA_PRIVCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_PRIVCFGR")
            .field("PRIV0", &self.PRIV0())
            .field("PRIV1", &self.PRIV1())
            .field("PRIV2", &self.PRIV2())
            .field("PRIV3", &self.PRIV3())
            .field("PRIV4", &self.PRIV4())
            .field("PRIV5", &self.PRIV5())
            .field("PRIV6", &self.PRIV6())
            .field("PRIV7", &self.PRIV7())
            .field("PRIV8", &self.PRIV8())
            .field("PRIV9", &self.PRIV9())
            .field("PRIV10", &self.PRIV10())
            .field("PRIV11", &self.PRIV11())
            .field("PRIV12", &self.PRIV12())
            .field("PRIV13", &self.PRIV13())
            .field("PRIV14", &self.PRIV14())
            .field("PRIV15", &self.PRIV15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_PRIVCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_PRIVCFGR {{ PRIV0: {=bool:?}, PRIV1: {=bool:?}, PRIV2: {=bool:?}, PRIV3: {=bool:?}, PRIV4: {=bool:?}, PRIV5: {=bool:?}, PRIV6: {=bool:?}, PRIV7: {=bool:?}, PRIV8: {=bool:?}, PRIV9: {=bool:?}, PRIV10: {=bool:?}, PRIV11: {=bool:?}, PRIV12: {=bool:?}, PRIV13: {=bool:?}, PRIV14: {=bool:?}, PRIV15: {=bool:?} }}",
            self.PRIV0(),
            self.PRIV1(),
            self.PRIV2(),
            self.PRIV3(),
            self.PRIV4(),
            self.PRIV5(),
            self.PRIV6(),
            self.PRIV7(),
            self.PRIV8(),
            self.PRIV9(),
            self.PRIV10(),
            self.PRIV11(),
            self.PRIV12(),
            self.PRIV13(),
            self.PRIV14(),
            self.PRIV15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_RCFGLOCKR(pub u32);
impl DMA_RCFGLOCKR {
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "lock the configuration of the DMA_SECCFGR"]
    #[inline(always)]
    pub const fn set_LOCK15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_RCFGLOCKR {
    #[inline(always)]
    fn default() -> DMA_RCFGLOCKR {
        DMA_RCFGLOCKR(0)
    }
}
impl core::fmt::Debug for DMA_RCFGLOCKR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RCFGLOCKR")
            .field("LOCK0", &self.LOCK0())
            .field("LOCK1", &self.LOCK1())
            .field("LOCK2", &self.LOCK2())
            .field("LOCK3", &self.LOCK3())
            .field("LOCK4", &self.LOCK4())
            .field("LOCK5", &self.LOCK5())
            .field("LOCK6", &self.LOCK6())
            .field("LOCK7", &self.LOCK7())
            .field("LOCK8", &self.LOCK8())
            .field("LOCK9", &self.LOCK9())
            .field("LOCK10", &self.LOCK10())
            .field("LOCK11", &self.LOCK11())
            .field("LOCK12", &self.LOCK12())
            .field("LOCK13", &self.LOCK13())
            .field("LOCK14", &self.LOCK14())
            .field("LOCK15", &self.LOCK15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_RCFGLOCKR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_RCFGLOCKR {{ LOCK0: {=bool:?}, LOCK1: {=bool:?}, LOCK2: {=bool:?}, LOCK3: {=bool:?}, LOCK4: {=bool:?}, LOCK5: {=bool:?}, LOCK6: {=bool:?}, LOCK7: {=bool:?}, LOCK8: {=bool:?}, LOCK9: {=bool:?}, LOCK10: {=bool:?}, LOCK11: {=bool:?}, LOCK12: {=bool:?}, LOCK13: {=bool:?}, LOCK14: {=bool:?}, LOCK15: {=bool:?} }}",
            self.LOCK0(),
            self.LOCK1(),
            self.LOCK2(),
            self.LOCK3(),
            self.LOCK4(),
            self.LOCK5(),
            self.LOCK6(),
            self.LOCK7(),
            self.LOCK8(),
            self.LOCK9(),
            self.LOCK10(),
            self.LOCK11(),
            self.LOCK12(),
            self.LOCK13(),
            self.LOCK14(),
            self.LOCK15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_SECCFGR(pub u32);
impl DMA_SECCFGR {
    #[doc = "Secure State of Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 0"]
    #[inline(always)]
    pub const fn set_SEC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure State of Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 1"]
    #[inline(always)]
    pub const fn set_SEC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure State of Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 2"]
    #[inline(always)]
    pub const fn set_SEC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure State of Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 3"]
    #[inline(always)]
    pub const fn set_SEC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Secure State of Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 4"]
    #[inline(always)]
    pub const fn set_SEC4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Secure State of Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 5"]
    #[inline(always)]
    pub const fn set_SEC5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Secure State of Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 6"]
    #[inline(always)]
    pub const fn set_SEC6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure State of Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 7"]
    #[inline(always)]
    pub const fn set_SEC7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Secure State of Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 8"]
    #[inline(always)]
    pub const fn set_SEC8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Secure State of Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 9"]
    #[inline(always)]
    pub const fn set_SEC9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Secure State of Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 10"]
    #[inline(always)]
    pub const fn set_SEC10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Secure State of Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 11"]
    #[inline(always)]
    pub const fn set_SEC11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Secure State of Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 12"]
    #[inline(always)]
    pub const fn set_SEC12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Secure State of Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 13"]
    #[inline(always)]
    pub const fn set_SEC13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Secure State of Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Secure State of Channel 14"]
    #[inline(always)]
    pub const fn set_SEC14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0x00008000"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0x00008000"]
    #[inline(always)]
    pub const fn set_SEC15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_SECCFGR {
    #[inline(always)]
    fn default() -> DMA_SECCFGR {
        DMA_SECCFGR(0)
    }
}
impl core::fmt::Debug for DMA_SECCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SECCFGR")
            .field("SEC0", &self.SEC0())
            .field("SEC1", &self.SEC1())
            .field("SEC2", &self.SEC2())
            .field("SEC3", &self.SEC3())
            .field("SEC4", &self.SEC4())
            .field("SEC5", &self.SEC5())
            .field("SEC6", &self.SEC6())
            .field("SEC7", &self.SEC7())
            .field("SEC8", &self.SEC8())
            .field("SEC9", &self.SEC9())
            .field("SEC10", &self.SEC10())
            .field("SEC11", &self.SEC11())
            .field("SEC12", &self.SEC12())
            .field("SEC13", &self.SEC13())
            .field("SEC14", &self.SEC14())
            .field("SEC15", &self.SEC15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_SECCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_SECCFGR {{ SEC0: {=bool:?}, SEC1: {=bool:?}, SEC2: {=bool:?}, SEC3: {=bool:?}, SEC4: {=bool:?}, SEC5: {=bool:?}, SEC6: {=bool:?}, SEC7: {=bool:?}, SEC8: {=bool:?}, SEC9: {=bool:?}, SEC10: {=bool:?}, SEC11: {=bool:?}, SEC12: {=bool:?}, SEC13: {=bool:?}, SEC14: {=bool:?}, SEC15: {=bool:?} }}",
            self.SEC0(),
            self.SEC1(),
            self.SEC2(),
            self.SEC3(),
            self.SEC4(),
            self.SEC5(),
            self.SEC6(),
            self.SEC7(),
            self.SEC8(),
            self.SEC9(),
            self.SEC10(),
            self.SEC11(),
            self.SEC12(),
            self.SEC13(),
            self.SEC14(),
            self.SEC15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_SMISR(pub u32);
impl DMA_SMISR {
    #[doc = "Masked Interrupt State of Secure Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 0"]
    #[inline(always)]
    pub const fn set_MIS0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 1"]
    #[inline(always)]
    pub const fn set_MIS1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 2"]
    #[inline(always)]
    pub const fn set_MIS2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 3"]
    #[inline(always)]
    pub const fn set_MIS3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 4"]
    #[inline(always)]
    pub const fn set_MIS4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 5"]
    #[inline(always)]
    pub const fn set_MIS5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 6"]
    #[inline(always)]
    pub const fn set_MIS6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 7"]
    #[inline(always)]
    pub const fn set_MIS7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 8"]
    #[inline(always)]
    pub const fn set_MIS8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 9"]
    #[inline(always)]
    pub const fn set_MIS9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 10"]
    #[inline(always)]
    pub const fn set_MIS10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 11"]
    #[inline(always)]
    pub const fn set_MIS11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 12"]
    #[inline(always)]
    pub const fn set_MIS12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 13"]
    #[inline(always)]
    pub const fn set_MIS13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Masked Interrupt State of Secure Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Masked Interrupt State of Secure Channel 14"]
    #[inline(always)]
    pub const fn set_MIS14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "0x00008000"]
    #[must_use]
    #[inline(always)]
    pub const fn MIS15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0x00008000"]
    #[inline(always)]
    pub const fn set_MIS15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DMA_SMISR {
    #[inline(always)]
    fn default() -> DMA_SMISR {
        DMA_SMISR(0)
    }
}
impl core::fmt::Debug for DMA_SMISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_SMISR")
            .field("MIS0", &self.MIS0())
            .field("MIS1", &self.MIS1())
            .field("MIS2", &self.MIS2())
            .field("MIS3", &self.MIS3())
            .field("MIS4", &self.MIS4())
            .field("MIS5", &self.MIS5())
            .field("MIS6", &self.MIS6())
            .field("MIS7", &self.MIS7())
            .field("MIS8", &self.MIS8())
            .field("MIS9", &self.MIS9())
            .field("MIS10", &self.MIS10())
            .field("MIS11", &self.MIS11())
            .field("MIS12", &self.MIS12())
            .field("MIS13", &self.MIS13())
            .field("MIS14", &self.MIS14())
            .field("MIS15", &self.MIS15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA_SMISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA_SMISR {{ MIS0: {=bool:?}, MIS1: {=bool:?}, MIS2: {=bool:?}, MIS3: {=bool:?}, MIS4: {=bool:?}, MIS5: {=bool:?}, MIS6: {=bool:?}, MIS7: {=bool:?}, MIS8: {=bool:?}, MIS9: {=bool:?}, MIS10: {=bool:?}, MIS11: {=bool:?}, MIS12: {=bool:?}, MIS13: {=bool:?}, MIS14: {=bool:?}, MIS15: {=bool:?} }}",
            self.MIS0(),
            self.MIS1(),
            self.MIS2(),
            self.MIS3(),
            self.MIS4(),
            self.MIS5(),
            self.MIS6(),
            self.MIS7(),
            self.MIS8(),
            self.MIS9(),
            self.MIS10(),
            self.MIS11(),
            self.MIS12(),
            self.MIS13(),
            self.MIS14(),
            self.MIS15()
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (73c44de 2026-04-17))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM {
    ptr: *mut u8,
}
unsafe impl Send for HSEM {}
unsafe impl Sync for HSEM {}
impl HSEM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSEM register"]
    #[inline(always)]
    pub const fn R(self, n: usize) -> crate::common::Reg<HSEM_R, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "HSEM read lock register"]
    #[inline(always)]
    pub const fn RLR(self, n: usize) -> crate::common::Reg<HSEM_RLR, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt enable register"]
    #[inline(always)]
    pub const fn C1IER(self) -> crate::common::Reg<HSEM_C1IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt clear register"]
    #[inline(always)]
    pub const fn C1ICR(self) -> crate::common::Reg<HSEM_C1ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt status register"]
    #[inline(always)]
    pub const fn C1ISR(self) -> crate::common::Reg<HSEM_C1ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "HSEM Masked non-secure interrupt status register"]
    #[inline(always)]
    pub const fn C1MISR(self) -> crate::common::Reg<HSEM_C1MISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt enable register"]
    #[inline(always)]
    pub const fn C2IER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt clear register"]
    #[inline(always)]
    pub const fn C2ICR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt status register"]
    #[inline(always)]
    pub const fn C2ISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "HSEM Masked non-secure interrupt status register"]
    #[inline(always)]
    pub const fn C2MISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt enable register"]
    #[inline(always)]
    pub const fn C3IER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt clear register"]
    #[inline(always)]
    pub const fn C3ICR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "HSEM non-secure Interrupt status register"]
    #[inline(always)]
    pub const fn C3ISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "HSEM Masked non-secure interrupt status register"]
    #[inline(always)]
    pub const fn C3MISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "HSEM secure interrupt enable register"]
    #[inline(always)]
    pub const fn SC1IER(self) -> crate::common::Reg<HSEM_SC1IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "HSEM secure interrupt clear register"]
    #[inline(always)]
    pub const fn SC1ICR(self) -> crate::common::Reg<HSEM_SC1ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "HSEM Secure interrupt status register"]
    #[inline(always)]
    pub const fn SC1ISR(self) -> crate::common::Reg<HSEM_SC1ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "HSEM Secure masked interrupt status register"]
    #[inline(always)]
    pub const fn SC1MISR(self) -> crate::common::Reg<HSEM_SC1MISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "HSEM secure interrupt enable register"]
    #[inline(always)]
    pub const fn SC2IER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "HSEM secure interrupt clear register"]
    #[inline(always)]
    pub const fn SC2ICR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "HSEM Secure interrupt status register"]
    #[inline(always)]
    pub const fn SC2ISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "HSEM Secure masked interrupt status register"]
    #[inline(always)]
    pub const fn SC2MISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "HSEM secure interrupt enable register"]
    #[inline(always)]
    pub const fn SC3IER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "HSEM secure interrupt clear register"]
    #[inline(always)]
    pub const fn SC3ICR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "HSEM Secure interrupt status register"]
    #[inline(always)]
    pub const fn SC3ISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "HSEM Secure masked interrupt status register"]
    #[inline(always)]
    pub const fn SC3MISR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "HSEM Security configuration register"]
    #[inline(always)]
    pub const fn SECCFGR(self) -> crate::common::Reg<HSEM_SECCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "HSEM Privilege configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(self) -> crate::common::Reg<HSEM_PRIVCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "HSEM processor 1 CID configuration register"]
    #[inline(always)]
    pub const fn C1CIDCFGR(self) -> crate::common::Reg<HSEM_C1CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "HSEM processor 2 CID configuration register"]
    #[inline(always)]
    pub const fn C2CIDCFGR(self) -> crate::common::Reg<HSEM_C2CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "HSEM processor 3 CID configuration register"]
    #[inline(always)]
    pub const fn C3CIDCFGR(self) -> crate::common::Reg<HSEM_C3CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "HSEM Clear register"]
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<HSEM_CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "HSEM clear key register"]
    #[inline(always)]
    pub const fn KEYR(self) -> crate::common::Reg<HSEM_KEYR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "HSEM CID configuration register"]
    #[inline(always)]
    pub const fn G0CIDCFGR(self) -> crate::common::Reg<HSEM_G0CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "HSEM CID configuration register"]
    #[inline(always)]
    pub const fn G1CIDCFGR(self) -> crate::common::Reg<HSEM_G1CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "HSEM CID configuration register"]
    #[inline(always)]
    pub const fn G2CIDCFGR(self) -> crate::common::Reg<HSEM_G2CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "HSEM CID configuration register"]
    #[inline(always)]
    pub const fn G3CIDCFGR(self) -> crate::common::Reg<HSEM_G3CIDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "HSEM hardware configuration register 2"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<HSEM_HWCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "HSEM hardware configuration register 1"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<HSEM_HWCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "HSEM IP version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<HSEM_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "HSEM IP identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<HSEM_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "HSEM size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<HSEM_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C1CIDCFGR(pub u32);
impl HSEM_C1CIDCFGR {
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPUn CID domain identification"]
    #[must_use]
    #[inline(always)]
    pub const fn CID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CPUn CID domain identification"]
    #[inline(always)]
    pub const fn set_CID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for HSEM_C1CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_C1CIDCFGR {
        HSEM_C1CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_C1CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1CIDCFGR")
            .field("CFEN", &self.CFEN())
            .field("CID", &self.CID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C1CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_C1CIDCFGR {{ CFEN: {=bool:?}, CID: {=u8:?} }}",
            self.CFEN(),
            self.CID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C1ICR(pub u32);
impl HSEM_C1ICR {
    #[doc = "non-secure Interrupt(n) semaphore x clear bit. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn ISC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "non-secure Interrupt(n) semaphore x clear bit. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_ISC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_C1ICR {
    #[inline(always)]
    fn default() -> HSEM_C1ICR {
        HSEM_C1ICR(0)
    }
}
impl core::fmt::Debug for HSEM_C1ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1ICR")
            .field("ISC", &self.ISC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C1ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_C1ICR {{ ISC: {=u16:?} }}", self.ISC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C1IER(pub u32);
impl HSEM_C1IER {
    #[doc = "non-secure Interrupt(n) semaphore x enable bit. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn ISE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "non-secure Interrupt(n) semaphore x enable bit. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_ISE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "semaphore 0 , interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 0 , interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "semaphore 1 , interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 1 , interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "semaphore 2 , interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 2 , interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "semaphore 3 , interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 3 , interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "semaphore 4 , interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 4 , interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "semaphore 5 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 5 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "semaphore 6 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 6 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "semaphore 7 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 7 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "semaphore 8 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 8 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "semaphore 9 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 9 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "semaphore 10 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 10 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "semaphore 11 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 11 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "semaphore 12 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 12 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "semaphore 13 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 13 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "semaphore 14 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 14 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "semaphore 15 interrupt 0 enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ISE15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "semaphore 15 interrupt 0 enable bit."]
    #[inline(always)]
    pub const fn set_ISE15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for HSEM_C1IER {
    #[inline(always)]
    fn default() -> HSEM_C1IER {
        HSEM_C1IER(0)
    }
}
impl core::fmt::Debug for HSEM_C1IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1IER")
            .field("ISE", &self.ISE())
            .field("ISE0", &self.ISE0())
            .field("ISE1", &self.ISE1())
            .field("ISE2", &self.ISE2())
            .field("ISE3", &self.ISE3())
            .field("ISE4", &self.ISE4())
            .field("ISE5", &self.ISE5())
            .field("ISE6", &self.ISE6())
            .field("ISE7", &self.ISE7())
            .field("ISE8", &self.ISE8())
            .field("ISE9", &self.ISE9())
            .field("ISE10", &self.ISE10())
            .field("ISE11", &self.ISE11())
            .field("ISE12", &self.ISE12())
            .field("ISE13", &self.ISE13())
            .field("ISE14", &self.ISE14())
            .field("ISE15", &self.ISE15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C1IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_C1IER {{ ISE: {=u16:?}, ISE0: {=bool:?}, ISE1: {=bool:?}, ISE2: {=bool:?}, ISE3: {=bool:?}, ISE4: {=bool:?}, ISE5: {=bool:?}, ISE6: {=bool:?}, ISE7: {=bool:?}, ISE8: {=bool:?}, ISE9: {=bool:?}, ISE10: {=bool:?}, ISE11: {=bool:?}, ISE12: {=bool:?}, ISE13: {=bool:?}, ISE14: {=bool:?}, ISE15: {=bool:?} }}",
            self.ISE(),
            self.ISE0(),
            self.ISE1(),
            self.ISE2(),
            self.ISE3(),
            self.ISE4(),
            self.ISE5(),
            self.ISE6(),
            self.ISE7(),
            self.ISE8(),
            self.ISE9(),
            self.ISE10(),
            self.ISE11(),
            self.ISE12(),
            self.ISE13(),
            self.ISE14(),
            self.ISE15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C1ISR(pub u32);
impl HSEM_C1ISR {
    #[doc = "non-secure Interrupt(n) semaphore x status bit before enable (mask). (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn ISF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "non-secure Interrupt(n) semaphore x status bit before enable (mask). (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_ISF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_C1ISR {
    #[inline(always)]
    fn default() -> HSEM_C1ISR {
        HSEM_C1ISR(0)
    }
}
impl core::fmt::Debug for HSEM_C1ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1ISR")
            .field("ISF", &self.ISF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C1ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_C1ISR {{ ISF: {=u16:?} }}", self.ISF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C1MISR(pub u32);
impl HSEM_C1MISR {
    #[doc = "masked non-secure interrupt(n) semaphore x status bit after enable (mask). (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn MISF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "masked non-secure interrupt(n) semaphore x status bit after enable (mask). (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_MISF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_C1MISR {
    #[inline(always)]
    fn default() -> HSEM_C1MISR {
        HSEM_C1MISR(0)
    }
}
impl core::fmt::Debug for HSEM_C1MISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C1MISR")
            .field("MISF", &self.MISF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C1MISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_C1MISR {{ MISF: {=u16:?} }}", self.MISF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C2CIDCFGR(pub u32);
impl HSEM_C2CIDCFGR {
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPUn CID domain identification"]
    #[must_use]
    #[inline(always)]
    pub const fn CID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CPUn CID domain identification"]
    #[inline(always)]
    pub const fn set_CID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for HSEM_C2CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_C2CIDCFGR {
        HSEM_C2CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_C2CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C2CIDCFGR")
            .field("CFEN", &self.CFEN())
            .field("CID", &self.CID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C2CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_C2CIDCFGR {{ CFEN: {=bool:?}, CID: {=u8:?} }}",
            self.CFEN(),
            self.CID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_C3CIDCFGR(pub u32);
impl HSEM_C3CIDCFGR {
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPUn CID filtering enabled for HSEM_(S)CnIER, HSEM_(S)CnICR, HSEM_(S)CnISR, and HSEM_(S)CnMISR registers and for white list filter usage in HSEM_GpCIDCFGR.SEM_WLIST_Cn"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPUn CID domain identification"]
    #[must_use]
    #[inline(always)]
    pub const fn CID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CPUn CID domain identification"]
    #[inline(always)]
    pub const fn set_CID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for HSEM_C3CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_C3CIDCFGR {
        HSEM_C3CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_C3CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_C3CIDCFGR")
            .field("CFEN", &self.CFEN())
            .field("CID", &self.CID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_C3CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_C3CIDCFGR {{ CFEN: {=bool:?}, CID: {=u8:?} }}",
            self.CFEN(),
            self.CID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_CR(pub u32);
impl HSEM_CR {
    #[doc = "Semaphore CID filtered."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore CID filtered."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LOCKID of semaphores to be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKID(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "LOCKID of semaphores to be cleared."]
    #[inline(always)]
    pub const fn set_LOCKID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "SEC value of semaphores to be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SEC value of semaphores to be cleared."]
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PRIV value of semaphores to be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PRIV value of semaphores to be cleared."]
    #[inline(always)]
    pub const fn set_PRIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Semaphore clear Key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Semaphore clear Key."]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HSEM_CR {
    #[inline(always)]
    fn default() -> HSEM_CR {
        HSEM_CR(0)
    }
}
impl core::fmt::Debug for HSEM_CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_CR")
            .field("CFEN", &self.CFEN())
            .field("LOCKID", &self.LOCKID())
            .field("SEC", &self.SEC())
            .field("PRIV", &self.PRIV())
            .field("KEY", &self.KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_CR {{ CFEN: {=bool:?}, LOCKID: {=u8:?}, SEC: {=bool:?}, PRIV: {=bool:?}, KEY: {=u16:?} }}",
            self.CFEN(),
            self.LOCKID(),
            self.SEC(),
            self.PRIV(),
            self.KEY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_G0CIDCFGR(pub u32);
impl HSEM_G0CIDCFGR {
    #[doc = "Semaphore group p global CID filtering enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore group p global CID filtering enable."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HSEM_G0CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_G0CIDCFGR {
        HSEM_G0CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_G0CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_G0CIDCFGR")
            .field("CFEN", &self.CFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_G0CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_G0CIDCFGR {{ CFEN: {=bool:?} }}", self.CFEN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_G1CIDCFGR(pub u32);
impl HSEM_G1CIDCFGR {
    #[doc = "Semaphore group p global CID filtering enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore group p global CID filtering enable."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HSEM_G1CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_G1CIDCFGR {
        HSEM_G1CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_G1CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_G1CIDCFGR")
            .field("CFEN", &self.CFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_G1CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_G1CIDCFGR {{ CFEN: {=bool:?} }}", self.CFEN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_G2CIDCFGR(pub u32);
impl HSEM_G2CIDCFGR {
    #[doc = "Semaphore group p global CID filtering enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore group p global CID filtering enable."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HSEM_G2CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_G2CIDCFGR {
        HSEM_G2CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_G2CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_G2CIDCFGR")
            .field("CFEN", &self.CFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_G2CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_G2CIDCFGR {{ CFEN: {=bool:?} }}", self.CFEN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_G3CIDCFGR(pub u32);
impl HSEM_G3CIDCFGR {
    #[doc = "Semaphore group p global CID filtering enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore group p global CID filtering enable."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for HSEM_G3CIDCFGR {
    #[inline(always)]
    fn default() -> HSEM_G3CIDCFGR {
        HSEM_G3CIDCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_G3CIDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_G3CIDCFGR")
            .field("CFEN", &self.CFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_G3CIDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_G3CIDCFGR {{ CFEN: {=bool:?} }}", self.CFEN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_HWCFGR1(pub u32);
impl HSEM_HWCFGR1 {
    #[doc = "Hardware configuration number of semaphores"]
    #[must_use]
    #[inline(always)]
    pub const fn NBSEM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Hardware configuration number of semaphores"]
    #[inline(always)]
    pub const fn set_NBSEM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Hardware configuration number of interrupts/supported number of master ID"]
    #[must_use]
    #[inline(always)]
    pub const fn NBINT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware configuration number of interrupts/supported number of master ID"]
    #[inline(always)]
    pub const fn set_NBINT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Hardware Configuration security protection enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SECEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Configuration security protection enable"]
    #[inline(always)]
    pub const fn set_SECEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Hardware Configuration privilege protection enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIVEN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Configuration privilege protection enable"]
    #[inline(always)]
    pub const fn set_PRIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Hardware Configuration compartment filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Configuration compartment filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for HSEM_HWCFGR1 {
    #[inline(always)]
    fn default() -> HSEM_HWCFGR1 {
        HSEM_HWCFGR1(0)
    }
}
impl core::fmt::Debug for HSEM_HWCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_HWCFGR1")
            .field("NBSEM", &self.NBSEM())
            .field("NBINT", &self.NBINT())
            .field("SECEN", &self.SECEN())
            .field("PRIVEN", &self.PRIVEN())
            .field("CFEN", &self.CFEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_HWCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_HWCFGR1 {{ NBSEM: {=u8:?}, NBINT: {=u8:?}, SECEN: {=bool:?}, PRIVEN: {=bool:?}, CFEN: {=bool:?} }}",
            self.NBSEM(),
            self.NBINT(),
            self.SECEN(),
            self.PRIVEN(),
            self.CFEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_HWCFGR2(pub u32);
impl HSEM_HWCFGR2 {
    #[doc = "Hardware configuration valid bus masters ID1"]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERID1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware configuration valid bus masters ID1"]
    #[inline(always)]
    pub const fn set_MASTERID1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Hardware configuration valid bus masters ID2"]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERID2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware configuration valid bus masters ID2"]
    #[inline(always)]
    pub const fn set_MASTERID2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Hardware configuration valid bus masters ID3"]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERID3(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware configuration valid bus masters ID3"]
    #[inline(always)]
    pub const fn set_MASTERID3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Hardware configuration valid bus masters ID4"]
    #[must_use]
    #[inline(always)]
    pub const fn MASTERID4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware configuration valid bus masters ID4"]
    #[inline(always)]
    pub const fn set_MASTERID4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for HSEM_HWCFGR2 {
    #[inline(always)]
    fn default() -> HSEM_HWCFGR2 {
        HSEM_HWCFGR2(0)
    }
}
impl core::fmt::Debug for HSEM_HWCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_HWCFGR2")
            .field("MASTERID1", &self.MASTERID1())
            .field("MASTERID2", &self.MASTERID2())
            .field("MASTERID3", &self.MASTERID3())
            .field("MASTERID4", &self.MASTERID4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_HWCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_HWCFGR2 {{ MASTERID1: {=u8:?}, MASTERID2: {=u8:?}, MASTERID3: {=u8:?}, MASTERID4: {=u8:?} }}",
            self.MASTERID1(),
            self.MASTERID2(),
            self.MASTERID3(),
            self.MASTERID4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_IPIDR(pub u32);
impl HSEM_IPIDR {
    #[doc = "IP identification"]
    #[must_use]
    #[inline(always)]
    pub const fn IPID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP identification"]
    #[inline(always)]
    pub const fn set_IPID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HSEM_IPIDR {
    #[inline(always)]
    fn default() -> HSEM_IPIDR {
        HSEM_IPIDR(0)
    }
}
impl core::fmt::Debug for HSEM_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_IPIDR")
            .field("IPID", &self.IPID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_IPIDR {{ IPID: {=u32:?} }}", self.IPID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_KEYR(pub u32);
impl HSEM_KEYR {
    #[doc = "Semaphore Clear Key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Semaphore Clear Key."]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HSEM_KEYR {
    #[inline(always)]
    fn default() -> HSEM_KEYR {
        HSEM_KEYR(0)
    }
}
impl core::fmt::Debug for HSEM_KEYR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_KEYR")
            .field("KEY", &self.KEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_KEYR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_KEYR {{ KEY: {=u16:?} }}", self.KEY())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_PRIVCFGR(pub u32);
impl HSEM_PRIVCFGR {
    #[doc = "Semaphore x privilege attribute. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Semaphore x privilege attribute. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_PRIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_PRIVCFGR {
    #[inline(always)]
    fn default() -> HSEM_PRIVCFGR {
        HSEM_PRIVCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_PRIVCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_PRIVCFGR")
            .field("PRIV", &self.PRIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_PRIVCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_PRIVCFGR {{ PRIV: {=u16:?} }}", self.PRIV())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_R(pub u32);
impl HSEM_R {
    #[doc = "Semaphore PROCID."]
    #[must_use]
    #[inline(always)]
    pub const fn PROCID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Semaphore PROCID."]
    #[inline(always)]
    pub const fn set_PROCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Semaphore LOCKID."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKID(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Semaphore LOCKID."]
    #[inline(always)]
    pub const fn set_LOCKID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Semaphore secure."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore secure."]
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Semaphore privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore privilege."]
    #[inline(always)]
    pub const fn set_PRIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Semaphore CID filter indication."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore CID filter indication."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Lock indication."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock indication."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HSEM_R {
    #[inline(always)]
    fn default() -> HSEM_R {
        HSEM_R(0)
    }
}
impl core::fmt::Debug for HSEM_R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_R")
            .field("PROCID", &self.PROCID())
            .field("LOCKID", &self.LOCKID())
            .field("SEC", &self.SEC())
            .field("PRIV", &self.PRIV())
            .field("CFEN", &self.CFEN())
            .field("LOCK", &self.LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_R {{ PROCID: {=u8:?}, LOCKID: {=u8:?}, SEC: {=bool:?}, PRIV: {=bool:?}, CFEN: {=bool:?}, LOCK: {=bool:?} }}",
            self.PROCID(),
            self.LOCKID(),
            self.SEC(),
            self.PRIV(),
            self.CFEN(),
            self.LOCK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_RLR(pub u32);
impl HSEM_RLR {
    #[doc = "Semaphore PROCID."]
    #[must_use]
    #[inline(always)]
    pub const fn PROCID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Semaphore PROCID."]
    #[inline(always)]
    pub const fn set_PROCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Semaphore LOCKID."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKID(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Semaphore LOCKID."]
    #[inline(always)]
    pub const fn set_LOCKID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Semaphore secure."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore secure."]
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Semaphore privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore privilege."]
    #[inline(always)]
    pub const fn set_PRIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Semaphore CID filter indication."]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore CID filter indication."]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Lock indication."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock indication."]
    #[inline(always)]
    pub const fn set_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HSEM_RLR {
    #[inline(always)]
    fn default() -> HSEM_RLR {
        HSEM_RLR(0)
    }
}
impl core::fmt::Debug for HSEM_RLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_RLR")
            .field("PROCID", &self.PROCID())
            .field("LOCKID", &self.LOCKID())
            .field("SEC", &self.SEC())
            .field("PRIV", &self.PRIV())
            .field("CFEN", &self.CFEN())
            .field("LOCK", &self.LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_RLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_RLR {{ PROCID: {=u8:?}, LOCKID: {=u8:?}, SEC: {=bool:?}, PRIV: {=bool:?}, CFEN: {=bool:?}, LOCK: {=bool:?} }}",
            self.PROCID(),
            self.LOCKID(),
            self.SEC(),
            self.PRIV(),
            self.CFEN(),
            self.LOCK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SC1ICR(pub u32);
impl HSEM_SC1ICR {
    #[doc = "Secure interrupt(n) semaphore x clear bit. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn SISC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Secure interrupt(n) semaphore x clear bit. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_SISC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_SC1ICR {
    #[inline(always)]
    fn default() -> HSEM_SC1ICR {
        HSEM_SC1ICR(0)
    }
}
impl core::fmt::Debug for HSEM_SC1ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SC1ICR")
            .field("SISC", &self.SISC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SC1ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SC1ICR {{ SISC: {=u16:?} }}", self.SISC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SC1IER(pub u32);
impl HSEM_SC1IER {
    #[doc = "Secure interrupt(n) semaphore x enable bit. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn SISE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Secure interrupt(n) semaphore x enable bit. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_SISE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_SC1IER {
    #[inline(always)]
    fn default() -> HSEM_SC1IER {
        HSEM_SC1IER(0)
    }
}
impl core::fmt::Debug for HSEM_SC1IER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SC1IER")
            .field("SISE", &self.SISE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SC1IER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SC1IER {{ SISE: {=u16:?} }}", self.SISE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SC1ISR(pub u32);
impl HSEM_SC1ISR {
    #[doc = "Secure interrupt(n) semaphore x status bit before enable (mask). (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn SISF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Secure interrupt(n) semaphore x status bit before enable (mask). (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_SISF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_SC1ISR {
    #[inline(always)]
    fn default() -> HSEM_SC1ISR {
        HSEM_SC1ISR(0)
    }
}
impl core::fmt::Debug for HSEM_SC1ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SC1ISR")
            .field("SISF", &self.SISF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SC1ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SC1ISR {{ SISF: {=u16:?} }}", self.SISF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SC1MISR(pub u32);
impl HSEM_SC1MISR {
    #[doc = "Secure masked interrupt(n) semaphore x status bit after enable (mask). (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn SMISF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Secure masked interrupt(n) semaphore x status bit after enable (mask). (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_SMISF(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_SC1MISR {
    #[inline(always)]
    fn default() -> HSEM_SC1MISR {
        HSEM_SC1MISR(0)
    }
}
impl core::fmt::Debug for HSEM_SC1MISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SC1MISR")
            .field("SMISF", &self.SMISF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SC1MISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SC1MISR {{ SMISF: {=u16:?} }}", self.SMISF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SECCFGR(pub u32);
impl HSEM_SECCFGR {
    #[doc = "Semaphore x security attribute. (x = 0 to 15)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Semaphore x security attribute. (x = 0 to 15)"]
    #[inline(always)]
    pub const fn set_SEC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HSEM_SECCFGR {
    #[inline(always)]
    fn default() -> HSEM_SECCFGR {
        HSEM_SECCFGR(0)
    }
}
impl core::fmt::Debug for HSEM_SECCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SECCFGR")
            .field("SEC", &self.SEC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SECCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SECCFGR {{ SEC: {=u16:?} }}", self.SEC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_SIDR(pub u32);
impl HSEM_SIDR {
    #[doc = "IP size identification"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP size identification"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HSEM_SIDR {
    #[inline(always)]
    fn default() -> HSEM_SIDR {
        HSEM_SIDR(0)
    }
}
impl core::fmt::Debug for HSEM_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HSEM_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HSEM_VERR(pub u32);
impl HSEM_VERR {
    #[doc = "IP minor revision number"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "IP minor revision number"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "IP major revision number"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "IP major revision number"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for HSEM_VERR {
    #[inline(always)]
    fn default() -> HSEM_VERR {
        HSEM_VERR(0)
    }
}
impl core::fmt::Debug for HSEM_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HSEM_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HSEM_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
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

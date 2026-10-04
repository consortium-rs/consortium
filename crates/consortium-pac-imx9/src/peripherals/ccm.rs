#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM {
    ptr: *mut u8,
}
unsafe impl Send for CCM {}
unsafe impl Sync for CCM {}
impl CCM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_CONTROL_RW(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 123usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 128usize) as _) }
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_CONTROL_SET(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 123usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 128usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_CONTROL_CLR(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 123usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 128usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_CONTROL_TOG(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 123usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 128usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_STATUS0(
        self,
        n: usize,
    ) -> crate::common::Reg<CCM_CLOCK_ROOT_STATUS0, crate::common::RW> {
        assert!(n < 123usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 128usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CLOCK_ROOT_AUTHEN(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 123usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 128usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4800usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4804usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4808usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x480cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_AUTHEN_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4810usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_AUTHEN_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4814usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_AUTHEN_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4818usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED0_AUTHEN_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x481cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4820usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4824usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4828usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x482cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_AUTHEN_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4830usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_AUTHEN_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4834usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_AUTHEN_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4838usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED1_AUTHEN_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x483cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4840usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4844usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4848usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x484cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_AUTHEN_RW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4850usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_AUTHEN_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4854usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_AUTHEN_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4858usize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED2_AUTHEN_TOG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x485cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_AUTHEN_RW(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4860usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_RW(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4860usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_AUTHEN_SET(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4864usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_SET(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4864usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_AUTHEN_CLR(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4868usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_CLR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4868usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_AUTHEN_TOG(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x486cusize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_SHARED_TOG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x486cusize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_AUTHEN_RW(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c00usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_RW(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c00usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_AUTHEN_SET(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c04usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_SET(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c04usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_AUTHEN_CLR(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c08usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_CLR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c08usize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_AUTHEN_TOG(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c0cusize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn GPR_PRIVATE_TOG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4c0cusize + n * 32usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_DIRECT(
        self,
        n: usize,
    ) -> crate::common::Reg<CCM_OSCPLL_DIRECT, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5000usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_LPM_STATUS0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5004usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_LPM_STATUS1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5008usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_LPM0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5010usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_LPM1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5014usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_LPM_CUR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x501cusize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_STATUS0(
        self,
        n: usize,
    ) -> crate::common::Reg<CCM_OSCPLL_STATUS0, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5020usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_STATUS1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5024usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn OSCPLL_AUTHEN(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5030usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_DIRECT(
        self,
        n: usize,
    ) -> crate::common::Reg<CCM_LPCG_DIRECT, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8000usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_LPM_STATUS0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8004usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_LPM_STATUS1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8008usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_LPM0(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8010usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_LPM1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8014usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_LPM_CUR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x801cusize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_STATUS0(
        self,
        n: usize,
    ) -> crate::common::Reg<CCM_LPCG_STATUS0, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8020usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_STATUS1(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8024usize + n * 64usize) as _)
        }
    }
    #[inline(always)]
    pub const fn LPCG_AUTHEN(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 256usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8030usize + n * 64usize) as _)
        }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT(pub u32);
impl CCM_CLOCK_ROOT {
    #[doc = "Clock division fraction."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock division fraction."]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock division fraction."]
    #[must_use]
    #[inline(always)]
    pub const fn MUX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Clock division fraction."]
    #[inline(always)]
    pub const fn set_MUX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for CCM_CLOCK_ROOT {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT {
        CCM_CLOCK_ROOT(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT")
            .field("DIV", &self.DIV())
            .field("MUX", &self.MUX())
            .field("OFF", &self.OFF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT {{ DIV: {=u8:?}, MUX: {=u8:?}, OFF: {=bool:?} }}",
            self.DIV(),
            self.MUX(),
            self.OFF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_AUTHEN_LOCK(pub u32);
impl CCM_CLOCK_ROOT_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_CLOCK_ROOT_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_AUTHEN_LOCK {
        CCM_CLOCK_ROOT_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_AUTHEN_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_AUTHEN_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_AUTHEN_TZ(pub u32);
impl CCM_CLOCK_ROOT_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_CLOCK_ROOT_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_AUTHEN_TZ {
        CCM_CLOCK_ROOT_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_AUTHEN_WHITE(pub u32);
impl CCM_CLOCK_ROOT_AUTHEN_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_CLOCK_ROOT_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_AUTHEN_WHITE {
        CCM_CLOCK_ROOT_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_AUTHEN_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_STATUS0(pub u32);
impl CCM_CLOCK_ROOT_STATUS0 {
    #[doc = "Current clock root DIV setting"]
    #[must_use]
    #[inline(always)]
    pub const fn DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Current clock root DIV setting"]
    #[inline(always)]
    pub const fn set_DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Current clock root DIV setting"]
    #[must_use]
    #[inline(always)]
    pub const fn MUX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Current clock root DIV setting"]
    #[inline(always)]
    pub const fn set_MUX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Current clock root MUX setting"]
    #[must_use]
    #[inline(always)]
    pub const fn OFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Current clock root MUX setting"]
    #[inline(always)]
    pub const fn set_OFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Changing"]
    #[must_use]
    #[inline(always)]
    pub const fn CHANGING(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Changing"]
    #[inline(always)]
    pub const fn set_CHANGING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CCM_CLOCK_ROOT_STATUS0 {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_STATUS0 {
        CCM_CLOCK_ROOT_STATUS0(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_STATUS0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_STATUS0")
            .field("DIV", &self.DIV())
            .field("MUX", &self.MUX())
            .field("OFF", &self.OFF())
            .field("CHANGING", &self.CHANGING())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_STATUS0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_STATUS0 {{ DIV: {=u8:?}, MUX: {=u8:?}, OFF: {=bool:?}, CHANGING: {=bool:?} }}",
            self.DIV(),
            self.MUX(),
            self.OFF(),
            self.CHANGING()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_STATUS0_SLICE(pub u32);
impl CCM_CLOCK_ROOT_STATUS0_SLICE {
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for CCM_CLOCK_ROOT_STATUS0_SLICE {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_STATUS0_SLICE {
        CCM_CLOCK_ROOT_STATUS0_SLICE(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_STATUS0_SLICE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_STATUS0_SLICE")
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_STATUS0_SLICE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_STATUS0_SLICE {{ BUSY: {=bool:?} }}",
            self.BUSY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_CLOCK_ROOT_STATUS0_UPDATE(pub u32);
impl CCM_CLOCK_ROOT_STATUS0_UPDATE {
    #[doc = "Update forward"]
    #[must_use]
    #[inline(always)]
    pub const fn FORWARD(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Update forward"]
    #[inline(always)]
    pub const fn set_FORWARD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Update reverse"]
    #[must_use]
    #[inline(always)]
    pub const fn REVERSE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Update reverse"]
    #[inline(always)]
    pub const fn set_REVERSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CCM_CLOCK_ROOT_STATUS0_UPDATE {
    #[inline(always)]
    fn default() -> CCM_CLOCK_ROOT_STATUS0_UPDATE {
        CCM_CLOCK_ROOT_STATUS0_UPDATE(0)
    }
}
impl core::fmt::Debug for CCM_CLOCK_ROOT_STATUS0_UPDATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_CLOCK_ROOT_STATUS0_UPDATE")
            .field("FORWARD", &self.FORWARD())
            .field("REVERSE", &self.REVERSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_CLOCK_ROOT_STATUS0_UPDATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_CLOCK_ROOT_STATUS0_UPDATE {{ FORWARD: {=bool:?}, REVERSE: {=bool:?} }}",
            self.FORWARD(),
            self.REVERSE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_PRIVATE(pub u32);
impl CCM_GPR_PRIVATE {
    #[doc = "GP register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GP register"]
    #[inline(always)]
    pub const fn set_GPR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CCM_GPR_PRIVATE {
    #[inline(always)]
    fn default() -> CCM_GPR_PRIVATE {
        CCM_GPR_PRIVATE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_PRIVATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_PRIVATE")
            .field("GPR", &self.GPR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_PRIVATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_GPR_PRIVATE {{ GPR: {=u32:?} }}", self.GPR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_PRIVATE_LOCK(pub u32);
impl CCM_GPR_PRIVATE_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_GPR_PRIVATE_LOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_PRIVATE_LOCK {
        CCM_GPR_PRIVATE_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_PRIVATE_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_PRIVATE_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_PRIVATE_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_PRIVATE_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_PRIVATE_TZ(pub u32);
impl CCM_GPR_PRIVATE_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_GPR_PRIVATE_TZ {
    #[inline(always)]
    fn default() -> CCM_GPR_PRIVATE_TZ {
        CCM_GPR_PRIVATE_TZ(0)
    }
}
impl core::fmt::Debug for CCM_GPR_PRIVATE_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_PRIVATE_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_PRIVATE_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_PRIVATE_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_PRIVATE_WHITE(pub u32);
impl CCM_GPR_PRIVATE_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_GPR_PRIVATE_WHITE {
    #[inline(always)]
    fn default() -> CCM_GPR_PRIVATE_WHITE {
        CCM_GPR_PRIVATE_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_PRIVATE_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_PRIVATE_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_PRIVATE_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_GPR_PRIVATE_WHITE {{ LIST: {=u16:?} }}", self.LIST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED0_AUTHEN_LOCK(pub u32);
impl CCM_GPR_SHARED0_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_GPR_SHARED0_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED0_AUTHEN_LOCK {
        CCM_GPR_SHARED0_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED0_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED0_AUTHEN_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED0_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED0_AUTHEN_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED0_AUTHEN_TZ(pub u32);
impl CCM_GPR_SHARED0_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_GPR_SHARED0_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED0_AUTHEN_TZ {
        CCM_GPR_SHARED0_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED0_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED0_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED0_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED0_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED0_AUTHEN_WHITE(pub u32);
impl CCM_GPR_SHARED0_AUTHEN_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_GPR_SHARED0_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED0_AUTHEN_WHITE {
        CCM_GPR_SHARED0_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED0_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED0_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED0_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED0_AUTHEN_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_AUTHEN_LOCK(pub u32);
impl CCM_GPR_SHARED1_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_GPR_SHARED1_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_AUTHEN_LOCK {
        CCM_GPR_SHARED1_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_AUTHEN_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_AUTHEN_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_AUTHEN_TZ(pub u32);
impl CCM_GPR_SHARED1_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_GPR_SHARED1_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_AUTHEN_TZ {
        CCM_GPR_SHARED1_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_AUTHEN_WHITE(pub u32);
impl CCM_GPR_SHARED1_AUTHEN_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_GPR_SHARED1_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_AUTHEN_WHITE {
        CCM_GPR_SHARED1_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_AUTHEN_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE0_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE0_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE0_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE0_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE0_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE0_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE0_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE0_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE0_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE1_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE1_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE1_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE1_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE1_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE1_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE1_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE1_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE1_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE2_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE2_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE2_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE2_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE2_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE2_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE2_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE2_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE2_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE3_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE3_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE3_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE3_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE3_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE3_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE3_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE3_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE3_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE4_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE4_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE4_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE4_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE4_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE4_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE4_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE4_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE4_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_CORE5_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_CORE5_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_CORE5_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_CORE5_CLOCK {
        CCM_GPR_SHARED1_CA55_CORE5_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_CORE5_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_CORE5_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_CORE5_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_CORE5_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK(pub u32);
impl CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {
    #[must_use]
    #[inline(always)]
    pub const fn SELECT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SELECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {
        CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK")
            .field("SELECT", &self.SELECT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED1_CA55_PLATFORM_CLOCK {{ SELECT: {=bool:?} }}",
            self.SELECT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK(pub u32);
impl CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {
    #[must_use]
    #[inline(always)]
    pub const fn SEL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {
        CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK")
            .field("SEL", &self.SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED2_ANAMIX_TEMPSENSE_CLK {{ SEL: {=bool:?} }}",
            self.SEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED2_AUTHEN_LOCK(pub u32);
impl CCM_GPR_SHARED2_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_GPR_SHARED2_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED2_AUTHEN_LOCK {
        CCM_GPR_SHARED2_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED2_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED2_AUTHEN_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED2_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED2_AUTHEN_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED2_AUTHEN_TZ(pub u32);
impl CCM_GPR_SHARED2_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_GPR_SHARED2_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED2_AUTHEN_TZ {
        CCM_GPR_SHARED2_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED2_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED2_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED2_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED2_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED2_AUTHEN_WHITE(pub u32);
impl CCM_GPR_SHARED2_AUTHEN_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_GPR_SHARED2_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED2_AUTHEN_WHITE {
        CCM_GPR_SHARED2_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED2_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED2_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED2_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED2_AUTHEN_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED2_DRAM_PLL(pub u32);
impl CCM_GPR_SHARED2_DRAM_PLL {
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_GPR_SHARED2_DRAM_PLL {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED2_DRAM_PLL {
        CCM_GPR_SHARED2_DRAM_PLL(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED2_DRAM_PLL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED2_DRAM_PLL")
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED2_DRAM_PLL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED2_DRAM_PLL {{ BYPASS: {=bool:?} }}",
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED_LOCK(pub u32);
impl CCM_GPR_SHARED_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_GPR_SHARED_LOCK {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED_LOCK {
        CCM_GPR_SHARED_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED_LOCK")
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED_LOCK {{ TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED_TZ(pub u32);
impl CCM_GPR_SHARED_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_GPR_SHARED_TZ {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED_TZ {
        CCM_GPR_SHARED_TZ(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_GPR_SHARED_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_GPR_SHARED_WHITE(pub u32);
impl CCM_GPR_SHARED_WHITE {
    #[doc = "White list settings"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list settings"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_GPR_SHARED_WHITE {
    #[inline(always)]
    fn default() -> CCM_GPR_SHARED_WHITE {
        CCM_GPR_SHARED_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_GPR_SHARED_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_GPR_SHARED_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_GPR_SHARED_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_GPR_SHARED_WHITE {{ LIST: {=u16:?} }}", self.LIST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_AUTHEN_ACK(pub u32);
impl CCM_LPCG_AUTHEN_ACK {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for CCM_LPCG_AUTHEN_ACK {
    #[inline(always)]
    fn default() -> CCM_LPCG_AUTHEN_ACK {
        CCM_LPCG_AUTHEN_ACK(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_AUTHEN_ACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_AUTHEN_ACK")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_AUTHEN_ACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_LPCG_AUTHEN_ACK {{ MODE: {=bool:?} }}", self.MODE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_AUTHEN_CPULPM(pub u32);
impl CCM_LPCG_AUTHEN_CPULPM {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for CCM_LPCG_AUTHEN_CPULPM {
    #[inline(always)]
    fn default() -> CCM_LPCG_AUTHEN_CPULPM {
        CCM_LPCG_AUTHEN_CPULPM(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_AUTHEN_CPULPM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_AUTHEN_CPULPM")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_AUTHEN_CPULPM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_AUTHEN_CPULPM {{ MODE: {=bool:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_AUTHEN_LOCK(pub u32);
impl CCM_LPCG_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_LPCG_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_LPCG_AUTHEN_LOCK {
        CCM_LPCG_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_AUTHEN_LOCK")
            .field("MODE", &self.MODE())
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_AUTHEN_LOCK {{ MODE: {=bool:?}, TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.MODE(),
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_AUTHEN_TZ(pub u32);
impl CCM_LPCG_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_LPCG_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_LPCG_AUTHEN_TZ {
        CCM_LPCG_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_AUTHEN_WHITE(pub u32);
impl CCM_LPCG_AUTHEN_WHITE {
    #[doc = "White list"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_LPCG_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_LPCG_AUTHEN_WHITE {
        CCM_LPCG_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_LPCG_AUTHEN_WHITE {{ LIST: {=u16:?} }}", self.LIST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_DIRECT(pub u32);
impl CCM_LPCG_DIRECT {
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_LPCG_DIRECT {
    #[inline(always)]
    fn default() -> CCM_LPCG_DIRECT {
        CCM_LPCG_DIRECT(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_DIRECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_DIRECT")
            .field("ON", &self.ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_DIRECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_LPCG_DIRECT {{ ON: {=bool:?} }}", self.ON())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_DIRECT_CLKOFF_ACK(pub u32);
impl CCM_LPCG_DIRECT_CLKOFF_ACK {
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for CCM_LPCG_DIRECT_CLKOFF_ACK {
    #[inline(always)]
    fn default() -> CCM_LPCG_DIRECT_CLKOFF_ACK {
        CCM_LPCG_DIRECT_CLKOFF_ACK(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_DIRECT_CLKOFF_ACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_DIRECT_CLKOFF_ACK")
            .field("TIMEOUT", &self.TIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_DIRECT_CLKOFF_ACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_DIRECT_CLKOFF_ACK {{ TIMEOUT: {=bool:?} }}",
            self.TIMEOUT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT(pub u32);
impl CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {
    #[inline(always)]
    fn default() -> CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {
        CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT")
            .field("EN", &self.EN())
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_DIRECT_CLKOFF_ACK_TIMEOUT {{ EN: {=bool:?}, CLR: {=bool:?} }}",
            self.EN(),
            self.CLR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_DIRECT_QACCEPT_N(pub u32);
impl CCM_LPCG_DIRECT_QACCEPT_N {
    #[doc = "timeout counter"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "timeout counter"]
    #[inline(always)]
    pub const fn set_TIMEOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for CCM_LPCG_DIRECT_QACCEPT_N {
    #[inline(always)]
    fn default() -> CCM_LPCG_DIRECT_QACCEPT_N {
        CCM_LPCG_DIRECT_QACCEPT_N(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_DIRECT_QACCEPT_N {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_DIRECT_QACCEPT_N")
            .field("TIMEOUT", &self.TIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_DIRECT_QACCEPT_N {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_DIRECT_QACCEPT_N {{ TIMEOUT: {=u8:?} }}",
            self.TIMEOUT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM0_LPM_SETTING(pub u32);
impl CCM_LPCG_LPM0_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn D0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_LPCG_LPM0_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM0_LPM_SETTING {
        CCM_LPCG_LPM0_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM0_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM0_LPM_SETTING")
            .field("D0", &self.D0())
            .field("D1", &self.D1())
            .field("D2", &self.D2())
            .field("D3", &self.D3())
            .field("D4", &self.D4())
            .field("D5", &self.D5())
            .field("D6", &self.D6())
            .field("D7", &self.D7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM0_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM0_LPM_SETTING {{ D0: {=u8:?}, D1: {=u8:?}, D2: {=u8:?}, D3: {=u8:?}, D4: {=u8:?}, D5: {=u8:?}, D6: {=u8:?}, D7: {=u8:?} }}",
            self.D0(),
            self.D1(),
            self.D2(),
            self.D3(),
            self.D4(),
            self.D5(),
            self.D6(),
            self.D7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM1_LPM_SETTING(pub u32);
impl CCM_LPCG_LPM1_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn D8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_LPCG_LPM1_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM1_LPM_SETTING {
        CCM_LPCG_LPM1_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM1_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM1_LPM_SETTING")
            .field("D8", &self.D8())
            .field("D9", &self.D9())
            .field("D10", &self.D10())
            .field("D11", &self.D11())
            .field("D12", &self.D12())
            .field("D13", &self.D13())
            .field("D14", &self.D14())
            .field("D15", &self.D15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM1_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM1_LPM_SETTING {{ D8: {=u8:?}, D9: {=u8:?}, D10: {=u8:?}, D11: {=u8:?}, D12: {=u8:?}, D13: {=u8:?}, D14: {=u8:?}, D15: {=u8:?} }}",
            self.D8(),
            self.D9(),
            self.D10(),
            self.D11(),
            self.D12(),
            self.D13(),
            self.D14(),
            self.D15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM_CUR_LPM_SETTING(pub u32);
impl CCM_LPCG_LPM_CUR_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn CUR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CUR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for CCM_LPCG_LPM_CUR_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM_CUR_LPM_SETTING {
        CCM_LPCG_LPM_CUR_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM_CUR_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM_CUR_LPM_SETTING")
            .field("CUR", &self.CUR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM_CUR_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM_CUR_LPM_SETTING {{ CUR: {=u8:?} }}",
            self.CUR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM_STATUS0_CPU_MODE(pub u32);
impl CCM_LPCG_LPM_STATUS0_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_LPCG_LPM_STATUS0_CPU_MODE {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM_STATUS0_CPU_MODE {
        CCM_LPCG_LPM_STATUS0_CPU_MODE(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM_STATUS0_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM_STATUS0_CPU_MODE")
            .field("DOMAIN0", &self.DOMAIN0())
            .field("DOMAIN1", &self.DOMAIN1())
            .field("DOMAIN2", &self.DOMAIN2())
            .field("DOMAIN3", &self.DOMAIN3())
            .field("DOMAIN4", &self.DOMAIN4())
            .field("DOMAIN5", &self.DOMAIN5())
            .field("DOMAIN6", &self.DOMAIN6())
            .field("DOMAIN7", &self.DOMAIN7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM_STATUS0_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM_STATUS0_CPU_MODE {{ DOMAIN0: {=u8:?}, DOMAIN1: {=u8:?}, DOMAIN2: {=u8:?}, DOMAIN3: {=u8:?}, DOMAIN4: {=u8:?}, DOMAIN5: {=u8:?}, DOMAIN6: {=u8:?}, DOMAIN7: {=u8:?} }}",
            self.DOMAIN0(),
            self.DOMAIN1(),
            self.DOMAIN2(),
            self.DOMAIN3(),
            self.DOMAIN4(),
            self.DOMAIN5(),
            self.DOMAIN6(),
            self.DOMAIN7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM_STATUS0_TRANS_REQ(pub u32);
impl CCM_LPCG_LPM_STATUS0_TRANS_REQ {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN4(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN6(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN7(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CCM_LPCG_LPM_STATUS0_TRANS_REQ {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM_STATUS0_TRANS_REQ {
        CCM_LPCG_LPM_STATUS0_TRANS_REQ(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM_STATUS0_TRANS_REQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM_STATUS0_TRANS_REQ")
            .field("DOMAIN0", &self.DOMAIN0())
            .field("DOMAIN1", &self.DOMAIN1())
            .field("DOMAIN2", &self.DOMAIN2())
            .field("DOMAIN3", &self.DOMAIN3())
            .field("DOMAIN4", &self.DOMAIN4())
            .field("DOMAIN5", &self.DOMAIN5())
            .field("DOMAIN6", &self.DOMAIN6())
            .field("DOMAIN7", &self.DOMAIN7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM_STATUS0_TRANS_REQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM_STATUS0_TRANS_REQ {{ DOMAIN0: {=bool:?}, DOMAIN1: {=bool:?}, DOMAIN2: {=bool:?}, DOMAIN3: {=bool:?}, DOMAIN4: {=bool:?}, DOMAIN5: {=bool:?}, DOMAIN6: {=bool:?}, DOMAIN7: {=bool:?} }}",
            self.DOMAIN0(),
            self.DOMAIN1(),
            self.DOMAIN2(),
            self.DOMAIN3(),
            self.DOMAIN4(),
            self.DOMAIN5(),
            self.DOMAIN6(),
            self.DOMAIN7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM_STATUS1_CPU_MODE(pub u32);
impl CCM_LPCG_LPM_STATUS1_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_LPCG_LPM_STATUS1_CPU_MODE {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM_STATUS1_CPU_MODE {
        CCM_LPCG_LPM_STATUS1_CPU_MODE(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM_STATUS1_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM_STATUS1_CPU_MODE")
            .field("DOMAIN8", &self.DOMAIN8())
            .field("DOMAIN9", &self.DOMAIN9())
            .field("DOMAIN10", &self.DOMAIN10())
            .field("DOMAIN11", &self.DOMAIN11())
            .field("DOMAIN12", &self.DOMAIN12())
            .field("DOMAIN13", &self.DOMAIN13())
            .field("DOMAIN14", &self.DOMAIN14())
            .field("DOMAIN15", &self.DOMAIN15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM_STATUS1_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM_STATUS1_CPU_MODE {{ DOMAIN8: {=u8:?}, DOMAIN9: {=u8:?}, DOMAIN10: {=u8:?}, DOMAIN11: {=u8:?}, DOMAIN12: {=u8:?}, DOMAIN13: {=u8:?}, DOMAIN14: {=u8:?}, DOMAIN15: {=u8:?} }}",
            self.DOMAIN8(),
            self.DOMAIN9(),
            self.DOMAIN10(),
            self.DOMAIN11(),
            self.DOMAIN12(),
            self.DOMAIN13(),
            self.DOMAIN14(),
            self.DOMAIN15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_LPM_STATUS1_TRANS_REQ(pub u32);
impl CCM_LPCG_LPM_STATUS1_TRANS_REQ {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN8(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN9(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN11(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN12(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN13(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN14(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN15(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CCM_LPCG_LPM_STATUS1_TRANS_REQ {
    #[inline(always)]
    fn default() -> CCM_LPCG_LPM_STATUS1_TRANS_REQ {
        CCM_LPCG_LPM_STATUS1_TRANS_REQ(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_LPM_STATUS1_TRANS_REQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_LPM_STATUS1_TRANS_REQ")
            .field("DOMAIN8", &self.DOMAIN8())
            .field("DOMAIN9", &self.DOMAIN9())
            .field("DOMAIN10", &self.DOMAIN10())
            .field("DOMAIN11", &self.DOMAIN11())
            .field("DOMAIN12", &self.DOMAIN12())
            .field("DOMAIN13", &self.DOMAIN13())
            .field("DOMAIN14", &self.DOMAIN14())
            .field("DOMAIN15", &self.DOMAIN15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_LPM_STATUS1_TRANS_REQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_LPM_STATUS1_TRANS_REQ {{ DOMAIN8: {=bool:?}, DOMAIN9: {=bool:?}, DOMAIN10: {=bool:?}, DOMAIN11: {=bool:?}, DOMAIN12: {=bool:?}, DOMAIN13: {=bool:?}, DOMAIN14: {=bool:?}, DOMAIN15: {=bool:?} }}",
            self.DOMAIN8(),
            self.DOMAIN9(),
            self.DOMAIN10(),
            self.DOMAIN11(),
            self.DOMAIN12(),
            self.DOMAIN13(),
            self.DOMAIN14(),
            self.DOMAIN15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_STATUS0(pub u32);
impl CCM_LPCG_STATUS0 {
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_LPCG_STATUS0 {
    #[inline(always)]
    fn default() -> CCM_LPCG_STATUS0 {
        CCM_LPCG_STATUS0(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_STATUS0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_STATUS0")
            .field("ON", &self.ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_STATUS0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_LPCG_STATUS0 {{ ON: {=bool:?} }}", self.ON())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_LPCG_STATUS1_DOMAIN(pub u32);
impl CCM_LPCG_STATUS1_DOMAIN {
    #[doc = "Domain active"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain active"]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Domain active"]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain active"]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_LPCG_STATUS1_DOMAIN {
    #[inline(always)]
    fn default() -> CCM_LPCG_STATUS1_DOMAIN {
        CCM_LPCG_STATUS1_DOMAIN(0)
    }
}
impl core::fmt::Debug for CCM_LPCG_STATUS1_DOMAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_LPCG_STATUS1_DOMAIN")
            .field("ACTIVE", &self.ACTIVE())
            .field("ENABLE", &self.ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_LPCG_STATUS1_DOMAIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_LPCG_STATUS1_DOMAIN {{ ACTIVE: {=u16:?}, ENABLE: {=u16:?} }}",
            self.ACTIVE(),
            self.ENABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_AUTHEN_AUTO(pub u32);
impl CCM_OSCPLL_AUTHEN_AUTO {
    #[must_use]
    #[inline(always)]
    pub const fn CTRL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for CCM_OSCPLL_AUTHEN_AUTO {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_AUTHEN_AUTO {
        CCM_OSCPLL_AUTHEN_AUTO(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_AUTHEN_AUTO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_AUTHEN_AUTO")
            .field("CTRL", &self.CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_AUTHEN_AUTO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_AUTHEN_AUTO {{ CTRL: {=bool:?} }}",
            self.CTRL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_AUTHEN_LOCK(pub u32);
impl CCM_OSCPLL_AUTHEN_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TZ(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TZ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCM_OSCPLL_AUTHEN_LOCK {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_AUTHEN_LOCK {
        CCM_OSCPLL_AUTHEN_LOCK(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_AUTHEN_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_AUTHEN_LOCK")
            .field("MODE", &self.MODE())
            .field("TZ", &self.TZ())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_AUTHEN_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_AUTHEN_LOCK {{ MODE: {=bool:?}, TZ: {=bool:?}, LIST: {=bool:?} }}",
            self.MODE(),
            self.TZ(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_AUTHEN_TZ(pub u32);
impl CCM_OSCPLL_AUTHEN_TZ {
    #[must_use]
    #[inline(always)]
    pub const fn USER(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for CCM_OSCPLL_AUTHEN_TZ {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_AUTHEN_TZ {
        CCM_OSCPLL_AUTHEN_TZ(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_AUTHEN_TZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_AUTHEN_TZ")
            .field("USER", &self.USER())
            .field("NS", &self.NS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_AUTHEN_TZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_AUTHEN_TZ {{ USER: {=bool:?}, NS: {=bool:?} }}",
            self.USER(),
            self.NS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_AUTHEN_WHITE(pub u32);
impl CCM_OSCPLL_AUTHEN_WHITE {
    #[doc = "White list"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "White list"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_OSCPLL_AUTHEN_WHITE {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_AUTHEN_WHITE {
        CCM_OSCPLL_AUTHEN_WHITE(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_AUTHEN_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_AUTHEN_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_AUTHEN_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_AUTHEN_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_DIRECT(pub u32);
impl CCM_OSCPLL_DIRECT {
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_OSCPLL_DIRECT {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_DIRECT {
        CCM_OSCPLL_DIRECT(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_DIRECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_DIRECT")
            .field("ON", &self.ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_DIRECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_OSCPLL_DIRECT {{ ON: {=bool:?} }}", self.ON())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM0_LPM_SETTING(pub u32);
impl CCM_OSCPLL_LPM0_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn D0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for CCM_OSCPLL_LPM0_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM0_LPM_SETTING {
        CCM_OSCPLL_LPM0_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM0_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM0_LPM_SETTING")
            .field("D0", &self.D0())
            .field("D1", &self.D1())
            .field("D2", &self.D2())
            .field("D3", &self.D3())
            .field("D4", &self.D4())
            .field("D5", &self.D5())
            .field("D6", &self.D6())
            .field("D7", &self.D7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM0_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM0_LPM_SETTING {{ D0: {=u8:?}, D1: {=u8:?}, D2: {=u8:?}, D3: {=u8:?}, D4: {=u8:?}, D5: {=u8:?}, D6: {=u8:?}, D7: {=u8:?} }}",
            self.D0(),
            self.D1(),
            self.D2(),
            self.D3(),
            self.D4(),
            self.D5(),
            self.D6(),
            self.D7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM1_LPM_SETTING(pub u32);
impl CCM_OSCPLL_LPM1_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn D8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn D15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_D15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for CCM_OSCPLL_LPM1_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM1_LPM_SETTING {
        CCM_OSCPLL_LPM1_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM1_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM1_LPM_SETTING")
            .field("D8", &self.D8())
            .field("D9", &self.D9())
            .field("D10", &self.D10())
            .field("D11", &self.D11())
            .field("D12", &self.D12())
            .field("D13", &self.D13())
            .field("D14", &self.D14())
            .field("D15", &self.D15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM1_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM1_LPM_SETTING {{ D8: {=u8:?}, D9: {=u8:?}, D10: {=u8:?}, D11: {=u8:?}, D12: {=u8:?}, D13: {=u8:?}, D14: {=u8:?}, D15: {=u8:?} }}",
            self.D8(),
            self.D9(),
            self.D10(),
            self.D11(),
            self.D12(),
            self.D13(),
            self.D14(),
            self.D15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM_CUR_LPM_SETTING(pub u32);
impl CCM_OSCPLL_LPM_CUR_LPM_SETTING {
    #[must_use]
    #[inline(always)]
    pub const fn CUR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CUR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for CCM_OSCPLL_LPM_CUR_LPM_SETTING {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM_CUR_LPM_SETTING {
        CCM_OSCPLL_LPM_CUR_LPM_SETTING(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM_CUR_LPM_SETTING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM_CUR_LPM_SETTING")
            .field("CUR", &self.CUR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM_CUR_LPM_SETTING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM_CUR_LPM_SETTING {{ CUR: {=u8:?} }}",
            self.CUR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM_STATUS0_CPU_MODE(pub u32);
impl CCM_OSCPLL_LPM_STATUS0_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_OSCPLL_LPM_STATUS0_CPU_MODE {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM_STATUS0_CPU_MODE {
        CCM_OSCPLL_LPM_STATUS0_CPU_MODE(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM_STATUS0_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM_STATUS0_CPU_MODE")
            .field("DOMAIN0", &self.DOMAIN0())
            .field("DOMAIN1", &self.DOMAIN1())
            .field("DOMAIN2", &self.DOMAIN2())
            .field("DOMAIN3", &self.DOMAIN3())
            .field("DOMAIN4", &self.DOMAIN4())
            .field("DOMAIN5", &self.DOMAIN5())
            .field("DOMAIN6", &self.DOMAIN6())
            .field("DOMAIN7", &self.DOMAIN7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM_STATUS0_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM_STATUS0_CPU_MODE {{ DOMAIN0: {=u8:?}, DOMAIN1: {=u8:?}, DOMAIN2: {=u8:?}, DOMAIN3: {=u8:?}, DOMAIN4: {=u8:?}, DOMAIN5: {=u8:?}, DOMAIN6: {=u8:?}, DOMAIN7: {=u8:?} }}",
            self.DOMAIN0(),
            self.DOMAIN1(),
            self.DOMAIN2(),
            self.DOMAIN3(),
            self.DOMAIN4(),
            self.DOMAIN5(),
            self.DOMAIN6(),
            self.DOMAIN7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM_STATUS0_TRANS_REQ(pub u32);
impl CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN4(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN5(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN6(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN7(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {
        CCM_OSCPLL_LPM_STATUS0_TRANS_REQ(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM_STATUS0_TRANS_REQ")
            .field("DOMAIN0", &self.DOMAIN0())
            .field("DOMAIN1", &self.DOMAIN1())
            .field("DOMAIN2", &self.DOMAIN2())
            .field("DOMAIN3", &self.DOMAIN3())
            .field("DOMAIN4", &self.DOMAIN4())
            .field("DOMAIN5", &self.DOMAIN5())
            .field("DOMAIN6", &self.DOMAIN6())
            .field("DOMAIN7", &self.DOMAIN7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM_STATUS0_TRANS_REQ {{ DOMAIN0: {=bool:?}, DOMAIN1: {=bool:?}, DOMAIN2: {=bool:?}, DOMAIN3: {=bool:?}, DOMAIN4: {=bool:?}, DOMAIN5: {=bool:?}, DOMAIN6: {=bool:?}, DOMAIN7: {=bool:?} }}",
            self.DOMAIN0(),
            self.DOMAIN1(),
            self.DOMAIN2(),
            self.DOMAIN3(),
            self.DOMAIN4(),
            self.DOMAIN5(),
            self.DOMAIN6(),
            self.DOMAIN7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM_STATUS1_CPU_MODE(pub u32);
impl CCM_OSCPLL_LPM_STATUS1_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for CCM_OSCPLL_LPM_STATUS1_CPU_MODE {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM_STATUS1_CPU_MODE {
        CCM_OSCPLL_LPM_STATUS1_CPU_MODE(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM_STATUS1_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM_STATUS1_CPU_MODE")
            .field("DOMAIN8", &self.DOMAIN8())
            .field("DOMAIN9", &self.DOMAIN9())
            .field("DOMAIN10", &self.DOMAIN10())
            .field("DOMAIN11", &self.DOMAIN11())
            .field("DOMAIN12", &self.DOMAIN12())
            .field("DOMAIN13", &self.DOMAIN13())
            .field("DOMAIN14", &self.DOMAIN14())
            .field("DOMAIN15", &self.DOMAIN15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM_STATUS1_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM_STATUS1_CPU_MODE {{ DOMAIN8: {=u8:?}, DOMAIN9: {=u8:?}, DOMAIN10: {=u8:?}, DOMAIN11: {=u8:?}, DOMAIN12: {=u8:?}, DOMAIN13: {=u8:?}, DOMAIN14: {=u8:?}, DOMAIN15: {=u8:?} }}",
            self.DOMAIN8(),
            self.DOMAIN9(),
            self.DOMAIN10(),
            self.DOMAIN11(),
            self.DOMAIN12(),
            self.DOMAIN13(),
            self.DOMAIN14(),
            self.DOMAIN15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_LPM_STATUS1_TRANS_REQ(pub u32);
impl CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN8(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN9(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN11(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN12(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN13(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN14(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN15(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DOMAIN15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {
        CCM_OSCPLL_LPM_STATUS1_TRANS_REQ(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_LPM_STATUS1_TRANS_REQ")
            .field("DOMAIN8", &self.DOMAIN8())
            .field("DOMAIN9", &self.DOMAIN9())
            .field("DOMAIN10", &self.DOMAIN10())
            .field("DOMAIN11", &self.DOMAIN11())
            .field("DOMAIN12", &self.DOMAIN12())
            .field("DOMAIN13", &self.DOMAIN13())
            .field("DOMAIN14", &self.DOMAIN14())
            .field("DOMAIN15", &self.DOMAIN15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_LPM_STATUS1_TRANS_REQ {{ DOMAIN8: {=bool:?}, DOMAIN9: {=bool:?}, DOMAIN10: {=bool:?}, DOMAIN11: {=bool:?}, DOMAIN12: {=bool:?}, DOMAIN13: {=bool:?}, DOMAIN14: {=bool:?}, DOMAIN15: {=bool:?} }}",
            self.DOMAIN8(),
            self.DOMAIN9(),
            self.DOMAIN10(),
            self.DOMAIN11(),
            self.DOMAIN12(),
            self.DOMAIN13(),
            self.DOMAIN14(),
            self.DOMAIN15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_STATUS0(pub u32);
impl CCM_OSCPLL_STATUS0 {
    #[must_use]
    #[inline(always)]
    pub const fn ON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CCM_OSCPLL_STATUS0 {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_STATUS0 {
        CCM_OSCPLL_STATUS0(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_STATUS0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_STATUS0")
            .field("ON", &self.ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_STATUS0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_OSCPLL_STATUS0 {{ ON: {=bool:?} }}", self.ON())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_STATUS0_IN(pub u32);
impl CCM_OSCPLL_STATUS0_IN {
    #[must_use]
    #[inline(always)]
    pub const fn USE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_USE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for CCM_OSCPLL_STATUS0_IN {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_STATUS0_IN {
        CCM_OSCPLL_STATUS0_IN(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_STATUS0_IN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_STATUS0_IN")
            .field("USE", &self.USE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_STATUS0_IN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CCM_OSCPLL_STATUS0_IN {{ USE: {=bool:?} }}", self.USE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_STATUS0_STATUS(pub u32);
impl CCM_OSCPLL_STATUS0_STATUS {
    #[must_use]
    #[inline(always)]
    pub const fn EARLY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EARLY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn LATE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CCM_OSCPLL_STATUS0_STATUS {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_STATUS0_STATUS {
        CCM_OSCPLL_STATUS0_STATUS(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_STATUS0_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_STATUS0_STATUS")
            .field("EARLY", &self.EARLY())
            .field("LATE", &self.LATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_STATUS0_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_STATUS0_STATUS {{ EARLY: {=bool:?}, LATE: {=bool:?} }}",
            self.EARLY(),
            self.LATE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCM_OSCPLL_STATUS1_DOMAIN(pub u32);
impl CCM_OSCPLL_STATUS1_DOMAIN {
    #[doc = "Domain active"]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain active"]
    #[inline(always)]
    pub const fn set_ACTIVE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Domain enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain enable"]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CCM_OSCPLL_STATUS1_DOMAIN {
    #[inline(always)]
    fn default() -> CCM_OSCPLL_STATUS1_DOMAIN {
        CCM_OSCPLL_STATUS1_DOMAIN(0)
    }
}
impl core::fmt::Debug for CCM_OSCPLL_STATUS1_DOMAIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_OSCPLL_STATUS1_DOMAIN")
            .field("ACTIVE", &self.ACTIVE())
            .field("ENABLE", &self.ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCM_OSCPLL_STATUS1_DOMAIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCM_OSCPLL_STATUS1_DOMAIN {{ ACTIVE: {=u16:?}, ENABLE: {=u16:?} }}",
            self.ACTIVE(),
            self.ENABLE()
        )
    }
}

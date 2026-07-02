#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL {
    ptr: *mut u8,
}
unsafe impl Send for GPC_GLOBAL {}
unsafe impl Sync for GPC_GLOBAL {}
impl GPC_GLOBAL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GPC_GLOB_AUTHEN_CTRL(
        self,
    ) -> crate::common::Reg<GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_MASTER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_SYS_SLEEP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_CPU_DOMAIN_ASSIGNMENT(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_PMIC_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_PMIC_STBY_ACK_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_ROSC_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn GPC_ELE_HDSK_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU(pub u32);
impl GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {
    #[must_use]
    #[inline(always)]
    pub const fn DOMAIN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_DOMAIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {
        GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU")
            .field("DOMAIN", &self.DOMAIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_CPU_DOMAIN_ASSIGNMENT_CPU {{ DOMAIN: {=u8:?} }}",
            self.DOMAIN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK(pub u32);
impl GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {
        GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_ELE_HDSK_CTRL_ELE_HDSK {{ EN: {=bool:?} }}",
            self.EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL(pub u32);
impl GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {
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
    pub const fn NONSECURE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NONSECURE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {
        GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL")
            .field("USER", &self.USER())
            .field("NONSECURE", &self.NONSECURE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL {{ USER: {=bool:?}, NONSECURE: {=bool:?} }}",
            self.USER(),
            self.NONSECURE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK(pub u32);
impl GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SETTING(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SETTING(&mut self, val: bool) {
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
impl Default for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {
        GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK")
            .field("CFG", &self.CFG())
            .field("SETTING", &self.SETTING())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_LOCK {{ CFG: {=bool:?}, SETTING: {=bool:?}, LIST: {=bool:?} }}",
            self.CFG(),
            self.SETTING(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE(pub u32);
impl GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {
        GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_GLOB_AUTHEN_CTRL_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_MASTER_CPU(pub u32);
impl GPC_GLOBAL_GPC_MASTER_CPU {
    #[must_use]
    #[inline(always)]
    pub const fn MASTER(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_MASTER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for GPC_GLOBAL_GPC_MASTER_CPU {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_MASTER_CPU {
        GPC_GLOBAL_GPC_MASTER_CPU(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_MASTER_CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_MASTER_CPU")
            .field("MASTER", &self.MASTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_MASTER_CPU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_MASTER_CPU {{ MASTER: {=u16:?} }}",
            self.MASTER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY(pub u32);
impl GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {
        GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_PMIC_CTRL_PMIC_STBY {{ EN: {=bool:?} }}",
            self.EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT(pub u32);
impl GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {
        GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT")
            .field("CFG", &self.CFG())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_OFF_CNT {{ CFG: {=u16:?}, MODE: {=u8:?} }}",
            self.CFG(),
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT(pub u32);
impl GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn CFG(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_CFG(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {
        GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT")
            .field("CFG", &self.CFG())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_PMIC_STBY_ACK_CTRL_STBY_ON_CNT {{ CFG: {=u16:?}, MODE: {=u8:?} }}",
            self.CFG(),
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF(pub u32);
impl GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {
        GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_ROSC_CTRL_ROSC_OFF {{ EN: {=bool:?} }}",
            self.EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU(pub u32);
impl GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {
    #[inline(always)]
    fn default() -> GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {
        GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU(0)
    }
}
impl core::fmt::Debug for GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_GLOBAL_GPC_SYS_SLEEP_FORCE_CPU {{ DISABLE: {=u16:?} }}",
            self.DISABLE()
        )
    }
}

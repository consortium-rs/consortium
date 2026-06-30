#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC {
    ptr: *mut u8,
}
unsafe impl Send for GPC {}
unsafe impl Sync for GPC {}
impl GPC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CMC_AUTHEN_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_AUTHEN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_MISC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_MODE_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_MODE_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_PIN_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_IRQ_WAKEUP_MASK(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CMC_NON_IRQ_WAKEUP_MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_IRQ_WAKEUP_STAT(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn CMC_NON_IRQ_WAKEUP_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_A55_HDSK_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_SLEEP_A55_HDSK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_A55_HDSK_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_SSAR_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_SLEEP_SSAR_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_SSAR_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_RESET_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_SLEEP_RESET_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_RESET_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_SYSMAN_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_SLEEP_SYSMAN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SLEEP_SYSMAN_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_POWER_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_WAKEUP_POWER_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_POWER_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_SSAR_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_WAKEUP_SSAR_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_SSAR_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_A55_HDSK_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_WAKEUP_A55_HDSK_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_A55_HDSK_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_SYSMAN_CTRL(
        self,
    ) -> crate::common::Reg<GPC_CMC_WAKEUP_SYSMAN_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_WAKEUP_SYSMAN_STAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_SYS_SLEEP_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn CMC_DEBUG(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_AUTHEN_CTRL(pub u32);
impl GPC_CMC_AUTHEN_CTRL {
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
impl Default for GPC_CMC_AUTHEN_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_AUTHEN_CTRL {
        GPC_CMC_AUTHEN_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_AUTHEN_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_AUTHEN_CTRL")
            .field("USER", &self.USER())
            .field("NONSECURE", &self.NONSECURE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_AUTHEN_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_AUTHEN_CTRL {{ USER: {=bool:?}, NONSECURE: {=bool:?} }}",
            self.USER(),
            self.NONSECURE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_AUTHEN_CTRL_LOCK(pub u32);
impl GPC_CMC_AUTHEN_CTRL_LOCK {
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
impl Default for GPC_CMC_AUTHEN_CTRL_LOCK {
    #[inline(always)]
    fn default() -> GPC_CMC_AUTHEN_CTRL_LOCK {
        GPC_CMC_AUTHEN_CTRL_LOCK(0)
    }
}
impl core::fmt::Debug for GPC_CMC_AUTHEN_CTRL_LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_AUTHEN_CTRL_LOCK")
            .field("CFG", &self.CFG())
            .field("SETTING", &self.SETTING())
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_AUTHEN_CTRL_LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_AUTHEN_CTRL_LOCK {{ CFG: {=bool:?}, SETTING: {=bool:?}, LIST: {=bool:?} }}",
            self.CFG(),
            self.SETTING(),
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_AUTHEN_CTRL_WHITE(pub u32);
impl GPC_CMC_AUTHEN_CTRL_WHITE {
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn LIST(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_LIST(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for GPC_CMC_AUTHEN_CTRL_WHITE {
    #[inline(always)]
    fn default() -> GPC_CMC_AUTHEN_CTRL_WHITE {
        GPC_CMC_AUTHEN_CTRL_WHITE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_AUTHEN_CTRL_WHITE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_AUTHEN_CTRL_WHITE")
            .field("LIST", &self.LIST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_AUTHEN_CTRL_WHITE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_AUTHEN_CTRL_WHITE {{ LIST: {=u16:?} }}",
            self.LIST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_DEBUG_PRETEND(pub u32);
impl GPC_CMC_DEBUG_PRETEND {
    #[must_use]
    #[inline(always)]
    pub const fn SLEEP(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SLEEP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_CMC_DEBUG_PRETEND {
    #[inline(always)]
    fn default() -> GPC_CMC_DEBUG_PRETEND {
        GPC_CMC_DEBUG_PRETEND(0)
    }
}
impl core::fmt::Debug for GPC_CMC_DEBUG_PRETEND {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_DEBUG_PRETEND")
            .field("SLEEP", &self.SLEEP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_DEBUG_PRETEND {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_DEBUG_PRETEND {{ SLEEP: {=bool:?} }}",
            self.SLEEP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP(pub u32);
impl GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {
        GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP")
            .field("MASK", &self.MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_IRQ_WAKEUP_MASK_IRQ_WAKEUP {{ MASK: {=u32:?} }}",
            self.MASK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP(pub u32);
impl GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {
    #[doc = "IRQ status"]
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IRQ status"]
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {
        GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_IRQ_WAKEUP_STAT_IRQ_WAKEUP {{ STAT: {=u32:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MISC_GIC_WAKEUP(pub u32);
impl GPC_CMC_MISC_GIC_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GPC_CMC_MISC_GIC_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_MISC_GIC_WAKEUP {
        GPC_CMC_MISC_GIC_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MISC_GIC_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MISC_GIC_WAKEUP")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MISC_GIC_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MISC_GIC_WAKEUP {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MISC_IRQ(pub u32);
impl GPC_CMC_MISC_IRQ {
    #[must_use]
    #[inline(always)]
    pub const fn MUX(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MUX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for GPC_CMC_MISC_IRQ {
    #[inline(always)]
    fn default() -> GPC_CMC_MISC_IRQ {
        GPC_CMC_MISC_IRQ(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MISC_IRQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MISC_IRQ")
            .field("MUX", &self.MUX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MISC_IRQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPC_CMC_MISC_IRQ {{ MUX: {=bool:?} }}", self.MUX())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MISC_NMI(pub u32);
impl GPC_CMC_MISC_NMI {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_CMC_MISC_NMI {
    #[inline(always)]
    fn default() -> GPC_CMC_MISC_NMI {
        GPC_CMC_MISC_NMI(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MISC_NMI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MISC_NMI")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MISC_NMI {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPC_CMC_MISC_NMI {{ STAT: {=bool:?} }}", self.STAT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MISC_SLEEP_HOLD(pub u32);
impl GPC_CMC_MISC_SLEEP_HOLD {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for GPC_CMC_MISC_SLEEP_HOLD {
    #[inline(always)]
    fn default() -> GPC_CMC_MISC_SLEEP_HOLD {
        GPC_CMC_MISC_SLEEP_HOLD(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MISC_SLEEP_HOLD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MISC_SLEEP_HOLD")
            .field("EN", &self.EN())
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MISC_SLEEP_HOLD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MISC_SLEEP_HOLD {{ EN: {=bool:?}, STAT: {=bool:?} }}",
            self.EN(),
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MISC_SW(pub u32);
impl GPC_CMC_MISC_SW {
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for GPC_CMC_MISC_SW {
    #[inline(always)]
    fn default() -> GPC_CMC_MISC_SW {
        GPC_CMC_MISC_SW(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MISC_SW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MISC_SW")
            .field("WAKEUP", &self.WAKEUP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MISC_SW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPC_CMC_MISC_SW {{ WAKEUP: {=bool:?} }}", self.WAKEUP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_CTRL_CPU_MODE(pub u32);
impl GPC_CMC_MODE_CTRL_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn TARGET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TARGET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for GPC_CMC_MODE_CTRL_CPU_MODE {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_CTRL_CPU_MODE {
        GPC_CMC_MODE_CTRL_CPU_MODE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_CTRL_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_CTRL_CPU_MODE")
            .field("TARGET", &self.TARGET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_CTRL_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_CTRL_CPU_MODE {{ TARGET: {=u8:?} }}",
            self.TARGET()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_CTRL_WFE(pub u32);
impl GPC_CMC_MODE_CTRL_WFE {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for GPC_CMC_MODE_CTRL_WFE {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_CTRL_WFE {
        GPC_CMC_MODE_CTRL_WFE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_CTRL_WFE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_CTRL_WFE")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_CTRL_WFE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPC_CMC_MODE_CTRL_WFE {{ EN: {=bool:?} }}", self.EN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_CPU_MODE(pub u32);
impl GPC_CMC_MODE_STAT_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn CURRENT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CURRENT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PREVIOUS(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PREVIOUS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_CPU_MODE {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_CPU_MODE {
        GPC_CMC_MODE_STAT_CPU_MODE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_CPU_MODE")
            .field("CURRENT", &self.CURRENT())
            .field("PREVIOUS", &self.PREVIOUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_CPU_MODE {{ CURRENT: {=u8:?}, PREVIOUS: {=u8:?} }}",
            self.CURRENT(),
            self.PREVIOUS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_FSM(pub u32);
impl GPC_CMC_MODE_STAT_FSM {
    #[must_use]
    #[inline(always)]
    pub const fn STATE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_FSM {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_FSM {
        GPC_CMC_MODE_STAT_FSM(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_FSM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_FSM")
            .field("STATE", &self.STATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_FSM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_FSM {{ STATE: {=u8:?} }}",
            self.STATE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_SLEEP(pub u32);
impl GPC_CMC_MODE_STAT_SLEEP {
    #[must_use]
    #[inline(always)]
    pub const fn REQUEST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REQUEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_SLEEP {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_SLEEP {
        GPC_CMC_MODE_STAT_SLEEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_SLEEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_SLEEP")
            .field("REQUEST", &self.REQUEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_SLEEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_SLEEP {{ REQUEST: {=bool:?} }}",
            self.REQUEST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_SLEEPING(pub u32);
impl GPC_CMC_MODE_STAT_SLEEPING {
    #[must_use]
    #[inline(always)]
    pub const fn IDLE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_IDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_SLEEPING {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_SLEEPING {
        GPC_CMC_MODE_STAT_SLEEPING(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_SLEEPING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_SLEEPING")
            .field("IDLE", &self.IDLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_SLEEPING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_SLEEPING {{ IDLE: {=bool:?} }}",
            self.IDLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_SLEEP_TRANS(pub u32);
impl GPC_CMC_MODE_STAT_SLEEP_TRANS {
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_SLEEP_TRANS {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_SLEEP_TRANS {
        GPC_CMC_MODE_STAT_SLEEP_TRANS(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_SLEEP_TRANS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_SLEEP_TRANS")
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_SLEEP_TRANS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_SLEEP_TRANS {{ BUSY: {=bool:?} }}",
            self.BUSY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_WAKEUP(pub u32);
impl GPC_CMC_MODE_STAT_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn REQUEST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REQUEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_WAKEUP {
        GPC_CMC_MODE_STAT_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_WAKEUP")
            .field("REQUEST", &self.REQUEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_WAKEUP {{ REQUEST: {=bool:?} }}",
            self.REQUEST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_WAKEUP_TRANS(pub u32);
impl GPC_CMC_MODE_STAT_WAKEUP_TRANS {
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_WAKEUP_TRANS {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_WAKEUP_TRANS {
        GPC_CMC_MODE_STAT_WAKEUP_TRANS(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_WAKEUP_TRANS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_WAKEUP_TRANS")
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_WAKEUP_TRANS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_WAKEUP_TRANS {{ BUSY: {=bool:?} }}",
            self.BUSY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_MODE_STAT_WFE(pub u32);
impl GPC_CMC_MODE_STAT_WFE {
    #[must_use]
    #[inline(always)]
    pub const fn REQUEST(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_REQUEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for GPC_CMC_MODE_STAT_WFE {
    #[inline(always)]
    fn default() -> GPC_CMC_MODE_STAT_WFE {
        GPC_CMC_MODE_STAT_WFE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_MODE_STAT_WFE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_MODE_STAT_WFE")
            .field("REQUEST", &self.REQUEST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_MODE_STAT_WFE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_MODE_STAT_WFE {{ REQUEST: {=bool:?} }}",
            self.REQUEST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP(pub u32);
impl GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {
        GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP")
            .field("MASK", &self.MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_NON_IRQ_WAKEUP_MASK_DEBUG_WAKEUP {{ MASK: {=bool:?} }}",
            self.MASK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP(pub u32);
impl GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn MASK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MASK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {
        GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP")
            .field("MASK", &self.MASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_NON_IRQ_WAKEUP_MASK_EVENT_WAKEUP {{ MASK: {=bool:?} }}",
            self.MASK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP(pub u32);
impl GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {
        GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_NON_IRQ_WAKEUP_STAT_DEBUG_WAKEUP {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP(pub u32);
impl GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {
        GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_NON_IRQ_WAKEUP_STAT_EVENT_WAKEUP {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_A55_HDSK_DONE(pub u32);
impl GPC_CMC_PIN_STAT_A55_HDSK_DONE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_A55_HDSK_DONE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_A55_HDSK_DONE {
        GPC_CMC_PIN_STAT_A55_HDSK_DONE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_A55_HDSK_DONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_A55_HDSK_DONE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_A55_HDSK_DONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_A55_HDSK_DONE {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_A55_HDSK_REQUEST(pub u32);
impl GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {
        GPC_CMC_PIN_STAT_A55_HDSK_REQUEST(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_A55_HDSK_REQUEST")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_A55_HDSK_REQUEST {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_CPU_MODE(pub u32);
impl GPC_CMC_PIN_STAT_CPU_MODE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_CPU_MODE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_CPU_MODE {
        GPC_CMC_PIN_STAT_CPU_MODE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_CPU_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_CPU_MODE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_CPU_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_CPU_MODE {{ STAT: {=u8:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK(pub u32);
impl GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {
        GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_DEBUG_WAKEUP_ACK {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_POWER_DONE(pub u32);
impl GPC_CMC_PIN_STAT_POWER_DONE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_POWER_DONE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_POWER_DONE {
        GPC_CMC_PIN_STAT_POWER_DONE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_POWER_DONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_POWER_DONE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_POWER_DONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_POWER_DONE {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_POWER_REQUEST(pub u32);
impl GPC_CMC_PIN_STAT_POWER_REQUEST {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_POWER_REQUEST {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_POWER_REQUEST {
        GPC_CMC_PIN_STAT_POWER_REQUEST(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_POWER_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_POWER_REQUEST")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_POWER_REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_POWER_REQUEST {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_RESET_DONE(pub u32);
impl GPC_CMC_PIN_STAT_RESET_DONE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_RESET_DONE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_RESET_DONE {
        GPC_CMC_PIN_STAT_RESET_DONE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_RESET_DONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_RESET_DONE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_RESET_DONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_RESET_DONE {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_RESET_REQUEST(pub u32);
impl GPC_CMC_PIN_STAT_RESET_REQUEST {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_RESET_REQUEST {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_RESET_REQUEST {
        GPC_CMC_PIN_STAT_RESET_REQUEST(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_RESET_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_RESET_REQUEST")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_RESET_REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_RESET_REQUEST {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_SSAR_DONE(pub u32);
impl GPC_CMC_PIN_STAT_SSAR_DONE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_SSAR_DONE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_SSAR_DONE {
        GPC_CMC_PIN_STAT_SSAR_DONE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_SSAR_DONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_SSAR_DONE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_SSAR_DONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_SSAR_DONE {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_SSAR_REQUEST(pub u32);
impl GPC_CMC_PIN_STAT_SSAR_REQUEST {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_SSAR_REQUEST {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_SSAR_REQUEST {
        GPC_CMC_PIN_STAT_SSAR_REQUEST(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_SSAR_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_SSAR_REQUEST")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_SSAR_REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_SSAR_REQUEST {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_SYSMAN_DONE(pub u32);
impl GPC_CMC_PIN_STAT_SYSMAN_DONE {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_SYSMAN_DONE {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_SYSMAN_DONE {
        GPC_CMC_PIN_STAT_SYSMAN_DONE(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_SYSMAN_DONE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_SYSMAN_DONE")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_SYSMAN_DONE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_SYSMAN_DONE {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_PIN_STAT_SYSMAN_REQUEST(pub u32);
impl GPC_CMC_PIN_STAT_SYSMAN_REQUEST {
    #[must_use]
    #[inline(always)]
    pub const fn STAT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for GPC_CMC_PIN_STAT_SYSMAN_REQUEST {
    #[inline(always)]
    fn default() -> GPC_CMC_PIN_STAT_SYSMAN_REQUEST {
        GPC_CMC_PIN_STAT_SYSMAN_REQUEST(0)
    }
}
impl core::fmt::Debug for GPC_CMC_PIN_STAT_SYSMAN_REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_PIN_STAT_SYSMAN_REQUEST")
            .field("STAT", &self.STAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_PIN_STAT_SYSMAN_REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_PIN_STAT_SYSMAN_REQUEST {{ STAT: {=bool:?} }}",
            self.STAT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_A55_HDSK_CTRL(pub u32);
impl GPC_CMC_SLEEP_A55_HDSK_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_SLEEP_A55_HDSK_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_A55_HDSK_CTRL {
        GPC_CMC_SLEEP_A55_HDSK_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_A55_HDSK_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_A55_HDSK_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_A55_HDSK_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_A55_HDSK_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT(pub u32);
impl GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {
        GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_A55_HDSK_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP(pub u32);
impl GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {
        GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_A55_HDSK_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_A55_HDSK_STAT_RSP(pub u32);
impl GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {
        GPC_CMC_SLEEP_A55_HDSK_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_A55_HDSK_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_A55_HDSK_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_RESET_CTRL(pub u32);
impl GPC_CMC_SLEEP_RESET_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_SLEEP_RESET_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_RESET_CTRL {
        GPC_CMC_SLEEP_RESET_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_RESET_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_RESET_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_RESET_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_RESET_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_RESET_CTRL_CNT(pub u32);
impl GPC_CMC_SLEEP_RESET_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_SLEEP_RESET_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_RESET_CTRL_CNT {
        GPC_CMC_SLEEP_RESET_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_RESET_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_RESET_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_RESET_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_RESET_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_RESET_CTRL_STEP(pub u32);
impl GPC_CMC_SLEEP_RESET_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_RESET_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_RESET_CTRL_STEP {
        GPC_CMC_SLEEP_RESET_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_RESET_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_RESET_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_RESET_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_RESET_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_RESET_STAT_RSP(pub u32);
impl GPC_CMC_SLEEP_RESET_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_RESET_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_RESET_STAT_RSP {
        GPC_CMC_SLEEP_RESET_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_RESET_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_RESET_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_RESET_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_RESET_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SSAR_CTRL(pub u32);
impl GPC_CMC_SLEEP_SSAR_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_SLEEP_SSAR_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SSAR_CTRL {
        GPC_CMC_SLEEP_SSAR_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SSAR_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SSAR_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SSAR_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SSAR_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SSAR_CTRL_CNT(pub u32);
impl GPC_CMC_SLEEP_SSAR_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_SLEEP_SSAR_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SSAR_CTRL_CNT {
        GPC_CMC_SLEEP_SSAR_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SSAR_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SSAR_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SSAR_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SSAR_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SSAR_CTRL_STEP(pub u32);
impl GPC_CMC_SLEEP_SSAR_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_SSAR_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SSAR_CTRL_STEP {
        GPC_CMC_SLEEP_SSAR_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SSAR_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SSAR_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SSAR_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SSAR_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SSAR_STAT_RSP(pub u32);
impl GPC_CMC_SLEEP_SSAR_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_SSAR_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SSAR_STAT_RSP {
        GPC_CMC_SLEEP_SSAR_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SSAR_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SSAR_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SSAR_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SSAR_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SYSMAN_CTRL(pub u32);
impl GPC_CMC_SLEEP_SYSMAN_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_SLEEP_SYSMAN_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SYSMAN_CTRL {
        GPC_CMC_SLEEP_SYSMAN_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SYSMAN_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SYSMAN_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SYSMAN_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SYSMAN_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SYSMAN_CTRL_CNT(pub u32);
impl GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {
        GPC_CMC_SLEEP_SYSMAN_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SYSMAN_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SYSMAN_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SYSMAN_CTRL_STEP(pub u32);
impl GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {
        GPC_CMC_SLEEP_SYSMAN_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SYSMAN_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SYSMAN_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SLEEP_SYSMAN_STAT_RSP(pub u32);
impl GPC_CMC_SLEEP_SYSMAN_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_SLEEP_SYSMAN_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_SLEEP_SYSMAN_STAT_RSP {
        GPC_CMC_SLEEP_SYSMAN_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SLEEP_SYSMAN_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SLEEP_SYSMAN_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SLEEP_SYSMAN_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SLEEP_SYSMAN_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SYS_SLEEP_CTRL_SS(pub u32);
impl GPC_CMC_SYS_SLEEP_CTRL_SS {
    #[must_use]
    #[inline(always)]
    pub const fn WAIT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WAIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STOP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STOP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SUSPEND(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SUSPEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for GPC_CMC_SYS_SLEEP_CTRL_SS {
    #[inline(always)]
    fn default() -> GPC_CMC_SYS_SLEEP_CTRL_SS {
        GPC_CMC_SYS_SLEEP_CTRL_SS(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SYS_SLEEP_CTRL_SS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SYS_SLEEP_CTRL_SS")
            .field("WAIT", &self.WAIT())
            .field("STOP", &self.STOP())
            .field("SUSPEND", &self.SUSPEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SYS_SLEEP_CTRL_SS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SYS_SLEEP_CTRL_SS {{ WAIT: {=bool:?}, STOP: {=bool:?}, SUSPEND: {=bool:?} }}",
            self.WAIT(),
            self.STOP(),
            self.SUSPEND()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP(pub u32);
impl GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {
    #[inline(always)]
    fn default() -> GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {
        GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP")
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SYS_SLEEP_CTRL_SYS_SLEEP {{ BUSY: {=bool:?} }}",
            self.BUSY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP(pub u32);
impl GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {
    #[must_use]
    #[inline(always)]
    pub const fn BUSY(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_BUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {
    #[inline(always)]
    fn default() -> GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {
        GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP")
            .field("BUSY", &self.BUSY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_SYS_SLEEP_CTRL_SYS_WAKEUP {{ BUSY: {=bool:?} }}",
            self.BUSY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_A55_HDSK_CTRL(pub u32);
impl GPC_CMC_WAKEUP_A55_HDSK_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_WAKEUP_A55_HDSK_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_A55_HDSK_CTRL {
        GPC_CMC_WAKEUP_A55_HDSK_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_A55_HDSK_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_A55_HDSK_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_A55_HDSK_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_A55_HDSK_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT(pub u32);
impl GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {
        GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_A55_HDSK_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP(pub u32);
impl GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {
        GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_A55_HDSK_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP(pub u32);
impl GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {
        GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_A55_HDSK_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_POWER_CTRL(pub u32);
impl GPC_CMC_WAKEUP_POWER_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_WAKEUP_POWER_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_POWER_CTRL {
        GPC_CMC_WAKEUP_POWER_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_POWER_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_POWER_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_POWER_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_POWER_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_POWER_CTRL_CNT(pub u32);
impl GPC_CMC_WAKEUP_POWER_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_WAKEUP_POWER_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_POWER_CTRL_CNT {
        GPC_CMC_WAKEUP_POWER_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_POWER_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_POWER_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_POWER_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_POWER_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_POWER_CTRL_STEP(pub u32);
impl GPC_CMC_WAKEUP_POWER_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_POWER_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_POWER_CTRL_STEP {
        GPC_CMC_WAKEUP_POWER_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_POWER_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_POWER_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_POWER_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_POWER_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_POWER_STAT_RSP(pub u32);
impl GPC_CMC_WAKEUP_POWER_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_POWER_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_POWER_STAT_RSP {
        GPC_CMC_WAKEUP_POWER_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_POWER_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_POWER_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_POWER_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_POWER_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SSAR_CTRL(pub u32);
impl GPC_CMC_WAKEUP_SSAR_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SSAR_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SSAR_CTRL {
        GPC_CMC_WAKEUP_SSAR_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SSAR_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SSAR_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SSAR_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SSAR_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SSAR_CTRL_CNT(pub u32);
impl GPC_CMC_WAKEUP_SSAR_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SSAR_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SSAR_CTRL_CNT {
        GPC_CMC_WAKEUP_SSAR_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SSAR_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SSAR_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SSAR_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SSAR_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SSAR_CTRL_STEP(pub u32);
impl GPC_CMC_WAKEUP_SSAR_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SSAR_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SSAR_CTRL_STEP {
        GPC_CMC_WAKEUP_SSAR_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SSAR_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SSAR_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SSAR_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SSAR_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SSAR_STAT_RSP(pub u32);
impl GPC_CMC_WAKEUP_SSAR_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SSAR_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SSAR_STAT_RSP {
        GPC_CMC_WAKEUP_SSAR_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SSAR_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SSAR_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SSAR_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SSAR_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SYSMAN_CTRL(pub u32);
impl GPC_CMC_WAKEUP_SYSMAN_CTRL {
    #[must_use]
    #[inline(always)]
    pub const fn DISABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DISABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SYSMAN_CTRL {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SYSMAN_CTRL {
        GPC_CMC_WAKEUP_SYSMAN_CTRL(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SYSMAN_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SYSMAN_CTRL")
            .field("DISABLE", &self.DISABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SYSMAN_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SYSMAN_CTRL {{ DISABLE: {=bool:?} }}",
            self.DISABLE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT(pub u32);
impl GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {
        GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT")
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SYSMAN_CTRL_CNT {{ MODE: {=u8:?} }}",
            self.MODE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP(pub u32);
impl GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {
        GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SYSMAN_CTRL_STEP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPC_CMC_WAKEUP_SYSMAN_STAT_RSP(pub u32);
impl GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {
    #[must_use]
    #[inline(always)]
    pub const fn CNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_CNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {
    #[inline(always)]
    fn default() -> GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {
        GPC_CMC_WAKEUP_SYSMAN_STAT_RSP(0)
    }
}
impl core::fmt::Debug for GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPC_CMC_WAKEUP_SYSMAN_STAT_RSP")
            .field("CNT", &self.CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPC_CMC_WAKEUP_SYSMAN_STAT_RSP {{ CNT: {=u32:?} }}",
            self.CNT()
        )
    }
}

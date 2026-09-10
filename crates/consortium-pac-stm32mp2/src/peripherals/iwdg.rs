#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG {
    ptr: *mut u8,
}
unsafe impl Send for IWDG {}
unsafe impl Sync for IWDG {}
impl IWDG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IWDG key register"]
    #[inline(always)]
    pub const fn KR(self) -> crate::common::Reg<IWDG_KR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IWDG prescaler register"]
    #[inline(always)]
    pub const fn PR(self) -> crate::common::Reg<IWDG_PR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "IWDG reload register"]
    #[inline(always)]
    pub const fn RLR(self) -> crate::common::Reg<IWDG_RLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "IWDG status register"]
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<IWDG_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "IWDG window register"]
    #[inline(always)]
    pub const fn WINR(self) -> crate::common::Reg<IWDG_WINR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "IWDG early wakeup interrupt register"]
    #[inline(always)]
    pub const fn EWCR(self) -> crate::common::Reg<IWDG_EWCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "IWDG Interrupt clear register"]
    #[inline(always)]
    pub const fn ICR(self) -> crate::common::Reg<IWDG_ICR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "IWDG hardware configuration register"]
    #[inline(always)]
    pub const fn HWCFGR(self) -> crate::common::Reg<IWDG_HWCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "IWDG version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<IWDG_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "IWDG identification register"]
    #[inline(always)]
    pub const fn IDR(self) -> crate::common::Reg<IWDG_IDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "IWDG size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<IWDG_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_EWCR(pub u32);
impl IWDG_EWCR {
    #[doc = "Watchdog counter window value"]
    #[must_use]
    #[inline(always)]
    pub const fn EWIT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter window value"]
    #[inline(always)]
    pub const fn set_EWIT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Watchdog early interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn EWIE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog early interrupt enable"]
    #[inline(always)]
    pub const fn set_EWIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IWDG_EWCR {
    #[inline(always)]
    fn default() -> IWDG_EWCR {
        IWDG_EWCR(0)
    }
}
impl core::fmt::Debug for IWDG_EWCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_EWCR")
            .field("EWIT", &self.EWIT())
            .field("EWIE", &self.EWIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_EWCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IWDG_EWCR {{ EWIT: {=u16:?}, EWIE: {=bool:?} }}",
            self.EWIT(),
            self.EWIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_HWCFGR(pub u32);
impl IWDG_HWCFGR {
    #[doc = "Support of Window function"]
    #[must_use]
    #[inline(always)]
    pub const fn WINDOW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Support of Window function"]
    #[inline(always)]
    pub const fn set_WINDOW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Prescaler default value"]
    #[must_use]
    #[inline(always)]
    pub const fn PR_DEFAULT(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler default value"]
    #[inline(always)]
    pub const fn set_PR_DEFAULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IWDG_HWCFGR {
    #[inline(always)]
    fn default() -> IWDG_HWCFGR {
        IWDG_HWCFGR(0)
    }
}
impl core::fmt::Debug for IWDG_HWCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_HWCFGR")
            .field("WINDOW", &self.WINDOW())
            .field("PR_DEFAULT", &self.PR_DEFAULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_HWCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IWDG_HWCFGR {{ WINDOW: {=u8:?}, PR_DEFAULT: {=u8:?} }}",
            self.WINDOW(),
            self.PR_DEFAULT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_ICR(pub u32);
impl IWDG_ICR {
    #[doc = "Watchdog early interrupt acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn EWIC(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog early interrupt acknowledge"]
    #[inline(always)]
    pub const fn set_EWIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IWDG_ICR {
    #[inline(always)]
    fn default() -> IWDG_ICR {
        IWDG_ICR(0)
    }
}
impl core::fmt::Debug for IWDG_ICR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_ICR")
            .field("EWIC", &self.EWIC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_ICR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_ICR {{ EWIC: {=bool:?} }}", self.EWIC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_IDR(pub u32);
impl IWDG_IDR {
    #[doc = "IWDG identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IWDG identifier"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IWDG_IDR {
    #[inline(always)]
    fn default() -> IWDG_IDR {
        IWDG_IDR(0)
    }
}
impl core::fmt::Debug for IWDG_IDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_IDR").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_IDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_IDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_KR(pub u32);
impl IWDG_KR {
    #[doc = "Key value (write only, read 0x0000)"]
    #[must_use]
    #[inline(always)]
    pub const fn KEY(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Key value (write only, read 0x0000)"]
    #[inline(always)]
    pub const fn set_KEY(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for IWDG_KR {
    #[inline(always)]
    fn default() -> IWDG_KR {
        IWDG_KR(0)
    }
}
impl core::fmt::Debug for IWDG_KR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_KR").field("KEY", &self.KEY()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_KR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_KR {{ KEY: {=u16:?} }}", self.KEY())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_PR(pub u32);
impl IWDG_PR {
    #[doc = "Prescaler divider"]
    #[must_use]
    #[inline(always)]
    pub const fn PR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler divider"]
    #[inline(always)]
    pub const fn set_PR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for IWDG_PR {
    #[inline(always)]
    fn default() -> IWDG_PR {
        IWDG_PR(0)
    }
}
impl core::fmt::Debug for IWDG_PR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_PR").field("PR", &self.PR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_PR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_PR {{ PR: {=u8:?} }}", self.PR())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_RLR(pub u32);
impl IWDG_RLR {
    #[doc = "Watchdog counter reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn RL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter reload value"]
    #[inline(always)]
    pub const fn set_RL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for IWDG_RLR {
    #[inline(always)]
    fn default() -> IWDG_RLR {
        IWDG_RLR(0)
    }
}
impl core::fmt::Debug for IWDG_RLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_RLR").field("RL", &self.RL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_RLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_RLR {{ RL: {=u16:?} }}", self.RL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_SIDR(pub u32);
impl IWDG_SIDR {
    #[doc = "IWDG size identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IWDG size identifier"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IWDG_SIDR {
    #[inline(always)]
    fn default() -> IWDG_SIDR {
        IWDG_SIDR(0)
    }
}
impl core::fmt::Debug for IWDG_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_SR(pub u32);
impl IWDG_SR {
    #[doc = "Watchdog prescaler value update"]
    #[must_use]
    #[inline(always)]
    pub const fn PVU(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog prescaler value update"]
    #[inline(always)]
    pub const fn set_PVU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog counter reload value update"]
    #[must_use]
    #[inline(always)]
    pub const fn RVU(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog counter reload value update"]
    #[inline(always)]
    pub const fn set_RVU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Watchdog counter window value update"]
    #[must_use]
    #[inline(always)]
    pub const fn WVU(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog counter window value update"]
    #[inline(always)]
    pub const fn set_WVU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Watchdog interrupt comparator value update"]
    #[must_use]
    #[inline(always)]
    pub const fn EWU(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt comparator value update"]
    #[inline(always)]
    pub const fn set_EWU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Enable status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn ONF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Enable status bit"]
    #[inline(always)]
    pub const fn set_ONF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Watchdog Early Interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn EWIF(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Early Interrupt flag"]
    #[inline(always)]
    pub const fn set_EWIF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for IWDG_SR {
    #[inline(always)]
    fn default() -> IWDG_SR {
        IWDG_SR(0)
    }
}
impl core::fmt::Debug for IWDG_SR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_SR")
            .field("PVU", &self.PVU())
            .field("RVU", &self.RVU())
            .field("WVU", &self.WVU())
            .field("EWU", &self.EWU())
            .field("ONF", &self.ONF())
            .field("EWIF", &self.EWIF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_SR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IWDG_SR {{ PVU: {=bool:?}, RVU: {=bool:?}, WVU: {=bool:?}, EWU: {=bool:?}, ONF: {=bool:?}, EWIF: {=bool:?} }}",
            self.PVU(),
            self.RVU(),
            self.WVU(),
            self.EWU(),
            self.ONF(),
            self.EWIF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_VERR(pub u32);
impl IWDG_VERR {
    #[doc = "Minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IWDG_VERR {
    #[inline(always)]
    fn default() -> IWDG_VERR {
        IWDG_VERR(0)
    }
}
impl core::fmt::Debug for IWDG_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IWDG_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IWDG_WINR(pub u32);
impl IWDG_WINR {
    #[doc = "Watchdog counter window value"]
    #[must_use]
    #[inline(always)]
    pub const fn WIN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Watchdog counter window value"]
    #[inline(always)]
    pub const fn set_WIN(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for IWDG_WINR {
    #[inline(always)]
    fn default() -> IWDG_WINR {
        IWDG_WINR(0)
    }
}
impl core::fmt::Debug for IWDG_WINR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_WINR")
            .field("WIN", &self.WIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IWDG_WINR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IWDG_WINR {{ WIN: {=u16:?} }}", self.WIN())
    }
}

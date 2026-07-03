#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API for STM32MP2 general-purpose timers"]

#[doc = "General-purpose timer register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIM_GP {
    ptr: *mut u8,
}
unsafe impl Send for TIM_GP {}
unsafe impl Sync for TIM_GP {}

impl TIM_GP {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }

    #[inline(always)]
    pub const fn CR1(self) -> crate::common::Reg<TIM_CR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x00usize) as _) }
    }

    #[inline(always)]
    pub const fn DIER(self) -> crate::common::Reg<TIM_DIER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }

    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<TIM_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }

    #[inline(always)]
    pub const fn EGR(self) -> crate::common::Reg<TIM_EGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }

    #[inline(always)]
    pub const fn CNT(self) -> crate::common::Reg<TIM_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }

    #[inline(always)]
    pub const fn PSC(self) -> crate::common::Reg<TIM_PSC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }

    #[inline(always)]
    pub const fn ARR(self) -> crate::common::Reg<TIM_ARR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }

    #[inline(always)]
    pub const fn CCR(self, n: usize) -> crate::common::Reg<TIM_CCR, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIM_CR1(pub u32);
impl TIM_CR1 {
    #[must_use]
    #[inline(always)]
    pub const fn CEN(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    #[inline(always)]
    pub const fn set_CEN(&mut self, val: bool) {
        self.0 = (self.0 & !0x01) | val as u32;
    }
}
impl Default for TIM_CR1 {
    fn default() -> Self {
        Self(0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIM_DIER(pub u32);
impl TIM_DIER {
    #[must_use]
    #[inline(always)]
    pub const fn UIE(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    #[inline(always)]
    pub const fn set_UIE(&mut self, val: bool) {
        self.0 = (self.0 & !0x01) | val as u32;
    }

    #[must_use]
    #[inline(always)]
    pub const fn CCIE(&self, n: usize) -> bool {
        assert!(n < 4usize);
        (self.0 & (1 << (n + 1))) != 0
    }

    #[inline(always)]
    pub const fn set_CCIE(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let bit = 1 << (n + 1);
        self.0 = (self.0 & !bit) | ((val as u32) << (n + 1));
    }
}
impl Default for TIM_DIER {
    fn default() -> Self {
        Self(0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIM_SR(pub u32);
impl TIM_SR {
    #[must_use]
    #[inline(always)]
    pub const fn UIF(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    #[inline(always)]
    pub const fn set_UIF(&mut self, val: bool) {
        self.0 = (self.0 & !0x01) | val as u32;
    }

    #[must_use]
    #[inline(always)]
    pub const fn CCIF(&self, n: usize) -> bool {
        assert!(n < 4usize);
        (self.0 & (1 << (n + 1))) != 0
    }

    #[inline(always)]
    pub const fn set_CCIF(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let bit = 1 << (n + 1);
        self.0 = (self.0 & !bit) | ((val as u32) << (n + 1));
    }
}
impl Default for TIM_SR {
    fn default() -> Self {
        Self(0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIM_EGR(pub u32);
impl TIM_EGR {
    #[inline(always)]
    pub const fn set_UG(&mut self, val: bool) {
        self.0 = (self.0 & !0x01) | val as u32;
    }
}
impl Default for TIM_EGR {
    fn default() -> Self {
        Self(0)
    }
}

macro_rules! value_reg {
    ($name:ident, $field:ident, $setter:ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $name(pub u32);
        impl $name {
            #[must_use]
            #[inline(always)]
            pub const fn $field(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub const fn $setter(&mut self, val: u32) {
                self.0 = val;
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }
    };
}

value_reg!(TIM_CNT, CNT, set_CNT);
value_reg!(TIM_PSC, PSC, set_PSC);
value_reg!(TIM_ARR, ARR, set_ARR);
value_reg!(TIM_CCR, CCR, set_CCR);

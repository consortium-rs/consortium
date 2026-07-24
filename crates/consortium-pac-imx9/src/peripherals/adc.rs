#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC {
    ptr: *mut u8,
}
unsafe impl Send for ADC {}
unsafe impl Sync for ADC {}
impl ADC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCR(self) -> crate::common::Reg<ADC_MCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MSR(self) -> crate::common::Reg<ADC_MSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<ADC_ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn CEOCFR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn CEOCFR1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn IMR(self) -> crate::common::Reg<ADC_IMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn CIMR0(self) -> crate::common::Reg<ADC_CIMR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn CIMR1(self) -> crate::common::Reg<ADC_CIMR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn WTISR(self) -> crate::common::Reg<ADC_WTISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn WTIMR(self) -> crate::common::Reg<ADC_WTIMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn DMAE(self) -> crate::common::Reg<ADC_DMAE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn DMAR0(self) -> crate::common::Reg<ADC_DMAR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn DMAR1(self) -> crate::common::Reg<ADC_DMAR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR0(self) -> crate::common::Reg<ADC_THRHLR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR1(self) -> crate::common::Reg<ADC_THRHLR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR2(self) -> crate::common::Reg<ADC_THRHLR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR3(self) -> crate::common::Reg<ADC_THRHLR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn PSCR(self) -> crate::common::Reg<ADC_PSCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn PSR0(self) -> crate::common::Reg<ADC_PSR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn PSR1(self) -> crate::common::Reg<ADC_PSR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn CTR0(self) -> crate::common::Reg<ADC_CTR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn CTR1(self) -> crate::common::Reg<ADC_CTR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn NCMR0(self) -> crate::common::Reg<ADC_NCMR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn NCMR1(self) -> crate::common::Reg<ADC_NCMR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn JCMR0(self) -> crate::common::Reg<ADC_JCMR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn JCMR1(self) -> crate::common::Reg<ADC_JCMR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn USROFSGN(self) -> crate::common::Reg<ADC_USROFSGN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn PDEDR(self) -> crate::common::Reg<ADC_PDEDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn PCDR(self, n: usize) -> crate::common::Reg<ADC_PCDR, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn ICDR(self, n: usize) -> crate::common::Reg<ADC_ICDR, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[inline(always)]
    pub const fn THRHLR4(self) -> crate::common::Reg<ADC_THRHLR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR5(self) -> crate::common::Reg<ADC_THRHLR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR6(self) -> crate::common::Reg<ADC_THRHLR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[inline(always)]
    pub const fn THRHLR7(self) -> crate::common::Reg<ADC_THRHLR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[inline(always)]
    pub const fn CWSELR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[inline(always)]
    pub const fn CWSELR4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[inline(always)]
    pub const fn CWENR0(self) -> crate::common::Reg<ADC_CWENR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[inline(always)]
    pub const fn CWENR1(self) -> crate::common::Reg<ADC_CWENR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[inline(always)]
    pub const fn AWORR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[inline(always)]
    pub const fn AWORR1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[inline(always)]
    pub const fn STCR1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[inline(always)]
    pub const fn STCR2(self) -> crate::common::Reg<ADC_STCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[inline(always)]
    pub const fn STCR3(self) -> crate::common::Reg<ADC_STCR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[inline(always)]
    pub const fn STBRR(self) -> crate::common::Reg<ADC_STBRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[inline(always)]
    pub const fn STSR1(self) -> crate::common::Reg<ADC_STSR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[inline(always)]
    pub const fn STSR2(self) -> crate::common::Reg<ADC_STSR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[inline(always)]
    pub const fn STSR3(self) -> crate::common::Reg<ADC_STSR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[inline(always)]
    pub const fn STSR4(self) -> crate::common::Reg<ADC_STSR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[inline(always)]
    pub const fn STDR1(self) -> crate::common::Reg<ADC_STDR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[inline(always)]
    pub const fn STDR2(self) -> crate::common::Reg<ADC_STDR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW0R(self) -> crate::common::Reg<ADC_STAW0R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW1AR(self) -> crate::common::Reg<ADC_STAW1AR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW1BR(self) -> crate::common::Reg<ADC_STAW1BR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW2R(self) -> crate::common::Reg<ADC_STAW2R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[inline(always)]
    pub const fn STAW3R(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW4R(self) -> crate::common::Reg<ADC_STAW4R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[inline(always)]
    pub const fn STAW5R(self) -> crate::common::Reg<ADC_STAW5R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[inline(always)]
    pub const fn CALSTAT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[inline(always)]
    pub const fn CALCFG0(self) -> crate::common::Reg<ADC_CALCFG0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWORR0_AWOR(pub u32);
impl ADC_AWORR0_AWOR {
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_AWORR0_AWOR {
    #[inline(always)]
    fn default() -> ADC_AWORR0_AWOR {
        ADC_AWORR0_AWOR(0)
    }
}
impl core::fmt::Debug for ADC_AWORR0_AWOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWORR0_AWOR")
            .field("CH0", &self.CH0())
            .field("CH1", &self.CH1())
            .field("CH2", &self.CH2())
            .field("CH3", &self.CH3())
            .field("CH4", &self.CH4())
            .field("CH5", &self.CH5())
            .field("CH6", &self.CH6())
            .field("CH7", &self.CH7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWORR0_AWOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_AWORR0_AWOR {{ CH0: {=bool:?}, CH1: {=bool:?}, CH2: {=bool:?}, CH3: {=bool:?}, CH4: {=bool:?}, CH5: {=bool:?}, CH6: {=bool:?}, CH7: {=bool:?} }}",
            self.CH0(),
            self.CH1(),
            self.CH2(),
            self.CH3(),
            self.CH4(),
            self.CH5(),
            self.CH6(),
            self.CH7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_AWORR1_AWOR(pub u32);
impl ADC_AWORR1_AWOR {
    #[must_use]
    #[inline(always)]
    pub const fn CH32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_AWORR1_AWOR {
    #[inline(always)]
    fn default() -> ADC_AWORR1_AWOR {
        ADC_AWORR1_AWOR(0)
    }
}
impl core::fmt::Debug for ADC_AWORR1_AWOR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_AWORR1_AWOR")
            .field("CH32", &self.CH32())
            .field("CH33", &self.CH33())
            .field("CH34", &self.CH34())
            .field("CH35", &self.CH35())
            .field("CH36", &self.CH36())
            .field("CH37", &self.CH37())
            .field("CH38", &self.CH38())
            .field("CH39", &self.CH39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_AWORR1_AWOR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_AWORR1_AWOR {{ CH32: {=bool:?}, CH33: {=bool:?}, CH34: {=bool:?}, CH35: {=bool:?}, CH36: {=bool:?}, CH37: {=bool:?}, CH38: {=bool:?}, CH39: {=bool:?} }}",
            self.CH32(),
            self.CH33(),
            self.CH34(),
            self.CH35(),
            self.CH36(),
            self.CH37(),
            self.CH38(),
            self.CH39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CALCFG0(pub u32);
impl ADC_CALCFG0 {
    #[must_use]
    #[inline(always)]
    pub const fn LDFAIL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_LDFAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn BNDFLCNT(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_BNDFLCNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for ADC_CALCFG0 {
    #[inline(always)]
    fn default() -> ADC_CALCFG0 {
        ADC_CALCFG0(0)
    }
}
impl core::fmt::Debug for ADC_CALCFG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALCFG0")
            .field("LDFAIL", &self.LDFAIL())
            .field("BNDFLCNT", &self.BNDFLCNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CALCFG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CALCFG0 {{ LDFAIL: {=bool:?}, BNDFLCNT: {=u8:?} }}",
            self.LDFAIL(),
            self.BNDFLCNT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CALSTAT_TEST(pub u32);
impl ADC_CALSTAT_TEST {
    #[doc = "TEST_RESULT"]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TEST_RESULT"]
    #[inline(always)]
    pub const fn set_RESULT(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ADC_CALSTAT_TEST {
    #[inline(always)]
    fn default() -> ADC_CALSTAT_TEST {
        ADC_CALSTAT_TEST(0)
    }
}
impl core::fmt::Debug for ADC_CALSTAT_TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CALSTAT_TEST")
            .field("RESULT", &self.RESULT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CALSTAT_TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_CALSTAT_TEST {{ RESULT: {=u16:?} }}", self.RESULT())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CEOCFR0_EOC(pub u32);
impl ADC_CEOCFR0_EOC {
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CEOCFR0_EOC {
    #[inline(always)]
    fn default() -> ADC_CEOCFR0_EOC {
        ADC_CEOCFR0_EOC(0)
    }
}
impl core::fmt::Debug for ADC_CEOCFR0_EOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CEOCFR0_EOC")
            .field("CH0", &self.CH0())
            .field("CH1", &self.CH1())
            .field("CH2", &self.CH2())
            .field("CH3", &self.CH3())
            .field("CH4", &self.CH4())
            .field("CH5", &self.CH5())
            .field("CH6", &self.CH6())
            .field("CH7", &self.CH7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CEOCFR0_EOC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CEOCFR0_EOC {{ CH0: {=bool:?}, CH1: {=bool:?}, CH2: {=bool:?}, CH3: {=bool:?}, CH4: {=bool:?}, CH5: {=bool:?}, CH6: {=bool:?}, CH7: {=bool:?} }}",
            self.CH0(),
            self.CH1(),
            self.CH2(),
            self.CH3(),
            self.CH4(),
            self.CH5(),
            self.CH6(),
            self.CH7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CEOCFR1_EOC(pub u32);
impl ADC_CEOCFR1_EOC {
    #[must_use]
    #[inline(always)]
    pub const fn CH32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CEOCFR1_EOC {
    #[inline(always)]
    fn default() -> ADC_CEOCFR1_EOC {
        ADC_CEOCFR1_EOC(0)
    }
}
impl core::fmt::Debug for ADC_CEOCFR1_EOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CEOCFR1_EOC")
            .field("CH32", &self.CH32())
            .field("CH33", &self.CH33())
            .field("CH34", &self.CH34())
            .field("CH35", &self.CH35())
            .field("CH36", &self.CH36())
            .field("CH37", &self.CH37())
            .field("CH38", &self.CH38())
            .field("CH39", &self.CH39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CEOCFR1_EOC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CEOCFR1_EOC {{ CH32: {=bool:?}, CH33: {=bool:?}, CH34: {=bool:?}, CH35: {=bool:?}, CH36: {=bool:?}, CH37: {=bool:?}, CH38: {=bool:?}, CH39: {=bool:?} }}",
            self.CH32(),
            self.CH33(),
            self.CH34(),
            self.CH35(),
            self.CH36(),
            self.CH37(),
            self.CH38(),
            self.CH39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CIMR0(pub u32);
impl ADC_CIMR0 {
    #[must_use]
    #[inline(always)]
    pub const fn CIM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CIMR0 {
    #[inline(always)]
    fn default() -> ADC_CIMR0 {
        ADC_CIMR0(0)
    }
}
impl core::fmt::Debug for ADC_CIMR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CIMR0")
            .field("CIM0", &self.CIM0())
            .field("CIM1", &self.CIM1())
            .field("CIM2", &self.CIM2())
            .field("CIM3", &self.CIM3())
            .field("CIM4", &self.CIM4())
            .field("CIM5", &self.CIM5())
            .field("CIM6", &self.CIM6())
            .field("CIM7", &self.CIM7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CIMR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CIMR0 {{ CIM0: {=bool:?}, CIM1: {=bool:?}, CIM2: {=bool:?}, CIM3: {=bool:?}, CIM4: {=bool:?}, CIM5: {=bool:?}, CIM6: {=bool:?}, CIM7: {=bool:?} }}",
            self.CIM0(),
            self.CIM1(),
            self.CIM2(),
            self.CIM3(),
            self.CIM4(),
            self.CIM5(),
            self.CIM6(),
            self.CIM7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CIMR1(pub u32);
impl ADC_CIMR1 {
    #[must_use]
    #[inline(always)]
    pub const fn CIM32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CIM39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CIM39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CIMR1 {
    #[inline(always)]
    fn default() -> ADC_CIMR1 {
        ADC_CIMR1(0)
    }
}
impl core::fmt::Debug for ADC_CIMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CIMR1")
            .field("CIM32", &self.CIM32())
            .field("CIM33", &self.CIM33())
            .field("CIM34", &self.CIM34())
            .field("CIM35", &self.CIM35())
            .field("CIM36", &self.CIM36())
            .field("CIM37", &self.CIM37())
            .field("CIM38", &self.CIM38())
            .field("CIM39", &self.CIM39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CIMR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CIMR1 {{ CIM32: {=bool:?}, CIM33: {=bool:?}, CIM34: {=bool:?}, CIM35: {=bool:?}, CIM36: {=bool:?}, CIM37: {=bool:?}, CIM38: {=bool:?}, CIM39: {=bool:?} }}",
            self.CIM32(),
            self.CIM33(),
            self.CIM34(),
            self.CIM35(),
            self.CIM36(),
            self.CIM37(),
            self.CIM38(),
            self.CIM39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CTR0(pub u32);
impl ADC_CTR0 {
    #[doc = "Sampling Phase Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn INPSAMP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sampling Phase Duration"]
    #[inline(always)]
    pub const fn set_INPSAMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ADC_CTR0 {
    #[inline(always)]
    fn default() -> ADC_CTR0 {
        ADC_CTR0(0)
    }
}
impl core::fmt::Debug for ADC_CTR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTR0")
            .field("INPSAMP", &self.INPSAMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CTR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_CTR0 {{ INPSAMP: {=u8:?} }}", self.INPSAMP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CTR1(pub u32);
impl ADC_CTR1 {
    #[doc = "Sampling Phase Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn INPSAMP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Sampling Phase Duration"]
    #[inline(always)]
    pub const fn set_INPSAMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ADC_CTR1 {
    #[inline(always)]
    fn default() -> ADC_CTR1 {
        ADC_CTR1(0)
    }
}
impl core::fmt::Debug for ADC_CTR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTR1")
            .field("INPSAMP", &self.INPSAMP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CTR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_CTR1 {{ INPSAMP: {=u8:?} }}", self.INPSAMP())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CWENR0(pub u32);
impl ADC_CWENR0 {
    #[must_use]
    #[inline(always)]
    pub const fn CWEN0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CWENR0 {
    #[inline(always)]
    fn default() -> ADC_CWENR0 {
        ADC_CWENR0(0)
    }
}
impl core::fmt::Debug for ADC_CWENR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CWENR0")
            .field("CWEN0", &self.CWEN0())
            .field("CWEN1", &self.CWEN1())
            .field("CWEN2", &self.CWEN2())
            .field("CWEN3", &self.CWEN3())
            .field("CWEN4", &self.CWEN4())
            .field("CWEN5", &self.CWEN5())
            .field("CWEN6", &self.CWEN6())
            .field("CWEN7", &self.CWEN7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CWENR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CWENR0 {{ CWEN0: {=bool:?}, CWEN1: {=bool:?}, CWEN2: {=bool:?}, CWEN3: {=bool:?}, CWEN4: {=bool:?}, CWEN5: {=bool:?}, CWEN6: {=bool:?}, CWEN7: {=bool:?} }}",
            self.CWEN0(),
            self.CWEN1(),
            self.CWEN2(),
            self.CWEN3(),
            self.CWEN4(),
            self.CWEN5(),
            self.CWEN6(),
            self.CWEN7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CWENR1(pub u32);
impl ADC_CWENR1 {
    #[must_use]
    #[inline(always)]
    pub const fn CWEN32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CWEN39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CWEN39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_CWENR1 {
    #[inline(always)]
    fn default() -> ADC_CWENR1 {
        ADC_CWENR1(0)
    }
}
impl core::fmt::Debug for ADC_CWENR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CWENR1")
            .field("CWEN32", &self.CWEN32())
            .field("CWEN33", &self.CWEN33())
            .field("CWEN34", &self.CWEN34())
            .field("CWEN35", &self.CWEN35())
            .field("CWEN36", &self.CWEN36())
            .field("CWEN37", &self.CWEN37())
            .field("CWEN38", &self.CWEN38())
            .field("CWEN39", &self.CWEN39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CWENR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CWENR1 {{ CWEN32: {=bool:?}, CWEN33: {=bool:?}, CWEN34: {=bool:?}, CWEN35: {=bool:?}, CWEN36: {=bool:?}, CWEN37: {=bool:?}, CWEN38: {=bool:?}, CWEN39: {=bool:?} }}",
            self.CWEN32(),
            self.CWEN33(),
            self.CWEN34(),
            self.CWEN35(),
            self.CWEN36(),
            self.CWEN37(),
            self.CWEN38(),
            self.CWEN39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CWSELR0_WSEL(pub u32);
impl ADC_CWSELR0_WSEL {
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Channel Watchdog Select for Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 0"]
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Channel Watchdog Select for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 1"]
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Channel Watchdog Select for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn CH3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 2"]
    #[inline(always)]
    pub const fn set_CH3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Channel Watchdog Select for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn CH4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 3"]
    #[inline(always)]
    pub const fn set_CH4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Channel Watchdog Select for Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn CH5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 4"]
    #[inline(always)]
    pub const fn set_CH5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Channel Watchdog Select for Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn CH6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 5"]
    #[inline(always)]
    pub const fn set_CH6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Channel Watchdog Select for Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn CH7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 6"]
    #[inline(always)]
    pub const fn set_CH7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for ADC_CWSELR0_WSEL {
    #[inline(always)]
    fn default() -> ADC_CWSELR0_WSEL {
        ADC_CWSELR0_WSEL(0)
    }
}
impl core::fmt::Debug for ADC_CWSELR0_WSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CWSELR0_WSEL")
            .field("CH0", &self.CH0())
            .field("CH1", &self.CH1())
            .field("CH2", &self.CH2())
            .field("CH3", &self.CH3())
            .field("CH4", &self.CH4())
            .field("CH5", &self.CH5())
            .field("CH6", &self.CH6())
            .field("CH7", &self.CH7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CWSELR0_WSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CWSELR0_WSEL {{ CH0: {=u8:?}, CH1: {=u8:?}, CH2: {=u8:?}, CH3: {=u8:?}, CH4: {=u8:?}, CH5: {=u8:?}, CH6: {=u8:?}, CH7: {=u8:?} }}",
            self.CH0(),
            self.CH1(),
            self.CH2(),
            self.CH3(),
            self.CH4(),
            self.CH5(),
            self.CH6(),
            self.CH7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CWSELR4_WSEL(pub u32);
impl ADC_CWSELR4_WSEL {
    #[must_use]
    #[inline(always)]
    pub const fn CH32(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CH32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Channel Watchdog Select for Channel 32"]
    #[must_use]
    #[inline(always)]
    pub const fn CH33(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 32"]
    #[inline(always)]
    pub const fn set_CH33(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Channel Watchdog Select for Channel 33"]
    #[must_use]
    #[inline(always)]
    pub const fn CH34(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 33"]
    #[inline(always)]
    pub const fn set_CH34(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Channel Watchdog Select for Channel 34"]
    #[must_use]
    #[inline(always)]
    pub const fn CH35(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 34"]
    #[inline(always)]
    pub const fn set_CH35(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Channel Watchdog Select for Channel 35"]
    #[must_use]
    #[inline(always)]
    pub const fn CH36(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 35"]
    #[inline(always)]
    pub const fn set_CH36(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Channel Watchdog Select for Channel 36"]
    #[must_use]
    #[inline(always)]
    pub const fn CH37(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 36"]
    #[inline(always)]
    pub const fn set_CH37(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Channel Watchdog Select for Channel 37"]
    #[must_use]
    #[inline(always)]
    pub const fn CH38(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 37"]
    #[inline(always)]
    pub const fn set_CH38(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Channel Watchdog Select for Channel 38"]
    #[must_use]
    #[inline(always)]
    pub const fn CH39(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Channel Watchdog Select for Channel 38"]
    #[inline(always)]
    pub const fn set_CH39(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for ADC_CWSELR4_WSEL {
    #[inline(always)]
    fn default() -> ADC_CWSELR4_WSEL {
        ADC_CWSELR4_WSEL(0)
    }
}
impl core::fmt::Debug for ADC_CWSELR4_WSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CWSELR4_WSEL")
            .field("CH32", &self.CH32())
            .field("CH33", &self.CH33())
            .field("CH34", &self.CH34())
            .field("CH35", &self.CH35())
            .field("CH36", &self.CH36())
            .field("CH37", &self.CH37())
            .field("CH38", &self.CH38())
            .field("CH39", &self.CH39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CWSELR4_WSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CWSELR4_WSEL {{ CH32: {=u8:?}, CH33: {=u8:?}, CH34: {=u8:?}, CH35: {=u8:?}, CH36: {=u8:?}, CH37: {=u8:?}, CH38: {=u8:?}, CH39: {=u8:?} }}",
            self.CH32(),
            self.CH33(),
            self.CH34(),
            self.CH35(),
            self.CH36(),
            self.CH37(),
            self.CH38(),
            self.CH39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_DMAE(pub u32);
impl ADC_DMAE {
    #[must_use]
    #[inline(always)]
    pub const fn DMAEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DCLR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for ADC_DMAE {
    #[inline(always)]
    fn default() -> ADC_DMAE {
        ADC_DMAE(0)
    }
}
impl core::fmt::Debug for ADC_DMAE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DMAE")
            .field("DMAEN", &self.DMAEN())
            .field("DCLR", &self.DCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_DMAE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_DMAE {{ DMAEN: {=bool:?}, DCLR: {=bool:?} }}",
            self.DMAEN(),
            self.DCLR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_DMAR0(pub u32);
impl ADC_DMAR0 {
    #[must_use]
    #[inline(always)]
    pub const fn DMA0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_DMAR0 {
    #[inline(always)]
    fn default() -> ADC_DMAR0 {
        ADC_DMAR0(0)
    }
}
impl core::fmt::Debug for ADC_DMAR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DMAR0")
            .field("DMA0", &self.DMA0())
            .field("DMA1", &self.DMA1())
            .field("DMA2", &self.DMA2())
            .field("DMA3", &self.DMA3())
            .field("DMA4", &self.DMA4())
            .field("DMA5", &self.DMA5())
            .field("DMA6", &self.DMA6())
            .field("DMA7", &self.DMA7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_DMAR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_DMAR0 {{ DMA0: {=bool:?}, DMA1: {=bool:?}, DMA2: {=bool:?}, DMA3: {=bool:?}, DMA4: {=bool:?}, DMA5: {=bool:?}, DMA6: {=bool:?}, DMA7: {=bool:?} }}",
            self.DMA0(),
            self.DMA1(),
            self.DMA2(),
            self.DMA3(),
            self.DMA4(),
            self.DMA5(),
            self.DMA6(),
            self.DMA7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_DMAR1(pub u32);
impl ADC_DMAR1 {
    #[must_use]
    #[inline(always)]
    pub const fn DMA32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn DMA39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_DMA39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_DMAR1 {
    #[inline(always)]
    fn default() -> ADC_DMAR1 {
        ADC_DMAR1(0)
    }
}
impl core::fmt::Debug for ADC_DMAR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DMAR1")
            .field("DMA32", &self.DMA32())
            .field("DMA33", &self.DMA33())
            .field("DMA34", &self.DMA34())
            .field("DMA35", &self.DMA35())
            .field("DMA36", &self.DMA36())
            .field("DMA37", &self.DMA37())
            .field("DMA38", &self.DMA38())
            .field("DMA39", &self.DMA39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_DMAR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_DMAR1 {{ DMA32: {=bool:?}, DMA33: {=bool:?}, DMA34: {=bool:?}, DMA35: {=bool:?}, DMA36: {=bool:?}, DMA37: {=bool:?}, DMA38: {=bool:?}, DMA39: {=bool:?} }}",
            self.DMA32(),
            self.DMA33(),
            self.DMA34(),
            self.DMA35(),
            self.DMA36(),
            self.DMA37(),
            self.DMA38(),
            self.DMA39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_ICDR(pub u32);
impl ADC_ICDR {
    #[doc = "Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn CDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel Converted Data"]
    #[inline(always)]
    pub const fn set_CDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Channel Converted Data"]
    #[inline(always)]
    pub const fn set_RESULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OVERW(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OVERW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for ADC_ICDR {
    #[inline(always)]
    fn default() -> ADC_ICDR {
        ADC_ICDR(0)
    }
}
impl core::fmt::Debug for ADC_ICDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ICDR")
            .field("CDATA", &self.CDATA())
            .field("RESULT", &self.RESULT())
            .field("OVERW", &self.OVERW())
            .field("VALID", &self.VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_ICDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_ICDR {{ CDATA: {=u16:?}, RESULT: {=u8:?}, OVERW: {=bool:?}, VALID: {=bool:?} }}",
            self.CDATA(),
            self.RESULT(),
            self.OVERW(),
            self.VALID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_IMR(pub u32);
impl ADC_IMR {
    #[must_use]
    #[inline(always)]
    pub const fn MSKECH(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKECH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKEOC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKEOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKJECH(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKJECH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKJEOC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKJEOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ADC_IMR {
    #[inline(always)]
    fn default() -> ADC_IMR {
        ADC_IMR(0)
    }
}
impl core::fmt::Debug for ADC_IMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_IMR")
            .field("MSKECH", &self.MSKECH())
            .field("MSKEOC", &self.MSKEOC())
            .field("MSKJECH", &self.MSKJECH())
            .field("MSKJEOC", &self.MSKJEOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_IMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_IMR {{ MSKECH: {=bool:?}, MSKEOC: {=bool:?}, MSKJECH: {=bool:?}, MSKJEOC: {=bool:?} }}",
            self.MSKECH(),
            self.MSKEOC(),
            self.MSKJECH(),
            self.MSKJEOC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_ISR(pub u32);
impl ADC_ISR {
    #[must_use]
    #[inline(always)]
    pub const fn ECH(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ECH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn EOC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn JECH(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_JECH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn JEOC(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_JEOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ADC_ISR {
    #[inline(always)]
    fn default() -> ADC_ISR {
        ADC_ISR(0)
    }
}
impl core::fmt::Debug for ADC_ISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_ISR")
            .field("ECH", &self.ECH())
            .field("EOC", &self.EOC())
            .field("JECH", &self.JECH())
            .field("JEOC", &self.JEOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_ISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_ISR {{ ECH: {=bool:?}, EOC: {=bool:?}, JECH: {=bool:?}, JEOC: {=bool:?} }}",
            self.ECH(),
            self.EOC(),
            self.JECH(),
            self.JEOC()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JCMR0(pub u32);
impl ADC_JCMR0 {
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_JCMR0 {
    #[inline(always)]
    fn default() -> ADC_JCMR0 {
        ADC_JCMR0(0)
    }
}
impl core::fmt::Debug for ADC_JCMR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JCMR0")
            .field("CH0", &self.CH0())
            .field("CH1", &self.CH1())
            .field("CH2", &self.CH2())
            .field("CH3", &self.CH3())
            .field("CH4", &self.CH4())
            .field("CH5", &self.CH5())
            .field("CH6", &self.CH6())
            .field("CH7", &self.CH7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JCMR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_JCMR0 {{ CH0: {=bool:?}, CH1: {=bool:?}, CH2: {=bool:?}, CH3: {=bool:?}, CH4: {=bool:?}, CH5: {=bool:?}, CH6: {=bool:?}, CH7: {=bool:?} }}",
            self.CH0(),
            self.CH1(),
            self.CH2(),
            self.CH3(),
            self.CH4(),
            self.CH5(),
            self.CH6(),
            self.CH7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_JCMR1(pub u32);
impl ADC_JCMR1 {
    #[must_use]
    #[inline(always)]
    pub const fn CH32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_JCMR1 {
    #[inline(always)]
    fn default() -> ADC_JCMR1 {
        ADC_JCMR1(0)
    }
}
impl core::fmt::Debug for ADC_JCMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_JCMR1")
            .field("CH32", &self.CH32())
            .field("CH33", &self.CH33())
            .field("CH34", &self.CH34())
            .field("CH35", &self.CH35())
            .field("CH36", &self.CH36())
            .field("CH37", &self.CH37())
            .field("CH38", &self.CH38())
            .field("CH39", &self.CH39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_JCMR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_JCMR1 {{ CH32: {=bool:?}, CH33: {=bool:?}, CH34: {=bool:?}, CH35: {=bool:?}, CH36: {=bool:?}, CH37: {=bool:?}, CH38: {=bool:?}, CH39: {=bool:?} }}",
            self.CH32(),
            self.CH33(),
            self.CH34(),
            self.CH35(),
            self.CH36(),
            self.CH37(),
            self.CH38(),
            self.CH39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_MCR(pub u32);
impl ADC_MCR {
    #[must_use]
    #[inline(always)]
    pub const fn PWDN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PWDN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACKO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACKO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ABORT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ABORT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ABORTCHAIN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ABORTCHAIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ADCLKSE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ADCLKSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TSAMP(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_TSAMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NRSMPL(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_NRSMPL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AVGEN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AVGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CALSTART(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CALSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn STCL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_STCL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Start Injection Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn JSTART(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start Injection Conversion"]
    #[inline(always)]
    pub const fn set_JSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Start Injection Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn JEDGE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Start Injection Conversion"]
    #[inline(always)]
    pub const fn set_JEDGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn JTRGEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_JTRGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Normal Conversion Start"]
    #[must_use]
    #[inline(always)]
    pub const fn NSTART(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Conversion Start"]
    #[inline(always)]
    pub const fn set_NSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Normal Conversion Start"]
    #[must_use]
    #[inline(always)]
    pub const fn EDGE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Conversion Start"]
    #[inline(always)]
    pub const fn set_EDGE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn TRGEN(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_TRGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WLSIDE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WLSIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OWREN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OWREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_MCR {
    #[inline(always)]
    fn default() -> ADC_MCR {
        ADC_MCR(0)
    }
}
impl core::fmt::Debug for ADC_MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_MCR")
            .field("PWDN", &self.PWDN())
            .field("ACKO", &self.ACKO())
            .field("ABORT", &self.ABORT())
            .field("ABORTCHAIN", &self.ABORTCHAIN())
            .field("ADCLKSE", &self.ADCLKSE())
            .field("TSAMP", &self.TSAMP())
            .field("NRSMPL", &self.NRSMPL())
            .field("AVGEN", &self.AVGEN())
            .field("CALSTART", &self.CALSTART())
            .field("STCL", &self.STCL())
            .field("JSTART", &self.JSTART())
            .field("JEDGE", &self.JEDGE())
            .field("JTRGEN", &self.JTRGEN())
            .field("NSTART", &self.NSTART())
            .field("EDGE", &self.EDGE())
            .field("TRGEN", &self.TRGEN())
            .field("MODE", &self.MODE())
            .field("WLSIDE", &self.WLSIDE())
            .field("OWREN", &self.OWREN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_MCR {{ PWDN: {=bool:?}, ACKO: {=bool:?}, ABORT: {=bool:?}, ABORTCHAIN: {=bool:?}, ADCLKSE: {=bool:?}, TSAMP: {=u8:?}, NRSMPL: {=u8:?}, AVGEN: {=bool:?}, CALSTART: {=bool:?}, STCL: {=bool:?}, JSTART: {=bool:?}, JEDGE: {=bool:?}, JTRGEN: {=bool:?}, NSTART: {=bool:?}, EDGE: {=bool:?}, TRGEN: {=bool:?}, MODE: {=bool:?}, WLSIDE: {=bool:?}, OWREN: {=bool:?} }}",
            self.PWDN(),
            self.ACKO(),
            self.ABORT(),
            self.ABORTCHAIN(),
            self.ADCLKSE(),
            self.TSAMP(),
            self.NRSMPL(),
            self.AVGEN(),
            self.CALSTART(),
            self.STCL(),
            self.JSTART(),
            self.JEDGE(),
            self.JTRGEN(),
            self.NSTART(),
            self.EDGE(),
            self.TRGEN(),
            self.MODE(),
            self.WLSIDE(),
            self.OWREN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_MSR(pub u32);
impl ADC_MSR {
    #[must_use]
    #[inline(always)]
    pub const fn ADCSTATUS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ADCSTATUS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ACKO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ACKO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CHADDR(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_CHADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn JSTART(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_JSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn JABORT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_JABORT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn NSTART(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_NSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CALBUSY(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CALBUSY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CALFAIL(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CALFAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CALIBRTD(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CALIBRTD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_MSR {
    #[inline(always)]
    fn default() -> ADC_MSR {
        ADC_MSR(0)
    }
}
impl core::fmt::Debug for ADC_MSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_MSR")
            .field("ADCSTATUS", &self.ADCSTATUS())
            .field("ACKO", &self.ACKO())
            .field("CHADDR", &self.CHADDR())
            .field("JSTART", &self.JSTART())
            .field("JABORT", &self.JABORT())
            .field("NSTART", &self.NSTART())
            .field("CALBUSY", &self.CALBUSY())
            .field("CALFAIL", &self.CALFAIL())
            .field("CALIBRTD", &self.CALIBRTD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_MSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_MSR {{ ADCSTATUS: {=u8:?}, ACKO: {=bool:?}, CHADDR: {=u8:?}, JSTART: {=bool:?}, JABORT: {=bool:?}, NSTART: {=bool:?}, CALBUSY: {=bool:?}, CALFAIL: {=bool:?}, CALIBRTD: {=bool:?} }}",
            self.ADCSTATUS(),
            self.ACKO(),
            self.CHADDR(),
            self.JSTART(),
            self.JABORT(),
            self.NSTART(),
            self.CALBUSY(),
            self.CALFAIL(),
            self.CALIBRTD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_MSR_SELF_TEST(pub u32);
impl ADC_MSR_SELF_TEST {
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for ADC_MSR_SELF_TEST {
    #[inline(always)]
    fn default() -> ADC_MSR_SELF_TEST {
        ADC_MSR_SELF_TEST(0)
    }
}
impl core::fmt::Debug for ADC_MSR_SELF_TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_MSR_SELF_TEST")
            .field("S", &self.S())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_MSR_SELF_TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_MSR_SELF_TEST {{ S: {=bool:?} }}", self.S())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_NCMR0(pub u32);
impl ADC_NCMR0 {
    #[must_use]
    #[inline(always)]
    pub const fn CH0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_NCMR0 {
    #[inline(always)]
    fn default() -> ADC_NCMR0 {
        ADC_NCMR0(0)
    }
}
impl core::fmt::Debug for ADC_NCMR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_NCMR0")
            .field("CH0", &self.CH0())
            .field("CH1", &self.CH1())
            .field("CH2", &self.CH2())
            .field("CH3", &self.CH3())
            .field("CH4", &self.CH4())
            .field("CH5", &self.CH5())
            .field("CH6", &self.CH6())
            .field("CH7", &self.CH7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_NCMR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_NCMR0 {{ CH0: {=bool:?}, CH1: {=bool:?}, CH2: {=bool:?}, CH3: {=bool:?}, CH4: {=bool:?}, CH5: {=bool:?}, CH6: {=bool:?}, CH7: {=bool:?} }}",
            self.CH0(),
            self.CH1(),
            self.CH2(),
            self.CH3(),
            self.CH4(),
            self.CH5(),
            self.CH6(),
            self.CH7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_NCMR1(pub u32);
impl ADC_NCMR1 {
    #[must_use]
    #[inline(always)]
    pub const fn CH32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn CH39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_CH39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_NCMR1 {
    #[inline(always)]
    fn default() -> ADC_NCMR1 {
        ADC_NCMR1(0)
    }
}
impl core::fmt::Debug for ADC_NCMR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_NCMR1")
            .field("CH32", &self.CH32())
            .field("CH33", &self.CH33())
            .field("CH34", &self.CH34())
            .field("CH35", &self.CH35())
            .field("CH36", &self.CH36())
            .field("CH37", &self.CH37())
            .field("CH38", &self.CH38())
            .field("CH39", &self.CH39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_NCMR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_NCMR1 {{ CH32: {=bool:?}, CH33: {=bool:?}, CH34: {=bool:?}, CH35: {=bool:?}, CH36: {=bool:?}, CH37: {=bool:?}, CH38: {=bool:?}, CH39: {=bool:?} }}",
            self.CH32(),
            self.CH33(),
            self.CH34(),
            self.CH35(),
            self.CH36(),
            self.CH37(),
            self.CH38(),
            self.CH39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PCDR(pub u32);
impl ADC_PCDR {
    #[doc = "Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn CDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Channel Converted Data"]
    #[inline(always)]
    pub const fn set_CDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn RESULT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Channel Converted Data"]
    #[inline(always)]
    pub const fn set_RESULT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn OVERW(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OVERW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for ADC_PCDR {
    #[inline(always)]
    fn default() -> ADC_PCDR {
        ADC_PCDR(0)
    }
}
impl core::fmt::Debug for ADC_PCDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PCDR")
            .field("CDATA", &self.CDATA())
            .field("RESULT", &self.RESULT())
            .field("OVERW", &self.OVERW())
            .field("VALID", &self.VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PCDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_PCDR {{ CDATA: {=u16:?}, RESULT: {=u8:?}, OVERW: {=bool:?}, VALID: {=bool:?} }}",
            self.CDATA(),
            self.RESULT(),
            self.OVERW(),
            self.VALID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PDEDR(pub u32);
impl ADC_PDEDR {
    #[doc = "Power Down Exist Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn PDED(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Power Down Exist Delay"]
    #[inline(always)]
    pub const fn set_PDED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ADC_PDEDR {
    #[inline(always)]
    fn default() -> ADC_PDEDR {
        ADC_PDEDR(0)
    }
}
impl core::fmt::Debug for ADC_PDEDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PDEDR")
            .field("PDED", &self.PDED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PDEDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_PDEDR {{ PDED: {=u8:?} }}", self.PDED())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PSCR(pub u32);
impl ADC_PSCR {
    #[must_use]
    #[inline(always)]
    pub const fn PRECONV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRECONV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PREVAL0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_PREVAL0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Internal Presampling Voltage Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn PREVAL1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Internal Presampling Voltage Selection"]
    #[inline(always)]
    pub const fn set_PREVAL1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
}
impl Default for ADC_PSCR {
    #[inline(always)]
    fn default() -> ADC_PSCR {
        ADC_PSCR(0)
    }
}
impl core::fmt::Debug for ADC_PSCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PSCR")
            .field("PRECONV", &self.PRECONV())
            .field("PREVAL0", &self.PREVAL0())
            .field("PREVAL1", &self.PREVAL1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PSCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_PSCR {{ PRECONV: {=bool:?}, PREVAL0: {=u8:?}, PREVAL1: {=u8:?} }}",
            self.PRECONV(),
            self.PREVAL0(),
            self.PREVAL1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PSR0(pub u32);
impl ADC_PSR0 {
    #[must_use]
    #[inline(always)]
    pub const fn PRES0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_PSR0 {
    #[inline(always)]
    fn default() -> ADC_PSR0 {
        ADC_PSR0(0)
    }
}
impl core::fmt::Debug for ADC_PSR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PSR0")
            .field("PRES0", &self.PRES0())
            .field("PRES1", &self.PRES1())
            .field("PRES2", &self.PRES2())
            .field("PRES3", &self.PRES3())
            .field("PRES4", &self.PRES4())
            .field("PRES5", &self.PRES5())
            .field("PRES6", &self.PRES6())
            .field("PRES7", &self.PRES7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PSR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_PSR0 {{ PRES0: {=bool:?}, PRES1: {=bool:?}, PRES2: {=bool:?}, PRES3: {=bool:?}, PRES4: {=bool:?}, PRES5: {=bool:?}, PRES6: {=bool:?}, PRES7: {=bool:?} }}",
            self.PRES0(),
            self.PRES1(),
            self.PRES2(),
            self.PRES3(),
            self.PRES4(),
            self.PRES5(),
            self.PRES6(),
            self.PRES7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_PSR1(pub u32);
impl ADC_PSR1 {
    #[must_use]
    #[inline(always)]
    pub const fn PRES32(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES33(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES33(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES34(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES34(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES35(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES36(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES36(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES37(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES37(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES38(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES38(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn PRES39(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_PRES39(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for ADC_PSR1 {
    #[inline(always)]
    fn default() -> ADC_PSR1 {
        ADC_PSR1(0)
    }
}
impl core::fmt::Debug for ADC_PSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_PSR1")
            .field("PRES32", &self.PRES32())
            .field("PRES33", &self.PRES33())
            .field("PRES34", &self.PRES34())
            .field("PRES35", &self.PRES35())
            .field("PRES36", &self.PRES36())
            .field("PRES37", &self.PRES37())
            .field("PRES38", &self.PRES38())
            .field("PRES39", &self.PRES39())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_PSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_PSR1 {{ PRES32: {=bool:?}, PRES33: {=bool:?}, PRES34: {=bool:?}, PRES35: {=bool:?}, PRES36: {=bool:?}, PRES37: {=bool:?}, PRES38: {=bool:?}, PRES39: {=bool:?} }}",
            self.PRES32(),
            self.PRES33(),
            self.PRES34(),
            self.PRES35(),
            self.PRES36(),
            self.PRES37(),
            self.PRES38(),
            self.PRES39()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW0R(pub u32);
impl ADC_STAW0R {
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold Value for Algorithm S Step0"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold Value for Algorithm S Step0"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "High Threshold Value for Algorithm S Step0"]
    #[must_use]
    #[inline(always)]
    pub const fn WDTE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "High Threshold Value for Algorithm S Step0"]
    #[inline(always)]
    pub const fn set_WDTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AWDE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AWDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_STAW0R {
    #[inline(always)]
    fn default() -> ADC_STAW0R {
        ADC_STAW0R(0)
    }
}
impl core::fmt::Debug for ADC_STAW0R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW0R")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .field("WDTE", &self.WDTE())
            .field("AWDE", &self.AWDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW0R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW0R {{ THRL: {=u16:?}, THRH: {=u16:?}, WDTE: {=bool:?}, AWDE: {=bool:?} }}",
            self.THRL(),
            self.THRH(),
            self.WDTE(),
            self.AWDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW1AR(pub u32);
impl ADC_STAW1AR {
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold Value for Algorithm S Step1"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold Value for Algorithm S Step1"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "High Threshold Value for Algorithm S Step1"]
    #[must_use]
    #[inline(always)]
    pub const fn AWDE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "High Threshold Value for Algorithm S Step1"]
    #[inline(always)]
    pub const fn set_AWDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_STAW1AR {
    #[inline(always)]
    fn default() -> ADC_STAW1AR {
        ADC_STAW1AR(0)
    }
}
impl core::fmt::Debug for ADC_STAW1AR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW1AR")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .field("AWDE", &self.AWDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW1AR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW1AR {{ THRL: {=u16:?}, THRH: {=u16:?}, AWDE: {=bool:?} }}",
            self.THRL(),
            self.THRH(),
            self.AWDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW1BR(pub u32);
impl ADC_STAW1BR {
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold Value for Algorithm S Step1"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold Value for Algorithm S Step1"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_STAW1BR {
    #[inline(always)]
    fn default() -> ADC_STAW1BR {
        ADC_STAW1BR(0)
    }
}
impl core::fmt::Debug for ADC_STAW1BR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW1BR")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW1BR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW1BR {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW2R(pub u32);
impl ADC_STAW2R {
    #[doc = "Threshold level low"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Threshold level low"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Threshold level low"]
    #[must_use]
    #[inline(always)]
    pub const fn AWDE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Threshold level low"]
    #[inline(always)]
    pub const fn set_AWDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_STAW2R {
    #[inline(always)]
    fn default() -> ADC_STAW2R {
        ADC_STAW2R(0)
    }
}
impl core::fmt::Debug for ADC_STAW2R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW2R")
            .field("THRL", &self.THRL())
            .field("AWDE", &self.AWDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW2R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW2R {{ THRL: {=u16:?}, AWDE: {=bool:?} }}",
            self.THRL(),
            self.AWDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW4R(pub u32);
impl ADC_STAW4R {
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold Value for Step0 of Algorithm C"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold Value for Step0 of Algorithm C"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "High Threshold Value for Step0 of Algorithm C"]
    #[must_use]
    #[inline(always)]
    pub const fn WDTE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "High Threshold Value for Step0 of Algorithm C"]
    #[inline(always)]
    pub const fn set_WDTE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn AWDE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_AWDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_STAW4R {
    #[inline(always)]
    fn default() -> ADC_STAW4R {
        ADC_STAW4R(0)
    }
}
impl core::fmt::Debug for ADC_STAW4R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW4R")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .field("WDTE", &self.WDTE())
            .field("AWDE", &self.AWDE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW4R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW4R {{ THRL: {=u16:?}, THRH: {=u16:?}, WDTE: {=bool:?}, AWDE: {=bool:?} }}",
            self.THRL(),
            self.THRH(),
            self.WDTE(),
            self.AWDE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STAW5R(pub u32);
impl ADC_STAW5R {
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold Value for Step N of Algorithm C"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold Value for Step N of Algorithm C"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_STAW5R {
    #[inline(always)]
    fn default() -> ADC_STAW5R {
        ADC_STAW5R(0)
    }
}
impl core::fmt::Debug for ADC_STAW5R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STAW5R")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STAW5R {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STAW5R {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STBRR(pub u32);
impl ADC_STBRR {
    #[doc = "Algorithm Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn BR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Algorithm Baud Rate"]
    #[inline(always)]
    pub const fn set_BR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Algorithm Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn WDT(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Algorithm Baud Rate"]
    #[inline(always)]
    pub const fn set_WDT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for ADC_STBRR {
    #[inline(always)]
    fn default() -> ADC_STBRR {
        ADC_STBRR(0)
    }
}
impl core::fmt::Debug for ADC_STBRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STBRR")
            .field("BR", &self.BR())
            .field("WDT", &self.WDT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STBRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STBRR {{ BR: {=u8:?}, WDT: {=u8:?} }}",
            self.BR(),
            self.WDT()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR1_INPSAMP(pub u32);
impl ADC_STCR1_INPSAMP {
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_S(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Sampling Configuration for Algorithm S"]
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sampling Configuration for Algorithm S"]
    #[inline(always)]
    pub const fn set_C(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ADC_STCR1_INPSAMP {
    #[inline(always)]
    fn default() -> ADC_STCR1_INPSAMP {
        ADC_STCR1_INPSAMP(0)
    }
}
impl core::fmt::Debug for ADC_STCR1_INPSAMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR1_INPSAMP")
            .field("S", &self.S())
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR1_INPSAMP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR1_INPSAMP {{ S: {=u8:?}, C: {=u8:?} }}",
            self.S(),
            self.C()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR2(pub u32);
impl ADC_STCR2 {
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDTERR(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn SERR(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_SERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Error Fault Injection Field (write-only)"]
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDSERR(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Error Fault Injection Field (write-only)"]
    #[inline(always)]
    pub const fn set_MSKWDSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for ADC_STCR2 {
    #[inline(always)]
    fn default() -> ADC_STCR2 {
        ADC_STCR2(0)
    }
}
impl core::fmt::Debug for ADC_STCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR2")
            .field("EN", &self.EN())
            .field("MSKWDTERR", &self.MSKWDTERR())
            .field("SERR", &self.SERR())
            .field("MSKWDSERR", &self.MSKWDSERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR2 {{ EN: {=bool:?}, MSKWDTERR: {=bool:?}, SERR: {=bool:?}, MSKWDSERR: {=bool:?} }}",
            self.EN(),
            self.MSKWDTERR(),
            self.SERR(),
            self.MSKWDSERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR2_FMA(pub u32);
impl ADC_STCR2_FMA {
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDTERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDSERR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for ADC_STCR2_FMA {
    #[inline(always)]
    fn default() -> ADC_STCR2_FMA {
        ADC_STCR2_FMA(0)
    }
}
impl core::fmt::Debug for ADC_STCR2_FMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR2_FMA")
            .field("S", &self.S())
            .field("C", &self.C())
            .field("WDTERR", &self.WDTERR())
            .field("WDSERR", &self.WDSERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR2_FMA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR2_FMA {{ S: {=bool:?}, C: {=bool:?}, WDTERR: {=bool:?}, WDSERR: {=bool:?} }}",
            self.S(),
            self.C(),
            self.WDTERR(),
            self.WDSERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR2_MSKERR(pub u32);
impl ADC_STCR2_MSKERR {
    #[must_use]
    #[inline(always)]
    pub const fn S0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for ADC_STCR2_MSKERR {
    #[inline(always)]
    fn default() -> ADC_STCR2_MSKERR {
        ADC_STCR2_MSKERR(0)
    }
}
impl core::fmt::Debug for ADC_STCR2_MSKERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR2_MSKERR")
            .field("S0", &self.S0())
            .field("S1", &self.S1())
            .field("S2", &self.S2())
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR2_MSKERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR2_MSKERR {{ S0: {=bool:?}, S1: {=bool:?}, S2: {=bool:?}, C: {=bool:?} }}",
            self.S0(),
            self.S1(),
            self.S2(),
            self.C()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR2_MSKST(pub u32);
impl ADC_STCR2_MSKST {
    #[must_use]
    #[inline(always)]
    pub const fn EOC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for ADC_STCR2_MSKST {
    #[inline(always)]
    fn default() -> ADC_STCR2_MSKST {
        ADC_STCR2_MSKST(0)
    }
}
impl core::fmt::Debug for ADC_STCR2_MSKST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR2_MSKST")
            .field("EOC", &self.EOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR2_MSKST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_STCR2_MSKST {{ EOC: {=bool:?} }}", self.EOC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR2_MSKWDG_EOA(pub u32);
impl ADC_STCR2_MSKWDG_EOA {
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for ADC_STCR2_MSKWDG_EOA {
    #[inline(always)]
    fn default() -> ADC_STCR2_MSKWDG_EOA {
        ADC_STCR2_MSKWDG_EOA(0)
    }
}
impl core::fmt::Debug for ADC_STCR2_MSKWDG_EOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR2_MSKWDG_EOA")
            .field("S", &self.S())
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR2_MSKWDG_EOA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR2_MSKWDG_EOA {{ S: {=bool:?}, C: {=bool:?} }}",
            self.S(),
            self.C()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STCR3(pub u32);
impl ADC_STCR3 {
    #[doc = "Self-Test Step Select"]
    #[must_use]
    #[inline(always)]
    pub const fn MSTEP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Self-Test Step Select"]
    #[inline(always)]
    pub const fn set_MSTEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Self-Test Step Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ALG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Self-Test Step Select"]
    #[inline(always)]
    pub const fn set_ALG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for ADC_STCR3 {
    #[inline(always)]
    fn default() -> ADC_STCR3 {
        ADC_STCR3(0)
    }
}
impl core::fmt::Debug for ADC_STCR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STCR3")
            .field("MSTEP", &self.MSTEP())
            .field("ALG", &self.ALG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STCR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STCR3 {{ MSTEP: {=u8:?}, ALG: {=u8:?} }}",
            self.MSTEP(),
            self.ALG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STDR1(pub u32);
impl ADC_STDR1 {
    #[doc = "Test Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn TCDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Test Channel Converted Data"]
    #[inline(always)]
    pub const fn set_TCDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Test Channel Converted Data"]
    #[must_use]
    #[inline(always)]
    pub const fn OWERWR(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Test Channel Converted Data"]
    #[inline(always)]
    pub const fn set_OWERWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for ADC_STDR1 {
    #[inline(always)]
    fn default() -> ADC_STDR1 {
        ADC_STDR1(0)
    }
}
impl core::fmt::Debug for ADC_STDR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STDR1")
            .field("TCDATA", &self.TCDATA())
            .field("OWERWR", &self.OWERWR())
            .field("VALID", &self.VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STDR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STDR1 {{ TCDATA: {=u16:?}, OWERWR: {=bool:?}, VALID: {=bool:?} }}",
            self.TCDATA(),
            self.OWERWR(),
            self.VALID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STDR2(pub u32);
impl ADC_STDR2 {
    #[doc = "Integer Data"]
    #[must_use]
    #[inline(always)]
    pub const fn IDATA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer Data"]
    #[inline(always)]
    pub const fn set_IDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Integer Data"]
    #[must_use]
    #[inline(always)]
    pub const fn OVERWR(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Integer Data"]
    #[inline(always)]
    pub const fn set_OVERWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Fractional Data"]
    #[must_use]
    #[inline(always)]
    pub const fn FDATA(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Fractional Data"]
    #[inline(always)]
    pub const fn set_FDATA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for ADC_STDR2 {
    #[inline(always)]
    fn default() -> ADC_STDR2 {
        ADC_STDR2(0)
    }
}
impl core::fmt::Debug for ADC_STDR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STDR2")
            .field("IDATA", &self.IDATA())
            .field("OVERWR", &self.OVERWR())
            .field("VALID", &self.VALID())
            .field("FDATA", &self.FDATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STDR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STDR2 {{ IDATA: {=u16:?}, OVERWR: {=bool:?}, VALID: {=bool:?}, FDATA: {=u16:?} }}",
            self.IDATA(),
            self.OVERWR(),
            self.VALID(),
            self.FDATA()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR1(pub u32);
impl ADC_STSR1 {
    #[must_use]
    #[inline(always)]
    pub const fn OVERWR(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_OVERWR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDTERR(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDTERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDSERR(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDSERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for ADC_STSR1 {
    #[inline(always)]
    fn default() -> ADC_STSR1 {
        ADC_STSR1(0)
    }
}
impl core::fmt::Debug for ADC_STSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR1")
            .field("OVERWR", &self.OVERWR())
            .field("WDTERR", &self.WDTERR())
            .field("WDSERR", &self.WDSERR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STSR1 {{ OVERWR: {=bool:?}, WDTERR: {=bool:?}, WDSERR: {=bool:?} }}",
            self.OVERWR(),
            self.WDTERR(),
            self.WDSERR()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR1_ERR(pub u32);
impl ADC_STSR1_ERR {
    #[doc = "Algorithm C Step Number Error"]
    #[must_use]
    #[inline(always)]
    pub const fn S0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Algorithm C Step Number Error"]
    #[inline(always)]
    pub const fn set_S0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn S2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for ADC_STSR1_ERR {
    #[inline(always)]
    fn default() -> ADC_STSR1_ERR {
        ADC_STSR1_ERR(0)
    }
}
impl core::fmt::Debug for ADC_STSR1_ERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR1_ERR")
            .field("S0", &self.S0())
            .field("S1", &self.S1())
            .field("S2", &self.S2())
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR1_ERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STSR1_ERR {{ S0: {=bool:?}, S1: {=bool:?}, S2: {=bool:?}, C: {=bool:?} }}",
            self.S0(),
            self.S1(),
            self.S2(),
            self.C()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR1_ST(pub u32);
impl ADC_STSR1_ST {
    #[must_use]
    #[inline(always)]
    pub const fn EOC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_EOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for ADC_STSR1_ST {
    #[inline(always)]
    fn default() -> ADC_STSR1_ST {
        ADC_STSR1_ST(0)
    }
}
impl core::fmt::Debug for ADC_STSR1_ST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR1_ST")
            .field("EOC", &self.EOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR1_ST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_STSR1_ST {{ EOC: {=bool:?} }}", self.EOC())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR1_STEP(pub u32);
impl ADC_STSR1_STEP {
    #[doc = "Algorithm C Step Number Error"]
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Algorithm C Step Number Error"]
    #[inline(always)]
    pub const fn set_C(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
}
impl Default for ADC_STSR1_STEP {
    #[inline(always)]
    fn default() -> ADC_STSR1_STEP {
        ADC_STSR1_STEP(0)
    }
}
impl core::fmt::Debug for ADC_STSR1_STEP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR1_STEP")
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR1_STEP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_STSR1_STEP {{ C: {=u8:?} }}", self.C())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR1_WDG_EOA(pub u32);
impl ADC_STSR1_WDG_EOA {
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn C(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for ADC_STSR1_WDG_EOA {
    #[inline(always)]
    fn default() -> ADC_STSR1_WDG_EOA {
        ADC_STSR1_WDG_EOA(0)
    }
}
impl core::fmt::Debug for ADC_STSR1_WDG_EOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR1_WDG_EOA")
            .field("S", &self.S())
            .field("C", &self.C())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR1_WDG_EOA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STSR1_WDG_EOA {{ S: {=bool:?}, C: {=bool:?} }}",
            self.S(),
            self.C()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR2(pub u32);
impl ADC_STSR2 {
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Test Channel Converted Data when ERR_S1 Occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Test Channel Converted Data when ERR_S1 Occurred"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Test Channel Converted Data when ERR_S1 Occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn OVFL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Test Channel Converted Data when ERR_S1 Occurred"]
    #[inline(always)]
    pub const fn set_OVFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ADC_STSR2 {
    #[inline(always)]
    fn default() -> ADC_STSR2 {
        ADC_STSR2(0)
    }
}
impl core::fmt::Debug for ADC_STSR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR2")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .field("OVFL", &self.OVFL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STSR2 {{ DATA0: {=u16:?}, DATA1: {=u16:?}, OVFL: {=bool:?} }}",
            self.DATA0(),
            self.DATA1(),
            self.OVFL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR3(pub u32);
impl ADC_STSR3 {
    #[must_use]
    #[inline(always)]
    pub const fn DATA0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DATA0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Test Channel Converted Data when ERR_S0 Occurred"]
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Test Channel Converted Data when ERR_S0 Occurred"]
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_STSR3 {
    #[inline(always)]
    fn default() -> ADC_STSR3 {
        ADC_STSR3(0)
    }
}
impl core::fmt::Debug for ADC_STSR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR3")
            .field("DATA0", &self.DATA0())
            .field("DATA1", &self.DATA1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_STSR3 {{ DATA0: {=u16:?}, DATA1: {=u16:?} }}",
            self.DATA0(),
            self.DATA1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_STSR4(pub u32);
impl ADC_STSR4 {
    #[must_use]
    #[inline(always)]
    pub const fn DATA1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_DATA1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_STSR4 {
    #[inline(always)]
    fn default() -> ADC_STSR4 {
        ADC_STSR4(0)
    }
}
impl core::fmt::Debug for ADC_STSR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_STSR4")
            .field("DATA1", &self.DATA1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_STSR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ADC_STSR4 {{ DATA1: {=u16:?} }}", self.DATA1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR0(pub u32);
impl ADC_THRHLR0 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR0 {
    #[inline(always)]
    fn default() -> ADC_THRHLR0 {
        ADC_THRHLR0(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR0")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR0 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR1(pub u32);
impl ADC_THRHLR1 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR1 {
    #[inline(always)]
    fn default() -> ADC_THRHLR1 {
        ADC_THRHLR1(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR1")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR1 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR2(pub u32);
impl ADC_THRHLR2 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR2 {
    #[inline(always)]
    fn default() -> ADC_THRHLR2 {
        ADC_THRHLR2(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR2")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR2 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR3(pub u32);
impl ADC_THRHLR3 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR3 {
    #[inline(always)]
    fn default() -> ADC_THRHLR3 {
        ADC_THRHLR3(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR3")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR3 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR4(pub u32);
impl ADC_THRHLR4 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR4 {
    #[inline(always)]
    fn default() -> ADC_THRHLR4 {
        ADC_THRHLR4(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR4")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR4 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR5(pub u32);
impl ADC_THRHLR5 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR5 {
    #[inline(always)]
    fn default() -> ADC_THRHLR5 {
        ADC_THRHLR5(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR5")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR5 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR6(pub u32);
impl ADC_THRHLR6 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR6 {
    #[inline(always)]
    fn default() -> ADC_THRHLR6 {
        ADC_THRHLR6(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR6")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR6 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_THRHLR7(pub u32);
impl ADC_THRHLR7 {
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn THRH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Low Threshold"]
    #[inline(always)]
    pub const fn set_THRH(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for ADC_THRHLR7 {
    #[inline(always)]
    fn default() -> ADC_THRHLR7 {
        ADC_THRHLR7(0)
    }
}
impl core::fmt::Debug for ADC_THRHLR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_THRHLR7")
            .field("THRL", &self.THRL())
            .field("THRH", &self.THRH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_THRHLR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_THRHLR7 {{ THRL: {=u16:?}, THRH: {=u16:?} }}",
            self.THRL(),
            self.THRH()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_USROFSGN(pub u32);
impl ADC_USROFSGN {
    #[doc = "User Defined Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSUSER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "User Defined Offset"]
    #[inline(always)]
    pub const fn set_OFFSUSER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "User Defined Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn GAINUSER(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "User Defined Offset"]
    #[inline(always)]
    pub const fn set_GAINUSER(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for ADC_USROFSGN {
    #[inline(always)]
    fn default() -> ADC_USROFSGN {
        ADC_USROFSGN(0)
    }
}
impl core::fmt::Debug for ADC_USROFSGN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_USROFSGN")
            .field("OFFSUSER", &self.OFFSUSER())
            .field("GAINUSER", &self.GAINUSER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_USROFSGN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_USROFSGN {{ OFFSUSER: {=u8:?}, GAINUSER: {=u16:?} }}",
            self.OFFSUSER(),
            self.GAINUSER()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_WTIMR(pub u32);
impl ADC_WTIMR {
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG0L(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG0H(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG0H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG1L(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG1H(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG1H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG2L(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG2L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG2H(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG2H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG3L(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG3L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG3H(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG3H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG4L(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG4L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG4H(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG4H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG5L(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG5L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG5H(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG5H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG6L(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG6L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG6H(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG6H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG7L(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG7L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn MSKWDG7H(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_MSKWDG7H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for ADC_WTIMR {
    #[inline(always)]
    fn default() -> ADC_WTIMR {
        ADC_WTIMR(0)
    }
}
impl core::fmt::Debug for ADC_WTIMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_WTIMR")
            .field("MSKWDG0L", &self.MSKWDG0L())
            .field("MSKWDG0H", &self.MSKWDG0H())
            .field("MSKWDG1L", &self.MSKWDG1L())
            .field("MSKWDG1H", &self.MSKWDG1H())
            .field("MSKWDG2L", &self.MSKWDG2L())
            .field("MSKWDG2H", &self.MSKWDG2H())
            .field("MSKWDG3L", &self.MSKWDG3L())
            .field("MSKWDG3H", &self.MSKWDG3H())
            .field("MSKWDG4L", &self.MSKWDG4L())
            .field("MSKWDG4H", &self.MSKWDG4H())
            .field("MSKWDG5L", &self.MSKWDG5L())
            .field("MSKWDG5H", &self.MSKWDG5H())
            .field("MSKWDG6L", &self.MSKWDG6L())
            .field("MSKWDG6H", &self.MSKWDG6H())
            .field("MSKWDG7L", &self.MSKWDG7L())
            .field("MSKWDG7H", &self.MSKWDG7H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_WTIMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_WTIMR {{ MSKWDG0L: {=bool:?}, MSKWDG0H: {=bool:?}, MSKWDG1L: {=bool:?}, MSKWDG1H: {=bool:?}, MSKWDG2L: {=bool:?}, MSKWDG2H: {=bool:?}, MSKWDG3L: {=bool:?}, MSKWDG3H: {=bool:?}, MSKWDG4L: {=bool:?}, MSKWDG4H: {=bool:?}, MSKWDG5L: {=bool:?}, MSKWDG5H: {=bool:?}, MSKWDG6L: {=bool:?}, MSKWDG6H: {=bool:?}, MSKWDG7L: {=bool:?}, MSKWDG7H: {=bool:?} }}",
            self.MSKWDG0L(),
            self.MSKWDG0H(),
            self.MSKWDG1L(),
            self.MSKWDG1H(),
            self.MSKWDG2L(),
            self.MSKWDG2H(),
            self.MSKWDG3L(),
            self.MSKWDG3H(),
            self.MSKWDG4L(),
            self.MSKWDG4H(),
            self.MSKWDG5L(),
            self.MSKWDG5H(),
            self.MSKWDG6L(),
            self.MSKWDG6H(),
            self.MSKWDG7L(),
            self.MSKWDG7H()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_WTISR(pub u32);
impl ADC_WTISR {
    #[must_use]
    #[inline(always)]
    pub const fn WDG0L(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG0H(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG0H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG1L(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG1H(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG1H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG2L(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG2L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG2H(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG2H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG3L(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG3L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG3H(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG3H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG4L(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG4L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG4H(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG4H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG5L(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG5L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG5H(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG5H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG6L(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG6L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG6H(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG6H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG7L(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG7L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn WDG7H(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_WDG7H(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for ADC_WTISR {
    #[inline(always)]
    fn default() -> ADC_WTISR {
        ADC_WTISR(0)
    }
}
impl core::fmt::Debug for ADC_WTISR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_WTISR")
            .field("WDG0L", &self.WDG0L())
            .field("WDG0H", &self.WDG0H())
            .field("WDG1L", &self.WDG1L())
            .field("WDG1H", &self.WDG1H())
            .field("WDG2L", &self.WDG2L())
            .field("WDG2H", &self.WDG2H())
            .field("WDG3L", &self.WDG3L())
            .field("WDG3H", &self.WDG3H())
            .field("WDG4L", &self.WDG4L())
            .field("WDG4H", &self.WDG4H())
            .field("WDG5L", &self.WDG5L())
            .field("WDG5H", &self.WDG5H())
            .field("WDG6L", &self.WDG6L())
            .field("WDG6H", &self.WDG6H())
            .field("WDG7L", &self.WDG7L())
            .field("WDG7H", &self.WDG7H())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_WTISR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_WTISR {{ WDG0L: {=bool:?}, WDG0H: {=bool:?}, WDG1L: {=bool:?}, WDG1H: {=bool:?}, WDG2L: {=bool:?}, WDG2H: {=bool:?}, WDG3L: {=bool:?}, WDG3H: {=bool:?}, WDG4L: {=bool:?}, WDG4H: {=bool:?}, WDG5L: {=bool:?}, WDG5H: {=bool:?}, WDG6L: {=bool:?}, WDG6H: {=bool:?}, WDG7L: {=bool:?}, WDG7H: {=bool:?} }}",
            self.WDG0L(),
            self.WDG0H(),
            self.WDG1L(),
            self.WDG1H(),
            self.WDG2L(),
            self.WDG2H(),
            self.WDG3L(),
            self.WDG3H(),
            self.WDG4L(),
            self.WDG4H(),
            self.WDG5L(),
            self.WDG5H(),
            self.WDG6L(),
            self.WDG6H(),
            self.WDG7L(),
            self.WDG7H()
        )
    }
}

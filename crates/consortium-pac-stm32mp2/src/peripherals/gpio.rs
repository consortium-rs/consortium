#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO {
    ptr: *mut u8,
}
unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}
impl GPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO mode register"]
    #[inline(always)]
    pub const fn MODER(self) -> crate::common::Reg<GPIO_MODER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPIO output type register"]
    #[inline(always)]
    pub const fn OTYPER(self) -> crate::common::Reg<GPIO_OTYPER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPIO output speed register"]
    #[inline(always)]
    pub const fn OSPEEDR(self) -> crate::common::Reg<GPIO_OSPEEDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn PUPDR(self) -> crate::common::Reg<GPIO_PUPDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GPIO input data register"]
    #[inline(always)]
    pub const fn IDR(self) -> crate::common::Reg<GPIO_IDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GPIO output data register"]
    #[inline(always)]
    pub const fn ODR(self) -> crate::common::Reg<GPIO_ODR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GPIO bit set/reset register"]
    #[inline(always)]
    pub const fn BSRR(self) -> crate::common::Reg<GPIO_BSRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "GPIO configuration lock register"]
    #[inline(always)]
    pub const fn LCKR(self) -> crate::common::Reg<GPIO_LCKR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "GPIO alternate function registers"]
    #[inline(always)]
    pub const fn AFR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "GPIO bit reset register"]
    #[inline(always)]
    pub const fn BRR(self) -> crate::common::Reg<GPIO_BRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "GPIO bit option register"]
    #[inline(always)]
    pub const fn OR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "GPIO secure configuration register"]
    #[inline(always)]
    pub const fn SECCFGR(self) -> crate::common::Reg<GPIO_SECCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPIO privileged configuration register"]
    #[inline(always)]
    pub const fn PRIVCFGR(self) -> crate::common::Reg<GPIO_PRIVCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GPIO resource configuration lock register"]
    #[inline(always)]
    pub const fn RCFGLOCKR(self) -> crate::common::Reg<GPIO_RCFGLOCKR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GPIO delay low register"]
    #[inline(always)]
    pub const fn DELAYRL(self) -> crate::common::Reg<GPIO_DELAYRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GPIO delay high register"]
    #[inline(always)]
    pub const fn DELAYRH(self) -> crate::common::Reg<GPIO_DELAYRH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPIO PIO control low register"]
    #[inline(always)]
    pub const fn ADVCFGRL(self) -> crate::common::Reg<GPIO_ADVCFGRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "GPIO delay high register"]
    #[inline(always)]
    pub const fn ADVCFGRH(self) -> crate::common::Reg<GPIO_ADVCFGRH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 0"]
    #[inline(always)]
    pub const fn CIDCFGR0(self) -> crate::common::Reg<GPIO_CIDCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 0"]
    #[inline(always)]
    pub const fn SEMCR0(self) -> crate::common::Reg<GPIO_SEMCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 1"]
    #[inline(always)]
    pub const fn CIDCFGR1(self) -> crate::common::Reg<GPIO_CIDCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 1"]
    #[inline(always)]
    pub const fn SEMCR1(self) -> crate::common::Reg<GPIO_SEMCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 2"]
    #[inline(always)]
    pub const fn CIDCFGR2(self) -> crate::common::Reg<GPIO_CIDCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 2"]
    #[inline(always)]
    pub const fn SEMCR2(self) -> crate::common::Reg<GPIO_SEMCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 3"]
    #[inline(always)]
    pub const fn CIDCFGR3(self) -> crate::common::Reg<GPIO_CIDCFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 3"]
    #[inline(always)]
    pub const fn SEMCR3(self) -> crate::common::Reg<GPIO_SEMCR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 4"]
    #[inline(always)]
    pub const fn CIDCFGR4(self) -> crate::common::Reg<GPIO_CIDCFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 4"]
    #[inline(always)]
    pub const fn SEMCR4(self) -> crate::common::Reg<GPIO_SEMCR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 5"]
    #[inline(always)]
    pub const fn CIDCFGR5(self) -> crate::common::Reg<GPIO_CIDCFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 5"]
    #[inline(always)]
    pub const fn SEMCR5(self) -> crate::common::Reg<GPIO_SEMCR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 6"]
    #[inline(always)]
    pub const fn CIDCFGR6(self) -> crate::common::Reg<GPIO_CIDCFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 6"]
    #[inline(always)]
    pub const fn SEMCR6(self) -> crate::common::Reg<GPIO_SEMCR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 7"]
    #[inline(always)]
    pub const fn CIDCFGR7(self) -> crate::common::Reg<GPIO_CIDCFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 7"]
    #[inline(always)]
    pub const fn SEMCR7(self) -> crate::common::Reg<GPIO_SEMCR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 8"]
    #[inline(always)]
    pub const fn CIDCFGR8(self) -> crate::common::Reg<GPIO_CIDCFGR8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 8"]
    #[inline(always)]
    pub const fn SEMCR8(self) -> crate::common::Reg<GPIO_SEMCR8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 9"]
    #[inline(always)]
    pub const fn CIDCFGR9(self) -> crate::common::Reg<GPIO_CIDCFGR9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 9"]
    #[inline(always)]
    pub const fn SEMCR9(self) -> crate::common::Reg<GPIO_SEMCR9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 10"]
    #[inline(always)]
    pub const fn CIDCFGR10(self) -> crate::common::Reg<GPIO_CIDCFGR10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 10"]
    #[inline(always)]
    pub const fn SEMCR10(self) -> crate::common::Reg<GPIO_SEMCR10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 11"]
    #[inline(always)]
    pub const fn CIDCFGR11(self) -> crate::common::Reg<GPIO_CIDCFGR11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 11"]
    #[inline(always)]
    pub const fn SEMCR11(self) -> crate::common::Reg<GPIO_SEMCR11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 12"]
    #[inline(always)]
    pub const fn CIDCFGR12(self) -> crate::common::Reg<GPIO_CIDCFGR12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 12"]
    #[inline(always)]
    pub const fn SEMCR12(self) -> crate::common::Reg<GPIO_SEMCR12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 13"]
    #[inline(always)]
    pub const fn CIDCFGR13(self) -> crate::common::Reg<GPIO_CIDCFGR13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 13"]
    #[inline(always)]
    pub const fn SEMCR13(self) -> crate::common::Reg<GPIO_SEMCR13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 14"]
    #[inline(always)]
    pub const fn CIDCFGR14(self) -> crate::common::Reg<GPIO_CIDCFGR14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 14"]
    #[inline(always)]
    pub const fn SEMCR14(self) -> crate::common::Reg<GPIO_SEMCR14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "GPIO CID configuration register for I/O pin 15"]
    #[inline(always)]
    pub const fn CIDCFGR15(self) -> crate::common::Reg<GPIO_CIDCFGR15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "GPIO semaphore control register for I/O pin 15"]
    #[inline(always)]
    pub const fn SEMCR15(self) -> crate::common::Reg<GPIO_SEMCR15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "GPIO hardware configuration register 10"]
    #[inline(always)]
    pub const fn HWCFGR10(self) -> crate::common::Reg<GPIO_HWCFGR10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 9"]
    #[inline(always)]
    pub const fn HWCFGR9(self) -> crate::common::Reg<GPIO_HWCFGR9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "GPIO hardware configuration register 8"]
    #[inline(always)]
    pub const fn HWCFGR8(self) -> crate::common::Reg<GPIO_HWCFGR8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 7"]
    #[inline(always)]
    pub const fn HWCFGR7(self) -> crate::common::Reg<GPIO_HWCFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 6"]
    #[inline(always)]
    pub const fn HWCFGR6(self) -> crate::common::Reg<GPIO_HWCFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 5"]
    #[inline(always)]
    pub const fn HWCFGR5(self) -> crate::common::Reg<GPIO_HWCFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "GPIO hardware configuration register 4"]
    #[inline(always)]
    pub const fn HWCFGR4(self) -> crate::common::Reg<GPIO_HWCFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 3"]
    #[inline(always)]
    pub const fn HWCFGR3(self) -> crate::common::Reg<GPIO_HWCFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 2"]
    #[inline(always)]
    pub const fn HWCFGR2(self) -> crate::common::Reg<GPIO_HWCFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "GPIO hardware configuration register 1"]
    #[inline(always)]
    pub const fn HWCFGR1(self) -> crate::common::Reg<GPIO_HWCFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "GPIO hardware configuration register 0"]
    #[inline(always)]
    pub const fn HWCFGR0(self) -> crate::common::Reg<GPIO_HWCFGR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "GPIO version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<GPIO_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "GPIO identification register"]
    #[inline(always)]
    pub const fn IPIDR(self) -> crate::common::Reg<GPIO_IPIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "GPIO size identification register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<GPIO_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_ADVCFGRH(pub u32);
impl GPIO_ADVCFGRH {
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRH15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y configuration (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRH15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_ADVCFGRH {
    #[inline(always)]
    fn default() -> GPIO_ADVCFGRH {
        GPIO_ADVCFGRH(0)
    }
}
impl core::fmt::Debug for GPIO_ADVCFGRH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_ADVCFGRH")
            .field("ADVCFGRH8", &self.ADVCFGRH8())
            .field("ADVCFGRH9", &self.ADVCFGRH9())
            .field("ADVCFGRH10", &self.ADVCFGRH10())
            .field("ADVCFGRH11", &self.ADVCFGRH11())
            .field("ADVCFGRH12", &self.ADVCFGRH12())
            .field("ADVCFGRH13", &self.ADVCFGRH13())
            .field("ADVCFGRH14", &self.ADVCFGRH14())
            .field("ADVCFGRH15", &self.ADVCFGRH15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_ADVCFGRH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_ADVCFGRH {{ ADVCFGRH8: {=u8:?}, ADVCFGRH9: {=u8:?}, ADVCFGRH10: {=u8:?}, ADVCFGRH11: {=u8:?}, ADVCFGRH12: {=u8:?}, ADVCFGRH13: {=u8:?}, ADVCFGRH14: {=u8:?}, ADVCFGRH15: {=u8:?} }}",
            self.ADVCFGRH8(),
            self.ADVCFGRH9(),
            self.ADVCFGRH10(),
            self.ADVCFGRH11(),
            self.ADVCFGRH12(),
            self.ADVCFGRH13(),
            self.ADVCFGRH14(),
            self.ADVCFGRH15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_ADVCFGRL(pub u32);
impl GPIO_ADVCFGRL {
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn ADVCFGRL7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin configuration (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_ADVCFGRL7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_ADVCFGRL {
    #[inline(always)]
    fn default() -> GPIO_ADVCFGRL {
        GPIO_ADVCFGRL(0)
    }
}
impl core::fmt::Debug for GPIO_ADVCFGRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_ADVCFGRL")
            .field("ADVCFGRL0", &self.ADVCFGRL0())
            .field("ADVCFGRL1", &self.ADVCFGRL1())
            .field("ADVCFGRL2", &self.ADVCFGRL2())
            .field("ADVCFGRL3", &self.ADVCFGRL3())
            .field("ADVCFGRL4", &self.ADVCFGRL4())
            .field("ADVCFGRL5", &self.ADVCFGRL5())
            .field("ADVCFGRL6", &self.ADVCFGRL6())
            .field("ADVCFGRL7", &self.ADVCFGRL7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_ADVCFGRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_ADVCFGRL {{ ADVCFGRL0: {=u8:?}, ADVCFGRL1: {=u8:?}, ADVCFGRL2: {=u8:?}, ADVCFGRL3: {=u8:?}, ADVCFGRL4: {=u8:?}, ADVCFGRL5: {=u8:?}, ADVCFGRL6: {=u8:?}, ADVCFGRL7: {=u8:?} }}",
            self.ADVCFGRL0(),
            self.ADVCFGRL1(),
            self.ADVCFGRL2(),
            self.ADVCFGRL3(),
            self.ADVCFGRL4(),
            self.ADVCFGRL5(),
            self.ADVCFGRL6(),
            self.ADVCFGRL7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_BRR(pub u32);
impl GPIO_BRR {
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset IO pin y (y= 0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_BRR {
    #[inline(always)]
    fn default() -> GPIO_BRR {
        GPIO_BRR(0)
    }
}
impl core::fmt::Debug for GPIO_BRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_BRR")
            .field("BR0", &self.BR0())
            .field("BR1", &self.BR1())
            .field("BR2", &self.BR2())
            .field("BR3", &self.BR3())
            .field("BR4", &self.BR4())
            .field("BR5", &self.BR5())
            .field("BR6", &self.BR6())
            .field("BR7", &self.BR7())
            .field("BR8", &self.BR8())
            .field("BR9", &self.BR9())
            .field("BR10", &self.BR10())
            .field("BR11", &self.BR11())
            .field("BR12", &self.BR12())
            .field("BR13", &self.BR13())
            .field("BR14", &self.BR14())
            .field("BR15", &self.BR15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_BRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_BRR {{ BR0: {=bool:?}, BR1: {=bool:?}, BR2: {=bool:?}, BR3: {=bool:?}, BR4: {=bool:?}, BR5: {=bool:?}, BR6: {=bool:?}, BR7: {=bool:?}, BR8: {=bool:?}, BR9: {=bool:?}, BR10: {=bool:?}, BR11: {=bool:?}, BR12: {=bool:?}, BR13: {=bool:?}, BR14: {=bool:?}, BR15: {=bool:?} }}",
            self.BR0(),
            self.BR1(),
            self.BR2(),
            self.BR3(),
            self.BR4(),
            self.BR5(),
            self.BR6(),
            self.BR7(),
            self.BR8(),
            self.BR9(),
            self.BR10(),
            self.BR11(),
            self.BR12(),
            self.BR13(),
            self.BR14(),
            self.BR15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_BSRR(pub u32);
impl GPIO_BSRR {
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BS15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x set I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BS15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR8(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR9(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR10(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR11(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR12(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR13(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR14(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[must_use]
    #[inline(always)]
    pub const fn BR15(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Port x reset I/O pin y (y=0 to 15)These bits are write-only. A read to these bits returns the value 0x0000.Note: If both BSx and BRx are set, BSx has priority.Successful write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can write this bit, as described in Section1.4: IO pin allocation to compartments."]
    #[inline(always)]
    pub const fn set_BR15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for GPIO_BSRR {
    #[inline(always)]
    fn default() -> GPIO_BSRR {
        GPIO_BSRR(0)
    }
}
impl core::fmt::Debug for GPIO_BSRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_BSRR")
            .field("BS0", &self.BS0())
            .field("BS1", &self.BS1())
            .field("BS2", &self.BS2())
            .field("BS3", &self.BS3())
            .field("BS4", &self.BS4())
            .field("BS5", &self.BS5())
            .field("BS6", &self.BS6())
            .field("BS7", &self.BS7())
            .field("BS8", &self.BS8())
            .field("BS9", &self.BS9())
            .field("BS10", &self.BS10())
            .field("BS11", &self.BS11())
            .field("BS12", &self.BS12())
            .field("BS13", &self.BS13())
            .field("BS14", &self.BS14())
            .field("BS15", &self.BS15())
            .field("BR0", &self.BR0())
            .field("BR1", &self.BR1())
            .field("BR2", &self.BR2())
            .field("BR3", &self.BR3())
            .field("BR4", &self.BR4())
            .field("BR5", &self.BR5())
            .field("BR6", &self.BR6())
            .field("BR7", &self.BR7())
            .field("BR8", &self.BR8())
            .field("BR9", &self.BR9())
            .field("BR10", &self.BR10())
            .field("BR11", &self.BR11())
            .field("BR12", &self.BR12())
            .field("BR13", &self.BR13())
            .field("BR14", &self.BR14())
            .field("BR15", &self.BR15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_BSRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_BSRR {{ BS0: {=bool:?}, BS1: {=bool:?}, BS2: {=bool:?}, BS3: {=bool:?}, BS4: {=bool:?}, BS5: {=bool:?}, BS6: {=bool:?}, BS7: {=bool:?}, BS8: {=bool:?}, BS9: {=bool:?}, BS10: {=bool:?}, BS11: {=bool:?}, BS12: {=bool:?}, BS13: {=bool:?}, BS14: {=bool:?}, BS15: {=bool:?}, BR0: {=bool:?}, BR1: {=bool:?}, BR2: {=bool:?}, BR3: {=bool:?}, BR4: {=bool:?}, BR5: {=bool:?}, BR6: {=bool:?}, BR7: {=bool:?}, BR8: {=bool:?}, BR9: {=bool:?}, BR10: {=bool:?}, BR11: {=bool:?}, BR12: {=bool:?}, BR13: {=bool:?}, BR14: {=bool:?}, BR15: {=bool:?} }}",
            self.BS0(),
            self.BS1(),
            self.BS2(),
            self.BS3(),
            self.BS4(),
            self.BS5(),
            self.BS6(),
            self.BS7(),
            self.BS8(),
            self.BS9(),
            self.BS10(),
            self.BS11(),
            self.BS12(),
            self.BS13(),
            self.BS14(),
            self.BS15(),
            self.BR0(),
            self.BR1(),
            self.BR2(),
            self.BR3(),
            self.BR4(),
            self.BR5(),
            self.BR6(),
            self.BR7(),
            self.BR8(),
            self.BR9(),
            self.BR10(),
            self.BR11(),
            self.BR12(),
            self.BR13(),
            self.BR14(),
            self.BR15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR0(pub u32);
impl GPIO_CIDCFGR0 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR0 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR0 {
        GPIO_CIDCFGR0(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR0")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR0 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR1(pub u32);
impl GPIO_CIDCFGR1 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR1 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR1 {
        GPIO_CIDCFGR1(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR1")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR1 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR10(pub u32);
impl GPIO_CIDCFGR10 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR10 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR10 {
        GPIO_CIDCFGR10(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR10")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR10 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR11(pub u32);
impl GPIO_CIDCFGR11 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR11 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR11 {
        GPIO_CIDCFGR11(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR11")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR11 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR12(pub u32);
impl GPIO_CIDCFGR12 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR12 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR12 {
        GPIO_CIDCFGR12(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR12")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR12 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR13(pub u32);
impl GPIO_CIDCFGR13 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR13 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR13 {
        GPIO_CIDCFGR13(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR13")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR13 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR14(pub u32);
impl GPIO_CIDCFGR14 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR14 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR14 {
        GPIO_CIDCFGR14(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR14")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR14 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR15(pub u32);
impl GPIO_CIDCFGR15 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR15 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR15 {
        GPIO_CIDCFGR15(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR15")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR15 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR2(pub u32);
impl GPIO_CIDCFGR2 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR2 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR2 {
        GPIO_CIDCFGR2(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR2")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR2 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR3(pub u32);
impl GPIO_CIDCFGR3 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR3 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR3 {
        GPIO_CIDCFGR3(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR3")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR3 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR4(pub u32);
impl GPIO_CIDCFGR4 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR4 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR4 {
        GPIO_CIDCFGR4(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR4")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR4 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR5(pub u32);
impl GPIO_CIDCFGR5 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR5 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR5 {
        GPIO_CIDCFGR5(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR5")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR5 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR6(pub u32);
impl GPIO_CIDCFGR6 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR6 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR6 {
        GPIO_CIDCFGR6(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR6")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR6 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR7(pub u32);
impl GPIO_CIDCFGR7 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR7 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR7 {
        GPIO_CIDCFGR7(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR7")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR7 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR8(pub u32);
impl GPIO_CIDCFGR8 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR8 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR8 {
        GPIO_CIDCFGR8(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR8")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR8 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_CIDCFGR9(pub u32);
impl GPIO_CIDCFGR9 {
    #[doc = "CID filtering enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CFEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CID filtering enable"]
    #[inline(always)]
    pub const fn set_CFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore enable"]
    #[inline(always)]
    pub const fn set_SEM_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Static CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Static CID"]
    #[inline(always)]
    pub const fn set_SCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMWLC3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore whitelist for compartment y (y=0 to 3)"]
    #[inline(always)]
    pub const fn set_SEMWLC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for GPIO_CIDCFGR9 {
    #[inline(always)]
    fn default() -> GPIO_CIDCFGR9 {
        GPIO_CIDCFGR9(0)
    }
}
impl core::fmt::Debug for GPIO_CIDCFGR9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_CIDCFGR9")
            .field("CFEN", &self.CFEN())
            .field("SEM_EN", &self.SEM_EN())
            .field("SCID", &self.SCID())
            .field("SEMWLC0", &self.SEMWLC0())
            .field("SEMWLC1", &self.SEMWLC1())
            .field("SEMWLC2", &self.SEMWLC2())
            .field("SEMWLC3", &self.SEMWLC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_CIDCFGR9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_CIDCFGR9 {{ CFEN: {=bool:?}, SEM_EN: {=bool:?}, SCID: {=u8:?}, SEMWLC0: {=bool:?}, SEMWLC1: {=bool:?}, SEMWLC2: {=bool:?}, SEMWLC3: {=bool:?} }}",
            self.CFEN(),
            self.SEM_EN(),
            self.SCID(),
            self.SEMWLC0(),
            self.SEMWLC1(),
            self.SEMWLC2(),
            self.SEMWLC3()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_DELAYRH(pub u32);
impl GPIO_DELAYRH {
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 8 to 15)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_DELAYRH {
    #[inline(always)]
    fn default() -> GPIO_DELAYRH {
        GPIO_DELAYRH(0)
    }
}
impl core::fmt::Debug for GPIO_DELAYRH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_DELAYRH")
            .field("DLY8", &self.DLY8())
            .field("DLY9", &self.DLY9())
            .field("DLY10", &self.DLY10())
            .field("DLY11", &self.DLY11())
            .field("DLY12", &self.DLY12())
            .field("DLY13", &self.DLY13())
            .field("DLY14", &self.DLY14())
            .field("DLY15", &self.DLY15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_DELAYRH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_DELAYRH {{ DLY8: {=u8:?}, DLY9: {=u8:?}, DLY10: {=u8:?}, DLY11: {=u8:?}, DLY12: {=u8:?}, DLY13: {=u8:?}, DLY14: {=u8:?}, DLY15: {=u8:?} }}",
            self.DLY8(),
            self.DLY9(),
            self.DLY10(),
            self.DLY11(),
            self.DLY12(),
            self.DLY13(),
            self.DLY14(),
            self.DLY15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_DELAYRL(pub u32);
impl GPIO_DELAYRL {
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn DLY7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "3:0\\]: Port x IO pin y delay setup (y = 0 to 7)Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_DLY7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_DELAYRL {
    #[inline(always)]
    fn default() -> GPIO_DELAYRL {
        GPIO_DELAYRL(0)
    }
}
impl core::fmt::Debug for GPIO_DELAYRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_DELAYRL")
            .field("DLY0", &self.DLY0())
            .field("DLY1", &self.DLY1())
            .field("DLY2", &self.DLY2())
            .field("DLY3", &self.DLY3())
            .field("DLY4", &self.DLY4())
            .field("DLY5", &self.DLY5())
            .field("DLY6", &self.DLY6())
            .field("DLY7", &self.DLY7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_DELAYRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_DELAYRL {{ DLY0: {=u8:?}, DLY1: {=u8:?}, DLY2: {=u8:?}, DLY3: {=u8:?}, DLY4: {=u8:?}, DLY5: {=u8:?}, DLY6: {=u8:?}, DLY7: {=u8:?} }}",
            self.DLY0(),
            self.DLY1(),
            self.DLY2(),
            self.DLY3(),
            self.DLY4(),
            self.DLY5(),
            self.DLY6(),
            self.DLY7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR0(pub u32);
impl GPIO_HWCFGR0 {
    #[doc = "Option register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn OR_RES(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Option register reset value"]
    #[inline(always)]
    pub const fn set_OR_RES(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR0 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR0 {
        GPIO_HWCFGR0(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR0")
            .field("OR_RES", &self.OR_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_HWCFGR0 {{ OR_RES: {=u16:?} }}", self.OR_RES())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR1(pub u32);
impl GPIO_HWCFGR1 {
    #[doc = "AF register high reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn AFRH_RES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AF register high reset value"]
    #[inline(always)]
    pub const fn set_AFRH_RES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR1 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR1 {
        GPIO_HWCFGR1(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR1")
            .field("AFRH_RES", &self.AFRH_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_HWCFGR1 {{ AFRH_RES: {=u32:?} }}", self.AFRH_RES())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR10(pub u32);
impl GPIO_HWCFGR10 {
    #[doc = "Bus interface configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_IOP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bus interface configuration"]
    #[inline(always)]
    pub const fn set_AHB_IOP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of AF available for each I/O"]
    #[must_use]
    #[inline(always)]
    pub const fn AFSIZE_CFG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of AF available for each I/O"]
    #[inline(always)]
    pub const fn set_AFSIZE_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Number of speed lines for each I/O"]
    #[must_use]
    #[inline(always)]
    pub const fn SPEED_CFG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of speed lines for each I/O"]
    #[inline(always)]
    pub const fn set_SPEED_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Lock mechanism activation"]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_CFG(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Lock mechanism activation"]
    #[inline(always)]
    pub const fn set_LOCK_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Security activation"]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_CFG(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Security activation"]
    #[inline(always)]
    pub const fn set_SEC_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Option register configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn OR_CFG(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Option register configuration"]
    #[inline(always)]
    pub const fn set_OR_CFG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for GPIO_HWCFGR10 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR10 {
        GPIO_HWCFGR10(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR10")
            .field("AHB_IOP", &self.AHB_IOP())
            .field("AFSIZE_CFG", &self.AFSIZE_CFG())
            .field("SPEED_CFG", &self.SPEED_CFG())
            .field("LOCK_CFG", &self.LOCK_CFG())
            .field("SEC_CFG", &self.SEC_CFG())
            .field("OR_CFG", &self.OR_CFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR10 {{ AHB_IOP: {=u8:?}, AFSIZE_CFG: {=u8:?}, SPEED_CFG: {=u8:?}, LOCK_CFG: {=u8:?}, SEC_CFG: {=u8:?}, OR_CFG: {=u8:?} }}",
            self.AHB_IOP(),
            self.AFSIZE_CFG(),
            self.SPEED_CFG(),
            self.LOCK_CFG(),
            self.SEC_CFG(),
            self.OR_CFG()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR2(pub u32);
impl GPIO_HWCFGR2 {
    #[doc = "AF register low reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn AFRL_RES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AF register low reset value"]
    #[inline(always)]
    pub const fn set_AFRL_RES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR2 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR2 {
        GPIO_HWCFGR2(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR2")
            .field("AFRL_RES", &self.AFRL_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_HWCFGR2 {{ AFRL_RES: {=u32:?} }}", self.AFRL_RES())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR3(pub u32);
impl GPIO_HWCFGR3 {
    #[doc = "Output data register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn ODR_RES(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Output data register reset value"]
    #[inline(always)]
    pub const fn set_ODR_RES(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Output type register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn OTYPER_RES(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Output type register reset value"]
    #[inline(always)]
    pub const fn set_OTYPER_RES(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for GPIO_HWCFGR3 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR3 {
        GPIO_HWCFGR3(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR3")
            .field("ODR_RES", &self.ODR_RES())
            .field("OTYPER_RES", &self.OTYPER_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR3 {{ ODR_RES: {=u16:?}, OTYPER_RES: {=u16:?} }}",
            self.ODR_RES(),
            self.OTYPER_RES()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR4(pub u32);
impl GPIO_HWCFGR4 {
    #[doc = "OSPEED register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED_RES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "OSPEED register reset value"]
    #[inline(always)]
    pub const fn set_OSPEED_RES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR4 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR4 {
        GPIO_HWCFGR4(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR4")
            .field("OSPEED_RES", &self.OSPEED_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR4 {{ OSPEED_RES: {=u32:?} }}",
            self.OSPEED_RES()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR5(pub u32);
impl GPIO_HWCFGR5 {
    #[doc = "Pull-up / pull-down register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn PUPDR_RES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pull-up / pull-down register reset value"]
    #[inline(always)]
    pub const fn set_PUPDR_RES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR5 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR5 {
        GPIO_HWCFGR5(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR5")
            .field("PUPDR_RES", &self.PUPDR_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR5 {{ PUPDR_RES: {=u32:?} }}",
            self.PUPDR_RES()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR6(pub u32);
impl GPIO_HWCFGR6 {
    #[doc = "MODER register reset value"]
    #[must_use]
    #[inline(always)]
    pub const fn MODER_RES(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MODER register reset value"]
    #[inline(always)]
    pub const fn set_MODER_RES(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR6 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR6 {
        GPIO_HWCFGR6(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR6")
            .field("MODER_RES", &self.MODER_RES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR6 {{ MODER_RES: {=u32:?} }}",
            self.MODER_RES()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR7(pub u32);
impl GPIO_HWCFGR7 {
    #[doc = "Indicate the priority AF for I/O0 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O0 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicate the priority AF for I/O1 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O1 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicate the priority AF for I/O2 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O2 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicate the priority AF for I/O3 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O3 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicate the priority AF for I/O4 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O4 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicate the priority AF for I/O5 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O5 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicate the priority AF for I/O6 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O6 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Indicate the priority AF for I/O7 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O7 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_HWCFGR7 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR7 {
        GPIO_HWCFGR7(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR7")
            .field("AF_PRIO0", &self.AF_PRIO0())
            .field("AF_PRIO1", &self.AF_PRIO1())
            .field("AF_PRIO2", &self.AF_PRIO2())
            .field("AF_PRIO3", &self.AF_PRIO3())
            .field("AF_PRIO4", &self.AF_PRIO4())
            .field("AF_PRIO5", &self.AF_PRIO5())
            .field("AF_PRIO6", &self.AF_PRIO6())
            .field("AF_PRIO7", &self.AF_PRIO7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR7 {{ AF_PRIO0: {=u8:?}, AF_PRIO1: {=u8:?}, AF_PRIO2: {=u8:?}, AF_PRIO3: {=u8:?}, AF_PRIO4: {=u8:?}, AF_PRIO5: {=u8:?}, AF_PRIO6: {=u8:?}, AF_PRIO7: {=u8:?} }}",
            self.AF_PRIO0(),
            self.AF_PRIO1(),
            self.AF_PRIO2(),
            self.AF_PRIO3(),
            self.AF_PRIO4(),
            self.AF_PRIO5(),
            self.AF_PRIO6(),
            self.AF_PRIO7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR8(pub u32);
impl GPIO_HWCFGR8 {
    #[doc = "Indicate the priority AF for I/O8 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O8 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicate the priority AF for I/O9 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O9 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicate the priority AF for I/O10 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O10 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicate the priority AF for I/O11 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O11 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Indicate the priority AF for I/O12 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O12 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Indicate the priority AF for I/O13 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O13 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicate the priority AF for I/O14 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O14 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Indicate the priority AF for I/O15 (0 to F)"]
    #[must_use]
    #[inline(always)]
    pub const fn AF_PRIO15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the priority AF for I/O15 (0 to F)"]
    #[inline(always)]
    pub const fn set_AF_PRIO15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPIO_HWCFGR8 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR8 {
        GPIO_HWCFGR8(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR8")
            .field("AF_PRIO8", &self.AF_PRIO8())
            .field("AF_PRIO9", &self.AF_PRIO9())
            .field("AF_PRIO10", &self.AF_PRIO10())
            .field("AF_PRIO11", &self.AF_PRIO11())
            .field("AF_PRIO12", &self.AF_PRIO12())
            .field("AF_PRIO13", &self.AF_PRIO13())
            .field("AF_PRIO14", &self.AF_PRIO14())
            .field("AF_PRIO15", &self.AF_PRIO15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_HWCFGR8 {{ AF_PRIO8: {=u8:?}, AF_PRIO9: {=u8:?}, AF_PRIO10: {=u8:?}, AF_PRIO11: {=u8:?}, AF_PRIO12: {=u8:?}, AF_PRIO13: {=u8:?}, AF_PRIO14: {=u8:?}, AF_PRIO15: {=u8:?} }}",
            self.AF_PRIO8(),
            self.AF_PRIO9(),
            self.AF_PRIO10(),
            self.AF_PRIO11(),
            self.AF_PRIO12(),
            self.AF_PRIO13(),
            self.AF_PRIO14(),
            self.AF_PRIO15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_HWCFGR9(pub u32);
impl GPIO_HWCFGR9 {
    #[doc = "Presence granularity, each bit indicate the presence of the IO"]
    #[must_use]
    #[inline(always)]
    pub const fn EN_IO(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Presence granularity, each bit indicate the presence of the IO"]
    #[inline(always)]
    pub const fn set_EN_IO(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for GPIO_HWCFGR9 {
    #[inline(always)]
    fn default() -> GPIO_HWCFGR9 {
        GPIO_HWCFGR9(0)
    }
}
impl core::fmt::Debug for GPIO_HWCFGR9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HWCFGR9")
            .field("EN_IO", &self.EN_IO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_HWCFGR9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_HWCFGR9 {{ EN_IO: {=u16:?} }}", self.EN_IO())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_IDR(pub u32);
impl GPIO_IDR {
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ID15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x input data I/O pin y (y=0 to 15)These bits are read-only. They contain the input value of the corresponding I/O port.Successful read to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero."]
    #[inline(always)]
    pub const fn set_ID15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_IDR {
    #[inline(always)]
    fn default() -> GPIO_IDR {
        GPIO_IDR(0)
    }
}
impl core::fmt::Debug for GPIO_IDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IDR")
            .field("ID0", &self.ID0())
            .field("ID1", &self.ID1())
            .field("ID2", &self.ID2())
            .field("ID3", &self.ID3())
            .field("ID4", &self.ID4())
            .field("ID5", &self.ID5())
            .field("ID6", &self.ID6())
            .field("ID7", &self.ID7())
            .field("ID8", &self.ID8())
            .field("ID9", &self.ID9())
            .field("ID10", &self.ID10())
            .field("ID11", &self.ID11())
            .field("ID12", &self.ID12())
            .field("ID13", &self.ID13())
            .field("ID14", &self.ID14())
            .field("ID15", &self.ID15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_IDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_IDR {{ ID0: {=bool:?}, ID1: {=bool:?}, ID2: {=bool:?}, ID3: {=bool:?}, ID4: {=bool:?}, ID5: {=bool:?}, ID6: {=bool:?}, ID7: {=bool:?}, ID8: {=bool:?}, ID9: {=bool:?}, ID10: {=bool:?}, ID11: {=bool:?}, ID12: {=bool:?}, ID13: {=bool:?}, ID14: {=bool:?}, ID15: {=bool:?} }}",
            self.ID0(),
            self.ID1(),
            self.ID2(),
            self.ID3(),
            self.ID4(),
            self.ID5(),
            self.ID6(),
            self.ID7(),
            self.ID8(),
            self.ID9(),
            self.ID10(),
            self.ID11(),
            self.ID12(),
            self.ID13(),
            self.ID14(),
            self.ID15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_IPIDR(pub u32);
impl GPIO_IPIDR {
    #[doc = "IP identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn IPID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "IP identifier"]
    #[inline(always)]
    pub const fn set_IPID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_IPIDR {
    #[inline(always)]
    fn default() -> GPIO_IPIDR {
        GPIO_IPIDR(0)
    }
}
impl core::fmt::Debug for GPIO_IPIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IPIDR")
            .field("IPID", &self.IPID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_IPIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_IPIDR {{ IPID: {=u32:?} }}", self.IPID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_LCKR(pub u32);
impl GPIO_LCKR {
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn LCK15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x lock I/O pin y (y=0 to 15)These bits are read/write but can only be written when the LCKK bit is '0â\u{80}\u{99}.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_LCK15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Lock key"]
    #[must_use]
    #[inline(always)]
    pub const fn LCKK(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock key"]
    #[inline(always)]
    pub const fn set_LCKK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GPIO_LCKR {
    #[inline(always)]
    fn default() -> GPIO_LCKR {
        GPIO_LCKR(0)
    }
}
impl core::fmt::Debug for GPIO_LCKR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_LCKR")
            .field("LCK0", &self.LCK0())
            .field("LCK1", &self.LCK1())
            .field("LCK2", &self.LCK2())
            .field("LCK3", &self.LCK3())
            .field("LCK4", &self.LCK4())
            .field("LCK5", &self.LCK5())
            .field("LCK6", &self.LCK6())
            .field("LCK7", &self.LCK7())
            .field("LCK8", &self.LCK8())
            .field("LCK9", &self.LCK9())
            .field("LCK10", &self.LCK10())
            .field("LCK11", &self.LCK11())
            .field("LCK12", &self.LCK12())
            .field("LCK13", &self.LCK13())
            .field("LCK14", &self.LCK14())
            .field("LCK15", &self.LCK15())
            .field("LCKK", &self.LCKK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_LCKR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_LCKR {{ LCK0: {=bool:?}, LCK1: {=bool:?}, LCK2: {=bool:?}, LCK3: {=bool:?}, LCK4: {=bool:?}, LCK5: {=bool:?}, LCK6: {=bool:?}, LCK7: {=bool:?}, LCK8: {=bool:?}, LCK9: {=bool:?}, LCK10: {=bool:?}, LCK11: {=bool:?}, LCK12: {=bool:?}, LCK13: {=bool:?}, LCK14: {=bool:?}, LCK15: {=bool:?}, LCKK: {=bool:?} }}",
            self.LCK0(),
            self.LCK1(),
            self.LCK2(),
            self.LCK3(),
            self.LCK4(),
            self.LCK5(),
            self.LCK6(),
            self.LCK7(),
            self.LCK8(),
            self.LCK9(),
            self.LCK10(),
            self.LCK11(),
            self.LCK12(),
            self.LCK13(),
            self.LCK14(),
            self.LCK15(),
            self.LCKK()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_MODER(pub u32);
impl GPIO_MODER {
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O mode.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_MODE15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for GPIO_MODER {
    #[inline(always)]
    fn default() -> GPIO_MODER {
        GPIO_MODER(0)
    }
}
impl core::fmt::Debug for GPIO_MODER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_MODER")
            .field("MODE0", &self.MODE0())
            .field("MODE1", &self.MODE1())
            .field("MODE2", &self.MODE2())
            .field("MODE3", &self.MODE3())
            .field("MODE4", &self.MODE4())
            .field("MODE5", &self.MODE5())
            .field("MODE6", &self.MODE6())
            .field("MODE7", &self.MODE7())
            .field("MODE8", &self.MODE8())
            .field("MODE9", &self.MODE9())
            .field("MODE10", &self.MODE10())
            .field("MODE11", &self.MODE11())
            .field("MODE12", &self.MODE12())
            .field("MODE13", &self.MODE13())
            .field("MODE14", &self.MODE14())
            .field("MODE15", &self.MODE15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_MODER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_MODER {{ MODE0: {=u8:?}, MODE1: {=u8:?}, MODE2: {=u8:?}, MODE3: {=u8:?}, MODE4: {=u8:?}, MODE5: {=u8:?}, MODE6: {=u8:?}, MODE7: {=u8:?}, MODE8: {=u8:?}, MODE9: {=u8:?}, MODE10: {=u8:?}, MODE11: {=u8:?}, MODE12: {=u8:?}, MODE13: {=u8:?}, MODE14: {=u8:?}, MODE15: {=u8:?} }}",
            self.MODE0(),
            self.MODE1(),
            self.MODE2(),
            self.MODE3(),
            self.MODE4(),
            self.MODE5(),
            self.MODE6(),
            self.MODE7(),
            self.MODE8(),
            self.MODE9(),
            self.MODE10(),
            self.MODE11(),
            self.MODE12(),
            self.MODE13(),
            self.MODE14(),
            self.MODE15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_ODR(pub u32);
impl GPIO_ODR {
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OD15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x output data I/O pin y (y=0 to 15)These bits can be read and written by software.Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OD15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_ODR {
    #[inline(always)]
    fn default() -> GPIO_ODR {
        GPIO_ODR(0)
    }
}
impl core::fmt::Debug for GPIO_ODR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_ODR")
            .field("OD0", &self.OD0())
            .field("OD1", &self.OD1())
            .field("OD2", &self.OD2())
            .field("OD3", &self.OD3())
            .field("OD4", &self.OD4())
            .field("OD5", &self.OD5())
            .field("OD6", &self.OD6())
            .field("OD7", &self.OD7())
            .field("OD8", &self.OD8())
            .field("OD9", &self.OD9())
            .field("OD10", &self.OD10())
            .field("OD11", &self.OD11())
            .field("OD12", &self.OD12())
            .field("OD13", &self.OD13())
            .field("OD14", &self.OD14())
            .field("OD15", &self.OD15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_ODR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_ODR {{ OD0: {=bool:?}, OD1: {=bool:?}, OD2: {=bool:?}, OD3: {=bool:?}, OD4: {=bool:?}, OD5: {=bool:?}, OD6: {=bool:?}, OD7: {=bool:?}, OD8: {=bool:?}, OD9: {=bool:?}, OD10: {=bool:?}, OD11: {=bool:?}, OD12: {=bool:?}, OD13: {=bool:?}, OD14: {=bool:?}, OD15: {=bool:?} }}",
            self.OD0(),
            self.OD1(),
            self.OD2(),
            self.OD3(),
            self.OD4(),
            self.OD5(),
            self.OD6(),
            self.OD7(),
            self.OD8(),
            self.OD9(),
            self.OD10(),
            self.OD11(),
            self.OD12(),
            self.OD13(),
            self.OD14(),
            self.OD15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_OSPEEDR(pub u32);
impl GPIO_OSPEEDR {
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OSPEED15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output speed.Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OSPEED15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for GPIO_OSPEEDR {
    #[inline(always)]
    fn default() -> GPIO_OSPEEDR {
        GPIO_OSPEEDR(0)
    }
}
impl core::fmt::Debug for GPIO_OSPEEDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_OSPEEDR")
            .field("OSPEED0", &self.OSPEED0())
            .field("OSPEED1", &self.OSPEED1())
            .field("OSPEED2", &self.OSPEED2())
            .field("OSPEED3", &self.OSPEED3())
            .field("OSPEED4", &self.OSPEED4())
            .field("OSPEED5", &self.OSPEED5())
            .field("OSPEED6", &self.OSPEED6())
            .field("OSPEED7", &self.OSPEED7())
            .field("OSPEED8", &self.OSPEED8())
            .field("OSPEED9", &self.OSPEED9())
            .field("OSPEED10", &self.OSPEED10())
            .field("OSPEED11", &self.OSPEED11())
            .field("OSPEED12", &self.OSPEED12())
            .field("OSPEED13", &self.OSPEED13())
            .field("OSPEED14", &self.OSPEED14())
            .field("OSPEED15", &self.OSPEED15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_OSPEEDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_OSPEEDR {{ OSPEED0: {=u8:?}, OSPEED1: {=u8:?}, OSPEED2: {=u8:?}, OSPEED3: {=u8:?}, OSPEED4: {=u8:?}, OSPEED5: {=u8:?}, OSPEED6: {=u8:?}, OSPEED7: {=u8:?}, OSPEED8: {=u8:?}, OSPEED9: {=u8:?}, OSPEED10: {=u8:?}, OSPEED11: {=u8:?}, OSPEED12: {=u8:?}, OSPEED13: {=u8:?}, OSPEED14: {=u8:?}, OSPEED15: {=u8:?} }}",
            self.OSPEED0(),
            self.OSPEED1(),
            self.OSPEED2(),
            self.OSPEED3(),
            self.OSPEED4(),
            self.OSPEED5(),
            self.OSPEED6(),
            self.OSPEED7(),
            self.OSPEED8(),
            self.OSPEED9(),
            self.OSPEED10(),
            self.OSPEED11(),
            self.OSPEED12(),
            self.OSPEED13(),
            self.OSPEED14(),
            self.OSPEED15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_OTYPER(pub u32);
impl GPIO_OTYPER {
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn OT15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O output type.Successful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_OT15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_OTYPER {
    #[inline(always)]
    fn default() -> GPIO_OTYPER {
        GPIO_OTYPER(0)
    }
}
impl core::fmt::Debug for GPIO_OTYPER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_OTYPER")
            .field("OT0", &self.OT0())
            .field("OT1", &self.OT1())
            .field("OT2", &self.OT2())
            .field("OT3", &self.OT3())
            .field("OT4", &self.OT4())
            .field("OT5", &self.OT5())
            .field("OT6", &self.OT6())
            .field("OT7", &self.OT7())
            .field("OT8", &self.OT8())
            .field("OT9", &self.OT9())
            .field("OT10", &self.OT10())
            .field("OT11", &self.OT11())
            .field("OT12", &self.OT12())
            .field("OT13", &self.OT13())
            .field("OT14", &self.OT14())
            .field("OT15", &self.OT15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_OTYPER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_OTYPER {{ OT0: {=bool:?}, OT1: {=bool:?}, OT2: {=bool:?}, OT3: {=bool:?}, OT4: {=bool:?}, OT5: {=bool:?}, OT6: {=bool:?}, OT7: {=bool:?}, OT8: {=bool:?}, OT9: {=bool:?}, OT10: {=bool:?}, OT11: {=bool:?}, OT12: {=bool:?}, OT13: {=bool:?}, OT14: {=bool:?}, OT15: {=bool:?} }}",
            self.OT0(),
            self.OT1(),
            self.OT2(),
            self.OT3(),
            self.OT4(),
            self.OT5(),
            self.OT6(),
            self.OT7(),
            self.OT8(),
            self.OT9(),
            self.OT10(),
            self.OT11(),
            self.OT12(),
            self.OT13(),
            self.OT14(),
            self.OT15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_PRIVCFGR(pub u32);
impl GPIO_PRIVCFGR {
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIV15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x privileged I/O pin y (y= 0 to 15)These bits are written by software to enabled the privileged mode for I/O port pin y.This bit can only be written by privileged application. If corresponding SECy bit is set in GPIOx_SECCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit in GPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_PRIV15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_PRIVCFGR {
    #[inline(always)]
    fn default() -> GPIO_PRIVCFGR {
        GPIO_PRIVCFGR(0)
    }
}
impl core::fmt::Debug for GPIO_PRIVCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PRIVCFGR")
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
impl defmt::Format for GPIO_PRIVCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_PRIVCFGR {{ PRIV0: {=bool:?}, PRIV1: {=bool:?}, PRIV2: {=bool:?}, PRIV3: {=bool:?}, PRIV4: {=bool:?}, PRIV5: {=bool:?}, PRIV6: {=bool:?}, PRIV7: {=bool:?}, PRIV8: {=bool:?}, PRIV9: {=bool:?}, PRIV10: {=bool:?}, PRIV11: {=bool:?}, PRIV12: {=bool:?}, PRIV13: {=bool:?}, PRIV14: {=bool:?}, PRIV15: {=bool:?} }}",
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
pub struct GPIO_PUPDR(pub u32);
impl GPIO_PUPDR {
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn PUPD15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "1:0\\]: Port x configuration I/O pin y (y=0 to 15)These bits are written by software to configure the I/O pull-up or pull-downSuccessful read or write to this bit must be secure if SECy bit is set in GPIOx_SECCFGR, and must be privileged if PRIVy bit is set in GPIOx_PRIVCFGR.Only authorized CPU can read or write this bit, as described in Section1.4: IO pin allocation to compartments. Illegal read returns zero. Illegal write is ignored."]
    #[inline(always)]
    pub const fn set_PUPD15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for GPIO_PUPDR {
    #[inline(always)]
    fn default() -> GPIO_PUPDR {
        GPIO_PUPDR(0)
    }
}
impl core::fmt::Debug for GPIO_PUPDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PUPDR")
            .field("PUPD0", &self.PUPD0())
            .field("PUPD1", &self.PUPD1())
            .field("PUPD2", &self.PUPD2())
            .field("PUPD3", &self.PUPD3())
            .field("PUPD4", &self.PUPD4())
            .field("PUPD5", &self.PUPD5())
            .field("PUPD6", &self.PUPD6())
            .field("PUPD7", &self.PUPD7())
            .field("PUPD8", &self.PUPD8())
            .field("PUPD9", &self.PUPD9())
            .field("PUPD10", &self.PUPD10())
            .field("PUPD11", &self.PUPD11())
            .field("PUPD12", &self.PUPD12())
            .field("PUPD13", &self.PUPD13())
            .field("PUPD14", &self.PUPD14())
            .field("PUPD15", &self.PUPD15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_PUPDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_PUPDR {{ PUPD0: {=u8:?}, PUPD1: {=u8:?}, PUPD2: {=u8:?}, PUPD3: {=u8:?}, PUPD4: {=u8:?}, PUPD5: {=u8:?}, PUPD6: {=u8:?}, PUPD7: {=u8:?}, PUPD8: {=u8:?}, PUPD9: {=u8:?}, PUPD10: {=u8:?}, PUPD11: {=u8:?}, PUPD12: {=u8:?}, PUPD13: {=u8:?}, PUPD14: {=u8:?}, PUPD15: {=u8:?} }}",
            self.PUPD0(),
            self.PUPD1(),
            self.PUPD2(),
            self.PUPD3(),
            self.PUPD4(),
            self.PUPD5(),
            self.PUPD6(),
            self.PUPD7(),
            self.PUPD8(),
            self.PUPD9(),
            self.PUPD10(),
            self.PUPD11(),
            self.PUPD12(),
            self.PUPD13(),
            self.PUPD14(),
            self.PUPD15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_RCFGLOCKR(pub u32);
impl GPIO_RCFGLOCKR {
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn RLOCK15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x security/privilege configuration lock for I/O pin y (y= 0 to 15)This bit can only be written by secure privileged application. This bit is cleared by default andonce set it cannot be reset until global GPIOx reset.If compartment filtering for this I/O pin is enabled (resp. disabled) using CFEN bit inGPIOx_CIDCFGRy, only an authorized compartment (resp. any compartment) can write thisbit, on top of secure and privileged restriction. Authorized compartment value depends on thevalue of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit inGPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_RLOCK15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_RCFGLOCKR {
    #[inline(always)]
    fn default() -> GPIO_RCFGLOCKR {
        GPIO_RCFGLOCKR(0)
    }
}
impl core::fmt::Debug for GPIO_RCFGLOCKR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_RCFGLOCKR")
            .field("RLOCK0", &self.RLOCK0())
            .field("RLOCK1", &self.RLOCK1())
            .field("RLOCK2", &self.RLOCK2())
            .field("RLOCK3", &self.RLOCK3())
            .field("RLOCK4", &self.RLOCK4())
            .field("RLOCK5", &self.RLOCK5())
            .field("RLOCK6", &self.RLOCK6())
            .field("RLOCK7", &self.RLOCK7())
            .field("RLOCK8", &self.RLOCK8())
            .field("RLOCK9", &self.RLOCK9())
            .field("RLOCK10", &self.RLOCK10())
            .field("RLOCK11", &self.RLOCK11())
            .field("RLOCK12", &self.RLOCK12())
            .field("RLOCK13", &self.RLOCK13())
            .field("RLOCK14", &self.RLOCK14())
            .field("RLOCK15", &self.RLOCK15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_RCFGLOCKR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_RCFGLOCKR {{ RLOCK0: {=bool:?}, RLOCK1: {=bool:?}, RLOCK2: {=bool:?}, RLOCK3: {=bool:?}, RLOCK4: {=bool:?}, RLOCK5: {=bool:?}, RLOCK6: {=bool:?}, RLOCK7: {=bool:?}, RLOCK8: {=bool:?}, RLOCK9: {=bool:?}, RLOCK10: {=bool:?}, RLOCK11: {=bool:?}, RLOCK12: {=bool:?}, RLOCK13: {=bool:?}, RLOCK14: {=bool:?}, RLOCK15: {=bool:?} }}",
            self.RLOCK0(),
            self.RLOCK1(),
            self.RLOCK2(),
            self.RLOCK3(),
            self.RLOCK4(),
            self.RLOCK5(),
            self.RLOCK6(),
            self.RLOCK7(),
            self.RLOCK8(),
            self.RLOCK9(),
            self.RLOCK10(),
            self.RLOCK11(),
            self.RLOCK12(),
            self.RLOCK13(),
            self.RLOCK14(),
            self.RLOCK15()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SECCFGR(pub u32);
impl GPIO_SECCFGR {
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Port x secure I/O pin y (y= 0 to 15)These bits are written by software to enabled the secure mode for I/O port pin y.If the system is not secure (TZEN=0) write to this bit is ignored.This bit can only be written by secure application. If corresponding PRIVy bit is set in GPIOx_PRIVCFGR this bit can only be written by secure privileged application.If compartment filtering for this I/O pin is enabled by setting CFEN bit in GPIOx_CIDCFGRy, only an allowed compartment can write this bit, on top of secure and privileged restriction. Authorized compartment value depends on the value of SEM_EN bit in GPIOx_CIDCFGRy and optionally the SEM_MUTEX bit in GPIOx_SEMCRy. See the bits descriptions for details."]
    #[inline(always)]
    pub const fn set_SEC15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for GPIO_SECCFGR {
    #[inline(always)]
    fn default() -> GPIO_SECCFGR {
        GPIO_SECCFGR(0)
    }
}
impl core::fmt::Debug for GPIO_SECCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SECCFGR")
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
impl defmt::Format for GPIO_SECCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SECCFGR {{ SEC0: {=bool:?}, SEC1: {=bool:?}, SEC2: {=bool:?}, SEC3: {=bool:?}, SEC4: {=bool:?}, SEC5: {=bool:?}, SEC6: {=bool:?}, SEC7: {=bool:?}, SEC8: {=bool:?}, SEC9: {=bool:?}, SEC10: {=bool:?}, SEC11: {=bool:?}, SEC12: {=bool:?}, SEC13: {=bool:?}, SEC14: {=bool:?}, SEC15: {=bool:?} }}",
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
pub struct GPIO_SEMCR0(pub u32);
impl GPIO_SEMCR0 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR0 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR0 {
        GPIO_SEMCR0(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR0")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR0 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR1(pub u32);
impl GPIO_SEMCR1 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR1 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR1 {
        GPIO_SEMCR1(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR1")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR1 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR10(pub u32);
impl GPIO_SEMCR10 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR10 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR10 {
        GPIO_SEMCR10(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR10")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR10 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR11(pub u32);
impl GPIO_SEMCR11 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR11 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR11 {
        GPIO_SEMCR11(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR11")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR11 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR12(pub u32);
impl GPIO_SEMCR12 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR12 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR12 {
        GPIO_SEMCR12(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR12")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR12 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR13(pub u32);
impl GPIO_SEMCR13 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR13 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR13 {
        GPIO_SEMCR13(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR13")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR13 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR14(pub u32);
impl GPIO_SEMCR14 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR14 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR14 {
        GPIO_SEMCR14(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR14")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR14 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR15(pub u32);
impl GPIO_SEMCR15 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR15 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR15 {
        GPIO_SEMCR15(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR15")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR15 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR2(pub u32);
impl GPIO_SEMCR2 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR2 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR2 {
        GPIO_SEMCR2(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR2")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR2 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR3(pub u32);
impl GPIO_SEMCR3 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR3 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR3 {
        GPIO_SEMCR3(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR3")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR3 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR4(pub u32);
impl GPIO_SEMCR4 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR4 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR4 {
        GPIO_SEMCR4(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR4")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR4 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR5(pub u32);
impl GPIO_SEMCR5 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR5 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR5 {
        GPIO_SEMCR5(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR5")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR5 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR6(pub u32);
impl GPIO_SEMCR6 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR6 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR6 {
        GPIO_SEMCR6(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR6")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR6 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR7(pub u32);
impl GPIO_SEMCR7 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR7 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR7 {
        GPIO_SEMCR7(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR7")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR7 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR8(pub u32);
impl GPIO_SEMCR8 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR8 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR8 {
        GPIO_SEMCR8(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR8")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR8 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SEMCR9(pub u32);
impl GPIO_SEMCR9 {
    #[doc = "Semaphore Mutex"]
    #[must_use]
    #[inline(always)]
    pub const fn SEM_MUTEX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Semaphore Mutex"]
    #[inline(always)]
    pub const fn set_SEM_MUTEX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Semaphore current CID"]
    #[must_use]
    #[inline(always)]
    pub const fn SEMCID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Semaphore current CID"]
    #[inline(always)]
    pub const fn set_SEMCID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GPIO_SEMCR9 {
    #[inline(always)]
    fn default() -> GPIO_SEMCR9 {
        GPIO_SEMCR9(0)
    }
}
impl core::fmt::Debug for GPIO_SEMCR9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SEMCR9")
            .field("SEM_MUTEX", &self.SEM_MUTEX())
            .field("SEMCID", &self.SEMCID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SEMCR9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_SEMCR9 {{ SEM_MUTEX: {=bool:?}, SEMCID: {=u8:?} }}",
            self.SEM_MUTEX(),
            self.SEMCID()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_SIDR(pub u32);
impl GPIO_SIDR {
    #[doc = "Size identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size identifier"]
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GPIO_SIDR {
    #[inline(always)]
    fn default() -> GPIO_SIDR {
        GPIO_SIDR(0)
    }
}
impl core::fmt::Debug for GPIO_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPIO_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO_VERR(pub u32);
impl GPIO_VERR {
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
impl Default for GPIO_VERR {
    #[inline(always)]
    fn default() -> GPIO_VERR {
        GPIO_VERR(0)
    }
}
impl core::fmt::Debug for GPIO_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPIO_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPIO_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC {
    ptr: *mut u8,
}
unsafe impl Send for RCC {}
unsafe impl Sync for RCC {}
impl RCC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RCC Secure Configuration Register x (x=0 to 3)"]
    #[inline(always)]
    pub const fn SECCFGR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "RCC Privileged Configuration Register x (x=0 to 3)"]
    #[inline(always)]
    pub const fn PRIVCFGR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "RCC Resource Configuration Lock Register x (x=0 to 3)"]
    #[inline(always)]
    pub const fn RCFGLOCKR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "RCC Global Reset Control Set Register"]
    #[inline(always)]
    pub const fn GRSTCSETR(self) -> crate::common::Reg<RCC_GRSTCSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "RCC CPU1 Reset Control Set Register"]
    #[inline(always)]
    pub const fn C1RSTCSETR(self) -> crate::common::Reg<RCC_C1RSTCSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "RCC CPU1 Processor 1 Reset Control Set Register"]
    #[inline(always)]
    pub const fn C1P1RSTCSETR(self) -> crate::common::Reg<RCC_C1P1RSTCSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "RCC CPU2 Reset Control Set Register"]
    #[inline(always)]
    pub const fn C2RSTCSETR(self) -> crate::common::Reg<RCC_C2RSTCSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "RCC Hardware Reset Status Clear Register"]
    #[inline(always)]
    pub const fn HWRSTSCLRR(self) -> crate::common::Reg<RCC_HWRSTSCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "RCC CPU1 Hardware Reset Status Clear Register"]
    #[inline(always)]
    pub const fn C1HWRSTSCLRR(self) -> crate::common::Reg<RCC_C1HWRSTSCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "RCC CPU2 Hardware Reset Status Clear Register"]
    #[inline(always)]
    pub const fn C2HWRSTSCLRR(self) -> crate::common::Reg<RCC_C2HWRSTSCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "RCC CPU1 BOOT Reset Status Set Register"]
    #[inline(always)]
    pub const fn C1BOOTRSTSSETR(self) -> crate::common::Reg<RCC_C1BOOTRSTSSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "RCC CPU1 BOOT Reset Status Clear Register"]
    #[inline(always)]
    pub const fn C1BOOTRSTSCLRR(self) -> crate::common::Reg<RCC_C1BOOTRSTSCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "RCC CPU2 BOOT Reset Status Set Register"]
    #[inline(always)]
    pub const fn C2BOOTRSTSSETR(self) -> crate::common::Reg<RCC_C2BOOTRSTSSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "RCC CPU2 BOOT Reset Status Clear Register"]
    #[inline(always)]
    pub const fn C2BOOTRSTSCLRR(self) -> crate::common::Reg<RCC_C2BOOTRSTSCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "RCC CPU1 Stop Request Set Register"]
    #[inline(always)]
    pub const fn C1SREQSETR(self) -> crate::common::Reg<RCC_C1SREQSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "RCC CPU1 Stop Request Clear Register"]
    #[inline(always)]
    pub const fn C1SREQCLRR(self) -> crate::common::Reg<RCC_C1SREQCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "RCC CPU Boot Control Register"]
    #[inline(always)]
    pub const fn CPUBOOTCR(self) -> crate::common::Reg<RCC_CPUBOOTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "RCC Standby Boot Control Register"]
    #[inline(always)]
    pub const fn STBYBOOTCR(self) -> crate::common::Reg<RCC_STBYBOOTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "RCC Legacy Boot Control Register"]
    #[inline(always)]
    pub const fn LEGBOOTCR(self) -> crate::common::Reg<RCC_LEGBOOTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn BDCR(self) -> crate::common::Reg<RCC_BDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "RCC SmartRun Domain Control Register"]
    #[inline(always)]
    pub const fn D3DCR(self) -> crate::common::Reg<RCC_D3DCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "RCC SmartRun Domain Status Register"]
    #[inline(always)]
    pub const fn D3DSR(self) -> crate::common::Reg<RCC_D3DSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "RCC Reset Duration Control Register"]
    #[inline(always)]
    pub const fn RDCR(self) -> crate::common::Reg<RCC_RDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "RCC C1MS Reset Delay Control Register"]
    #[inline(always)]
    pub const fn C1MSRDCR(self) -> crate::common::Reg<RCC_C1MSRDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "RCC PWR_LP Delay Control Register"]
    #[inline(always)]
    pub const fn PWRLPDLYCR(self) -> crate::common::Reg<RCC_PWRLPDLYCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "RCC CPU1 Clock Source Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn C1CIESETR(self) -> crate::common::Reg<RCC_C1CIESETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "RCC CPU1 Clock Source Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn C1CIFCLRR(self) -> crate::common::Reg<RCC_C1CIFCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "RCC CPU2 Clock Source Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn C2CIESETR(self) -> crate::common::Reg<RCC_C2CIESETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "RCC CPU2 Clock Source Interrupt Flag Clear Register"]
    #[inline(always)]
    pub const fn C2CIFCLRR(self) -> crate::common::Reg<RCC_C2CIFCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "RCC IWDG CPU1 Clock Freeze Set Register"]
    #[inline(always)]
    pub const fn IWDGC1FZSETR(self) -> crate::common::Reg<RCC_IWDGC1FZSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "RCC IWDG CPU1 Clock Freeze Clear Register"]
    #[inline(always)]
    pub const fn IWDGC1FZCLRR(self) -> crate::common::Reg<RCC_IWDGC1FZCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "RCC IWDG CPU1 Configuration Set Register"]
    #[inline(always)]
    pub const fn IWDGC1CFGSETR(self) -> crate::common::Reg<RCC_IWDGC1CFGSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "RCC IWDG CPU1 Configuration Clear Register"]
    #[inline(always)]
    pub const fn IWDGC1CFGCLRR(self) -> crate::common::Reg<RCC_IWDGC1CFGCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "RCC IWDG CPU2 Clock Freeze Set Register"]
    #[inline(always)]
    pub const fn IWDGC2FZSETR(self) -> crate::common::Reg<RCC_IWDGC2FZSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "RCC IWDG CPU2 Clock Freeze Clear Register"]
    #[inline(always)]
    pub const fn IWDGC2FZCLRR(self) -> crate::common::Reg<RCC_IWDGC2FZCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "RCC IWDG CPU2 Configuration Set Register"]
    #[inline(always)]
    pub const fn IWDGC2CFGSETR(self) -> crate::common::Reg<RCC_IWDGC2CFGSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "RCC IWDG CPU2 Configuration Clear Register"]
    #[inline(always)]
    pub const fn IWDGC2CFGCLRR(self) -> crate::common::Reg<RCC_IWDGC2CFGCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "RCC IWDG CPU3 Configuration Set Register"]
    #[inline(always)]
    pub const fn IWDGC3CFGSETR(self) -> crate::common::Reg<RCC_IWDGC3CFGSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "RCC IWDG CPU3 Configuration Clear Register"]
    #[inline(always)]
    pub const fn IWDGC3CFGCLRR(self) -> crate::common::Reg<RCC_IWDGC3CFGCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[doc = "RCC CPU3 Configuration Register"]
    #[inline(always)]
    pub const fn C3CFGR(self) -> crate::common::Reg<RCC_C3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "RCC MCO1 Configuration Register"]
    #[inline(always)]
    pub const fn MCO1CFGR(self) -> crate::common::Reg<RCC_MCO1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "RCC MCO2 Configuration Register"]
    #[inline(always)]
    pub const fn MCO2CFGR(self) -> crate::common::Reg<RCC_MCO2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "RCC Oscillator Clock Enable Set Register"]
    #[inline(always)]
    pub const fn OCENSETR(self) -> crate::common::Reg<RCC_OCENSETR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "RCC Oscillator Clock Enable Clear Register"]
    #[inline(always)]
    pub const fn OCENCLRR(self) -> crate::common::Reg<RCC_OCENCLRR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "RCC Oscillator Clock Ready Register"]
    #[inline(always)]
    pub const fn OCRDYR(self) -> crate::common::Reg<RCC_OCRDYR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "RCC HSI Configuration Register"]
    #[inline(always)]
    pub const fn HSICFGR(self) -> crate::common::Reg<RCC_HSICFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "RCC MSI Configuration Register"]
    #[inline(always)]
    pub const fn MSICFGR(self) -> crate::common::Reg<RCC_MSICFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04acusize) as _) }
    }
    #[doc = "RCC RTC Clock Divider Register"]
    #[inline(always)]
    pub const fn RTCDIVR(self) -> crate::common::Reg<RCC_RTCDIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "RCC APB1 Clock Divider Register"]
    #[inline(always)]
    pub const fn APB1DIVR(self) -> crate::common::Reg<RCC_APB1DIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "RCC APB2 Clock Divider Register"]
    #[inline(always)]
    pub const fn APB2DIVR(self) -> crate::common::Reg<RCC_APB2DIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "RCC APB3 Clock Divider Register"]
    #[inline(always)]
    pub const fn APB3DIVR(self) -> crate::common::Reg<RCC_APB3DIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "RCC APB4 Clock Divider Register"]
    #[inline(always)]
    pub const fn APB4DIVR(self) -> crate::common::Reg<RCC_APB4DIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "RCC APBDBG Clock Divider Register"]
    #[inline(always)]
    pub const fn APBDBGDIVR(self) -> crate::common::Reg<RCC_APBDBGDIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "RCC TIM Group 1 Prescaler Register"]
    #[inline(always)]
    pub const fn TIMG1PRER(self) -> crate::common::Reg<RCC_TIMG1PRER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "RCC TIM Group 2 Prescaler Register"]
    #[inline(always)]
    pub const fn TIMG2PRER(self) -> crate::common::Reg<RCC_TIMG2PRER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "RCC LSMCU Clock Divider Register"]
    #[inline(always)]
    pub const fn LSMCUDIVR(self) -> crate::common::Reg<RCC_LSMCUDIVR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "RCC DDRC AXI Ports Configuration Register"]
    #[inline(always)]
    pub const fn DDRCPCFGR(self) -> crate::common::Reg<RCC_DDRCPCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "RCC DDRC APB Configuration Register"]
    #[inline(always)]
    pub const fn DDRCAPBCFGR(self) -> crate::common::Reg<RCC_DDRCAPBCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "RCC DDRPHYC APB Configuration Register"]
    #[inline(always)]
    pub const fn DDRPHYCAPBCFGR(self) -> crate::common::Reg<RCC_DDRPHYCAPBCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "RCC DDRPHYC Configuration Register"]
    #[inline(always)]
    pub const fn DDRPHYCCFGR(self) -> crate::common::Reg<RCC_DDRPHYCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "RCC DDR Configuration Register"]
    #[inline(always)]
    pub const fn DDRCFGR(self) -> crate::common::Reg<RCC_DDRCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "RCC DDR Interface Configuration Register"]
    #[inline(always)]
    pub const fn DDRITFCFGR(self) -> crate::common::Reg<RCC_DDRITFCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "RCC SYSRAM Configuration Register"]
    #[inline(always)]
    pub const fn SYSRAMCFGR(self) -> crate::common::Reg<RCC_SYSRAMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "RCC VDERAM Configuration Register"]
    #[inline(always)]
    pub const fn VDERAMCFGR(self) -> crate::common::Reg<RCC_VDERAMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "RCC SRAM1 Configuration Register"]
    #[inline(always)]
    pub const fn SRAM1CFGR(self) -> crate::common::Reg<RCC_SRAM1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "RCC SRAM2 Configuration Register"]
    #[inline(always)]
    pub const fn SRAM2CFGR(self) -> crate::common::Reg<RCC_SRAM2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04fcusize) as _) }
    }
    #[doc = "RCC RETRAM Configuration Register"]
    #[inline(always)]
    pub const fn RETRAMCFGR(self) -> crate::common::Reg<RCC_RETRAMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "RCC BKPSRAM Configuration Register"]
    #[inline(always)]
    pub const fn BKPSRAMCFGR(self) -> crate::common::Reg<RCC_BKPSRAMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "RCC LPSRAM1 Configuration Register"]
    #[inline(always)]
    pub const fn LPSRAM1CFGR(self) -> crate::common::Reg<RCC_LPSRAM1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "RCC LPSRAM2 Configuration Register"]
    #[inline(always)]
    pub const fn LPSRAM2CFGR(self) -> crate::common::Reg<RCC_LPSRAM2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "RCC LPSRAM3 Configuration Register"]
    #[inline(always)]
    pub const fn LPSRAM3CFGR(self) -> crate::common::Reg<RCC_LPSRAM3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "RCC OSPI1 Configuration Register"]
    #[inline(always)]
    pub const fn OSPI1CFGR(self) -> crate::common::Reg<RCC_OSPI1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "RCC OSPI2 Configuration Register"]
    #[inline(always)]
    pub const fn OSPI2CFGR(self) -> crate::common::Reg<RCC_OSPI2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "RCC FMC Configuration Register"]
    #[inline(always)]
    pub const fn FMCCFGR(self) -> crate::common::Reg<RCC_FMCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "RCC Debug Configuration Register"]
    #[inline(always)]
    pub const fn DBGCFGR(self) -> crate::common::Reg<RCC_DBGCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "RCC STM Configuration Register"]
    #[inline(always)]
    pub const fn STMCFGR(self) -> crate::common::Reg<RCC_STMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "RCC ETR Configuration Register"]
    #[inline(always)]
    pub const fn ETRCFGR(self) -> crate::common::Reg<RCC_ETRCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "RCC GPIOA Configuration Register"]
    #[inline(always)]
    pub const fn GPIOACFGR(self) -> crate::common::Reg<RCC_GPIOACFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "RCC GPIOB Configuration Register"]
    #[inline(always)]
    pub const fn GPIOBCFGR(self) -> crate::common::Reg<RCC_GPIOBCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "RCC GPIOC Configuration Register"]
    #[inline(always)]
    pub const fn GPIOCCFGR(self) -> crate::common::Reg<RCC_GPIOCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "RCC GPIOD Configuration Register"]
    #[inline(always)]
    pub const fn GPIODCFGR(self) -> crate::common::Reg<RCC_GPIODCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "RCC GPIOE Configuration Register"]
    #[inline(always)]
    pub const fn GPIOECFGR(self) -> crate::common::Reg<RCC_GPIOECFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "RCC GPIOF Configuration Register"]
    #[inline(always)]
    pub const fn GPIOFCFGR(self) -> crate::common::Reg<RCC_GPIOFCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "RCC GPIOG Configuration Register"]
    #[inline(always)]
    pub const fn GPIOGCFGR(self) -> crate::common::Reg<RCC_GPIOGCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "RCC GPIOH Configuration Register"]
    #[inline(always)]
    pub const fn GPIOHCFGR(self) -> crate::common::Reg<RCC_GPIOHCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "RCC GPIOI Configuration Register"]
    #[inline(always)]
    pub const fn GPIOICFGR(self) -> crate::common::Reg<RCC_GPIOICFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "RCC GPIOJ Configuration Register"]
    #[inline(always)]
    pub const fn GPIOJCFGR(self) -> crate::common::Reg<RCC_GPIOJCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "RCC GPIOK Configuration Register"]
    #[inline(always)]
    pub const fn GPIOKCFGR(self) -> crate::common::Reg<RCC_GPIOKCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "RCC GPIOZ Configuration Register"]
    #[inline(always)]
    pub const fn GPIOZCFGR(self) -> crate::common::Reg<RCC_GPIOZCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "RCC HPDMA1 Configuration Register"]
    #[inline(always)]
    pub const fn HPDMA1CFGR(self) -> crate::common::Reg<RCC_HPDMA1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "RCC HPDMA2 Configuration Register"]
    #[inline(always)]
    pub const fn HPDMA2CFGR(self) -> crate::common::Reg<RCC_HPDMA2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "RCC HPDMA3 Configuration Register"]
    #[inline(always)]
    pub const fn HPDMA3CFGR(self) -> crate::common::Reg<RCC_HPDMA3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "RCC LPDMA Configuration Register"]
    #[inline(always)]
    pub const fn LPDMACFGR(self) -> crate::common::Reg<RCC_LPDMACFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "RCC HSEM Configuration Register"]
    #[inline(always)]
    pub const fn HSEMCFGR(self) -> crate::common::Reg<RCC_HSEMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "RCC IPCC1 Configuration Register"]
    #[inline(always)]
    pub const fn IPCC1CFGR(self) -> crate::common::Reg<RCC_IPCC1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "RCC IPCC2 Configuration Register"]
    #[inline(always)]
    pub const fn IPCC2CFGR(self) -> crate::common::Reg<RCC_IPCC2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "RCC RTC Configuration Register"]
    #[inline(always)]
    pub const fn RTCCFGR(self) -> crate::common::Reg<RCC_RTCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "RCC SYSCPU1 Configuration Register"]
    #[inline(always)]
    pub const fn SYSCPU1CFGR(self) -> crate::common::Reg<RCC_SYSCPU1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "RCC BSEC Configuration Register"]
    #[inline(always)]
    pub const fn BSECCFGR(self) -> crate::common::Reg<RCC_BSECCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL2CFGR1(self) -> crate::common::Reg<RCC_PLL2CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL2CFGR2(self) -> crate::common::Reg<RCC_PLL2CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL2CFGR3(self) -> crate::common::Reg<RCC_PLL2CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL2CFGR4(self) -> crate::common::Reg<RCC_PLL2CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL2CFGR5(self) -> crate::common::Reg<RCC_PLL2CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL2CFGR6(self) -> crate::common::Reg<RCC_PLL2CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL2CFGR7(self) -> crate::common::Reg<RCC_PLL2CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL3CFGR1(self) -> crate::common::Reg<RCC_PLL3CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL3CFGR2(self) -> crate::common::Reg<RCC_PLL3CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL3CFGR3(self) -> crate::common::Reg<RCC_PLL3CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL3CFGR4(self) -> crate::common::Reg<RCC_PLL3CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL3CFGR5(self) -> crate::common::Reg<RCC_PLL3CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL3CFGR6(self) -> crate::common::Reg<RCC_PLL3CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "RCC PLL{x} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL3CFGR7(self) -> crate::common::Reg<RCC_PLL3CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "RCC HSI Frequency Monitoring Control Register"]
    #[inline(always)]
    pub const fn HSIFMONCR(self) -> crate::common::Reg<RCC_HSIFMONCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "RCC HSI Frequency Value Register"]
    #[inline(always)]
    pub const fn HSIFVALR(self) -> crate::common::Reg<RCC_HSIFVALR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "RCC TIM1 Configuration Register"]
    #[inline(always)]
    pub const fn TIM1CFGR(self) -> crate::common::Reg<RCC_TIM1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "RCC TIM2 Configuration Register"]
    #[inline(always)]
    pub const fn TIM2CFGR(self) -> crate::common::Reg<RCC_TIM2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "RCC TIM3 Configuration Register"]
    #[inline(always)]
    pub const fn TIM3CFGR(self) -> crate::common::Reg<RCC_TIM3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "RCC TIM4 Configuration Register"]
    #[inline(always)]
    pub const fn TIM4CFGR(self) -> crate::common::Reg<RCC_TIM4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "RCC TIM5 Configuration Register"]
    #[inline(always)]
    pub const fn TIM5CFGR(self) -> crate::common::Reg<RCC_TIM5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "RCC TIM6 Configuration Register"]
    #[inline(always)]
    pub const fn TIM6CFGR(self) -> crate::common::Reg<RCC_TIM6CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "RCC TIM7 Configuration Register"]
    #[inline(always)]
    pub const fn TIM7CFGR(self) -> crate::common::Reg<RCC_TIM7CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "RCC TIM8 Configuration Register"]
    #[inline(always)]
    pub const fn TIM8CFGR(self) -> crate::common::Reg<RCC_TIM8CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "RCC TIM10 Configuration Register"]
    #[inline(always)]
    pub const fn TIM10CFGR(self) -> crate::common::Reg<RCC_TIM10CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "RCC TIM11 Configuration Register"]
    #[inline(always)]
    pub const fn TIM11CFGR(self) -> crate::common::Reg<RCC_TIM11CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "RCC TIM12 Configuration Register"]
    #[inline(always)]
    pub const fn TIM12CFGR(self) -> crate::common::Reg<RCC_TIM12CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "RCC TIM13 Configuration Register"]
    #[inline(always)]
    pub const fn TIM13CFGR(self) -> crate::common::Reg<RCC_TIM13CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "RCC TIM14 Configuration Register"]
    #[inline(always)]
    pub const fn TIM14CFGR(self) -> crate::common::Reg<RCC_TIM14CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "RCC TIM15 Configuration Register"]
    #[inline(always)]
    pub const fn TIM15CFGR(self) -> crate::common::Reg<RCC_TIM15CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "RCC TIM16 Configuration Register"]
    #[inline(always)]
    pub const fn TIM16CFGR(self) -> crate::common::Reg<RCC_TIM16CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0738usize) as _) }
    }
    #[doc = "RCC TIM17 Configuration Register"]
    #[inline(always)]
    pub const fn TIM17CFGR(self) -> crate::common::Reg<RCC_TIM17CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x073cusize) as _) }
    }
    #[doc = "RCC TIM20 Configuration Register"]
    #[inline(always)]
    pub const fn TIM20CFGR(self) -> crate::common::Reg<RCC_TIM20CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "RCC LPTIM1 Configuration Register"]
    #[inline(always)]
    pub const fn LPTIM1CFGR(self) -> crate::common::Reg<RCC_LPTIM1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0744usize) as _) }
    }
    #[doc = "RCC LPTIM2 Configuration Register"]
    #[inline(always)]
    pub const fn LPTIM2CFGR(self) -> crate::common::Reg<RCC_LPTIM2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0748usize) as _) }
    }
    #[doc = "RCC LPTIM3 Configuration Register"]
    #[inline(always)]
    pub const fn LPTIM3CFGR(self) -> crate::common::Reg<RCC_LPTIM3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[doc = "RCC LPTIM4 Configuration Register"]
    #[inline(always)]
    pub const fn LPTIM4CFGR(self) -> crate::common::Reg<RCC_LPTIM4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "RCC LPTIM5 Configuration Register"]
    #[inline(always)]
    pub const fn LPTIM5CFGR(self) -> crate::common::Reg<RCC_LPTIM5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0754usize) as _) }
    }
    #[doc = "RCC SPI1 Configuration Register"]
    #[inline(always)]
    pub const fn SPI1CFGR(self) -> crate::common::Reg<RCC_SPI1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0758usize) as _) }
    }
    #[doc = "RCC SPI2 Configuration Register"]
    #[inline(always)]
    pub const fn SPI2CFGR(self) -> crate::common::Reg<RCC_SPI2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x075cusize) as _) }
    }
    #[doc = "RCC SPI3 Configuration Register"]
    #[inline(always)]
    pub const fn SPI3CFGR(self) -> crate::common::Reg<RCC_SPI3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "RCC SPI4 Configuration Register"]
    #[inline(always)]
    pub const fn SPI4CFGR(self) -> crate::common::Reg<RCC_SPI4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "RCC SPI5 Configuration Register"]
    #[inline(always)]
    pub const fn SPI5CFGR(self) -> crate::common::Reg<RCC_SPI5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "RCC SPI6 Configuration Register"]
    #[inline(always)]
    pub const fn SPI6CFGR(self) -> crate::common::Reg<RCC_SPI6CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x076cusize) as _) }
    }
    #[doc = "RCC SPI7 Configuration Register"]
    #[inline(always)]
    pub const fn SPI7CFGR(self) -> crate::common::Reg<RCC_SPI7CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "RCC SPI8 Configuration Register"]
    #[inline(always)]
    pub const fn SPI8CFGR(self) -> crate::common::Reg<RCC_SPI8CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0774usize) as _) }
    }
    #[doc = "RCC SPDIFRX Configuration Register"]
    #[inline(always)]
    pub const fn SPDIFRXCFGR(self) -> crate::common::Reg<RCC_SPDIFRXCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0778usize) as _) }
    }
    #[doc = "RCC USART1 Configuration Register"]
    #[inline(always)]
    pub const fn USART1CFGR(self) -> crate::common::Reg<RCC_USART1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x077cusize) as _) }
    }
    #[doc = "RCC USART2 Configuration Register"]
    #[inline(always)]
    pub const fn USART2CFGR(self) -> crate::common::Reg<RCC_USART2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "RCC USART3 Configuration Register"]
    #[inline(always)]
    pub const fn USART3CFGR(self) -> crate::common::Reg<RCC_USART3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "RCC UART4 Configuration Register"]
    #[inline(always)]
    pub const fn UART4CFGR(self) -> crate::common::Reg<RCC_UART4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "RCC UART5 Configuration Register"]
    #[inline(always)]
    pub const fn UART5CFGR(self) -> crate::common::Reg<RCC_UART5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "RCC USART6 Configuration Register"]
    #[inline(always)]
    pub const fn USART6CFGR(self) -> crate::common::Reg<RCC_USART6CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "RCC UART7 Configuration Register"]
    #[inline(always)]
    pub const fn UART7CFGR(self) -> crate::common::Reg<RCC_UART7CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[doc = "RCC UART8 Configuration Register"]
    #[inline(always)]
    pub const fn UART8CFGR(self) -> crate::common::Reg<RCC_UART8CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[doc = "RCC UART9 Configuration Register"]
    #[inline(always)]
    pub const fn UART9CFGR(self) -> crate::common::Reg<RCC_UART9CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x079cusize) as _) }
    }
    #[doc = "RCC LPUART1 Configuration Register"]
    #[inline(always)]
    pub const fn LPUART1CFGR(self) -> crate::common::Reg<RCC_LPUART1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "RCC I2C1 Configuration Register"]
    #[inline(always)]
    pub const fn I2C1CFGR(self) -> crate::common::Reg<RCC_I2C1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "RCC I2C2 Configuration Register"]
    #[inline(always)]
    pub const fn I2C2CFGR(self) -> crate::common::Reg<RCC_I2C2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "RCC I2C3 Configuration Register"]
    #[inline(always)]
    pub const fn I2C3CFGR(self) -> crate::common::Reg<RCC_I2C3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07acusize) as _) }
    }
    #[doc = "RCC I2C4 Configuration Register"]
    #[inline(always)]
    pub const fn I2C4CFGR(self) -> crate::common::Reg<RCC_I2C4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
    #[doc = "RCC I2C5 Configuration Register"]
    #[inline(always)]
    pub const fn I2C5CFGR(self) -> crate::common::Reg<RCC_I2C5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    #[doc = "RCC I2C6 Configuration Register"]
    #[inline(always)]
    pub const fn I2C6CFGR(self) -> crate::common::Reg<RCC_I2C6CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b8usize) as _) }
    }
    #[doc = "RCC I2C7 Configuration Register"]
    #[inline(always)]
    pub const fn I2C7CFGR(self) -> crate::common::Reg<RCC_I2C7CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07bcusize) as _) }
    }
    #[doc = "RCC I2C8 Configuration Register"]
    #[inline(always)]
    pub const fn I2C8CFGR(self) -> crate::common::Reg<RCC_I2C8CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "RCC SAI1 Configuration Register"]
    #[inline(always)]
    pub const fn SAI1CFGR(self) -> crate::common::Reg<RCC_SAI1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    #[doc = "RCC SAI2 Configuration Register"]
    #[inline(always)]
    pub const fn SAI2CFGR(self) -> crate::common::Reg<RCC_SAI2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c8usize) as _) }
    }
    #[doc = "RCC SAI3 Configuration Register"]
    #[inline(always)]
    pub const fn SAI3CFGR(self) -> crate::common::Reg<RCC_SAI3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ccusize) as _) }
    }
    #[doc = "RCC SAI4 Configuration Register"]
    #[inline(always)]
    pub const fn SAI4CFGR(self) -> crate::common::Reg<RCC_SAI4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07d0usize) as _) }
    }
    #[doc = "RCC MDF1 Configuration Register"]
    #[inline(always)]
    pub const fn MDF1CFGR(self) -> crate::common::Reg<RCC_MDF1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07d8usize) as _) }
    }
    #[doc = "RCC ADF1 Configuration Register"]
    #[inline(always)]
    pub const fn ADF1CFGR(self) -> crate::common::Reg<RCC_ADF1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07dcusize) as _) }
    }
    #[doc = "RCC FDCAN Configuration Register"]
    #[inline(always)]
    pub const fn FDCANCFGR(self) -> crate::common::Reg<RCC_FDCANCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e0usize) as _) }
    }
    #[doc = "RCC HDP Configuration Register"]
    #[inline(always)]
    pub const fn HDPCFGR(self) -> crate::common::Reg<RCC_HDPCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e4usize) as _) }
    }
    #[doc = "RCC ADC12 Configuration Register"]
    #[inline(always)]
    pub const fn ADC12CFGR(self) -> crate::common::Reg<RCC_ADC12CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e8usize) as _) }
    }
    #[doc = "RCC ADC3 Configuration Register"]
    #[inline(always)]
    pub const fn ADC3CFGR(self) -> crate::common::Reg<RCC_ADC3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ecusize) as _) }
    }
    #[doc = "RCC Ethernet 1 Configuration Register"]
    #[inline(always)]
    pub const fn ETH1CFGR(self) -> crate::common::Reg<RCC_ETH1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[doc = "RCC Ethernet 2 Configuration Register"]
    #[inline(always)]
    pub const fn ETH2CFGR(self) -> crate::common::Reg<RCC_ETH2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[doc = "RCC USBH Configuration Register"]
    #[inline(always)]
    pub const fn USBHCFGR(self) -> crate::common::Reg<RCC_USBHCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
    #[doc = "RCC USB2PHY1 Configuration Register"]
    #[inline(always)]
    pub const fn USB2PHY1CFGR(self) -> crate::common::Reg<RCC_USB2PHY1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "RCC USB2PHY2 Configuration Register"]
    #[inline(always)]
    pub const fn USB2PHY2CFGR(self) -> crate::common::Reg<RCC_USB2PHY2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "RCC USB3DRD Configuration Register"]
    #[inline(always)]
    pub const fn USB3DRDCFGR(self) -> crate::common::Reg<RCC_USB3DRDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "RCC USB3PCIEPHY Configuration Register"]
    #[inline(always)]
    pub const fn USB3PCIEPHYCFGR(
        self,
    ) -> crate::common::Reg<RCC_USB3PCIEPHYCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "RCC PCIE Configuration Register"]
    #[inline(always)]
    pub const fn PCIECFGR(self) -> crate::common::Reg<RCC_PCIECFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "RCC UCPD Configuration Register"]
    #[inline(always)]
    pub const fn UCPDCFGR(self) -> crate::common::Reg<RCC_UCPDCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "RCC Ethernet switch Configuration Register"]
    #[inline(always)]
    pub const fn ETHSWCFGR(self) -> crate::common::Reg<RCC_ETHSWCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "RCC Ethernet switch ACM Configuration Register"]
    #[inline(always)]
    pub const fn ETHSWACMCFGR(self) -> crate::common::Reg<RCC_ETHSWACMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "RCC Ethernet switch ACM Message Configuration Register"]
    #[inline(always)]
    pub const fn ETHSWACMMSGCFGR(
        self,
    ) -> crate::common::Reg<RCC_ETHSWACMMSGCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "RCC STGEN Configuration Register"]
    #[inline(always)]
    pub const fn STGENCFGR(self) -> crate::common::Reg<RCC_STGENCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "RCC SDMMC1 Configuration Register"]
    #[inline(always)]
    pub const fn SDMMC1CFGR(self) -> crate::common::Reg<RCC_SDMMC1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0830usize) as _) }
    }
    #[doc = "RCC SDMMC2 Configuration Register"]
    #[inline(always)]
    pub const fn SDMMC2CFGR(self) -> crate::common::Reg<RCC_SDMMC2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0834usize) as _) }
    }
    #[doc = "RCC SDMMC3 Configuration Register"]
    #[inline(always)]
    pub const fn SDMMC3CFGR(self) -> crate::common::Reg<RCC_SDMMC3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0838usize) as _) }
    }
    #[doc = "RCC GPU Configuration Register"]
    #[inline(always)]
    pub const fn GPUCFGR(self) -> crate::common::Reg<RCC_GPUCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x083cusize) as _) }
    }
    #[doc = "RCC LTDC Configuration Register"]
    #[inline(always)]
    pub const fn LTDCCFGR(self) -> crate::common::Reg<RCC_LTDCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "RCC DSI Configuration Register"]
    #[inline(always)]
    pub const fn DSICFGR(self) -> crate::common::Reg<RCC_DSICFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "RCC LVDS Configuration Register"]
    #[inline(always)]
    pub const fn LVDSCFGR(self) -> crate::common::Reg<RCC_LVDSCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "RCC CSI2 Configuration Register"]
    #[inline(always)]
    pub const fn CSI2CFGR(self) -> crate::common::Reg<RCC_CSI2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[doc = "RCC DCMIPP Configuration Register"]
    #[inline(always)]
    pub const fn DCMIPPCFGR(self) -> crate::common::Reg<RCC_DCMIPPCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x085cusize) as _) }
    }
    #[doc = "RCC CCI Configuration Register"]
    #[inline(always)]
    pub const fn CCICFGR(self) -> crate::common::Reg<RCC_CCICFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[doc = "RCC VDEC Configuration Register"]
    #[inline(always)]
    pub const fn VDECCFGR(self) -> crate::common::Reg<RCC_VDECCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0864usize) as _) }
    }
    #[doc = "RCC VENC Configuration Register"]
    #[inline(always)]
    pub const fn VENCCFGR(self) -> crate::common::Reg<RCC_VENCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0868usize) as _) }
    }
    #[doc = "RCC RNG Configuration Register"]
    #[inline(always)]
    pub const fn RNGCFGR(self) -> crate::common::Reg<RCC_RNGCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0870usize) as _) }
    }
    #[doc = "RCC PKA Configuration Register"]
    #[inline(always)]
    pub const fn PKACFGR(self) -> crate::common::Reg<RCC_PKACFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0874usize) as _) }
    }
    #[doc = "RCC SAES Configuration Register"]
    #[inline(always)]
    pub const fn SAESCFGR(self) -> crate::common::Reg<RCC_SAESCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0878usize) as _) }
    }
    #[doc = "RCC HASH Configuration Register"]
    #[inline(always)]
    pub const fn HASHCFGR(self) -> crate::common::Reg<RCC_HASHCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x087cusize) as _) }
    }
    #[doc = "RCC CRYP1 Configuration Register"]
    #[inline(always)]
    pub const fn CRYP1CFGR(self) -> crate::common::Reg<RCC_CRYP1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "RCC CRYP2 Configuration Register"]
    #[inline(always)]
    pub const fn CRYP2CFGR(self) -> crate::common::Reg<RCC_CRYP2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[doc = "RCC IWDG1 Configuration Register"]
    #[inline(always)]
    pub const fn IWDG1CFGR(self) -> crate::common::Reg<RCC_IWDG1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0888usize) as _) }
    }
    #[doc = "RCC IWDG2 Configuration Register"]
    #[inline(always)]
    pub const fn IWDG2CFGR(self) -> crate::common::Reg<RCC_IWDG2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x088cusize) as _) }
    }
    #[doc = "RCC IWDG3 Configuration Register"]
    #[inline(always)]
    pub const fn IWDG3CFGR(self) -> crate::common::Reg<RCC_IWDG3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "RCC IWDG4 Configuration Register"]
    #[inline(always)]
    pub const fn IWDG4CFGR(self) -> crate::common::Reg<RCC_IWDG4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    #[doc = "RCC IWDG5 Configuration Register"]
    #[inline(always)]
    pub const fn IWDG5CFGR(self) -> crate::common::Reg<RCC_IWDG5CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0898usize) as _) }
    }
    #[doc = "RCC WWDG1 Configuration Register"]
    #[inline(always)]
    pub const fn WWDG1CFGR(self) -> crate::common::Reg<RCC_WWDG1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x089cusize) as _) }
    }
    #[doc = "RCC WWDG2 Configuration Register"]
    #[inline(always)]
    pub const fn WWDG2CFGR(self) -> crate::common::Reg<RCC_WWDG2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "RCC VREF Configuration Register"]
    #[inline(always)]
    pub const fn VREFCFGR(self) -> crate::common::Reg<RCC_VREFCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a8usize) as _) }
    }
    #[doc = "RCC DTS Configuration Register"]
    #[inline(always)]
    pub const fn DTSCFGR(self) -> crate::common::Reg<RCC_DTSCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08acusize) as _) }
    }
    #[doc = "RCC CRC Configuration Register"]
    #[inline(always)]
    pub const fn CRCCFGR(self) -> crate::common::Reg<RCC_CRCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08b4usize) as _) }
    }
    #[doc = "RCC SERC Configuration Register"]
    #[inline(always)]
    pub const fn SERCCFGR(self) -> crate::common::Reg<RCC_SERCCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08b8usize) as _) }
    }
    #[doc = "RCC OCTOSPI IOMANAGER Configuration Register"]
    #[inline(always)]
    pub const fn OSPIIOMCFGR(self) -> crate::common::Reg<RCC_OSPIIOMCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08bcusize) as _) }
    }
    #[doc = "RCC GICV2M Configuration Register"]
    #[inline(always)]
    pub const fn GICV2MCFGR(self) -> crate::common::Reg<RCC_GICV2MCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[doc = "RCC I3C1 Configuration Register"]
    #[inline(always)]
    pub const fn I3C1CFGR(self) -> crate::common::Reg<RCC_I3C1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c8usize) as _) }
    }
    #[doc = "RCC I3C2 Configuration Register"]
    #[inline(always)]
    pub const fn I3C2CFGR(self) -> crate::common::Reg<RCC_I3C2CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08ccusize) as _) }
    }
    #[doc = "RCC I3C3 Configuration Register"]
    #[inline(always)]
    pub const fn I3C3CFGR(self) -> crate::common::Reg<RCC_I3C3CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[doc = "RCC I3C4 Configuration Register"]
    #[inline(always)]
    pub const fn I3C4CFGR(self) -> crate::common::Reg<RCC_I3C4CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d4usize) as _) }
    }
    #[doc = "RCC MUXSEL Configuration Register"]
    #[inline(always)]
    pub const fn MUXSELCFGR(self) -> crate::common::Reg<RCC_MUXSELCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "RCC Cross Bar x Configuration Register (x=0 to 63)"]
    #[inline(always)]
    pub const fn XBARxCFGR(self, n: usize) -> crate::common::Reg<RCC_XBARxCFGR, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize + n * 4usize) as _)
        }
    }
    #[doc = "RCC Pre Divider x Configuration Register (x=0 to 63)"]
    #[inline(always)]
    pub const fn PREDIVxCFGR(
        self,
        n: usize,
    ) -> crate::common::Reg<RCC_PREDIVxCFGR, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize + n * 4usize) as _)
        }
    }
    #[doc = "RCC Pre Divider Status Register 1"]
    #[inline(always)]
    pub const fn PREDIVSR1(self) -> crate::common::Reg<RCC_PREDIVSR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1218usize) as _) }
    }
    #[doc = "RCC Pre Divider Status Register 2"]
    #[inline(always)]
    pub const fn PREDIVSR2(self) -> crate::common::Reg<RCC_PREDIVSR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121cusize) as _) }
    }
    #[doc = "RCC Final Divider x Configuration Register (x=0 to 63)"]
    #[inline(always)]
    pub const fn FINDIVxCFGR(
        self,
        n: usize,
    ) -> crate::common::Reg<RCC_FINDIVxCFGR, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1224usize + n * 4usize) as _)
        }
    }
    #[doc = "RCC Final Divider Status Register 1"]
    #[inline(always)]
    pub const fn FINDIVSR1(self) -> crate::common::Reg<RCC_FINDIVSR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1324usize) as _) }
    }
    #[doc = "RCC Final Divider Status Register 2"]
    #[inline(always)]
    pub const fn FINDIVSR2(self) -> crate::common::Reg<RCC_FINDIVSR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1328usize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator and Observation 0 clock Configuration Register"]
    #[inline(always)]
    pub const fn FCALCOBS0CFGR(self) -> crate::common::Reg<RCC_FCALCOBS0CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1340usize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator and Observation 1 clock Configuration Register"]
    #[inline(always)]
    pub const fn FCALCOBS1CFGR(self) -> crate::common::Reg<RCC_FCALCOBS1CFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1344usize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator Reference clock Configuration Register"]
    #[inline(always)]
    pub const fn FCALCREFCFGR(self) -> crate::common::Reg<RCC_FCALCREFCFGR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator Control Register 1"]
    #[inline(always)]
    pub const fn FCALCCR1(self) -> crate::common::Reg<RCC_FCALCCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x134cusize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator Control Register 2"]
    #[inline(always)]
    pub const fn FCALCCR2(self) -> crate::common::Reg<RCC_FCALCCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1354usize) as _) }
    }
    #[doc = "RCC Clock Frequency Calculator Status Register"]
    #[inline(always)]
    pub const fn FCALCSR(self) -> crate::common::Reg<RCC_FCALCSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1358usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL4CFGR1(self) -> crate::common::Reg<RCC_PLL4CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1360usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL4CFGR2(self) -> crate::common::Reg<RCC_PLL4CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1364usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL4CFGR3(self) -> crate::common::Reg<RCC_PLL4CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1368usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL4CFGR4(self) -> crate::common::Reg<RCC_PLL4CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x136cusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL4CFGR5(self) -> crate::common::Reg<RCC_PLL4CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1370usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL4CFGR6(self) -> crate::common::Reg<RCC_PLL4CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1378usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL4CFGR7(self) -> crate::common::Reg<RCC_PLL4CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x137cusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL5CFGR1(self) -> crate::common::Reg<RCC_PLL5CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1388usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL5CFGR2(self) -> crate::common::Reg<RCC_PLL5CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x138cusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL5CFGR3(self) -> crate::common::Reg<RCC_PLL5CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1390usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL5CFGR4(self) -> crate::common::Reg<RCC_PLL5CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1394usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL5CFGR5(self) -> crate::common::Reg<RCC_PLL5CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1398usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL5CFGR6(self) -> crate::common::Reg<RCC_PLL5CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a0usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL5CFGR7(self) -> crate::common::Reg<RCC_PLL5CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a4usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL6CFGR1(self) -> crate::common::Reg<RCC_PLL6CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b0usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL6CFGR2(self) -> crate::common::Reg<RCC_PLL6CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b4usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL6CFGR3(self) -> crate::common::Reg<RCC_PLL6CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b8usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL6CFGR4(self) -> crate::common::Reg<RCC_PLL6CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13bcusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL6CFGR5(self) -> crate::common::Reg<RCC_PLL6CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c0usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL6CFGR6(self) -> crate::common::Reg<RCC_PLL6CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c8usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL6CFGR7(self) -> crate::common::Reg<RCC_PLL6CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13ccusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL7CFGR1(self) -> crate::common::Reg<RCC_PLL7CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d8usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL7CFGR2(self) -> crate::common::Reg<RCC_PLL7CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13dcusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL7CFGR3(self) -> crate::common::Reg<RCC_PLL7CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e0usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL7CFGR4(self) -> crate::common::Reg<RCC_PLL7CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e4usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL7CFGR5(self) -> crate::common::Reg<RCC_PLL7CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e8usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL7CFGR6(self) -> crate::common::Reg<RCC_PLL7CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f0usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL7CFGR7(self) -> crate::common::Reg<RCC_PLL7CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f4usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 1"]
    #[inline(always)]
    pub const fn PLL8CFGR1(self) -> crate::common::Reg<RCC_PLL8CFGR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1400usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 2"]
    #[inline(always)]
    pub const fn PLL8CFGR2(self) -> crate::common::Reg<RCC_PLL8CFGR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1404usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 3"]
    #[inline(always)]
    pub const fn PLL8CFGR3(self) -> crate::common::Reg<RCC_PLL8CFGR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1408usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 4"]
    #[inline(always)]
    pub const fn PLL8CFGR4(self) -> crate::common::Reg<RCC_PLL8CFGR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x140cusize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 5"]
    #[inline(always)]
    pub const fn PLL8CFGR5(self) -> crate::common::Reg<RCC_PLL8CFGR5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1410usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 6"]
    #[inline(always)]
    pub const fn PLL8CFGR6(self) -> crate::common::Reg<RCC_PLL8CFGR6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1418usize) as _) }
    }
    #[doc = "RCC PLL{y} Configuration Register 7"]
    #[inline(always)]
    pub const fn PLL8CFGR7(self) -> crate::common::Reg<RCC_PLL8CFGR7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x141cusize) as _) }
    }
    #[doc = "RCC Version register"]
    #[inline(always)]
    pub const fn VERR(self) -> crate::common::Reg<RCC_VERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfff4usize) as _) }
    }
    #[doc = "RCC ID register"]
    #[inline(always)]
    pub const fn IDR(self) -> crate::common::Reg<RCC_IDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfff8usize) as _) }
    }
    #[doc = "RCC Size ID register"]
    #[inline(always)]
    pub const fn SIDR(self) -> crate::common::Reg<RCC_SIDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfffcusize) as _) }
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ADC12CFGR(pub u32);
impl RCC_ADC12CFGR {
    #[doc = "ADC12 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC12RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC12 reset"]
    #[inline(always)]
    pub const fn set_ADC12RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC12 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC12EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC12 clock enable"]
    #[inline(always)]
    pub const fn set_ADC12EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC12 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC12LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC12 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ADC12LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC12 kernel clock source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC12KERSEL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ADC12 kernel clock source selection"]
    #[inline(always)]
    pub const fn set_ADC12KERSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for RCC_ADC12CFGR {
    #[inline(always)]
    fn default() -> RCC_ADC12CFGR {
        RCC_ADC12CFGR(0)
    }
}
impl core::fmt::Debug for RCC_ADC12CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ADC12CFGR")
            .field("ADC12RST", &self.ADC12RST())
            .field("ADC12EN", &self.ADC12EN())
            .field("ADC12LPEN", &self.ADC12LPEN())
            .field("ADC12KERSEL", &self.ADC12KERSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ADC12CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ADC12CFGR {{ ADC12RST: {=bool:?}, ADC12EN: {=bool:?}, ADC12LPEN: {=bool:?}, ADC12KERSEL: {=bool:?} }}",
            self.ADC12RST(),
            self.ADC12EN(),
            self.ADC12LPEN(),
            self.ADC12KERSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ADC3CFGR(pub u32);
impl RCC_ADC3CFGR {
    #[doc = "ADC3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3 reset"]
    #[inline(always)]
    pub const fn set_ADC3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3 clock enable"]
    #[inline(always)]
    pub const fn set_ADC3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADC3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ADC3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC3 kernel clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn ADC3KERSEL(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "ADC3 kernel clock source select"]
    #[inline(always)]
    pub const fn set_ADC3KERSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for RCC_ADC3CFGR {
    #[inline(always)]
    fn default() -> RCC_ADC3CFGR {
        RCC_ADC3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_ADC3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ADC3CFGR")
            .field("ADC3RST", &self.ADC3RST())
            .field("ADC3EN", &self.ADC3EN())
            .field("ADC3LPEN", &self.ADC3LPEN())
            .field("ADC3KERSEL", &self.ADC3KERSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ADC3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ADC3CFGR {{ ADC3RST: {=bool:?}, ADC3EN: {=bool:?}, ADC3LPEN: {=bool:?}, ADC3KERSEL: {=u8:?} }}",
            self.ADC3RST(),
            self.ADC3EN(),
            self.ADC3LPEN(),
            self.ADC3KERSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ADF1CFGR(pub u32);
impl RCC_ADF1CFGR {
    #[doc = "ADF1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ADF1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADF1 reset"]
    #[inline(always)]
    pub const fn set_ADF1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADF1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADF1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADF1 clock enable"]
    #[inline(always)]
    pub const fn set_ADF1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADF1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ADF1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ADF1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ADF1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ADF1 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADF1AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ADF1 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_ADF1AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_ADF1CFGR {
    #[inline(always)]
    fn default() -> RCC_ADF1CFGR {
        RCC_ADF1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_ADF1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ADF1CFGR")
            .field("ADF1RST", &self.ADF1RST())
            .field("ADF1EN", &self.ADF1EN())
            .field("ADF1LPEN", &self.ADF1LPEN())
            .field("ADF1AMEN", &self.ADF1AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ADF1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ADF1CFGR {{ ADF1RST: {=bool:?}, ADF1EN: {=bool:?}, ADF1LPEN: {=bool:?}, ADF1AMEN: {=bool:?} }}",
            self.ADF1RST(),
            self.ADF1EN(),
            self.ADF1LPEN(),
            self.ADF1AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_APB1DIVR(pub u32);
impl RCC_APB1DIVR {
    #[doc = "APB1 clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn APB1DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "APB1 clock divider"]
    #[inline(always)]
    pub const fn set_APB1DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "APB1 clock prescaler status"]
    #[must_use]
    #[inline(always)]
    pub const fn APB1DIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "APB1 clock prescaler status"]
    #[inline(always)]
    pub const fn set_APB1DIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_APB1DIVR {
    #[inline(always)]
    fn default() -> RCC_APB1DIVR {
        RCC_APB1DIVR(0)
    }
}
impl core::fmt::Debug for RCC_APB1DIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB1DIVR")
            .field("APB1DIV", &self.APB1DIV())
            .field("APB1DIVRDY", &self.APB1DIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_APB1DIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_APB1DIVR {{ APB1DIV: {=u8:?}, APB1DIVRDY: {=bool:?} }}",
            self.APB1DIV(),
            self.APB1DIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_APB2DIVR(pub u32);
impl RCC_APB2DIVR {
    #[doc = "APB2 clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn APB2DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "APB2 clock divider"]
    #[inline(always)]
    pub const fn set_APB2DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "APB2 clock prescaler status"]
    #[must_use]
    #[inline(always)]
    pub const fn APB2DIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "APB2 clock prescaler status"]
    #[inline(always)]
    pub const fn set_APB2DIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_APB2DIVR {
    #[inline(always)]
    fn default() -> RCC_APB2DIVR {
        RCC_APB2DIVR(0)
    }
}
impl core::fmt::Debug for RCC_APB2DIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB2DIVR")
            .field("APB2DIV", &self.APB2DIV())
            .field("APB2DIVRDY", &self.APB2DIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_APB2DIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_APB2DIVR {{ APB2DIV: {=u8:?}, APB2DIVRDY: {=bool:?} }}",
            self.APB2DIV(),
            self.APB2DIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_APB3DIVR(pub u32);
impl RCC_APB3DIVR {
    #[doc = "APB3 clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn APB3DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "APB3 clock divider"]
    #[inline(always)]
    pub const fn set_APB3DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "APB3 clock prescaler status"]
    #[must_use]
    #[inline(always)]
    pub const fn APB3DIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "APB3 clock prescaler status"]
    #[inline(always)]
    pub const fn set_APB3DIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_APB3DIVR {
    #[inline(always)]
    fn default() -> RCC_APB3DIVR {
        RCC_APB3DIVR(0)
    }
}
impl core::fmt::Debug for RCC_APB3DIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB3DIVR")
            .field("APB3DIV", &self.APB3DIV())
            .field("APB3DIVRDY", &self.APB3DIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_APB3DIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_APB3DIVR {{ APB3DIV: {=u8:?}, APB3DIVRDY: {=bool:?} }}",
            self.APB3DIV(),
            self.APB3DIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_APB4DIVR(pub u32);
impl RCC_APB4DIVR {
    #[doc = "APB4 clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn APB4DIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "APB4 clock divider"]
    #[inline(always)]
    pub const fn set_APB4DIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "APB4 clock divider status"]
    #[must_use]
    #[inline(always)]
    pub const fn APB4DIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "APB4 clock divider status"]
    #[inline(always)]
    pub const fn set_APB4DIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_APB4DIVR {
    #[inline(always)]
    fn default() -> RCC_APB4DIVR {
        RCC_APB4DIVR(0)
    }
}
impl core::fmt::Debug for RCC_APB4DIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APB4DIVR")
            .field("APB4DIV", &self.APB4DIV())
            .field("APB4DIVRDY", &self.APB4DIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_APB4DIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_APB4DIVR {{ APB4DIV: {=u8:?}, APB4DIVRDY: {=bool:?} }}",
            self.APB4DIV(),
            self.APB4DIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_APBDBGDIVR(pub u32);
impl RCC_APBDBGDIVR {
    #[doc = "APBDBG clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn APBDBGDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "APBDBG clock divider"]
    #[inline(always)]
    pub const fn set_APBDBGDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "APBDBG clock divider status"]
    #[must_use]
    #[inline(always)]
    pub const fn APBDBGDIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "APBDBG clock divider status"]
    #[inline(always)]
    pub const fn set_APBDBGDIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_APBDBGDIVR {
    #[inline(always)]
    fn default() -> RCC_APBDBGDIVR {
        RCC_APBDBGDIVR(0)
    }
}
impl core::fmt::Debug for RCC_APBDBGDIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_APBDBGDIVR")
            .field("APBDBGDIV", &self.APBDBGDIV())
            .field("APBDBGDIVRDY", &self.APBDBGDIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_APBDBGDIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_APBDBGDIVR {{ APBDBGDIV: {=u8:?}, APBDBGDIVRDY: {=bool:?} }}",
            self.APBDBGDIV(),
            self.APBDBGDIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_BDCR(pub u32);
impl RCC_BDCR {
    #[doc = "LSE oscillator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSEON(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSE oscillator enable"]
    #[inline(always)]
    pub const fn set_LSEON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE oscillator bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn LSEBYP(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE oscillator bypass"]
    #[inline(always)]
    pub const fn set_LSEBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LSE oscillator ready"]
    #[must_use]
    #[inline(always)]
    pub const fn LSERDY(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub const fn set_LSERDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LSE digital bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn LSEDIGBYP(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LSE digital bypass"]
    #[inline(always)]
    pub const fn set_LSEDIGBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LSE oscillator driving capability"]
    #[must_use]
    #[inline(always)]
    pub const fn LSEDRV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "LSE oscillator driving capability"]
    #[inline(always)]
    pub const fn set_LSEDRV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "LSE CSS enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSON(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "LSE CSS enable"]
    #[inline(always)]
    pub const fn set_LSECSSON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LSE clock glitch filter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSEGFON(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LSE clock glitch filter enable"]
    #[inline(always)]
    pub const fn set_LSEGFON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "LSE CSS failure detection"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSD(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LSE CSS failure detection"]
    #[inline(always)]
    pub const fn set_LSECSSD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LSI oscillator enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn LSION(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LSI oscillator enabled"]
    #[inline(always)]
    pub const fn set_LSION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LSI oscillator ready"]
    #[must_use]
    #[inline(always)]
    pub const fn LSIRDY(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LSI oscillator ready"]
    #[inline(always)]
    pub const fn set_LSIRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RTC clock source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCSRC(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "RTC clock source selection"]
    #[inline(always)]
    pub const fn set_RTCSRC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "RTC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCCKEN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "RTC clock enable"]
    #[inline(always)]
    pub const fn set_RTCCKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "MSI frequency selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIFREQSEL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "MSI frequency selection"]
    #[inline(always)]
    pub const fn set_MSIFREQSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "CPU3 SysTick clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn C3SYSTICKSEL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "CPU3 SysTick clock select"]
    #[inline(always)]
    pub const fn set_C3SYSTICKSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "V&lt;sub&gt;SW&lt;/sub&gt; domain software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn VSWRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;SW&lt;/sub&gt; domain software reset"]
    #[inline(always)]
    pub const fn set_VSWRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_BDCR {
    #[inline(always)]
    fn default() -> RCC_BDCR {
        RCC_BDCR(0)
    }
}
impl core::fmt::Debug for RCC_BDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_BDCR")
            .field("LSEON", &self.LSEON())
            .field("LSEBYP", &self.LSEBYP())
            .field("LSERDY", &self.LSERDY())
            .field("LSEDIGBYP", &self.LSEDIGBYP())
            .field("LSEDRV", &self.LSEDRV())
            .field("LSECSSON", &self.LSECSSON())
            .field("LSEGFON", &self.LSEGFON())
            .field("LSECSSD", &self.LSECSSD())
            .field("LSION", &self.LSION())
            .field("LSIRDY", &self.LSIRDY())
            .field("RTCSRC", &self.RTCSRC())
            .field("RTCCKEN", &self.RTCCKEN())
            .field("MSIFREQSEL", &self.MSIFREQSEL())
            .field("C3SYSTICKSEL", &self.C3SYSTICKSEL())
            .field("VSWRST", &self.VSWRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_BDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_BDCR {{ LSEON: {=bool:?}, LSEBYP: {=bool:?}, LSERDY: {=bool:?}, LSEDIGBYP: {=bool:?}, LSEDRV: {=u8:?}, LSECSSON: {=bool:?}, LSEGFON: {=bool:?}, LSECSSD: {=bool:?}, LSION: {=bool:?}, LSIRDY: {=bool:?}, RTCSRC: {=u8:?}, RTCCKEN: {=bool:?}, MSIFREQSEL: {=bool:?}, C3SYSTICKSEL: {=bool:?}, VSWRST: {=bool:?} }}",
            self.LSEON(),
            self.LSEBYP(),
            self.LSERDY(),
            self.LSEDIGBYP(),
            self.LSEDRV(),
            self.LSECSSON(),
            self.LSEGFON(),
            self.LSECSSD(),
            self.LSION(),
            self.LSIRDY(),
            self.RTCSRC(),
            self.RTCCKEN(),
            self.MSIFREQSEL(),
            self.C3SYSTICKSEL(),
            self.VSWRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_BKPSRAMCFGR(pub u32);
impl RCC_BKPSRAMCFGR {
    #[doc = "BKPSRAM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn BKPSRAMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BKPSRAM clock enable"]
    #[inline(always)]
    pub const fn set_BKPSRAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BKPSRAM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn BKPSRAMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BKPSRAM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_BKPSRAMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_BKPSRAMCFGR {
    #[inline(always)]
    fn default() -> RCC_BKPSRAMCFGR {
        RCC_BKPSRAMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_BKPSRAMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_BKPSRAMCFGR")
            .field("BKPSRAMEN", &self.BKPSRAMEN())
            .field("BKPSRAMLPEN", &self.BKPSRAMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_BKPSRAMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_BKPSRAMCFGR {{ BKPSRAMEN: {=bool:?}, BKPSRAMLPEN: {=bool:?} }}",
            self.BKPSRAMEN(),
            self.BKPSRAMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_BSECCFGR(pub u32);
impl RCC_BSECCFGR {
    #[doc = "BSEC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn BSECEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BSEC clock enable"]
    #[inline(always)]
    pub const fn set_BSECEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_PLL2CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BSECLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL2CFGR1 register"]
    #[inline(always)]
    pub const fn set_BSECLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_BSECCFGR {
    #[inline(always)]
    fn default() -> RCC_BSECCFGR {
        RCC_BSECCFGR(0)
    }
}
impl core::fmt::Debug for RCC_BSECCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_BSECCFGR")
            .field("BSECEN", &self.BSECEN())
            .field("BSECLPEN", &self.BSECLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_BSECCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_BSECCFGR {{ BSECEN: {=bool:?}, BSECLPEN: {=bool:?} }}",
            self.BSECEN(),
            self.BSECLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1BOOTRSTSCLRR(pub u32);
impl RCC_C1BOOTRSTSCLRR {
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PORRSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_PORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BORRSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOR reset flag"]
    #[inline(always)]
    pub const fn set_BORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NRST reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PADRSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRST reset flag"]
    #[inline(always)]
    pub const fn set_PADRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE CSS reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HCSSRSTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE CSS reset flag"]
    #[inline(always)]
    pub const fn set_HCSSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCORERSTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCORERSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCPURSTF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCPURSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CPU1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC1RSTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CPU2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC2RSTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IWDG1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1SYSRSTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG1SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IWDG2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2SYSRSTF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG2SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IWDG3 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3SYSRSTF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG3SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IWDG4 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4SYSRSTF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG4SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IWDG5 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5SYSRSTF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG5SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CPU1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1RSTF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 reset flag"]
    #[inline(always)]
    pub const fn set_C1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1P1RSTF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[inline(always)]
    pub const fn set_C1P1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RETCRCERRRSTF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[inline(always)]
    pub const fn set_RETCRCERRRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILCRCRSTF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILCRCRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILRESTRSTF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILRESTRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CPU1 system Standby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STBYC1RSTF(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system Standby reset flag"]
    #[inline(always)]
    pub const fn set_STBYC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "D1 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D1STBYRSTF(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "D1 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D1STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "D2 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D2STBYRSTF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "D2 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D2STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for RCC_C1BOOTRSTSCLRR {
    #[inline(always)]
    fn default() -> RCC_C1BOOTRSTSCLRR {
        RCC_C1BOOTRSTSCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C1BOOTRSTSCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1BOOTRSTSCLRR")
            .field("PORRSTF", &self.PORRSTF())
            .field("BORRSTF", &self.BORRSTF())
            .field("PADRSTF", &self.PADRSTF())
            .field("HCSSRSTF", &self.HCSSRSTF())
            .field("VCORERSTF", &self.VCORERSTF())
            .field("VCPURSTF", &self.VCPURSTF())
            .field("SYSC1RSTF", &self.SYSC1RSTF())
            .field("SYSC2RSTF", &self.SYSC2RSTF())
            .field("IWDG1SYSRSTF", &self.IWDG1SYSRSTF())
            .field("IWDG2SYSRSTF", &self.IWDG2SYSRSTF())
            .field("IWDG3SYSRSTF", &self.IWDG3SYSRSTF())
            .field("IWDG4SYSRSTF", &self.IWDG4SYSRSTF())
            .field("IWDG5SYSRSTF", &self.IWDG5SYSRSTF())
            .field("C1RSTF", &self.C1RSTF())
            .field("C1P1RSTF", &self.C1P1RSTF())
            .field("RETCRCERRRSTF", &self.RETCRCERRRSTF())
            .field("RETECCFAILCRCRSTF", &self.RETECCFAILCRCRSTF())
            .field("RETECCFAILRESTRSTF", &self.RETECCFAILRESTRSTF())
            .field("STBYC1RSTF", &self.STBYC1RSTF())
            .field("D1STBYRSTF", &self.D1STBYRSTF())
            .field("D2STBYRSTF", &self.D2STBYRSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1BOOTRSTSCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1BOOTRSTSCLRR {{ PORRSTF: {=bool:?}, BORRSTF: {=bool:?}, PADRSTF: {=bool:?}, HCSSRSTF: {=bool:?}, VCORERSTF: {=bool:?}, VCPURSTF: {=bool:?}, SYSC1RSTF: {=bool:?}, SYSC2RSTF: {=bool:?}, IWDG1SYSRSTF: {=bool:?}, IWDG2SYSRSTF: {=bool:?}, IWDG3SYSRSTF: {=bool:?}, IWDG4SYSRSTF: {=bool:?}, IWDG5SYSRSTF: {=bool:?}, C1RSTF: {=bool:?}, C1P1RSTF: {=bool:?}, RETCRCERRRSTF: {=bool:?}, RETECCFAILCRCRSTF: {=bool:?}, RETECCFAILRESTRSTF: {=bool:?}, STBYC1RSTF: {=bool:?}, D1STBYRSTF: {=bool:?}, D2STBYRSTF: {=bool:?} }}",
            self.PORRSTF(),
            self.BORRSTF(),
            self.PADRSTF(),
            self.HCSSRSTF(),
            self.VCORERSTF(),
            self.VCPURSTF(),
            self.SYSC1RSTF(),
            self.SYSC2RSTF(),
            self.IWDG1SYSRSTF(),
            self.IWDG2SYSRSTF(),
            self.IWDG3SYSRSTF(),
            self.IWDG4SYSRSTF(),
            self.IWDG5SYSRSTF(),
            self.C1RSTF(),
            self.C1P1RSTF(),
            self.RETCRCERRRSTF(),
            self.RETECCFAILCRCRSTF(),
            self.RETECCFAILRESTRSTF(),
            self.STBYC1RSTF(),
            self.D1STBYRSTF(),
            self.D2STBYRSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1BOOTRSTSSETR(pub u32);
impl RCC_C1BOOTRSTSSETR {
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PORRSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_PORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BORRSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOR reset flag"]
    #[inline(always)]
    pub const fn set_BORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NRST reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PADRSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRST reset flag"]
    #[inline(always)]
    pub const fn set_PADRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE CSS reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HCSSRSTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE CSS reset flag"]
    #[inline(always)]
    pub const fn set_HCSSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCORERSTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCORERSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCPURSTF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCPURSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CPU1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC1RSTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CPU2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC2RSTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IWDG1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1SYSRSTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG1SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IWDG2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2SYSRSTF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG2SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IWDG3 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3SYSRSTF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG3SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IWDG4 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4SYSRSTF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG4SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IWDG5 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5SYSRSTF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG5SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CPU1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1RSTF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 reset flag"]
    #[inline(always)]
    pub const fn set_C1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1P1RSTF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[inline(always)]
    pub const fn set_C1P1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RETCRCERRRSTF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[inline(always)]
    pub const fn set_RETCRCERRRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILCRCRSTF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILCRCRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILRESTRSTF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILRESTRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CPU1 system Standby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STBYC1RSTF(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system Standby reset flag"]
    #[inline(always)]
    pub const fn set_STBYC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "D1 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D1STBYRSTF(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "D1 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D1STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "D2 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D2STBYRSTF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "D2 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D2STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for RCC_C1BOOTRSTSSETR {
    #[inline(always)]
    fn default() -> RCC_C1BOOTRSTSSETR {
        RCC_C1BOOTRSTSSETR(0)
    }
}
impl core::fmt::Debug for RCC_C1BOOTRSTSSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1BOOTRSTSSETR")
            .field("PORRSTF", &self.PORRSTF())
            .field("BORRSTF", &self.BORRSTF())
            .field("PADRSTF", &self.PADRSTF())
            .field("HCSSRSTF", &self.HCSSRSTF())
            .field("VCORERSTF", &self.VCORERSTF())
            .field("VCPURSTF", &self.VCPURSTF())
            .field("SYSC1RSTF", &self.SYSC1RSTF())
            .field("SYSC2RSTF", &self.SYSC2RSTF())
            .field("IWDG1SYSRSTF", &self.IWDG1SYSRSTF())
            .field("IWDG2SYSRSTF", &self.IWDG2SYSRSTF())
            .field("IWDG3SYSRSTF", &self.IWDG3SYSRSTF())
            .field("IWDG4SYSRSTF", &self.IWDG4SYSRSTF())
            .field("IWDG5SYSRSTF", &self.IWDG5SYSRSTF())
            .field("C1RSTF", &self.C1RSTF())
            .field("C1P1RSTF", &self.C1P1RSTF())
            .field("RETCRCERRRSTF", &self.RETCRCERRRSTF())
            .field("RETECCFAILCRCRSTF", &self.RETECCFAILCRCRSTF())
            .field("RETECCFAILRESTRSTF", &self.RETECCFAILRESTRSTF())
            .field("STBYC1RSTF", &self.STBYC1RSTF())
            .field("D1STBYRSTF", &self.D1STBYRSTF())
            .field("D2STBYRSTF", &self.D2STBYRSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1BOOTRSTSSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1BOOTRSTSSETR {{ PORRSTF: {=bool:?}, BORRSTF: {=bool:?}, PADRSTF: {=bool:?}, HCSSRSTF: {=bool:?}, VCORERSTF: {=bool:?}, VCPURSTF: {=bool:?}, SYSC1RSTF: {=bool:?}, SYSC2RSTF: {=bool:?}, IWDG1SYSRSTF: {=bool:?}, IWDG2SYSRSTF: {=bool:?}, IWDG3SYSRSTF: {=bool:?}, IWDG4SYSRSTF: {=bool:?}, IWDG5SYSRSTF: {=bool:?}, C1RSTF: {=bool:?}, C1P1RSTF: {=bool:?}, RETCRCERRRSTF: {=bool:?}, RETECCFAILCRCRSTF: {=bool:?}, RETECCFAILRESTRSTF: {=bool:?}, STBYC1RSTF: {=bool:?}, D1STBYRSTF: {=bool:?}, D2STBYRSTF: {=bool:?} }}",
            self.PORRSTF(),
            self.BORRSTF(),
            self.PADRSTF(),
            self.HCSSRSTF(),
            self.VCORERSTF(),
            self.VCPURSTF(),
            self.SYSC1RSTF(),
            self.SYSC2RSTF(),
            self.IWDG1SYSRSTF(),
            self.IWDG2SYSRSTF(),
            self.IWDG3SYSRSTF(),
            self.IWDG4SYSRSTF(),
            self.IWDG5SYSRSTF(),
            self.C1RSTF(),
            self.C1P1RSTF(),
            self.RETCRCERRRSTF(),
            self.RETECCFAILCRCRSTF(),
            self.RETECCFAILRESTRSTF(),
            self.STBYC1RSTF(),
            self.D1STBYRSTF(),
            self.D2STBYRSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1CIESETR(pub u32);
impl RCC_C1CIESETR {
    #[doc = "LSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSIRDYIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_LSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSERDYIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_LSERDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIRDYIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_HSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSERDYIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_HSERDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIRDYIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_MSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL1 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL1RDYIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL1 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL1RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL2 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL2RDYIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL2 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL2RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PLL3 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL3RDYIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PLL3 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL3RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL4 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL4RDYIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLL4 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL4RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL5 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL5RDYIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLL5 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL5RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLL6 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL6RDYIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLL6 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL6RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PLL7 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL7RDYIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PLL7 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL7RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PLL8 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL8RDYIE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PLL8 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL8RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSE clock security system interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSIE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LSE clock security system interrupt enable"]
    #[inline(always)]
    pub const fn set_LSECSSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CPU1 wake-up from CStop interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WKUPIE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 wake-up from CStop interrupt enable"]
    #[inline(always)]
    pub const fn set_WKUPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for RCC_C1CIESETR {
    #[inline(always)]
    fn default() -> RCC_C1CIESETR {
        RCC_C1CIESETR(0)
    }
}
impl core::fmt::Debug for RCC_C1CIESETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1CIESETR")
            .field("LSIRDYIE", &self.LSIRDYIE())
            .field("LSERDYIE", &self.LSERDYIE())
            .field("HSIRDYIE", &self.HSIRDYIE())
            .field("HSERDYIE", &self.HSERDYIE())
            .field("MSIRDYIE", &self.MSIRDYIE())
            .field("PLL1RDYIE", &self.PLL1RDYIE())
            .field("PLL2RDYIE", &self.PLL2RDYIE())
            .field("PLL3RDYIE", &self.PLL3RDYIE())
            .field("PLL4RDYIE", &self.PLL4RDYIE())
            .field("PLL5RDYIE", &self.PLL5RDYIE())
            .field("PLL6RDYIE", &self.PLL6RDYIE())
            .field("PLL7RDYIE", &self.PLL7RDYIE())
            .field("PLL8RDYIE", &self.PLL8RDYIE())
            .field("LSECSSIE", &self.LSECSSIE())
            .field("WKUPIE", &self.WKUPIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1CIESETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1CIESETR {{ LSIRDYIE: {=bool:?}, LSERDYIE: {=bool:?}, HSIRDYIE: {=bool:?}, HSERDYIE: {=bool:?}, MSIRDYIE: {=bool:?}, PLL1RDYIE: {=bool:?}, PLL2RDYIE: {=bool:?}, PLL3RDYIE: {=bool:?}, PLL4RDYIE: {=bool:?}, PLL5RDYIE: {=bool:?}, PLL6RDYIE: {=bool:?}, PLL7RDYIE: {=bool:?}, PLL8RDYIE: {=bool:?}, LSECSSIE: {=bool:?}, WKUPIE: {=bool:?} }}",
            self.LSIRDYIE(),
            self.LSERDYIE(),
            self.HSIRDYIE(),
            self.HSERDYIE(),
            self.MSIRDYIE(),
            self.PLL1RDYIE(),
            self.PLL2RDYIE(),
            self.PLL3RDYIE(),
            self.PLL4RDYIE(),
            self.PLL5RDYIE(),
            self.PLL6RDYIE(),
            self.PLL7RDYIE(),
            self.PLL8RDYIE(),
            self.LSECSSIE(),
            self.WKUPIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1CIFCLRR(pub u32);
impl RCC_C1CIFCLRR {
    #[doc = "LSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSIRDYF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_LSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSERDYF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_LSERDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIRDYF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_HSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSERDYF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_HSERDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIRDYF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_MSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL1 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL1RDYF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL1 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL1RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL2 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL2RDYF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL2 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL2RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PLL3 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL3RDYF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PLL3 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL3RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL4 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL4RDYF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLL4 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL4RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL5 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL5RDYF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLL5 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL5RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLL6 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL6RDYF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLL6 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL6RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PLL7 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL7RDYF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PLL7 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL7RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PLL8 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL8RDYF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PLL8 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL8RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSE CSS interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LSE CSS interrupt flag"]
    #[inline(always)]
    pub const fn set_LSECSSF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CPU1 wake-up from CStop interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn WKUPF(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 wake-up from CStop interrupt flag"]
    #[inline(always)]
    pub const fn set_WKUPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for RCC_C1CIFCLRR {
    #[inline(always)]
    fn default() -> RCC_C1CIFCLRR {
        RCC_C1CIFCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C1CIFCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1CIFCLRR")
            .field("LSIRDYF", &self.LSIRDYF())
            .field("LSERDYF", &self.LSERDYF())
            .field("HSIRDYF", &self.HSIRDYF())
            .field("HSERDYF", &self.HSERDYF())
            .field("MSIRDYF", &self.MSIRDYF())
            .field("PLL1RDYF", &self.PLL1RDYF())
            .field("PLL2RDYF", &self.PLL2RDYF())
            .field("PLL3RDYF", &self.PLL3RDYF())
            .field("PLL4RDYF", &self.PLL4RDYF())
            .field("PLL5RDYF", &self.PLL5RDYF())
            .field("PLL6RDYF", &self.PLL6RDYF())
            .field("PLL7RDYF", &self.PLL7RDYF())
            .field("PLL8RDYF", &self.PLL8RDYF())
            .field("LSECSSF", &self.LSECSSF())
            .field("WKUPF", &self.WKUPF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1CIFCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1CIFCLRR {{ LSIRDYF: {=bool:?}, LSERDYF: {=bool:?}, HSIRDYF: {=bool:?}, HSERDYF: {=bool:?}, MSIRDYF: {=bool:?}, PLL1RDYF: {=bool:?}, PLL2RDYF: {=bool:?}, PLL3RDYF: {=bool:?}, PLL4RDYF: {=bool:?}, PLL5RDYF: {=bool:?}, PLL6RDYF: {=bool:?}, PLL7RDYF: {=bool:?}, PLL8RDYF: {=bool:?}, LSECSSF: {=bool:?}, WKUPF: {=bool:?} }}",
            self.LSIRDYF(),
            self.LSERDYF(),
            self.HSIRDYF(),
            self.HSERDYF(),
            self.MSIRDYF(),
            self.PLL1RDYF(),
            self.PLL2RDYF(),
            self.PLL3RDYF(),
            self.PLL4RDYF(),
            self.PLL5RDYF(),
            self.PLL6RDYF(),
            self.PLL7RDYF(),
            self.PLL8RDYF(),
            self.LSECSSF(),
            self.WKUPF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1HWRSTSCLRR(pub u32);
impl RCC_C1HWRSTSCLRR {
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCPURSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCPU&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCPURSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1RSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 reset flag"]
    #[inline(always)]
    pub const fn set_C1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C1P1RSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 processor core 1 reset flag"]
    #[inline(always)]
    pub const fn set_C1P1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_C1HWRSTSCLRR {
    #[inline(always)]
    fn default() -> RCC_C1HWRSTSCLRR {
        RCC_C1HWRSTSCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C1HWRSTSCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1HWRSTSCLRR")
            .field("VCPURSTF", &self.VCPURSTF())
            .field("C1RSTF", &self.C1RSTF())
            .field("C1P1RSTF", &self.C1P1RSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1HWRSTSCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1HWRSTSCLRR {{ VCPURSTF: {=bool:?}, C1RSTF: {=bool:?}, C1P1RSTF: {=bool:?} }}",
            self.VCPURSTF(),
            self.C1RSTF(),
            self.C1P1RSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1MSRDCR(pub u32);
impl RCC_C1MSRDCR {
    #[doc = "Minimum Standby reset duration"]
    #[must_use]
    #[inline(always)]
    pub const fn C1MSRD(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Minimum Standby reset duration"]
    #[inline(always)]
    pub const fn set_C1MSRD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trigger low-level pulse on NRSTC1MS pin"]
    #[must_use]
    #[inline(always)]
    pub const fn C1MSRST(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger low-level pulse on NRSTC1MS pin"]
    #[inline(always)]
    pub const fn set_C1MSRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for RCC_C1MSRDCR {
    #[inline(always)]
    fn default() -> RCC_C1MSRDCR {
        RCC_C1MSRDCR(0)
    }
}
impl core::fmt::Debug for RCC_C1MSRDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1MSRDCR")
            .field("C1MSRD", &self.C1MSRD())
            .field("C1MSRST", &self.C1MSRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1MSRDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1MSRDCR {{ C1MSRD: {=u8:?}, C1MSRST: {=bool:?} }}",
            self.C1MSRD(),
            self.C1MSRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1P1RSTCSETR(pub u32);
impl RCC_C1P1RSTCSETR {
    #[doc = "Bit fields for RCC_C2RSTCSETR register"]
    #[must_use]
    #[inline(always)]
    pub const fn C1P1PORRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_C2RSTCSETR register"]
    #[inline(always)]
    pub const fn set_C1P1PORRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_C2RSTCSETR register"]
    #[must_use]
    #[inline(always)]
    pub const fn C1P1RST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_C2RSTCSETR register"]
    #[inline(always)]
    pub const fn set_C1P1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_C1P1RSTCSETR {
    #[inline(always)]
    fn default() -> RCC_C1P1RSTCSETR {
        RCC_C1P1RSTCSETR(0)
    }
}
impl core::fmt::Debug for RCC_C1P1RSTCSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1P1RSTCSETR")
            .field("C1P1PORRST", &self.C1P1PORRST())
            .field("C1P1RST", &self.C1P1RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1P1RSTCSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1P1RSTCSETR {{ C1P1PORRST: {=bool:?}, C1P1RST: {=bool:?} }}",
            self.C1P1PORRST(),
            self.C1P1RST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1RSTCSETR(pub u32);
impl RCC_C1RSTCSETR {
    #[doc = "Bit fields for RCC_C1P1RSTCSETR register"]
    #[must_use]
    #[inline(always)]
    pub const fn C1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_C1P1RSTCSETR register"]
    #[inline(always)]
    pub const fn set_C1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_C1RSTCSETR {
    #[inline(always)]
    fn default() -> RCC_C1RSTCSETR {
        RCC_C1RSTCSETR(0)
    }
}
impl core::fmt::Debug for RCC_C1RSTCSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1RSTCSETR")
            .field("C1RST", &self.C1RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1RSTCSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_C1RSTCSETR {{ C1RST: {=bool:?} }}", self.C1RST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1SREQCLRR(pub u32);
impl RCC_C1SREQCLRR {
    #[doc = "Stop request for CPU1 processor core 0"]
    #[must_use]
    #[inline(always)]
    pub const fn STPREQ_P0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop request for CPU1 processor core 0"]
    #[inline(always)]
    pub const fn set_STPREQ_P0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop request for CPU1 processor core 1"]
    #[must_use]
    #[inline(always)]
    pub const fn STPREQ_P1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop request for CPU1 processor core 1"]
    #[inline(always)]
    pub const fn set_STPREQ_P1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enhanced CSleep request for CPU1 subsystem"]
    #[must_use]
    #[inline(always)]
    pub const fn ESLPREQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced CSleep request for CPU1 subsystem"]
    #[inline(always)]
    pub const fn set_ESLPREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_C1SREQCLRR {
    #[inline(always)]
    fn default() -> RCC_C1SREQCLRR {
        RCC_C1SREQCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C1SREQCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1SREQCLRR")
            .field("STPREQ_P0", &self.STPREQ_P0())
            .field("STPREQ_P1", &self.STPREQ_P1())
            .field("ESLPREQ", &self.ESLPREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1SREQCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1SREQCLRR {{ STPREQ_P0: {=bool:?}, STPREQ_P1: {=bool:?}, ESLPREQ: {=bool:?} }}",
            self.STPREQ_P0(),
            self.STPREQ_P1(),
            self.ESLPREQ()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C1SREQSETR(pub u32);
impl RCC_C1SREQSETR {
    #[doc = "Stop request for CPU1 processor core 0"]
    #[must_use]
    #[inline(always)]
    pub const fn STPREQ_P0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop request for CPU1 processor core 0"]
    #[inline(always)]
    pub const fn set_STPREQ_P0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop request for CPU1 processor core 1"]
    #[must_use]
    #[inline(always)]
    pub const fn STPREQ_P1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop request for CPU1 processor core 1"]
    #[inline(always)]
    pub const fn set_STPREQ_P1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enhanced CSleep request for CPU1 subsystem"]
    #[must_use]
    #[inline(always)]
    pub const fn ESLPREQ(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced CSleep request for CPU1 subsystem"]
    #[inline(always)]
    pub const fn set_ESLPREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_C1SREQSETR {
    #[inline(always)]
    fn default() -> RCC_C1SREQSETR {
        RCC_C1SREQSETR(0)
    }
}
impl core::fmt::Debug for RCC_C1SREQSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C1SREQSETR")
            .field("STPREQ_P0", &self.STPREQ_P0())
            .field("STPREQ_P1", &self.STPREQ_P1())
            .field("ESLPREQ", &self.ESLPREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C1SREQSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C1SREQSETR {{ STPREQ_P0: {=bool:?}, STPREQ_P1: {=bool:?}, ESLPREQ: {=bool:?} }}",
            self.STPREQ_P0(),
            self.STPREQ_P1(),
            self.ESLPREQ()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2BOOTRSTSCLRR(pub u32);
impl RCC_C2BOOTRSTSCLRR {
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PORRSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_PORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BORRSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOR reset flag"]
    #[inline(always)]
    pub const fn set_BORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NRST reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PADRSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRST reset flag"]
    #[inline(always)]
    pub const fn set_PADRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE CSS reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HCSSRSTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE CSS reset flag"]
    #[inline(always)]
    pub const fn set_HCSSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCORERSTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCORERSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC1RSTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CPU2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC2RSTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IWDG1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1SYSRSTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG1SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IWDG2 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2SYSRSTF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 reset flag"]
    #[inline(always)]
    pub const fn set_IWDG2SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IWDG3 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3SYSRSTF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 reset flag"]
    #[inline(always)]
    pub const fn set_IWDG3SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IWDG4 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4SYSRSTF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 reset flag"]
    #[inline(always)]
    pub const fn set_IWDG4SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IWDG5 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5SYSRSTF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 reset flag"]
    #[inline(always)]
    pub const fn set_IWDG5SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CPU2 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C2RSTF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 reset flag"]
    #[inline(always)]
    pub const fn set_C2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RETCRCERRRSTF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[inline(always)]
    pub const fn set_RETCRCERRRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILCRCRSTF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILCRCRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILRESTRSTF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILRESTRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CPU2 system Standby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STBYC2RSTF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system Standby reset flag"]
    #[inline(always)]
    pub const fn set_STBYC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "D2 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D2STBYRSTF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "D2 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D2STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for RCC_C2BOOTRSTSCLRR {
    #[inline(always)]
    fn default() -> RCC_C2BOOTRSTSCLRR {
        RCC_C2BOOTRSTSCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C2BOOTRSTSCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2BOOTRSTSCLRR")
            .field("PORRSTF", &self.PORRSTF())
            .field("BORRSTF", &self.BORRSTF())
            .field("PADRSTF", &self.PADRSTF())
            .field("HCSSRSTF", &self.HCSSRSTF())
            .field("VCORERSTF", &self.VCORERSTF())
            .field("SYSC1RSTF", &self.SYSC1RSTF())
            .field("SYSC2RSTF", &self.SYSC2RSTF())
            .field("IWDG1SYSRSTF", &self.IWDG1SYSRSTF())
            .field("IWDG2SYSRSTF", &self.IWDG2SYSRSTF())
            .field("IWDG3SYSRSTF", &self.IWDG3SYSRSTF())
            .field("IWDG4SYSRSTF", &self.IWDG4SYSRSTF())
            .field("IWDG5SYSRSTF", &self.IWDG5SYSRSTF())
            .field("C2RSTF", &self.C2RSTF())
            .field("RETCRCERRRSTF", &self.RETCRCERRRSTF())
            .field("RETECCFAILCRCRSTF", &self.RETECCFAILCRCRSTF())
            .field("RETECCFAILRESTRSTF", &self.RETECCFAILRESTRSTF())
            .field("STBYC2RSTF", &self.STBYC2RSTF())
            .field("D2STBYRSTF", &self.D2STBYRSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2BOOTRSTSCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C2BOOTRSTSCLRR {{ PORRSTF: {=bool:?}, BORRSTF: {=bool:?}, PADRSTF: {=bool:?}, HCSSRSTF: {=bool:?}, VCORERSTF: {=bool:?}, SYSC1RSTF: {=bool:?}, SYSC2RSTF: {=bool:?}, IWDG1SYSRSTF: {=bool:?}, IWDG2SYSRSTF: {=bool:?}, IWDG3SYSRSTF: {=bool:?}, IWDG4SYSRSTF: {=bool:?}, IWDG5SYSRSTF: {=bool:?}, C2RSTF: {=bool:?}, RETCRCERRRSTF: {=bool:?}, RETECCFAILCRCRSTF: {=bool:?}, RETECCFAILRESTRSTF: {=bool:?}, STBYC2RSTF: {=bool:?}, D2STBYRSTF: {=bool:?} }}",
            self.PORRSTF(),
            self.BORRSTF(),
            self.PADRSTF(),
            self.HCSSRSTF(),
            self.VCORERSTF(),
            self.SYSC1RSTF(),
            self.SYSC2RSTF(),
            self.IWDG1SYSRSTF(),
            self.IWDG2SYSRSTF(),
            self.IWDG3SYSRSTF(),
            self.IWDG4SYSRSTF(),
            self.IWDG5SYSRSTF(),
            self.C2RSTF(),
            self.RETCRCERRRSTF(),
            self.RETECCFAILCRCRSTF(),
            self.RETECCFAILRESTRSTF(),
            self.STBYC2RSTF(),
            self.D2STBYRSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2BOOTRSTSSETR(pub u32);
impl RCC_C2BOOTRSTSSETR {
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PORRSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_PORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BORRSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOR reset flag"]
    #[inline(always)]
    pub const fn set_BORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NRST reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PADRSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRST reset flag"]
    #[inline(always)]
    pub const fn set_PADRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE CSS reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HCSSRSTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE CSS reset flag"]
    #[inline(always)]
    pub const fn set_HCSSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCORERSTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCORERSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC1RSTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CPU2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC2RSTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IWDG1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1SYSRSTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG1SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IWDG2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2SYSRSTF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG2SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IWDG3 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3SYSRSTF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG3SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IWDG4 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4SYSRSTF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG4SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IWDG5 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5SYSRSTF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG5SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CPU2 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C2RSTF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 reset flag"]
    #[inline(always)]
    pub const fn set_C2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RETCRCERRRSTF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[inline(always)]
    pub const fn set_RETCRCERRRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILCRCRSTF(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILCRCRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILRESTRSTF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILRESTRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "CPU2 system Standby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn STBYC2RSTF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system Standby reset flag"]
    #[inline(always)]
    pub const fn set_STBYC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "D2 DStandby reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn D2STBYRSTF(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "D2 DStandby reset flag"]
    #[inline(always)]
    pub const fn set_D2STBYRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for RCC_C2BOOTRSTSSETR {
    #[inline(always)]
    fn default() -> RCC_C2BOOTRSTSSETR {
        RCC_C2BOOTRSTSSETR(0)
    }
}
impl core::fmt::Debug for RCC_C2BOOTRSTSSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2BOOTRSTSSETR")
            .field("PORRSTF", &self.PORRSTF())
            .field("BORRSTF", &self.BORRSTF())
            .field("PADRSTF", &self.PADRSTF())
            .field("HCSSRSTF", &self.HCSSRSTF())
            .field("VCORERSTF", &self.VCORERSTF())
            .field("SYSC1RSTF", &self.SYSC1RSTF())
            .field("SYSC2RSTF", &self.SYSC2RSTF())
            .field("IWDG1SYSRSTF", &self.IWDG1SYSRSTF())
            .field("IWDG2SYSRSTF", &self.IWDG2SYSRSTF())
            .field("IWDG3SYSRSTF", &self.IWDG3SYSRSTF())
            .field("IWDG4SYSRSTF", &self.IWDG4SYSRSTF())
            .field("IWDG5SYSRSTF", &self.IWDG5SYSRSTF())
            .field("C2RSTF", &self.C2RSTF())
            .field("RETCRCERRRSTF", &self.RETCRCERRRSTF())
            .field("RETECCFAILCRCRSTF", &self.RETECCFAILCRCRSTF())
            .field("RETECCFAILRESTRSTF", &self.RETECCFAILRESTRSTF())
            .field("STBYC2RSTF", &self.STBYC2RSTF())
            .field("D2STBYRSTF", &self.D2STBYRSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2BOOTRSTSSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C2BOOTRSTSSETR {{ PORRSTF: {=bool:?}, BORRSTF: {=bool:?}, PADRSTF: {=bool:?}, HCSSRSTF: {=bool:?}, VCORERSTF: {=bool:?}, SYSC1RSTF: {=bool:?}, SYSC2RSTF: {=bool:?}, IWDG1SYSRSTF: {=bool:?}, IWDG2SYSRSTF: {=bool:?}, IWDG3SYSRSTF: {=bool:?}, IWDG4SYSRSTF: {=bool:?}, IWDG5SYSRSTF: {=bool:?}, C2RSTF: {=bool:?}, RETCRCERRRSTF: {=bool:?}, RETECCFAILCRCRSTF: {=bool:?}, RETECCFAILRESTRSTF: {=bool:?}, STBYC2RSTF: {=bool:?}, D2STBYRSTF: {=bool:?} }}",
            self.PORRSTF(),
            self.BORRSTF(),
            self.PADRSTF(),
            self.HCSSRSTF(),
            self.VCORERSTF(),
            self.SYSC1RSTF(),
            self.SYSC2RSTF(),
            self.IWDG1SYSRSTF(),
            self.IWDG2SYSRSTF(),
            self.IWDG3SYSRSTF(),
            self.IWDG4SYSRSTF(),
            self.IWDG5SYSRSTF(),
            self.C2RSTF(),
            self.RETCRCERRRSTF(),
            self.RETECCFAILCRCRSTF(),
            self.RETECCFAILRESTRSTF(),
            self.STBYC2RSTF(),
            self.D2STBYRSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2CIESETR(pub u32);
impl RCC_C2CIESETR {
    #[doc = "LSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSIRDYIE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_LSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSERDYIE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_LSERDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIRDYIE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_HSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSERDYIE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt enable"]
    #[inline(always)]
    pub const fn set_HSERDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSI ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIRDYIE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSI ready interrupt enable"]
    #[inline(always)]
    pub const fn set_MSIRDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL1 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL1RDYIE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL1 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL1RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL2 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL2RDYIE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL2 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL2RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PLL3 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL3RDYIE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PLL3 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL3RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL4 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL4RDYIE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLL4 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL4RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL5 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL5RDYIE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLL5 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL5RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLL6 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL6RDYIE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLL6 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL6RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PLL7 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL7RDYIE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PLL7 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL7RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PLL8 ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL8RDYIE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PLL8 ready interrupt enable"]
    #[inline(always)]
    pub const fn set_PLL8RDYIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSE CSS interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSIE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LSE CSS interrupt enable"]
    #[inline(always)]
    pub const fn set_LSECSSIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CPU2 wake-up from CStop Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WKUPIE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 wake-up from CStop Interrupt Enable"]
    #[inline(always)]
    pub const fn set_WKUPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for RCC_C2CIESETR {
    #[inline(always)]
    fn default() -> RCC_C2CIESETR {
        RCC_C2CIESETR(0)
    }
}
impl core::fmt::Debug for RCC_C2CIESETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2CIESETR")
            .field("LSIRDYIE", &self.LSIRDYIE())
            .field("LSERDYIE", &self.LSERDYIE())
            .field("HSIRDYIE", &self.HSIRDYIE())
            .field("HSERDYIE", &self.HSERDYIE())
            .field("MSIRDYIE", &self.MSIRDYIE())
            .field("PLL1RDYIE", &self.PLL1RDYIE())
            .field("PLL2RDYIE", &self.PLL2RDYIE())
            .field("PLL3RDYIE", &self.PLL3RDYIE())
            .field("PLL4RDYIE", &self.PLL4RDYIE())
            .field("PLL5RDYIE", &self.PLL5RDYIE())
            .field("PLL6RDYIE", &self.PLL6RDYIE())
            .field("PLL7RDYIE", &self.PLL7RDYIE())
            .field("PLL8RDYIE", &self.PLL8RDYIE())
            .field("LSECSSIE", &self.LSECSSIE())
            .field("WKUPIE", &self.WKUPIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2CIESETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C2CIESETR {{ LSIRDYIE: {=bool:?}, LSERDYIE: {=bool:?}, HSIRDYIE: {=bool:?}, HSERDYIE: {=bool:?}, MSIRDYIE: {=bool:?}, PLL1RDYIE: {=bool:?}, PLL2RDYIE: {=bool:?}, PLL3RDYIE: {=bool:?}, PLL4RDYIE: {=bool:?}, PLL5RDYIE: {=bool:?}, PLL6RDYIE: {=bool:?}, PLL7RDYIE: {=bool:?}, PLL8RDYIE: {=bool:?}, LSECSSIE: {=bool:?}, WKUPIE: {=bool:?} }}",
            self.LSIRDYIE(),
            self.LSERDYIE(),
            self.HSIRDYIE(),
            self.HSERDYIE(),
            self.MSIRDYIE(),
            self.PLL1RDYIE(),
            self.PLL2RDYIE(),
            self.PLL3RDYIE(),
            self.PLL4RDYIE(),
            self.PLL5RDYIE(),
            self.PLL6RDYIE(),
            self.PLL7RDYIE(),
            self.PLL8RDYIE(),
            self.LSECSSIE(),
            self.WKUPIE()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2CIFCLRR(pub u32);
impl RCC_C2CIFCLRR {
    #[doc = "LSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSIRDYF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_LSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSERDYF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_LSERDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIRDYF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_HSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSERDYF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE ready interrupt flag"]
    #[inline(always)]
    pub const fn set_HSERDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSI ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIRDYF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSI ready interrupt flag"]
    #[inline(always)]
    pub const fn set_MSIRDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL1 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL1RDYF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL1 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL1RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL2 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL2RDYF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL2 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL2RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PLL3 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL3RDYF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PLL3 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL3RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL4 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL4RDYF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLL4 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL4RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL5 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL5RDYF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLL5 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL5RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLL6 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL6RDYF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLL6 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL6RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "PLL7 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL7RDYF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PLL7 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL7RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PLL8 ready interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLL8RDYF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PLL8 ready interrupt flag"]
    #[inline(always)]
    pub const fn set_PLL8RDYF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "LSE clock security system interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn LSECSSF(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LSE clock security system interrupt flag"]
    #[inline(always)]
    pub const fn set_LSECSSF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CPU2 wake-up from CStop interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn WKUPF(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 wake-up from CStop interrupt flag"]
    #[inline(always)]
    pub const fn set_WKUPF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for RCC_C2CIFCLRR {
    #[inline(always)]
    fn default() -> RCC_C2CIFCLRR {
        RCC_C2CIFCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C2CIFCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2CIFCLRR")
            .field("LSIRDYF", &self.LSIRDYF())
            .field("LSERDYF", &self.LSERDYF())
            .field("HSIRDYF", &self.HSIRDYF())
            .field("HSERDYF", &self.HSERDYF())
            .field("MSIRDYF", &self.MSIRDYF())
            .field("PLL1RDYF", &self.PLL1RDYF())
            .field("PLL2RDYF", &self.PLL2RDYF())
            .field("PLL3RDYF", &self.PLL3RDYF())
            .field("PLL4RDYF", &self.PLL4RDYF())
            .field("PLL5RDYF", &self.PLL5RDYF())
            .field("PLL6RDYF", &self.PLL6RDYF())
            .field("PLL7RDYF", &self.PLL7RDYF())
            .field("PLL8RDYF", &self.PLL8RDYF())
            .field("LSECSSF", &self.LSECSSF())
            .field("WKUPF", &self.WKUPF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2CIFCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C2CIFCLRR {{ LSIRDYF: {=bool:?}, LSERDYF: {=bool:?}, HSIRDYF: {=bool:?}, HSERDYF: {=bool:?}, MSIRDYF: {=bool:?}, PLL1RDYF: {=bool:?}, PLL2RDYF: {=bool:?}, PLL3RDYF: {=bool:?}, PLL4RDYF: {=bool:?}, PLL5RDYF: {=bool:?}, PLL6RDYF: {=bool:?}, PLL7RDYF: {=bool:?}, PLL8RDYF: {=bool:?}, LSECSSF: {=bool:?}, WKUPF: {=bool:?} }}",
            self.LSIRDYF(),
            self.LSERDYF(),
            self.HSIRDYF(),
            self.HSERDYF(),
            self.MSIRDYF(),
            self.PLL1RDYF(),
            self.PLL2RDYF(),
            self.PLL3RDYF(),
            self.PLL4RDYF(),
            self.PLL5RDYF(),
            self.PLL6RDYF(),
            self.PLL7RDYF(),
            self.PLL8RDYF(),
            self.LSECSSF(),
            self.WKUPF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2HWRSTSCLRR(pub u32);
impl RCC_C2HWRSTSCLRR {
    #[doc = "CPU2 reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn C2RSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 reset flag"]
    #[inline(always)]
    pub const fn set_C2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_C2HWRSTSCLRR {
    #[inline(always)]
    fn default() -> RCC_C2HWRSTSCLRR {
        RCC_C2HWRSTSCLRR(0)
    }
}
impl core::fmt::Debug for RCC_C2HWRSTSCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2HWRSTSCLRR")
            .field("C2RSTF", &self.C2RSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2HWRSTSCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_C2HWRSTSCLRR {{ C2RSTF: {=bool:?} }}", self.C2RSTF())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C2RSTCSETR(pub u32);
impl RCC_C2RSTCSETR {
    #[doc = "Bit fields for RCC_HWRSTSCLRR register"]
    #[must_use]
    #[inline(always)]
    pub const fn C2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_HWRSTSCLRR register"]
    #[inline(always)]
    pub const fn set_C2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_C2RSTCSETR {
    #[inline(always)]
    fn default() -> RCC_C2RSTCSETR {
        RCC_C2RSTCSETR(0)
    }
}
impl core::fmt::Debug for RCC_C2RSTCSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C2RSTCSETR")
            .field("C2RST", &self.C2RST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C2RSTCSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_C2RSTCSETR {{ C2RST: {=bool:?} }}", self.C2RST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_C3CFGR(pub u32);
impl RCC_C3CFGR {
    #[doc = "CPU3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn C3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPU3 reset"]
    #[inline(always)]
    pub const fn set_C3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU3 peripheral clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn C3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CPU3 peripheral clocks enable"]
    #[inline(always)]
    pub const fn set_C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU3 peripheral clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn C3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CPU3 peripheral clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_C3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CPU3 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn C3AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CPU3 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_C3AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LPTIM3 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM3C3EN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM3 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_LPTIM3C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LPTIM4 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM4C3EN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM4 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_LPTIM4C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPTIM5 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM5C3EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM5 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_LPTIM5C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPI8 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI8C3EN(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPI8 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_SPI8C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPUART1 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART1C3EN(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_LPUART1C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "I2C8 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C8C3EN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "I2C8 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_I2C8C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ADF1 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ADF1C3EN(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ADF1 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_ADF1C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "GPIOZ allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOZC3EN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "GPIOZ allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_GPIOZC3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPDMA1 allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPDMAC3EN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPDMA1 allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_LPDMAC3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RTC allocation to CPU3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCC3EN(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "RTC allocation to CPU3 enable"]
    #[inline(always)]
    pub const fn set_RTCC3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit fields for RCC_MCO1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C4C3EN(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_MCO1CFGR register"]
    #[inline(always)]
    pub const fn set_I3C4C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for RCC_C3CFGR {
    #[inline(always)]
    fn default() -> RCC_C3CFGR {
        RCC_C3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_C3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_C3CFGR")
            .field("C3RST", &self.C3RST())
            .field("C3EN", &self.C3EN())
            .field("C3LPEN", &self.C3LPEN())
            .field("C3AMEN", &self.C3AMEN())
            .field("LPTIM3C3EN", &self.LPTIM3C3EN())
            .field("LPTIM4C3EN", &self.LPTIM4C3EN())
            .field("LPTIM5C3EN", &self.LPTIM5C3EN())
            .field("SPI8C3EN", &self.SPI8C3EN())
            .field("LPUART1C3EN", &self.LPUART1C3EN())
            .field("I2C8C3EN", &self.I2C8C3EN())
            .field("ADF1C3EN", &self.ADF1C3EN())
            .field("GPIOZC3EN", &self.GPIOZC3EN())
            .field("LPDMAC3EN", &self.LPDMAC3EN())
            .field("RTCC3EN", &self.RTCC3EN())
            .field("I3C4C3EN", &self.I3C4C3EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_C3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_C3CFGR {{ C3RST: {=bool:?}, C3EN: {=bool:?}, C3LPEN: {=bool:?}, C3AMEN: {=bool:?}, LPTIM3C3EN: {=bool:?}, LPTIM4C3EN: {=bool:?}, LPTIM5C3EN: {=bool:?}, SPI8C3EN: {=bool:?}, LPUART1C3EN: {=bool:?}, I2C8C3EN: {=bool:?}, ADF1C3EN: {=bool:?}, GPIOZC3EN: {=bool:?}, LPDMAC3EN: {=bool:?}, RTCC3EN: {=bool:?}, I3C4C3EN: {=bool:?} }}",
            self.C3RST(),
            self.C3EN(),
            self.C3LPEN(),
            self.C3AMEN(),
            self.LPTIM3C3EN(),
            self.LPTIM4C3EN(),
            self.LPTIM5C3EN(),
            self.SPI8C3EN(),
            self.LPUART1C3EN(),
            self.I2C8C3EN(),
            self.ADF1C3EN(),
            self.GPIOZC3EN(),
            self.LPDMAC3EN(),
            self.RTCC3EN(),
            self.I3C4C3EN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CCICFGR(pub u32);
impl RCC_CCICFGR {
    #[doc = "CCI reset"]
    #[must_use]
    #[inline(always)]
    pub const fn CCIRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CCI reset"]
    #[inline(always)]
    pub const fn set_CCIRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CCI clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CCIEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CCI clock enable"]
    #[inline(always)]
    pub const fn set_CCIEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CCI clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn CCILPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CCI clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_CCILPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_CCICFGR {
    #[inline(always)]
    fn default() -> RCC_CCICFGR {
        RCC_CCICFGR(0)
    }
}
impl core::fmt::Debug for RCC_CCICFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CCICFGR")
            .field("CCIRST", &self.CCIRST())
            .field("CCIEN", &self.CCIEN())
            .field("CCILPEN", &self.CCILPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CCICFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CCICFGR {{ CCIRST: {=bool:?}, CCIEN: {=bool:?}, CCILPEN: {=bool:?} }}",
            self.CCIRST(),
            self.CCIEN(),
            self.CCILPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CPUBOOTCR(pub u32);
impl RCC_CPUBOOTCR {
    #[doc = "CPU2 to boot"]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_CPU2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 to boot"]
    #[inline(always)]
    pub const fn set_BOOT_CPU2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU1 boot"]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_CPU1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 boot"]
    #[inline(always)]
    pub const fn set_BOOT_CPU1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_CPUBOOTCR {
    #[inline(always)]
    fn default() -> RCC_CPUBOOTCR {
        RCC_CPUBOOTCR(0)
    }
}
impl core::fmt::Debug for RCC_CPUBOOTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CPUBOOTCR")
            .field("BOOT_CPU2", &self.BOOT_CPU2())
            .field("BOOT_CPU1", &self.BOOT_CPU1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CPUBOOTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CPUBOOTCR {{ BOOT_CPU2: {=bool:?}, BOOT_CPU1: {=bool:?} }}",
            self.BOOT_CPU2(),
            self.BOOT_CPU1()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CRCCFGR(pub u32);
impl RCC_CRCCFGR {
    #[doc = "CRC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CRC reset"]
    #[inline(always)]
    pub const fn set_CRCRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CRC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CRC clock enable"]
    #[inline(always)]
    pub const fn set_CRCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CRC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn CRCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CRC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_CRCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_CRCCFGR {
    #[inline(always)]
    fn default() -> RCC_CRCCFGR {
        RCC_CRCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_CRCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CRCCFGR")
            .field("CRCRST", &self.CRCRST())
            .field("CRCEN", &self.CRCEN())
            .field("CRCLPEN", &self.CRCLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CRCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CRCCFGR {{ CRCRST: {=bool:?}, CRCEN: {=bool:?}, CRCLPEN: {=bool:?} }}",
            self.CRCRST(),
            self.CRCEN(),
            self.CRCLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CRYP1CFGR(pub u32);
impl RCC_CRYP1CFGR {
    #[doc = "CRYP1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP1 reset"]
    #[inline(always)]
    pub const fn set_CRYP1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CRYP1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP1 clock enable"]
    #[inline(always)]
    pub const fn set_CRYP1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CRYP1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_CRYP1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_CRYP1CFGR {
    #[inline(always)]
    fn default() -> RCC_CRYP1CFGR {
        RCC_CRYP1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_CRYP1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CRYP1CFGR")
            .field("CRYP1RST", &self.CRYP1RST())
            .field("CRYP1EN", &self.CRYP1EN())
            .field("CRYP1LPEN", &self.CRYP1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CRYP1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CRYP1CFGR {{ CRYP1RST: {=bool:?}, CRYP1EN: {=bool:?}, CRYP1LPEN: {=bool:?} }}",
            self.CRYP1RST(),
            self.CRYP1EN(),
            self.CRYP1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CRYP2CFGR(pub u32);
impl RCC_CRYP2CFGR {
    #[doc = "CRYP2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP2 reset"]
    #[inline(always)]
    pub const fn set_CRYP2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CRYP2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP2 clock enable"]
    #[inline(always)]
    pub const fn set_CRYP2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CRYP2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn CRYP2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CRYP2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_CRYP2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_CRYP2CFGR {
    #[inline(always)]
    fn default() -> RCC_CRYP2CFGR {
        RCC_CRYP2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_CRYP2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CRYP2CFGR")
            .field("CRYP2RST", &self.CRYP2RST())
            .field("CRYP2EN", &self.CRYP2EN())
            .field("CRYP2LPEN", &self.CRYP2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CRYP2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CRYP2CFGR {{ CRYP2RST: {=bool:?}, CRYP2EN: {=bool:?}, CRYP2LPEN: {=bool:?} }}",
            self.CRYP2RST(),
            self.CRYP2EN(),
            self.CRYP2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_CSI2CFGR(pub u32);
impl RCC_CSI2CFGR {
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CSI2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[inline(always)]
    pub const fn set_CSI2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CSI2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[inline(always)]
    pub const fn set_CSI2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CSI2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_DCMIPPCFGR register"]
    #[inline(always)]
    pub const fn set_CSI2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_CSI2CFGR {
    #[inline(always)]
    fn default() -> RCC_CSI2CFGR {
        RCC_CSI2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_CSI2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSI2CFGR")
            .field("CSI2RST", &self.CSI2RST())
            .field("CSI2EN", &self.CSI2EN())
            .field("CSI2LPEN", &self.CSI2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_CSI2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_CSI2CFGR {{ CSI2RST: {=bool:?}, CSI2EN: {=bool:?}, CSI2LPEN: {=bool:?} }}",
            self.CSI2RST(),
            self.CSI2EN(),
            self.CSI2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_D3DCR(pub u32);
impl RCC_D3DCR {
    #[doc = "MSI oscillator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MSION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MSI oscillator enable"]
    #[inline(always)]
    pub const fn set_MSION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set by software to force the MSI on, even in Stop mode, in order to be quickly available as kernel"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIKERON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set by software to force the MSI on, even in Stop mode, in order to be quickly available as kernel"]
    #[inline(always)]
    pub const fn set_MSIKERON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MSI clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn MSIRDY(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MSI clock ready flag"]
    #[inline(always)]
    pub const fn set_MSIRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0x00010000"]
    #[must_use]
    #[inline(always)]
    pub const fn D3PERCKSEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "0x00010000"]
    #[inline(always)]
    pub const fn set_D3PERCKSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for RCC_D3DCR {
    #[inline(always)]
    fn default() -> RCC_D3DCR {
        RCC_D3DCR(0)
    }
}
impl core::fmt::Debug for RCC_D3DCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_D3DCR")
            .field("MSION", &self.MSION())
            .field("MSIKERON", &self.MSIKERON())
            .field("MSIRDY", &self.MSIRDY())
            .field("D3PERCKSEL", &self.D3PERCKSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_D3DCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_D3DCR {{ MSION: {=bool:?}, MSIKERON: {=bool:?}, MSIRDY: {=bool:?}, D3PERCKSEL: {=u8:?} }}",
            self.MSION(),
            self.MSIKERON(),
            self.MSIRDY(),
            self.D3PERCKSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_D3DSR(pub u32);
impl RCC_D3DSR {
    #[doc = "D3 domain state"]
    #[must_use]
    #[inline(always)]
    pub const fn D3STATE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "D3 domain state"]
    #[inline(always)]
    pub const fn set_D3STATE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for RCC_D3DSR {
    #[inline(always)]
    fn default() -> RCC_D3DSR {
        RCC_D3DSR(0)
    }
}
impl core::fmt::Debug for RCC_D3DSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_D3DSR")
            .field("D3STATE", &self.D3STATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_D3DSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_D3DSR {{ D3STATE: {=u8:?} }}", self.D3STATE())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DBGCFGR(pub u32);
impl RCC_DBGCFGR {
    #[doc = "Debug function clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DBGEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug function clock enable"]
    #[inline(always)]
    pub const fn set_DBGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock enable for trace port interface"]
    #[must_use]
    #[inline(always)]
    pub const fn TRACEEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Clock enable for trace port interface"]
    #[inline(always)]
    pub const fn set_TRACEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Debug function reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DBGRST(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Debug function reset"]
    #[inline(always)]
    pub const fn set_DBGRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for RCC_DBGCFGR {
    #[inline(always)]
    fn default() -> RCC_DBGCFGR {
        RCC_DBGCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DBGCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DBGCFGR")
            .field("DBGEN", &self.DBGEN())
            .field("TRACEEN", &self.TRACEEN())
            .field("DBGRST", &self.DBGRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DBGCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DBGCFGR {{ DBGEN: {=bool:?}, TRACEEN: {=bool:?}, DBGRST: {=bool:?} }}",
            self.DBGEN(),
            self.TRACEEN(),
            self.DBGRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DCMIPPCFGR(pub u32);
impl RCC_DCMIPPCFGR {
    #[doc = "DCMIPP reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DCMIPPRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCMIPP reset"]
    #[inline(always)]
    pub const fn set_DCMIPPRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DCMIPP clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DCMIPPEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DCMIPP clock enable"]
    #[inline(always)]
    pub const fn set_DCMIPPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DCMIPP clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DCMIPPLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DCMIPP clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DCMIPPLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_DCMIPPCFGR {
    #[inline(always)]
    fn default() -> RCC_DCMIPPCFGR {
        RCC_DCMIPPCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DCMIPPCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DCMIPPCFGR")
            .field("DCMIPPRST", &self.DCMIPPRST())
            .field("DCMIPPEN", &self.DCMIPPEN())
            .field("DCMIPPLPEN", &self.DCMIPPLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DCMIPPCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DCMIPPCFGR {{ DCMIPPRST: {=bool:?}, DCMIPPEN: {=bool:?}, DCMIPPLPEN: {=bool:?} }}",
            self.DCMIPPRST(),
            self.DCMIPPEN(),
            self.DCMIPPLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRCAPBCFGR(pub u32);
impl RCC_DDRCAPBCFGR {
    #[doc = "DDRC APB reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCAPBRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC APB reset"]
    #[inline(always)]
    pub const fn set_DDRCAPBRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DDRC APB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCAPBEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC APB clock enable"]
    #[inline(always)]
    pub const fn set_DDRCAPBEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDRC APB clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCAPBLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC APB clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DDRCAPBLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_DDRCAPBCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRCAPBCFGR {
        RCC_DDRCAPBCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRCAPBCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRCAPBCFGR")
            .field("DDRCAPBRST", &self.DDRCAPBRST())
            .field("DDRCAPBEN", &self.DDRCAPBEN())
            .field("DDRCAPBLPEN", &self.DDRCAPBLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRCAPBCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRCAPBCFGR {{ DDRCAPBRST: {=bool:?}, DDRCAPBEN: {=bool:?}, DDRCAPBLPEN: {=bool:?} }}",
            self.DDRCAPBRST(),
            self.DDRCAPBEN(),
            self.DDRCAPBLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRCFGR(pub u32);
impl RCC_DDRCFGR {
    #[doc = "DDR APB configuration reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCFGRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DDR APB configuration reset"]
    #[inline(always)]
    pub const fn set_DDRCFGRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DDR APB configuration clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCFGEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DDR APB configuration clock enable"]
    #[inline(always)]
    pub const fn set_DDRCFGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDR APB configuration clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCFGLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDR APB configuration clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DDRCFGLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_DDRCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRCFGR {
        RCC_DDRCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRCFGR")
            .field("DDRCFGRST", &self.DDRCFGRST())
            .field("DDRCFGEN", &self.DDRCFGEN())
            .field("DDRCFGLPEN", &self.DDRCFGLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRCFGR {{ DDRCFGRST: {=bool:?}, DDRCFGEN: {=bool:?}, DDRCFGLPEN: {=bool:?} }}",
            self.DDRCFGRST(),
            self.DDRCFGEN(),
            self.DDRCFGLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRCPCFGR(pub u32);
impl RCC_DDRCPCFGR {
    #[doc = "DDRC AXI port reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCPRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC AXI port reset"]
    #[inline(always)]
    pub const fn set_DDRCPRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DDRC AXI port clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCPEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC AXI port clock enable"]
    #[inline(always)]
    pub const fn set_DDRCPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDRC AXI port clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCPLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDRC AXI port clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DDRCPLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_DDRCPCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRCPCFGR {
        RCC_DDRCPCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRCPCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRCPCFGR")
            .field("DDRCPRST", &self.DDRCPRST())
            .field("DDRCPEN", &self.DDRCPEN())
            .field("DDRCPLPEN", &self.DDRCPLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRCPCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRCPCFGR {{ DDRCPRST: {=bool:?}, DDRCPEN: {=bool:?}, DDRCPLPEN: {=bool:?} }}",
            self.DDRCPRST(),
            self.DDRCPEN(),
            self.DDRCPLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRITFCFGR(pub u32);
impl RCC_DDRITFCFGR {
    #[doc = "DDR core and PHY reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DDR core and PHY reset"]
    #[inline(always)]
    pub const fn set_DDRRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RCC mode for DDR clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRCKMOD(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "RCC mode for DDR clock control"]
    #[inline(always)]
    pub const fn set_DDRCKMOD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DDR peripheral mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRSHR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DDR peripheral mode"]
    #[inline(always)]
    pub const fn set_DDRSHR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DDRPHY deep low-power mode (LP2 or LP3)"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRPHYDLP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRPHY deep low-power mode (LP2 or LP3)"]
    #[inline(always)]
    pub const fn set_DDRPHYDLP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_DDRITFCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRITFCFGR {
        RCC_DDRITFCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRITFCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRITFCFGR")
            .field("DDRRST", &self.DDRRST())
            .field("DDRCKMOD", &self.DDRCKMOD())
            .field("DDRSHR", &self.DDRSHR())
            .field("DDRPHYDLP", &self.DDRPHYDLP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRITFCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRITFCFGR {{ DDRRST: {=bool:?}, DDRCKMOD: {=u8:?}, DDRSHR: {=bool:?}, DDRPHYDLP: {=bool:?} }}",
            self.DDRRST(),
            self.DDRCKMOD(),
            self.DDRSHR(),
            self.DDRPHYDLP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRPHYCAPBCFGR(pub u32);
impl RCC_DDRPHYCAPBCFGR {
    #[doc = "DDRPHYC APB reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRPHYCAPBRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DDRPHYC APB reset"]
    #[inline(always)]
    pub const fn set_DDRPHYCAPBRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DDRPHYC APB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRPHYCAPBEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DDRPHYC APB clock enable"]
    #[inline(always)]
    pub const fn set_DDRPHYCAPBEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDRPHYC APB clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRPHYCAPBLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDRPHYC APB clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DDRPHYCAPBLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_DDRPHYCAPBCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRPHYCAPBCFGR {
        RCC_DDRPHYCAPBCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRPHYCAPBCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRPHYCAPBCFGR")
            .field("DDRPHYCAPBRST", &self.DDRPHYCAPBRST())
            .field("DDRPHYCAPBEN", &self.DDRPHYCAPBEN())
            .field("DDRPHYCAPBLPEN", &self.DDRPHYCAPBLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRPHYCAPBCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRPHYCAPBCFGR {{ DDRPHYCAPBRST: {=bool:?}, DDRPHYCAPBEN: {=bool:?}, DDRPHYCAPBLPEN: {=bool:?} }}",
            self.DDRPHYCAPBRST(),
            self.DDRPHYCAPBEN(),
            self.DDRPHYCAPBLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DDRPHYCCFGR(pub u32);
impl RCC_DDRPHYCCFGR {
    #[doc = "DDRPHYC kernel clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DDRPHYCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DDRPHYC kernel clock enable"]
    #[inline(always)]
    pub const fn set_DDRPHYCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_DDRPHYCCFGR {
    #[inline(always)]
    fn default() -> RCC_DDRPHYCCFGR {
        RCC_DDRPHYCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DDRPHYCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DDRPHYCCFGR")
            .field("DDRPHYCEN", &self.DDRPHYCEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DDRPHYCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DDRPHYCCFGR {{ DDRPHYCEN: {=bool:?} }}",
            self.DDRPHYCEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DSICFGR(pub u32);
impl RCC_DSICFGR {
    #[doc = "DSI reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DSI reset"]
    #[inline(always)]
    pub const fn set_DSIRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DSI clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DSI clock enable"]
    #[inline(always)]
    pub const fn set_DSIEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DSI clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DSILPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DSI clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DSILPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DSI byte lane clock source selection"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIBLSEL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DSI byte lane clock source selection"]
    #[inline(always)]
    pub const fn set_DSIBLSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DSIPHY reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn DSIPHYCKREFSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DSIPHY reference clock selection"]
    #[inline(always)]
    pub const fn set_DSIPHYCKREFSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RCC_DSICFGR {
    #[inline(always)]
    fn default() -> RCC_DSICFGR {
        RCC_DSICFGR(0)
    }
}
impl core::fmt::Debug for RCC_DSICFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DSICFGR")
            .field("DSIRST", &self.DSIRST())
            .field("DSIEN", &self.DSIEN())
            .field("DSILPEN", &self.DSILPEN())
            .field("DSIBLSEL", &self.DSIBLSEL())
            .field("DSIPHYCKREFSEL", &self.DSIPHYCKREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DSICFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DSICFGR {{ DSIRST: {=bool:?}, DSIEN: {=bool:?}, DSILPEN: {=bool:?}, DSIBLSEL: {=bool:?}, DSIPHYCKREFSEL: {=bool:?} }}",
            self.DSIRST(),
            self.DSIEN(),
            self.DSILPEN(),
            self.DSIBLSEL(),
            self.DSIPHYCKREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_DTSCFGR(pub u32);
impl RCC_DTSCFGR {
    #[doc = "DTS reset"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DTS reset"]
    #[inline(always)]
    pub const fn set_DTSRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DTS clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DTS clock enable"]
    #[inline(always)]
    pub const fn set_DTSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DTS clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DTS clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_DTSLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DTS kernel clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn DTSKERSEL(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "DTS kernel clock selection"]
    #[inline(always)]
    pub const fn set_DTSKERSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for RCC_DTSCFGR {
    #[inline(always)]
    fn default() -> RCC_DTSCFGR {
        RCC_DTSCFGR(0)
    }
}
impl core::fmt::Debug for RCC_DTSCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_DTSCFGR")
            .field("DTSRST", &self.DTSRST())
            .field("DTSEN", &self.DTSEN())
            .field("DTSLPEN", &self.DTSLPEN())
            .field("DTSKERSEL", &self.DTSKERSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_DTSCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_DTSCFGR {{ DTSRST: {=bool:?}, DTSEN: {=bool:?}, DTSLPEN: {=bool:?}, DTSKERSEL: {=u8:?} }}",
            self.DTSRST(),
            self.DTSEN(),
            self.DTSLPEN(),
            self.DTSKERSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETH1CFGR(pub u32);
impl RCC_ETH1CFGR {
    #[doc = "Ethernet 1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 reset"]
    #[inline(always)]
    pub const fn set_ETH1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ethernet 1 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1MACEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 bus clock enable"]
    #[inline(always)]
    pub const fn set_ETH1MACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ethernet 1 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1MACLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH1MACLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ethernet 1 kernel clocks enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1STPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 kernel clocks enable during CStop"]
    #[inline(always)]
    pub const fn set_ETH1STPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Ethernet 1 kernel clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 kernel clock enable"]
    #[inline(always)]
    pub const fn set_ETH1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Ethernet 1 kernel clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1LPEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 kernel clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ethernet 1 transmission clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1TXEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 transmission clock enable"]
    #[inline(always)]
    pub const fn set_ETH1TXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ethernet 1 transmission clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1TXLPEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 transmission clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH1TXLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Ethernet 1 reception clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1RXEN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 reception clock enable"]
    #[inline(always)]
    pub const fn set_ETH1RXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Ethernet 1 reception clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH1RXLPEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 1 reception clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH1RXLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for RCC_ETH1CFGR {
    #[inline(always)]
    fn default() -> RCC_ETH1CFGR {
        RCC_ETH1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETH1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETH1CFGR")
            .field("ETH1RST", &self.ETH1RST())
            .field("ETH1MACEN", &self.ETH1MACEN())
            .field("ETH1MACLPEN", &self.ETH1MACLPEN())
            .field("ETH1STPEN", &self.ETH1STPEN())
            .field("ETH1EN", &self.ETH1EN())
            .field("ETH1LPEN", &self.ETH1LPEN())
            .field("ETH1TXEN", &self.ETH1TXEN())
            .field("ETH1TXLPEN", &self.ETH1TXLPEN())
            .field("ETH1RXEN", &self.ETH1RXEN())
            .field("ETH1RXLPEN", &self.ETH1RXLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETH1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETH1CFGR {{ ETH1RST: {=bool:?}, ETH1MACEN: {=bool:?}, ETH1MACLPEN: {=bool:?}, ETH1STPEN: {=bool:?}, ETH1EN: {=bool:?}, ETH1LPEN: {=bool:?}, ETH1TXEN: {=bool:?}, ETH1TXLPEN: {=bool:?}, ETH1RXEN: {=bool:?}, ETH1RXLPEN: {=bool:?} }}",
            self.ETH1RST(),
            self.ETH1MACEN(),
            self.ETH1MACLPEN(),
            self.ETH1STPEN(),
            self.ETH1EN(),
            self.ETH1LPEN(),
            self.ETH1TXEN(),
            self.ETH1TXLPEN(),
            self.ETH1RXEN(),
            self.ETH1RXLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETH2CFGR(pub u32);
impl RCC_ETH2CFGR {
    #[doc = "Ethernet 2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 reset"]
    #[inline(always)]
    pub const fn set_ETH2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ethernet 2 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2MACEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 bus clock enable"]
    #[inline(always)]
    pub const fn set_ETH2MACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ethernet 2 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2MACLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH2MACLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ethernet 2 kernel clocks enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2STPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 kernel clocks enable during CStop"]
    #[inline(always)]
    pub const fn set_ETH2STPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Ethernet 2 kernel clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2EN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 kernel clocks enable"]
    #[inline(always)]
    pub const fn set_ETH2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Ethernet 2 kernel clocks enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2LPEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 kernel clocks enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ethernet 2 transmission clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2TXEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 transmission clock enable"]
    #[inline(always)]
    pub const fn set_ETH2TXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ethernet 2 transmission clock enable during CSleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2TXLPEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 transmission clock enable during CSleep mode"]
    #[inline(always)]
    pub const fn set_ETH2TXLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Ethernet 2 reception clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2RXEN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 reception clock enable"]
    #[inline(always)]
    pub const fn set_ETH2RXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Ethernet 2 reception clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETH2RXLPEN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet 2 reception clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETH2RXLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for RCC_ETH2CFGR {
    #[inline(always)]
    fn default() -> RCC_ETH2CFGR {
        RCC_ETH2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETH2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETH2CFGR")
            .field("ETH2RST", &self.ETH2RST())
            .field("ETH2MACEN", &self.ETH2MACEN())
            .field("ETH2MACLPEN", &self.ETH2MACLPEN())
            .field("ETH2STPEN", &self.ETH2STPEN())
            .field("ETH2EN", &self.ETH2EN())
            .field("ETH2LPEN", &self.ETH2LPEN())
            .field("ETH2TXEN", &self.ETH2TXEN())
            .field("ETH2TXLPEN", &self.ETH2TXLPEN())
            .field("ETH2RXEN", &self.ETH2RXEN())
            .field("ETH2RXLPEN", &self.ETH2RXLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETH2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETH2CFGR {{ ETH2RST: {=bool:?}, ETH2MACEN: {=bool:?}, ETH2MACLPEN: {=bool:?}, ETH2STPEN: {=bool:?}, ETH2EN: {=bool:?}, ETH2LPEN: {=bool:?}, ETH2TXEN: {=bool:?}, ETH2TXLPEN: {=bool:?}, ETH2RXEN: {=bool:?}, ETH2RXLPEN: {=bool:?} }}",
            self.ETH2RST(),
            self.ETH2MACEN(),
            self.ETH2MACLPEN(),
            self.ETH2STPEN(),
            self.ETH2EN(),
            self.ETH2LPEN(),
            self.ETH2TXEN(),
            self.ETH2TXLPEN(),
            self.ETH2RXEN(),
            self.ETH2RXLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETHSWACMCFGR(pub u32);
impl RCC_ETHSWACMCFGR {
    #[doc = "ETHSW_ACM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWACMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW_ACM clock enable"]
    #[inline(always)]
    pub const fn set_ETHSWACMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETHSW_ACM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWACMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW_ACM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETHSWACMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_ETHSWACMCFGR {
    #[inline(always)]
    fn default() -> RCC_ETHSWACMCFGR {
        RCC_ETHSWACMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETHSWACMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETHSWACMCFGR")
            .field("ETHSWACMEN", &self.ETHSWACMEN())
            .field("ETHSWACMLPEN", &self.ETHSWACMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETHSWACMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETHSWACMCFGR {{ ETHSWACMEN: {=bool:?}, ETHSWACMLPEN: {=bool:?} }}",
            self.ETHSWACMEN(),
            self.ETHSWACMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETHSWACMMSGCFGR(pub u32);
impl RCC_ETHSWACMMSGCFGR {
    #[doc = "ETHSW_ACM message buffer clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWACMMSGEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW_ACM message buffer clock enable"]
    #[inline(always)]
    pub const fn set_ETHSWACMMSGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETHSW_ACM message buffer clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWACMMSGLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW_ACM message buffer clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETHSWACMMSGLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_ETHSWACMMSGCFGR {
    #[inline(always)]
    fn default() -> RCC_ETHSWACMMSGCFGR {
        RCC_ETHSWACMMSGCFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETHSWACMMSGCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETHSWACMMSGCFGR")
            .field("ETHSWACMMSGEN", &self.ETHSWACMMSGEN())
            .field("ETHSWACMMSGLPEN", &self.ETHSWACMMSGLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETHSWACMMSGCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETHSWACMMSGCFGR {{ ETHSWACMMSGEN: {=bool:?}, ETHSWACMMSGLPEN: {=bool:?} }}",
            self.ETHSWACMMSGEN(),
            self.ETHSWACMMSGLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETHSWCFGR(pub u32);
impl RCC_ETHSWCFGR {
    #[doc = "ETHSW reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW reset"]
    #[inline(always)]
    pub const fn set_ETHSWRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ETHSW clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWMACEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW clock enable"]
    #[inline(always)]
    pub const fn set_ETHSWMACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETHSW clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWMACLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETHSWMACLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ETHSW ck_ker_ethsw kernel clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW ck_ker_ethsw kernel clock enable"]
    #[inline(always)]
    pub const fn set_ETHSWEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ETHSW ck_ker_ethsw kernel clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWLPEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW ck_ker_ethsw kernel clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETHSWLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "ETHSW ck_ker_ethswref kernel clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWREFEN(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW ck_ker_ethswref kernel clock enable"]
    #[inline(always)]
    pub const fn set_ETHSWREFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ETHSW ck_ker_ethswref kernel clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETHSWREFLPEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ETHSW ck_ker_ethswref kernel clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETHSWREFLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for RCC_ETHSWCFGR {
    #[inline(always)]
    fn default() -> RCC_ETHSWCFGR {
        RCC_ETHSWCFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETHSWCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETHSWCFGR")
            .field("ETHSWRST", &self.ETHSWRST())
            .field("ETHSWMACEN", &self.ETHSWMACEN())
            .field("ETHSWMACLPEN", &self.ETHSWMACLPEN())
            .field("ETHSWEN", &self.ETHSWEN())
            .field("ETHSWLPEN", &self.ETHSWLPEN())
            .field("ETHSWREFEN", &self.ETHSWREFEN())
            .field("ETHSWREFLPEN", &self.ETHSWREFLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETHSWCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETHSWCFGR {{ ETHSWRST: {=bool:?}, ETHSWMACEN: {=bool:?}, ETHSWMACLPEN: {=bool:?}, ETHSWEN: {=bool:?}, ETHSWLPEN: {=bool:?}, ETHSWREFEN: {=bool:?}, ETHSWREFLPEN: {=bool:?} }}",
            self.ETHSWRST(),
            self.ETHSWMACEN(),
            self.ETHSWMACLPEN(),
            self.ETHSWEN(),
            self.ETHSWLPEN(),
            self.ETHSWREFEN(),
            self.ETHSWREFLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_ETRCFGR(pub u32);
impl RCC_ETRCFGR {
    #[doc = "ETR clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ETREN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETR clocks enable"]
    #[inline(always)]
    pub const fn set_ETREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETR clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn ETRLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETR clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_ETRLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_ETRCFGR {
    #[inline(always)]
    fn default() -> RCC_ETRCFGR {
        RCC_ETRCFGR(0)
    }
}
impl core::fmt::Debug for RCC_ETRCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_ETRCFGR")
            .field("ETREN", &self.ETREN())
            .field("ETRLPEN", &self.ETRLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_ETRCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_ETRCFGR {{ ETREN: {=bool:?}, ETRLPEN: {=bool:?} }}",
            self.ETREN(),
            self.ETRLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCCR1(pub u32);
impl RCC_FCALCCR1 {
    #[doc = "Bit fields for RCC_FCALCCR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCRUN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCCR2 register"]
    #[inline(always)]
    pub const fn set_FCALCRUN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_FCALCCR1 {
    #[inline(always)]
    fn default() -> RCC_FCALCCR1 {
        RCC_FCALCCR1(0)
    }
}
impl core::fmt::Debug for RCC_FCALCCR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCCR1")
            .field("FCALCRUN", &self.FCALCRUN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCCR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_FCALCCR1 {{ FCALCRUN: {=bool:?} }}", self.FCALCRUN())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCCR2(pub u32);
impl RCC_FCALCCR2 {
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCMD(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[inline(always)]
    pub const fn set_FCALCMD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCTWC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[inline(always)]
    pub const fn set_FCALCTWC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCTYP(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCSR register"]
    #[inline(always)]
    pub const fn set_FCALCTYP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
}
impl Default for RCC_FCALCCR2 {
    #[inline(always)]
    fn default() -> RCC_FCALCCR2 {
        RCC_FCALCCR2(0)
    }
}
impl core::fmt::Debug for RCC_FCALCCR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCCR2")
            .field("FCALCMD", &self.FCALCMD())
            .field("FCALCTWC", &self.FCALCTWC())
            .field("FCALCTYP", &self.FCALCTYP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCCR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FCALCCR2 {{ FCALCMD: {=u8:?}, FCALCTWC: {=u8:?}, FCALCTYP: {=u8:?} }}",
            self.FCALCMD(),
            self.FCALCTWC(),
            self.FCALCTYP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCOBS0CFGR(pub u32);
impl RCC_FCALCOBS0CFGR {
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKINTSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKINTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKEXTSEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKEXTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCCKEXTSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_FCALCCKEXTSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSEXTSEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSEXTSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCCKINV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_FCALCCKINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSINV(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSDIV(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCCKEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_FCALCCKEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSEN(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCOBS1CFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_FCALCOBS0CFGR {
    #[inline(always)]
    fn default() -> RCC_FCALCOBS0CFGR {
        RCC_FCALCOBS0CFGR(0)
    }
}
impl core::fmt::Debug for RCC_FCALCOBS0CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCOBS0CFGR")
            .field("CKINTSEL", &self.CKINTSEL())
            .field("CKEXTSEL", &self.CKEXTSEL())
            .field("FCALCCKEXTSEL", &self.FCALCCKEXTSEL())
            .field("CKOBSEXTSEL", &self.CKOBSEXTSEL())
            .field("FCALCCKINV", &self.FCALCCKINV())
            .field("CKOBSINV", &self.CKOBSINV())
            .field("CKOBSDIV", &self.CKOBSDIV())
            .field("FCALCCKEN", &self.FCALCCKEN())
            .field("CKOBSEN", &self.CKOBSEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCOBS0CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FCALCOBS0CFGR {{ CKINTSEL: {=u8:?}, CKEXTSEL: {=u8:?}, FCALCCKEXTSEL: {=bool:?}, CKOBSEXTSEL: {=bool:?}, FCALCCKINV: {=bool:?}, CKOBSINV: {=bool:?}, CKOBSDIV: {=u8:?}, FCALCCKEN: {=bool:?}, CKOBSEN: {=bool:?} }}",
            self.CKINTSEL(),
            self.CKEXTSEL(),
            self.FCALCCKEXTSEL(),
            self.CKOBSEXTSEL(),
            self.FCALCCKINV(),
            self.CKOBSINV(),
            self.CKOBSDIV(),
            self.FCALCCKEN(),
            self.CKOBSEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCOBS1CFGR(pub u32);
impl RCC_FCALCOBS1CFGR {
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKINTSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKINTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKEXTSEL(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKEXTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSEXTSEL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSEXTSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSINV(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSINV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSDIV(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKOBSEN(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_CKOBSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCRSTN(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FCALCREFCFGR register"]
    #[inline(always)]
    pub const fn set_FCALCRSTN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for RCC_FCALCOBS1CFGR {
    #[inline(always)]
    fn default() -> RCC_FCALCOBS1CFGR {
        RCC_FCALCOBS1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_FCALCOBS1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCOBS1CFGR")
            .field("CKINTSEL", &self.CKINTSEL())
            .field("CKEXTSEL", &self.CKEXTSEL())
            .field("CKOBSEXTSEL", &self.CKOBSEXTSEL())
            .field("CKOBSINV", &self.CKOBSINV())
            .field("CKOBSDIV", &self.CKOBSDIV())
            .field("CKOBSEN", &self.CKOBSEN())
            .field("FCALCRSTN", &self.FCALCRSTN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCOBS1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FCALCOBS1CFGR {{ CKINTSEL: {=u8:?}, CKEXTSEL: {=u8:?}, CKOBSEXTSEL: {=bool:?}, CKOBSINV: {=bool:?}, CKOBSDIV: {=u8:?}, CKOBSEN: {=bool:?}, FCALCRSTN: {=bool:?} }}",
            self.CKINTSEL(),
            self.CKEXTSEL(),
            self.CKOBSEXTSEL(),
            self.CKOBSINV(),
            self.CKOBSDIV(),
            self.CKOBSEN(),
            self.FCALCRSTN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCREFCFGR(pub u32);
impl RCC_FCALCREFCFGR {
    #[doc = "Bit fields for RCC_FCALCCR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCREFCKSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_FCALCCR1 register"]
    #[inline(always)]
    pub const fn set_FCALCREFCKSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_FCALCREFCFGR {
    #[inline(always)]
    fn default() -> RCC_FCALCREFCFGR {
        RCC_FCALCREFCFGR(0)
    }
}
impl core::fmt::Debug for RCC_FCALCREFCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCREFCFGR")
            .field("FCALCREFCKSEL", &self.FCALCREFCKSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCREFCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FCALCREFCFGR {{ FCALCREFCKSEL: {=u8:?} }}",
            self.FCALCREFCKSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FCALCSR(pub u32);
impl RCC_FCALCSR {
    #[doc = "Bit fields for RCC_PLL4CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL4CFGR1 register"]
    #[inline(always)]
    pub const fn set_FVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FCALCSTS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR1 register"]
    #[inline(always)]
    pub const fn set_FCALCSTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for RCC_FCALCSR {
    #[inline(always)]
    fn default() -> RCC_FCALCSR {
        RCC_FCALCSR(0)
    }
}
impl core::fmt::Debug for RCC_FCALCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FCALCSR")
            .field("FVAL", &self.FVAL())
            .field("FCALCSTS", &self.FCALCSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FCALCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FCALCSR {{ FVAL: {=u32:?}, FCALCSTS: {=bool:?} }}",
            self.FVAL(),
            self.FCALCSTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FDCANCFGR(pub u32);
impl RCC_FDCANCFGR {
    #[doc = "FDCAN reset"]
    #[must_use]
    #[inline(always)]
    pub const fn FDCANRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FDCAN reset"]
    #[inline(always)]
    pub const fn set_FDCANRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FDCAN clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FDCANEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FDCAN clock enable"]
    #[inline(always)]
    pub const fn set_FDCANEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FDCAN clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn FDCANLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FDCAN clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_FDCANLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_FDCANCFGR {
    #[inline(always)]
    fn default() -> RCC_FDCANCFGR {
        RCC_FDCANCFGR(0)
    }
}
impl core::fmt::Debug for RCC_FDCANCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FDCANCFGR")
            .field("FDCANRST", &self.FDCANRST())
            .field("FDCANEN", &self.FDCANEN())
            .field("FDCANLPEN", &self.FDCANLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FDCANCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FDCANCFGR {{ FDCANRST: {=bool:?}, FDCANEN: {=bool:?}, FDCANLPEN: {=bool:?} }}",
            self.FDCANRST(),
            self.FDCANEN(),
            self.FDCANLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FINDIVSR1(pub u32);
impl RCC_FINDIVSR1 {
    #[doc = "Bit fields for RCC_FINDIVSR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FINDIVSTS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_FINDIVSR2 register"]
    #[inline(always)]
    pub const fn set_FINDIVSTS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_FINDIVSR1 {
    #[inline(always)]
    fn default() -> RCC_FINDIVSR1 {
        RCC_FINDIVSR1(0)
    }
}
impl core::fmt::Debug for RCC_FINDIVSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FINDIVSR1")
            .field("FINDIVSTS", &self.FINDIVSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FINDIVSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FINDIVSR1 {{ FINDIVSTS: {=u32:?} }}",
            self.FINDIVSTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FINDIVSR2(pub u32);
impl RCC_FINDIVSR2 {
    #[doc = "Bit fields for RCC_FCALCOBS0CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn FINDIVSTS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_FCALCOBS0CFGR register"]
    #[inline(always)]
    pub const fn set_FINDIVSTS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_FINDIVSR2 {
    #[inline(always)]
    fn default() -> RCC_FINDIVSR2 {
        RCC_FINDIVSR2(0)
    }
}
impl core::fmt::Debug for RCC_FINDIVSR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FINDIVSR2")
            .field("FINDIVSTS", &self.FINDIVSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FINDIVSR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FINDIVSR2 {{ FINDIVSTS: {=u32:?} }}",
            self.FINDIVSTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FINDIVxCFGR(pub u32);
impl RCC_FINDIVxCFGR {
    #[doc = "Bit fields for RCC_FINDIVSR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FINDIVx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_FINDIVSR1 register"]
    #[inline(always)]
    pub const fn set_FINDIVx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_FINDIVSR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FINDIVxEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_FINDIVSR1 register"]
    #[inline(always)]
    pub const fn set_FINDIVxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for RCC_FINDIVxCFGR {
    #[inline(always)]
    fn default() -> RCC_FINDIVxCFGR {
        RCC_FINDIVxCFGR(0)
    }
}
impl core::fmt::Debug for RCC_FINDIVxCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FINDIVxCFGR")
            .field("FINDIVx", &self.FINDIVx())
            .field("FINDIVxEN", &self.FINDIVxEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FINDIVxCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FINDIVxCFGR {{ FINDIVx: {=u8:?}, FINDIVxEN: {=bool:?} }}",
            self.FINDIVx(),
            self.FINDIVxEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_FMCCFGR(pub u32);
impl RCC_FMCCFGR {
    #[doc = "FMC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn FMCRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FMC reset"]
    #[inline(always)]
    pub const fn set_FMCRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FMC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FMCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FMC clock enable"]
    #[inline(always)]
    pub const fn set_FMCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FMC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn FMCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FMC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_FMCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_FMCCFGR {
    #[inline(always)]
    fn default() -> RCC_FMCCFGR {
        RCC_FMCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_FMCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_FMCCFGR")
            .field("FMCRST", &self.FMCRST())
            .field("FMCEN", &self.FMCEN())
            .field("FMCLPEN", &self.FMCLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_FMCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_FMCCFGR {{ FMCRST: {=bool:?}, FMCEN: {=bool:?}, FMCLPEN: {=bool:?} }}",
            self.FMCRST(),
            self.FMCEN(),
            self.FMCLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GICV2MCFGR(pub u32);
impl RCC_GICV2MCFGR {
    #[doc = "GICV2M clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn GICV2MEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GICV2M clock enable"]
    #[inline(always)]
    pub const fn set_GICV2MEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GICV2M clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn GICV2MLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GICV2M clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_GICV2MLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GICV2MCFGR {
    #[inline(always)]
    fn default() -> RCC_GICV2MCFGR {
        RCC_GICV2MCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GICV2MCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GICV2MCFGR")
            .field("GICV2MEN", &self.GICV2MEN())
            .field("GICV2MLPEN", &self.GICV2MLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GICV2MCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GICV2MCFGR {{ GICV2MEN: {=bool:?}, GICV2MLPEN: {=bool:?} }}",
            self.GICV2MEN(),
            self.GICV2MLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOACFGR(pub u32);
impl RCC_GPIOACFGR {
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOBCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOACFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOACFGR {
        RCC_GPIOACFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOACFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOACFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOACFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOACFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOBCFGR(pub u32);
impl RCC_GPIOBCFGR {
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOCCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOBCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOBCFGR {
        RCC_GPIOBCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOBCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOBCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOBCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOBCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOCCFGR(pub u32);
impl RCC_GPIOCCFGR {
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIODCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOCCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOCCFGR {
        RCC_GPIOCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOCCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOCCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIODCFGR(pub u32);
impl RCC_GPIODCFGR {
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOECFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIODCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIODCFGR {
        RCC_GPIODCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIODCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIODCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIODCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIODCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOECFGR(pub u32);
impl RCC_GPIOECFGR {
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOFCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOECFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOECFGR {
        RCC_GPIOECFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOECFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOECFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOECFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOECFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOFCFGR(pub u32);
impl RCC_GPIOFCFGR {
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOGCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOFCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOFCFGR {
        RCC_GPIOFCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOFCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOFCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOFCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOFCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOGCFGR(pub u32);
impl RCC_GPIOGCFGR {
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOHCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOGCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOGCFGR {
        RCC_GPIOGCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOGCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOGCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOGCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOGCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOHCFGR(pub u32);
impl RCC_GPIOHCFGR {
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOICFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOHCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOHCFGR {
        RCC_GPIOHCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOHCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOHCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOHCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOHCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOICFGR(pub u32);
impl RCC_GPIOICFGR {
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOJCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOICFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOICFGR {
        RCC_GPIOICFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOICFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOICFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOICFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOICFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOJCFGR(pub u32);
impl RCC_GPIOJCFGR {
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOKCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOJCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOJCFGR {
        RCC_GPIOJCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOJCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOJCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOJCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOJCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOKCFGR(pub u32);
impl RCC_GPIOKCFGR {
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOxLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_GPIOZCFGR register"]
    #[inline(always)]
    pub const fn set_GPIOxLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPIOKCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOKCFGR {
        RCC_GPIOKCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOKCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOKCFGR")
            .field("GPIOxRST", &self.GPIOxRST())
            .field("GPIOxEN", &self.GPIOxEN())
            .field("GPIOxLPEN", &self.GPIOxLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOKCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOKCFGR {{ GPIOxRST: {=bool:?}, GPIOxEN: {=bool:?}, GPIOxLPEN: {=bool:?} }}",
            self.GPIOxRST(),
            self.GPIOxEN(),
            self.GPIOxLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPIOZCFGR(pub u32);
impl RCC_GPIOZCFGR {
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOZRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[inline(always)]
    pub const fn set_GPIOZRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOZEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[inline(always)]
    pub const fn set_GPIOZEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOZLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[inline(always)]
    pub const fn set_GPIOZLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[must_use]
    #[inline(always)]
    pub const fn GPIOZAMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_HPDMA1CFGR register"]
    #[inline(always)]
    pub const fn set_GPIOZAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_GPIOZCFGR {
    #[inline(always)]
    fn default() -> RCC_GPIOZCFGR {
        RCC_GPIOZCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPIOZCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPIOZCFGR")
            .field("GPIOZRST", &self.GPIOZRST())
            .field("GPIOZEN", &self.GPIOZEN())
            .field("GPIOZLPEN", &self.GPIOZLPEN())
            .field("GPIOZAMEN", &self.GPIOZAMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPIOZCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPIOZCFGR {{ GPIOZRST: {=bool:?}, GPIOZEN: {=bool:?}, GPIOZLPEN: {=bool:?}, GPIOZAMEN: {=bool:?} }}",
            self.GPIOZRST(),
            self.GPIOZEN(),
            self.GPIOZLPEN(),
            self.GPIOZAMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GPUCFGR(pub u32);
impl RCC_GPUCFGR {
    #[doc = "GPU reset"]
    #[must_use]
    #[inline(always)]
    pub const fn GPURST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GPU reset"]
    #[inline(always)]
    pub const fn set_GPURST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "GPU clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn GPUEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPU clock enable"]
    #[inline(always)]
    pub const fn set_GPUEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GPU clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn GPULPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GPU clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_GPULPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_GPUCFGR {
    #[inline(always)]
    fn default() -> RCC_GPUCFGR {
        RCC_GPUCFGR(0)
    }
}
impl core::fmt::Debug for RCC_GPUCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GPUCFGR")
            .field("GPURST", &self.GPURST())
            .field("GPUEN", &self.GPUEN())
            .field("GPULPEN", &self.GPULPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GPUCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_GPUCFGR {{ GPURST: {=bool:?}, GPUEN: {=bool:?}, GPULPEN: {=bool:?} }}",
            self.GPURST(),
            self.GPUEN(),
            self.GPULPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_GRSTCSETR(pub u32);
impl RCC_GRSTCSETR {
    #[doc = "Bit fields for RCC_C1RSTCSETR register"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_C1RSTCSETR register"]
    #[inline(always)]
    pub const fn set_SYSRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_GRSTCSETR {
    #[inline(always)]
    fn default() -> RCC_GRSTCSETR {
        RCC_GRSTCSETR(0)
    }
}
impl core::fmt::Debug for RCC_GRSTCSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_GRSTCSETR")
            .field("SYSRST", &self.SYSRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_GRSTCSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_GRSTCSETR {{ SYSRST: {=bool:?} }}", self.SYSRST())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HASHCFGR(pub u32);
impl RCC_HASHCFGR {
    #[doc = "HASH reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HASHRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HASH reset"]
    #[inline(always)]
    pub const fn set_HASHRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HASH clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HASHEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HASH clock enable"]
    #[inline(always)]
    pub const fn set_HASHEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HASH clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn HASHLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HASH clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_HASHLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_HASHCFGR {
    #[inline(always)]
    fn default() -> RCC_HASHCFGR {
        RCC_HASHCFGR(0)
    }
}
impl core::fmt::Debug for RCC_HASHCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HASHCFGR")
            .field("HASHRST", &self.HASHRST())
            .field("HASHEN", &self.HASHEN())
            .field("HASHLPEN", &self.HASHLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HASHCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HASHCFGR {{ HASHRST: {=bool:?}, HASHEN: {=bool:?}, HASHLPEN: {=bool:?} }}",
            self.HASHRST(),
            self.HASHEN(),
            self.HASHLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HDPCFGR(pub u32);
impl RCC_HDPCFGR {
    #[doc = "HDP reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HDPRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HDP reset"]
    #[inline(always)]
    pub const fn set_HDPRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HDP clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HDPEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HDP clock enable"]
    #[inline(always)]
    pub const fn set_HDPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_HDPCFGR {
    #[inline(always)]
    fn default() -> RCC_HDPCFGR {
        RCC_HDPCFGR(0)
    }
}
impl core::fmt::Debug for RCC_HDPCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HDPCFGR")
            .field("HDPRST", &self.HDPRST())
            .field("HDPEN", &self.HDPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HDPCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HDPCFGR {{ HDPRST: {=bool:?}, HDPEN: {=bool:?} }}",
            self.HDPRST(),
            self.HDPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HPDMA1CFGR(pub u32);
impl RCC_HPDMA1CFGR {
    #[doc = "HPDMA1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA1 reset"]
    #[inline(always)]
    pub const fn set_HPDMA1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HPDMA1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA1 clock enable"]
    #[inline(always)]
    pub const fn set_HPDMA1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HPDMA1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_HPDMA1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_HPDMA1CFGR {
    #[inline(always)]
    fn default() -> RCC_HPDMA1CFGR {
        RCC_HPDMA1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_HPDMA1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HPDMA1CFGR")
            .field("HPDMA1RST", &self.HPDMA1RST())
            .field("HPDMA1EN", &self.HPDMA1EN())
            .field("HPDMA1LPEN", &self.HPDMA1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HPDMA1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HPDMA1CFGR {{ HPDMA1RST: {=bool:?}, HPDMA1EN: {=bool:?}, HPDMA1LPEN: {=bool:?} }}",
            self.HPDMA1RST(),
            self.HPDMA1EN(),
            self.HPDMA1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HPDMA2CFGR(pub u32);
impl RCC_HPDMA2CFGR {
    #[doc = "HPDMA2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA2 reset"]
    #[inline(always)]
    pub const fn set_HPDMA2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HPDMA2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA2 clock enable"]
    #[inline(always)]
    pub const fn set_HPDMA2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HPDMA2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_HPDMA2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_HPDMA2CFGR {
    #[inline(always)]
    fn default() -> RCC_HPDMA2CFGR {
        RCC_HPDMA2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_HPDMA2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HPDMA2CFGR")
            .field("HPDMA2RST", &self.HPDMA2RST())
            .field("HPDMA2EN", &self.HPDMA2EN())
            .field("HPDMA2LPEN", &self.HPDMA2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HPDMA2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HPDMA2CFGR {{ HPDMA2RST: {=bool:?}, HPDMA2EN: {=bool:?}, HPDMA2LPEN: {=bool:?} }}",
            self.HPDMA2RST(),
            self.HPDMA2EN(),
            self.HPDMA2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HPDMA3CFGR(pub u32);
impl RCC_HPDMA3CFGR {
    #[doc = "HPDMA3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA3 reset"]
    #[inline(always)]
    pub const fn set_HPDMA3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HPDMA3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA3 clock enable"]
    #[inline(always)]
    pub const fn set_HPDMA3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HPDMA3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn HPDMA3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HPDMA3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_HPDMA3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_HPDMA3CFGR {
    #[inline(always)]
    fn default() -> RCC_HPDMA3CFGR {
        RCC_HPDMA3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_HPDMA3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HPDMA3CFGR")
            .field("HPDMA3RST", &self.HPDMA3RST())
            .field("HPDMA3EN", &self.HPDMA3EN())
            .field("HPDMA3LPEN", &self.HPDMA3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HPDMA3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HPDMA3CFGR {{ HPDMA3RST: {=bool:?}, HPDMA3EN: {=bool:?}, HPDMA3LPEN: {=bool:?} }}",
            self.HPDMA3RST(),
            self.HPDMA3EN(),
            self.HPDMA3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HSEMCFGR(pub u32);
impl RCC_HSEMCFGR {
    #[doc = "HSEM reset"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEMRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HSEM reset"]
    #[inline(always)]
    pub const fn set_HSEMRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HSEM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HSEM clock enable"]
    #[inline(always)]
    pub const fn set_HSEMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSEM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HSEM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_HSEMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSEM autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEMAMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSEM autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_HSEMAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_HSEMCFGR {
    #[inline(always)]
    fn default() -> RCC_HSEMCFGR {
        RCC_HSEMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_HSEMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HSEMCFGR")
            .field("HSEMRST", &self.HSEMRST())
            .field("HSEMEN", &self.HSEMEN())
            .field("HSEMLPEN", &self.HSEMLPEN())
            .field("HSEMAMEN", &self.HSEMAMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HSEMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HSEMCFGR {{ HSEMRST: {=bool:?}, HSEMEN: {=bool:?}, HSEMLPEN: {=bool:?}, HSEMAMEN: {=bool:?} }}",
            self.HSEMRST(),
            self.HSEMEN(),
            self.HSEMLPEN(),
            self.HSEMAMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HSICFGR(pub u32);
impl RCC_HSICFGR {
    #[doc = "HSI clock trimming"]
    #[must_use]
    #[inline(always)]
    pub const fn HSITRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "HSI clock trimming"]
    #[inline(always)]
    pub const fn set_HSITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "HSI clock calibration"]
    #[must_use]
    #[inline(always)]
    pub const fn HSICAL(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "HSI clock calibration"]
    #[inline(always)]
    pub const fn set_HSICAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for RCC_HSICFGR {
    #[inline(always)]
    fn default() -> RCC_HSICFGR {
        RCC_HSICFGR(0)
    }
}
impl core::fmt::Debug for RCC_HSICFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HSICFGR")
            .field("HSITRIM", &self.HSITRIM())
            .field("HSICAL", &self.HSICAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HSICFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HSICFGR {{ HSITRIM: {=u8:?}, HSICAL: {=u16:?} }}",
            self.HSITRIM(),
            self.HSICAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HSIFMONCR(pub u32);
impl RCC_HSIFMONCR {
    #[doc = "HSI clock count reference value"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIREF(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "HSI clock count reference value"]
    #[inline(always)]
    pub const fn set_HSIREF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "HSI clock period monitor enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIMONEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "HSI clock period monitor enable"]
    #[inline(always)]
    pub const fn set_HSIMONEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "HSI clock count deviation value"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIDEV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "HSI clock count deviation value"]
    #[inline(always)]
    pub const fn set_HSIDEV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "HSI clock period monitor interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIMONIE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "HSI clock period monitor interrupt enable"]
    #[inline(always)]
    pub const fn set_HSIMONIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "HSI clock period monitor interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIMONF(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "HSI clock period monitor interrupt flag"]
    #[inline(always)]
    pub const fn set_HSIMONF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_HSIFMONCR {
    #[inline(always)]
    fn default() -> RCC_HSIFMONCR {
        RCC_HSIFMONCR(0)
    }
}
impl core::fmt::Debug for RCC_HSIFMONCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HSIFMONCR")
            .field("HSIREF", &self.HSIREF())
            .field("HSIMONEN", &self.HSIMONEN())
            .field("HSIDEV", &self.HSIDEV())
            .field("HSIMONIE", &self.HSIMONIE())
            .field("HSIMONF", &self.HSIMONF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HSIFMONCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HSIFMONCR {{ HSIREF: {=u16:?}, HSIMONEN: {=bool:?}, HSIDEV: {=u8:?}, HSIMONIE: {=bool:?}, HSIMONF: {=bool:?} }}",
            self.HSIREF(),
            self.HSIMONEN(),
            self.HSIDEV(),
            self.HSIMONIE(),
            self.HSIMONF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HSIFVALR(pub u32);
impl RCC_HSIFVALR {
    #[doc = "HSI clock-cycle counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIVAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "HSI clock-cycle counter value"]
    #[inline(always)]
    pub const fn set_HSIVAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for RCC_HSIFVALR {
    #[inline(always)]
    fn default() -> RCC_HSIFVALR {
        RCC_HSIFVALR(0)
    }
}
impl core::fmt::Debug for RCC_HSIFVALR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HSIFVALR")
            .field("HSIVAL", &self.HSIVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HSIFVALR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_HSIFVALR {{ HSIVAL: {=u16:?} }}", self.HSIVAL())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_HWRSTSCLRR(pub u32);
impl RCC_HWRSTSCLRR {
    #[doc = "POR/PDR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PORRSTF(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR/PDR reset flag"]
    #[inline(always)]
    pub const fn set_PORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOR reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn BORRSTF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOR reset flag"]
    #[inline(always)]
    pub const fn set_BORRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NRST reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PADRSTF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NRST reset flag"]
    #[inline(always)]
    pub const fn set_PADRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HSE CSS reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HCSSRSTF(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "HSE CSS reset flag"]
    #[inline(always)]
    pub const fn set_HCSSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn VCORERSTF(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "V&lt;sub&gt;DDCORE&lt;/sub&gt; reset flag"]
    #[inline(always)]
    pub const fn set_VCORERSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC1RSTF(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC1RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CPU2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSC2RSTF(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 system reset flag"]
    #[inline(always)]
    pub const fn set_SYSC2RSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IWDG1 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1SYSRSTF(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG1SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IWDG2 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2SYSRSTF(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG2SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "IWDG3 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3SYSRSTF(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG3SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IWDG4 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4SYSRSTF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG4SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "IWDG5 system reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5SYSRSTF(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 system reset flag"]
    #[inline(always)]
    pub const fn set_IWDG5SYSRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn RETCRCERRRSTF(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error reset flag"]
    #[inline(always)]
    pub const fn set_RETCRCERRRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILCRCRSTF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during the CRC computation phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILCRCRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[must_use]
    #[inline(always)]
    pub const fn RETECCFAILRESTRSTF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM ECC failure reset flag during system restoration phase"]
    #[inline(always)]
    pub const fn set_RETECCFAILRESTRSTF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for RCC_HWRSTSCLRR {
    #[inline(always)]
    fn default() -> RCC_HWRSTSCLRR {
        RCC_HWRSTSCLRR(0)
    }
}
impl core::fmt::Debug for RCC_HWRSTSCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_HWRSTSCLRR")
            .field("PORRSTF", &self.PORRSTF())
            .field("BORRSTF", &self.BORRSTF())
            .field("PADRSTF", &self.PADRSTF())
            .field("HCSSRSTF", &self.HCSSRSTF())
            .field("VCORERSTF", &self.VCORERSTF())
            .field("SYSC1RSTF", &self.SYSC1RSTF())
            .field("SYSC2RSTF", &self.SYSC2RSTF())
            .field("IWDG1SYSRSTF", &self.IWDG1SYSRSTF())
            .field("IWDG2SYSRSTF", &self.IWDG2SYSRSTF())
            .field("IWDG3SYSRSTF", &self.IWDG3SYSRSTF())
            .field("IWDG4SYSRSTF", &self.IWDG4SYSRSTF())
            .field("IWDG5SYSRSTF", &self.IWDG5SYSRSTF())
            .field("RETCRCERRRSTF", &self.RETCRCERRRSTF())
            .field("RETECCFAILCRCRSTF", &self.RETECCFAILCRCRSTF())
            .field("RETECCFAILRESTRSTF", &self.RETECCFAILRESTRSTF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_HWRSTSCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_HWRSTSCLRR {{ PORRSTF: {=bool:?}, BORRSTF: {=bool:?}, PADRSTF: {=bool:?}, HCSSRSTF: {=bool:?}, VCORERSTF: {=bool:?}, SYSC1RSTF: {=bool:?}, SYSC2RSTF: {=bool:?}, IWDG1SYSRSTF: {=bool:?}, IWDG2SYSRSTF: {=bool:?}, IWDG3SYSRSTF: {=bool:?}, IWDG4SYSRSTF: {=bool:?}, IWDG5SYSRSTF: {=bool:?}, RETCRCERRRSTF: {=bool:?}, RETECCFAILCRCRSTF: {=bool:?}, RETECCFAILRESTRSTF: {=bool:?} }}",
            self.PORRSTF(),
            self.BORRSTF(),
            self.PADRSTF(),
            self.HCSSRSTF(),
            self.VCORERSTF(),
            self.SYSC1RSTF(),
            self.SYSC2RSTF(),
            self.IWDG1SYSRSTF(),
            self.IWDG2SYSRSTF(),
            self.IWDG3SYSRSTF(),
            self.IWDG4SYSRSTF(),
            self.IWDG5SYSRSTF(),
            self.RETCRCERRRSTF(),
            self.RETECCFAILCRCRSTF(),
            self.RETECCFAILRESTRSTF()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C1CFGR(pub u32);
impl RCC_I2C1CFGR {
    #[doc = "I2C1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 reset"]
    #[inline(always)]
    pub const fn set_I2C1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 clock enable"]
    #[inline(always)]
    pub const fn set_I2C1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C1CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C1CFGR {
        RCC_I2C1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C1CFGR")
            .field("I2C1RST", &self.I2C1RST())
            .field("I2C1EN", &self.I2C1EN())
            .field("I2C1LPEN", &self.I2C1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C1CFGR {{ I2C1RST: {=bool:?}, I2C1EN: {=bool:?}, I2C1LPEN: {=bool:?} }}",
            self.I2C1RST(),
            self.I2C1EN(),
            self.I2C1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C2CFGR(pub u32);
impl RCC_I2C2CFGR {
    #[doc = "I2C2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C2 reset"]
    #[inline(always)]
    pub const fn set_I2C2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C2 clock enable"]
    #[inline(always)]
    pub const fn set_I2C2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C2CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C2CFGR {
        RCC_I2C2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C2CFGR")
            .field("I2C2RST", &self.I2C2RST())
            .field("I2C2EN", &self.I2C2EN())
            .field("I2C2LPEN", &self.I2C2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C2CFGR {{ I2C2RST: {=bool:?}, I2C2EN: {=bool:?}, I2C2LPEN: {=bool:?} }}",
            self.I2C2RST(),
            self.I2C2EN(),
            self.I2C2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C3CFGR(pub u32);
impl RCC_I2C3CFGR {
    #[doc = "I2C3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C3 reset"]
    #[inline(always)]
    pub const fn set_I2C3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C3 clock enable"]
    #[inline(always)]
    pub const fn set_I2C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C3CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C3CFGR {
        RCC_I2C3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C3CFGR")
            .field("I2C3RST", &self.I2C3RST())
            .field("I2C3EN", &self.I2C3EN())
            .field("I2C3LPEN", &self.I2C3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C3CFGR {{ I2C3RST: {=bool:?}, I2C3EN: {=bool:?}, I2C3LPEN: {=bool:?} }}",
            self.I2C3RST(),
            self.I2C3EN(),
            self.I2C3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C4CFGR(pub u32);
impl RCC_I2C4CFGR {
    #[doc = "I2C4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C4 reset"]
    #[inline(always)]
    pub const fn set_I2C4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C4 clock enable"]
    #[inline(always)]
    pub const fn set_I2C4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C4CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C4CFGR {
        RCC_I2C4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C4CFGR")
            .field("I2C4RST", &self.I2C4RST())
            .field("I2C4EN", &self.I2C4EN())
            .field("I2C4LPEN", &self.I2C4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C4CFGR {{ I2C4RST: {=bool:?}, I2C4EN: {=bool:?}, I2C4LPEN: {=bool:?} }}",
            self.I2C4RST(),
            self.I2C4EN(),
            self.I2C4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C5CFGR(pub u32);
impl RCC_I2C5CFGR {
    #[doc = "I2C5 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C5RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C5 reset"]
    #[inline(always)]
    pub const fn set_I2C5RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C5 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C5 clock enable"]
    #[inline(always)]
    pub const fn set_I2C5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C5 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C5 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C5CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C5CFGR {
        RCC_I2C5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C5CFGR")
            .field("I2C5RST", &self.I2C5RST())
            .field("I2C5EN", &self.I2C5EN())
            .field("I2C5LPEN", &self.I2C5LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C5CFGR {{ I2C5RST: {=bool:?}, I2C5EN: {=bool:?}, I2C5LPEN: {=bool:?} }}",
            self.I2C5RST(),
            self.I2C5EN(),
            self.I2C5LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C6CFGR(pub u32);
impl RCC_I2C6CFGR {
    #[doc = "I2C6 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C6RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C6 reset"]
    #[inline(always)]
    pub const fn set_I2C6RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C6 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C6EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C6 clock enable"]
    #[inline(always)]
    pub const fn set_I2C6EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C6 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C6LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C6 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C6LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C6CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C6CFGR {
        RCC_I2C6CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C6CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C6CFGR")
            .field("I2C6RST", &self.I2C6RST())
            .field("I2C6EN", &self.I2C6EN())
            .field("I2C6LPEN", &self.I2C6LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C6CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C6CFGR {{ I2C6RST: {=bool:?}, I2C6EN: {=bool:?}, I2C6LPEN: {=bool:?} }}",
            self.I2C6RST(),
            self.I2C6EN(),
            self.I2C6LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C7CFGR(pub u32);
impl RCC_I2C7CFGR {
    #[doc = "I2C7 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C7RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C7 reset"]
    #[inline(always)]
    pub const fn set_I2C7RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C7 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C7EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C7 clock enable"]
    #[inline(always)]
    pub const fn set_I2C7EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C7 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C7LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C7 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I2C7LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I2C7CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C7CFGR {
        RCC_I2C7CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C7CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C7CFGR")
            .field("I2C7RST", &self.I2C7RST())
            .field("I2C7EN", &self.I2C7EN())
            .field("I2C7LPEN", &self.I2C7LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C7CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C7CFGR {{ I2C7RST: {=bool:?}, I2C7EN: {=bool:?}, I2C7LPEN: {=bool:?} }}",
            self.I2C7RST(),
            self.I2C7EN(),
            self.I2C7LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I2C8CFGR(pub u32);
impl RCC_I2C8CFGR {
    #[doc = "I2C8 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C8RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C8 reset"]
    #[inline(always)]
    pub const fn set_I2C8RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C8 peripheral clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C8EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C8 peripheral clocks enable"]
    #[inline(always)]
    pub const fn set_I2C8EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I2C8 peripheral clocks enable during CSleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C8LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I2C8 peripheral clocks enable during CSleep mode"]
    #[inline(always)]
    pub const fn set_I2C8LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C8 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I2C8AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C8 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_I2C8AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_I2C8CFGR {
    #[inline(always)]
    fn default() -> RCC_I2C8CFGR {
        RCC_I2C8CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I2C8CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I2C8CFGR")
            .field("I2C8RST", &self.I2C8RST())
            .field("I2C8EN", &self.I2C8EN())
            .field("I2C8LPEN", &self.I2C8LPEN())
            .field("I2C8AMEN", &self.I2C8AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I2C8CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I2C8CFGR {{ I2C8RST: {=bool:?}, I2C8EN: {=bool:?}, I2C8LPEN: {=bool:?}, I2C8AMEN: {=bool:?} }}",
            self.I2C8RST(),
            self.I2C8EN(),
            self.I2C8LPEN(),
            self.I2C8AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I3C1CFGR(pub u32);
impl RCC_I3C1CFGR {
    #[doc = "I3C1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1 reset"]
    #[inline(always)]
    pub const fn set_I3C1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1 clock enable"]
    #[inline(always)]
    pub const fn set_I3C1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I3C1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I3C1CFGR {
    #[inline(always)]
    fn default() -> RCC_I3C1CFGR {
        RCC_I3C1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I3C1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I3C1CFGR")
            .field("I3C1RST", &self.I3C1RST())
            .field("I3C1EN", &self.I3C1EN())
            .field("I3C1LPEN", &self.I3C1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I3C1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I3C1CFGR {{ I3C1RST: {=bool:?}, I3C1EN: {=bool:?}, I3C1LPEN: {=bool:?} }}",
            self.I3C1RST(),
            self.I3C1EN(),
            self.I3C1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I3C2CFGR(pub u32);
impl RCC_I3C2CFGR {
    #[doc = "I3C2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2 reset"]
    #[inline(always)]
    pub const fn set_I3C2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2 clock enable"]
    #[inline(always)]
    pub const fn set_I3C2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I3C2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I3C2CFGR {
    #[inline(always)]
    fn default() -> RCC_I3C2CFGR {
        RCC_I3C2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I3C2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I3C2CFGR")
            .field("I3C2RST", &self.I3C2RST())
            .field("I3C2EN", &self.I3C2EN())
            .field("I3C2LPEN", &self.I3C2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I3C2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I3C2CFGR {{ I3C2RST: {=bool:?}, I3C2EN: {=bool:?}, I3C2LPEN: {=bool:?} }}",
            self.I3C2RST(),
            self.I3C2EN(),
            self.I3C2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I3C3CFGR(pub u32);
impl RCC_I3C3CFGR {
    #[doc = "I3C3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3 reset"]
    #[inline(always)]
    pub const fn set_I3C3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3 clock enable"]
    #[inline(always)]
    pub const fn set_I3C3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I3C3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_I3C3CFGR {
    #[inline(always)]
    fn default() -> RCC_I3C3CFGR {
        RCC_I3C3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I3C3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I3C3CFGR")
            .field("I3C3RST", &self.I3C3RST())
            .field("I3C3EN", &self.I3C3EN())
            .field("I3C3LPEN", &self.I3C3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I3C3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I3C3CFGR {{ I3C3RST: {=bool:?}, I3C3EN: {=bool:?}, I3C3LPEN: {=bool:?} }}",
            self.I3C3RST(),
            self.I3C3EN(),
            self.I3C3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_I3C4CFGR(pub u32);
impl RCC_I3C4CFGR {
    #[doc = "I3C4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I3C4 reset"]
    #[inline(always)]
    pub const fn set_I3C4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I3C4 clock enable"]
    #[inline(always)]
    pub const fn set_I3C4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "I3C4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "I3C4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_I3C4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I3C4 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn I3C4AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I3C4 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_I3C4AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_I3C4CFGR {
    #[inline(always)]
    fn default() -> RCC_I3C4CFGR {
        RCC_I3C4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_I3C4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_I3C4CFGR")
            .field("I3C4RST", &self.I3C4RST())
            .field("I3C4EN", &self.I3C4EN())
            .field("I3C4LPEN", &self.I3C4LPEN())
            .field("I3C4AMEN", &self.I3C4AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_I3C4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_I3C4CFGR {{ I3C4RST: {=bool:?}, I3C4EN: {=bool:?}, I3C4LPEN: {=bool:?}, I3C4AMEN: {=bool:?} }}",
            self.I3C4RST(),
            self.I3C4EN(),
            self.I3C4LPEN(),
            self.I3C4AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IDR(pub u32);
impl RCC_IDR {
    #[doc = "Bit fields for RCC_SIDR register"]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_SIDR register"]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_IDR {
    #[inline(always)]
    fn default() -> RCC_IDR {
        RCC_IDR(0)
    }
}
impl core::fmt::Debug for RCC_IDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IDR").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_IDR {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IPCC1CFGR(pub u32);
impl RCC_IPCC1CFGR {
    #[doc = "IPCC1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC1 reset"]
    #[inline(always)]
    pub const fn set_IPCC1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IPCC1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC1 clock enable"]
    #[inline(always)]
    pub const fn set_IPCC1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IPCC1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IPCC1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_IPCC1CFGR {
    #[inline(always)]
    fn default() -> RCC_IPCC1CFGR {
        RCC_IPCC1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IPCC1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IPCC1CFGR")
            .field("IPCC1RST", &self.IPCC1RST())
            .field("IPCC1EN", &self.IPCC1EN())
            .field("IPCC1LPEN", &self.IPCC1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IPCC1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IPCC1CFGR {{ IPCC1RST: {=bool:?}, IPCC1EN: {=bool:?}, IPCC1LPEN: {=bool:?} }}",
            self.IPCC1RST(),
            self.IPCC1EN(),
            self.IPCC1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IPCC2CFGR(pub u32);
impl RCC_IPCC2CFGR {
    #[doc = "IPCC2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC2 reset"]
    #[inline(always)]
    pub const fn set_IPCC2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IPCC2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC2 clock enable"]
    #[inline(always)]
    pub const fn set_IPCC2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IPCC2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IPCC2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IPCC2 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IPCC2AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IPCC2 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_IPCC2AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_IPCC2CFGR {
    #[inline(always)]
    fn default() -> RCC_IPCC2CFGR {
        RCC_IPCC2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IPCC2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IPCC2CFGR")
            .field("IPCC2RST", &self.IPCC2RST())
            .field("IPCC2EN", &self.IPCC2EN())
            .field("IPCC2LPEN", &self.IPCC2LPEN())
            .field("IPCC2AMEN", &self.IPCC2AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IPCC2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IPCC2CFGR {{ IPCC2RST: {=bool:?}, IPCC2EN: {=bool:?}, IPCC2LPEN: {=bool:?}, IPCC2AMEN: {=bool:?} }}",
            self.IPCC2RST(),
            self.IPCC2EN(),
            self.IPCC2LPEN(),
            self.IPCC2AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDG1CFGR(pub u32);
impl RCC_IWDG1CFGR {
    #[doc = "IWDG1 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 bus clock enable"]
    #[inline(always)]
    pub const fn set_IWDG1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDG1 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG1 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IWDG1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_IWDG1CFGR {
    #[inline(always)]
    fn default() -> RCC_IWDG1CFGR {
        RCC_IWDG1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IWDG1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDG1CFGR")
            .field("IWDG1EN", &self.IWDG1EN())
            .field("IWDG1LPEN", &self.IWDG1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDG1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDG1CFGR {{ IWDG1EN: {=bool:?}, IWDG1LPEN: {=bool:?} }}",
            self.IWDG1EN(),
            self.IWDG1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDG2CFGR(pub u32);
impl RCC_IWDG2CFGR {
    #[doc = "IWDG2 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 bus clock enable"]
    #[inline(always)]
    pub const fn set_IWDG2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDG2 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IWDG2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_IWDG2CFGR {
    #[inline(always)]
    fn default() -> RCC_IWDG2CFGR {
        RCC_IWDG2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IWDG2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDG2CFGR")
            .field("IWDG2EN", &self.IWDG2EN())
            .field("IWDG2LPEN", &self.IWDG2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDG2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDG2CFGR {{ IWDG2EN: {=bool:?}, IWDG2LPEN: {=bool:?} }}",
            self.IWDG2EN(),
            self.IWDG2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDG3CFGR(pub u32);
impl RCC_IWDG3CFGR {
    #[doc = "IWDG3 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 bus clock enable"]
    #[inline(always)]
    pub const fn set_IWDG3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDG3 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG3 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IWDG3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_IWDG3CFGR {
    #[inline(always)]
    fn default() -> RCC_IWDG3CFGR {
        RCC_IWDG3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IWDG3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDG3CFGR")
            .field("IWDG3EN", &self.IWDG3EN())
            .field("IWDG3LPEN", &self.IWDG3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDG3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDG3CFGR {{ IWDG3EN: {=bool:?}, IWDG3LPEN: {=bool:?} }}",
            self.IWDG3EN(),
            self.IWDG3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDG4CFGR(pub u32);
impl RCC_IWDG4CFGR {
    #[doc = "IWDG4 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 bus clock enable"]
    #[inline(always)]
    pub const fn set_IWDG4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDG4 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IWDG4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_IWDG4CFGR {
    #[inline(always)]
    fn default() -> RCC_IWDG4CFGR {
        RCC_IWDG4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IWDG4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDG4CFGR")
            .field("IWDG4EN", &self.IWDG4EN())
            .field("IWDG4LPEN", &self.IWDG4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDG4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDG4CFGR {{ IWDG4EN: {=bool:?}, IWDG4LPEN: {=bool:?} }}",
            self.IWDG4EN(),
            self.IWDG4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDG5CFGR(pub u32);
impl RCC_IWDG5CFGR {
    #[doc = "IWDG5 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 bus clock enable"]
    #[inline(always)]
    pub const fn set_IWDG5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IWDG5 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_IWDG5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IWDG5 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG5 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_IWDG5AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_IWDG5CFGR {
    #[inline(always)]
    fn default() -> RCC_IWDG5CFGR {
        RCC_IWDG5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_IWDG5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDG5CFGR")
            .field("IWDG5EN", &self.IWDG5EN())
            .field("IWDG5LPEN", &self.IWDG5LPEN())
            .field("IWDG5AMEN", &self.IWDG5AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDG5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDG5CFGR {{ IWDG5EN: {=bool:?}, IWDG5LPEN: {=bool:?}, IWDG5AMEN: {=bool:?} }}",
            self.IWDG5EN(),
            self.IWDG5LPEN(),
            self.IWDG5AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC1CFGCLRR(pub u32);
impl RCC_IWDGC1CFGCLRR {
    #[doc = "Clear of IWDG1 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear of IWDG1 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG1_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear of IWDG2 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear of IWDG2 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG2_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IWDG2 kernel reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2_KERRST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 kernel reset"]
    #[inline(always)]
    pub const fn set_IWDG2_KERRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RCC_IWDGC1CFGCLRR {
    #[inline(always)]
    fn default() -> RCC_IWDGC1CFGCLRR {
        RCC_IWDGC1CFGCLRR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC1CFGCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC1CFGCLRR")
            .field("IWDG1_SYSRSTEN", &self.IWDG1_SYSRSTEN())
            .field("IWDG2_SYSRSTEN", &self.IWDG2_SYSRSTEN())
            .field("IWDG2_KERRST", &self.IWDG2_KERRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC1CFGCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC1CFGCLRR {{ IWDG1_SYSRSTEN: {=bool:?}, IWDG2_SYSRSTEN: {=bool:?}, IWDG2_KERRST: {=bool:?} }}",
            self.IWDG1_SYSRSTEN(),
            self.IWDG2_SYSRSTEN(),
            self.IWDG2_KERRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC1CFGSETR(pub u32);
impl RCC_IWDGC1CFGSETR {
    #[doc = "Enable of IWDG1 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG1_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of IWDG1 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG1_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable of IWDG2 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of IWDG2 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG2_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IWDG2 kernel reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG2_KERRST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG2 kernel reset"]
    #[inline(always)]
    pub const fn set_IWDG2_KERRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RCC_IWDGC1CFGSETR {
    #[inline(always)]
    fn default() -> RCC_IWDGC1CFGSETR {
        RCC_IWDGC1CFGSETR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC1CFGSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC1CFGSETR")
            .field("IWDG1_SYSRSTEN", &self.IWDG1_SYSRSTEN())
            .field("IWDG2_SYSRSTEN", &self.IWDG2_SYSRSTEN())
            .field("IWDG2_KERRST", &self.IWDG2_KERRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC1CFGSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC1CFGSETR {{ IWDG1_SYSRSTEN: {=bool:?}, IWDG2_SYSRSTEN: {=bool:?}, IWDG2_KERRST: {=bool:?} }}",
            self.IWDG1_SYSRSTEN(),
            self.IWDG2_SYSRSTEN(),
            self.IWDG2_KERRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC1FZCLRR(pub u32);
impl RCC_IWDGC1FZCLRR {
    #[doc = "Unfreeze IWDG1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Unfreeze IWDG1 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unfreeze IWDG2 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unfreeze IWDG2 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_IWDGC1FZCLRR {
    #[inline(always)]
    fn default() -> RCC_IWDGC1FZCLRR {
        RCC_IWDGC1FZCLRR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC1FZCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC1FZCLRR")
            .field("FZ_IWDG1", &self.FZ_IWDG1())
            .field("FZ_IWDG2", &self.FZ_IWDG2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC1FZCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC1FZCLRR {{ FZ_IWDG1: {=bool:?}, FZ_IWDG2: {=bool:?} }}",
            self.FZ_IWDG1(),
            self.FZ_IWDG2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC1FZSETR(pub u32);
impl RCC_IWDGC1FZSETR {
    #[doc = "Freeze of IWDG1 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze of IWDG1 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Freeze of IWDG2 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze of IWDG2 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_IWDGC1FZSETR {
    #[inline(always)]
    fn default() -> RCC_IWDGC1FZSETR {
        RCC_IWDGC1FZSETR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC1FZSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC1FZSETR")
            .field("FZ_IWDG1", &self.FZ_IWDG1())
            .field("FZ_IWDG2", &self.FZ_IWDG2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC1FZSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC1FZSETR {{ FZ_IWDG1: {=bool:?}, FZ_IWDG2: {=bool:?} }}",
            self.FZ_IWDG1(),
            self.FZ_IWDG2()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC2CFGCLRR(pub u32);
impl RCC_IWDGC2CFGCLRR {
    #[doc = "Clear of IWDG3 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear of IWDG3 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG3_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear of IWDG4 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear of IWDG4 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG4_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IWDG4 kernel reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4_KERRST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 kernel reset"]
    #[inline(always)]
    pub const fn set_IWDG4_KERRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RCC_IWDGC2CFGCLRR {
    #[inline(always)]
    fn default() -> RCC_IWDGC2CFGCLRR {
        RCC_IWDGC2CFGCLRR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC2CFGCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC2CFGCLRR")
            .field("IWDG3_SYSRSTEN", &self.IWDG3_SYSRSTEN())
            .field("IWDG4_SYSRSTEN", &self.IWDG4_SYSRSTEN())
            .field("IWDG4_KERRST", &self.IWDG4_KERRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC2CFGCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC2CFGCLRR {{ IWDG3_SYSRSTEN: {=bool:?}, IWDG4_SYSRSTEN: {=bool:?}, IWDG4_KERRST: {=bool:?} }}",
            self.IWDG3_SYSRSTEN(),
            self.IWDG4_SYSRSTEN(),
            self.IWDG4_KERRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC2CFGSETR(pub u32);
impl RCC_IWDGC2CFGSETR {
    #[doc = "Enable of IWDG3 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG3_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of IWDG3 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG3_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable of IWDG4 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of IWDG4 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG4_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IWDG4 kernel reset"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG4_KERRST(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IWDG4 kernel reset"]
    #[inline(always)]
    pub const fn set_IWDG4_KERRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RCC_IWDGC2CFGSETR {
    #[inline(always)]
    fn default() -> RCC_IWDGC2CFGSETR {
        RCC_IWDGC2CFGSETR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC2CFGSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC2CFGSETR")
            .field("IWDG3_SYSRSTEN", &self.IWDG3_SYSRSTEN())
            .field("IWDG4_SYSRSTEN", &self.IWDG4_SYSRSTEN())
            .field("IWDG4_KERRST", &self.IWDG4_KERRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC2CFGSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC2CFGSETR {{ IWDG3_SYSRSTEN: {=bool:?}, IWDG4_SYSRSTEN: {=bool:?}, IWDG4_KERRST: {=bool:?} }}",
            self.IWDG3_SYSRSTEN(),
            self.IWDG4_SYSRSTEN(),
            self.IWDG4_KERRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC2FZCLRR(pub u32);
impl RCC_IWDGC2FZCLRR {
    #[doc = "Unfreeze of IWDG3 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Unfreeze of IWDG3 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unfreeze of IWDG4 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unfreeze of IWDG4 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_IWDGC2FZCLRR {
    #[inline(always)]
    fn default() -> RCC_IWDGC2FZCLRR {
        RCC_IWDGC2FZCLRR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC2FZCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC2FZCLRR")
            .field("FZ_IWDG3", &self.FZ_IWDG3())
            .field("FZ_IWDG4", &self.FZ_IWDG4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC2FZCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC2FZCLRR {{ FZ_IWDG3: {=bool:?}, FZ_IWDG4: {=bool:?} }}",
            self.FZ_IWDG3(),
            self.FZ_IWDG4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC2FZSETR(pub u32);
impl RCC_IWDGC2FZSETR {
    #[doc = "Freeze of IWDG3 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze of IWDG3 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Freeze of IWDG4 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn FZ_IWDG4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze of IWDG4 clock"]
    #[inline(always)]
    pub const fn set_FZ_IWDG4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RCC_IWDGC2FZSETR {
    #[inline(always)]
    fn default() -> RCC_IWDGC2FZSETR {
        RCC_IWDGC2FZSETR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC2FZSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC2FZSETR")
            .field("FZ_IWDG3", &self.FZ_IWDG3())
            .field("FZ_IWDG4", &self.FZ_IWDG4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC2FZSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC2FZSETR {{ FZ_IWDG3: {=bool:?}, FZ_IWDG4: {=bool:?} }}",
            self.FZ_IWDG3(),
            self.FZ_IWDG4()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC3CFGCLRR(pub u32);
impl RCC_IWDGC3CFGCLRR {
    #[doc = "Clear of IWDG5 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear of IWDG5 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG5_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_IWDGC3CFGCLRR {
    #[inline(always)]
    fn default() -> RCC_IWDGC3CFGCLRR {
        RCC_IWDGC3CFGCLRR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC3CFGCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC3CFGCLRR")
            .field("IWDG5_SYSRSTEN", &self.IWDG5_SYSRSTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC3CFGCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC3CFGCLRR {{ IWDG5_SYSRSTEN: {=bool:?} }}",
            self.IWDG5_SYSRSTEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_IWDGC3CFGSETR(pub u32);
impl RCC_IWDGC3CFGSETR {
    #[doc = "Enable of IWDG5 timeout event to reset the application"]
    #[must_use]
    #[inline(always)]
    pub const fn IWDG5_SYSRSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of IWDG5 timeout event to reset the application"]
    #[inline(always)]
    pub const fn set_IWDG5_SYSRSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_IWDGC3CFGSETR {
    #[inline(always)]
    fn default() -> RCC_IWDGC3CFGSETR {
        RCC_IWDGC3CFGSETR(0)
    }
}
impl core::fmt::Debug for RCC_IWDGC3CFGSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_IWDGC3CFGSETR")
            .field("IWDG5_SYSRSTEN", &self.IWDG5_SYSRSTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_IWDGC3CFGSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_IWDGC3CFGSETR {{ IWDG5_SYSRSTEN: {=bool:?} }}",
            self.IWDG5_SYSRSTEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LEGBOOTCR(pub u32);
impl RCC_LEGBOOTCR {
    #[doc = "Enable of the legacy selection mode to choose the first CPU to boot"]
    #[must_use]
    #[inline(always)]
    pub const fn LEGACY_BEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of the legacy selection mode to choose the first CPU to boot"]
    #[inline(always)]
    pub const fn set_LEGACY_BEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RCC_LEGBOOTCR {
    #[inline(always)]
    fn default() -> RCC_LEGBOOTCR {
        RCC_LEGBOOTCR(0)
    }
}
impl core::fmt::Debug for RCC_LEGBOOTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LEGBOOTCR")
            .field("LEGACY_BEN", &self.LEGACY_BEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LEGBOOTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LEGBOOTCR {{ LEGACY_BEN: {=bool:?} }}",
            self.LEGACY_BEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPDMACFGR(pub u32);
impl RCC_LPDMACFGR {
    #[doc = "LPDMA1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPDMARST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPDMA1 reset"]
    #[inline(always)]
    pub const fn set_LPDMARST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPDMA1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPDMAEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPDMA1 clock enable"]
    #[inline(always)]
    pub const fn set_LPDMAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPDMA1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPDMALPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPDMA1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPDMALPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPDMA1 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPDMAAMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPDMA1 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPDMAAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPDMACFGR {
    #[inline(always)]
    fn default() -> RCC_LPDMACFGR {
        RCC_LPDMACFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPDMACFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPDMACFGR")
            .field("LPDMARST", &self.LPDMARST())
            .field("LPDMAEN", &self.LPDMAEN())
            .field("LPDMALPEN", &self.LPDMALPEN())
            .field("LPDMAAMEN", &self.LPDMAAMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPDMACFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPDMACFGR {{ LPDMARST: {=bool:?}, LPDMAEN: {=bool:?}, LPDMALPEN: {=bool:?}, LPDMAAMEN: {=bool:?} }}",
            self.LPDMARST(),
            self.LPDMAEN(),
            self.LPDMALPEN(),
            self.LPDMAAMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPSRAM1CFGR(pub u32);
impl RCC_LPSRAM1CFGR {
    #[doc = "LPSRAM1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM1 clock enable"]
    #[inline(always)]
    pub const fn set_LPSRAM1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSRAM1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPSRAM1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPSRAM1 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM1AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM1 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPSRAM1AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPSRAM1CFGR {
    #[inline(always)]
    fn default() -> RCC_LPSRAM1CFGR {
        RCC_LPSRAM1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPSRAM1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPSRAM1CFGR")
            .field("LPSRAM1EN", &self.LPSRAM1EN())
            .field("LPSRAM1LPEN", &self.LPSRAM1LPEN())
            .field("LPSRAM1AMEN", &self.LPSRAM1AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPSRAM1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPSRAM1CFGR {{ LPSRAM1EN: {=bool:?}, LPSRAM1LPEN: {=bool:?}, LPSRAM1AMEN: {=bool:?} }}",
            self.LPSRAM1EN(),
            self.LPSRAM1LPEN(),
            self.LPSRAM1AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPSRAM2CFGR(pub u32);
impl RCC_LPSRAM2CFGR {
    #[doc = "LPSRAM2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM2 clock enable"]
    #[inline(always)]
    pub const fn set_LPSRAM2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSRAM2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPSRAM2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPSRAM2 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM2AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM2 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPSRAM2AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPSRAM2CFGR {
    #[inline(always)]
    fn default() -> RCC_LPSRAM2CFGR {
        RCC_LPSRAM2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPSRAM2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPSRAM2CFGR")
            .field("LPSRAM2EN", &self.LPSRAM2EN())
            .field("LPSRAM2LPEN", &self.LPSRAM2LPEN())
            .field("LPSRAM2AMEN", &self.LPSRAM2AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPSRAM2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPSRAM2CFGR {{ LPSRAM2EN: {=bool:?}, LPSRAM2LPEN: {=bool:?}, LPSRAM2AMEN: {=bool:?} }}",
            self.LPSRAM2EN(),
            self.LPSRAM2LPEN(),
            self.LPSRAM2AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPSRAM3CFGR(pub u32);
impl RCC_LPSRAM3CFGR {
    #[doc = "LPSRAM3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM3 clock enable"]
    #[inline(always)]
    pub const fn set_LPSRAM3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSRAM3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPSRAM3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPSRAM3 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPSRAM3AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPSRAM3 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPSRAM3AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPSRAM3CFGR {
    #[inline(always)]
    fn default() -> RCC_LPSRAM3CFGR {
        RCC_LPSRAM3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPSRAM3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPSRAM3CFGR")
            .field("LPSRAM3EN", &self.LPSRAM3EN())
            .field("LPSRAM3LPEN", &self.LPSRAM3LPEN())
            .field("LPSRAM3AMEN", &self.LPSRAM3AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPSRAM3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPSRAM3CFGR {{ LPSRAM3EN: {=bool:?}, LPSRAM3LPEN: {=bool:?}, LPSRAM3AMEN: {=bool:?} }}",
            self.LPSRAM3EN(),
            self.LPSRAM3LPEN(),
            self.LPSRAM3AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPTIM1CFGR(pub u32);
impl RCC_LPTIM1CFGR {
    #[doc = "LPTIM1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM1 reset"]
    #[inline(always)]
    pub const fn set_LPTIM1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM1 clock enable"]
    #[inline(always)]
    pub const fn set_LPTIM1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPTIM1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPTIM1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_LPTIM1CFGR {
    #[inline(always)]
    fn default() -> RCC_LPTIM1CFGR {
        RCC_LPTIM1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPTIM1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM1CFGR")
            .field("LPTIM1RST", &self.LPTIM1RST())
            .field("LPTIM1EN", &self.LPTIM1EN())
            .field("LPTIM1LPEN", &self.LPTIM1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPTIM1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPTIM1CFGR {{ LPTIM1RST: {=bool:?}, LPTIM1EN: {=bool:?}, LPTIM1LPEN: {=bool:?} }}",
            self.LPTIM1RST(),
            self.LPTIM1EN(),
            self.LPTIM1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPTIM2CFGR(pub u32);
impl RCC_LPTIM2CFGR {
    #[doc = "LPTIM2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM2 reset"]
    #[inline(always)]
    pub const fn set_LPTIM2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM2 clock enable"]
    #[inline(always)]
    pub const fn set_LPTIM2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPTIM2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPTIM2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_LPTIM2CFGR {
    #[inline(always)]
    fn default() -> RCC_LPTIM2CFGR {
        RCC_LPTIM2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPTIM2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM2CFGR")
            .field("LPTIM2RST", &self.LPTIM2RST())
            .field("LPTIM2EN", &self.LPTIM2EN())
            .field("LPTIM2LPEN", &self.LPTIM2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPTIM2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPTIM2CFGR {{ LPTIM2RST: {=bool:?}, LPTIM2EN: {=bool:?}, LPTIM2LPEN: {=bool:?} }}",
            self.LPTIM2RST(),
            self.LPTIM2EN(),
            self.LPTIM2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPTIM3CFGR(pub u32);
impl RCC_LPTIM3CFGR {
    #[doc = "LPTIM3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM3 reset"]
    #[inline(always)]
    pub const fn set_LPTIM3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM3 clock enable"]
    #[inline(always)]
    pub const fn set_LPTIM3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPTIM3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPTIM3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPTIM3 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM3AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM3 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPTIM3AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPTIM3CFGR {
    #[inline(always)]
    fn default() -> RCC_LPTIM3CFGR {
        RCC_LPTIM3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPTIM3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM3CFGR")
            .field("LPTIM3RST", &self.LPTIM3RST())
            .field("LPTIM3EN", &self.LPTIM3EN())
            .field("LPTIM3LPEN", &self.LPTIM3LPEN())
            .field("LPTIM3AMEN", &self.LPTIM3AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPTIM3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPTIM3CFGR {{ LPTIM3RST: {=bool:?}, LPTIM3EN: {=bool:?}, LPTIM3LPEN: {=bool:?}, LPTIM3AMEN: {=bool:?} }}",
            self.LPTIM3RST(),
            self.LPTIM3EN(),
            self.LPTIM3LPEN(),
            self.LPTIM3AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPTIM4CFGR(pub u32);
impl RCC_LPTIM4CFGR {
    #[doc = "LPTIM4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM4 reset"]
    #[inline(always)]
    pub const fn set_LPTIM4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM4 clock enable"]
    #[inline(always)]
    pub const fn set_LPTIM4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPTIM4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPTIM4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPTIM4 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM4AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM4 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPTIM4AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPTIM4CFGR {
    #[inline(always)]
    fn default() -> RCC_LPTIM4CFGR {
        RCC_LPTIM4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPTIM4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM4CFGR")
            .field("LPTIM4RST", &self.LPTIM4RST())
            .field("LPTIM4EN", &self.LPTIM4EN())
            .field("LPTIM4LPEN", &self.LPTIM4LPEN())
            .field("LPTIM4AMEN", &self.LPTIM4AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPTIM4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPTIM4CFGR {{ LPTIM4RST: {=bool:?}, LPTIM4EN: {=bool:?}, LPTIM4LPEN: {=bool:?}, LPTIM4AMEN: {=bool:?} }}",
            self.LPTIM4RST(),
            self.LPTIM4EN(),
            self.LPTIM4LPEN(),
            self.LPTIM4AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPTIM5CFGR(pub u32);
impl RCC_LPTIM5CFGR {
    #[doc = "LPTIM5 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM5RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM5 reset"]
    #[inline(always)]
    pub const fn set_LPTIM5RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTIM5 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM5 clock enable"]
    #[inline(always)]
    pub const fn set_LPTIM5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPTIM5 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM5 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPTIM5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPTIM5 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPTIM5AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPTIM5 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPTIM5AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPTIM5CFGR {
    #[inline(always)]
    fn default() -> RCC_LPTIM5CFGR {
        RCC_LPTIM5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPTIM5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPTIM5CFGR")
            .field("LPTIM5RST", &self.LPTIM5RST())
            .field("LPTIM5EN", &self.LPTIM5EN())
            .field("LPTIM5LPEN", &self.LPTIM5LPEN())
            .field("LPTIM5AMEN", &self.LPTIM5AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPTIM5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPTIM5CFGR {{ LPTIM5RST: {=bool:?}, LPTIM5EN: {=bool:?}, LPTIM5LPEN: {=bool:?}, LPTIM5AMEN: {=bool:?} }}",
            self.LPTIM5RST(),
            self.LPTIM5EN(),
            self.LPTIM5LPEN(),
            self.LPTIM5AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LPUART1CFGR(pub u32);
impl RCC_LPUART1CFGR {
    #[doc = "LPUART1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 reset"]
    #[inline(always)]
    pub const fn set_LPUART1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPUART1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 clock enable"]
    #[inline(always)]
    pub const fn set_LPUART1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPUART1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LPUART1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPUART1 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LPUART1AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_LPUART1AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_LPUART1CFGR {
    #[inline(always)]
    fn default() -> RCC_LPUART1CFGR {
        RCC_LPUART1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_LPUART1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LPUART1CFGR")
            .field("LPUART1RST", &self.LPUART1RST())
            .field("LPUART1EN", &self.LPUART1EN())
            .field("LPUART1LPEN", &self.LPUART1LPEN())
            .field("LPUART1AMEN", &self.LPUART1AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LPUART1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LPUART1CFGR {{ LPUART1RST: {=bool:?}, LPUART1EN: {=bool:?}, LPUART1LPEN: {=bool:?}, LPUART1AMEN: {=bool:?} }}",
            self.LPUART1RST(),
            self.LPUART1EN(),
            self.LPUART1LPEN(),
            self.LPUART1AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LSMCUDIVR(pub u32);
impl RCC_LSMCUDIVR {
    #[doc = "LSMCU clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn LSMCUDIV(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LSMCU clock divider"]
    #[inline(always)]
    pub const fn set_LSMCUDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LSMCU clock divider status"]
    #[must_use]
    #[inline(always)]
    pub const fn LSMCUDIVRDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LSMCU clock divider status"]
    #[inline(always)]
    pub const fn set_LSMCUDIVRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_LSMCUDIVR {
    #[inline(always)]
    fn default() -> RCC_LSMCUDIVR {
        RCC_LSMCUDIVR(0)
    }
}
impl core::fmt::Debug for RCC_LSMCUDIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LSMCUDIVR")
            .field("LSMCUDIV", &self.LSMCUDIV())
            .field("LSMCUDIVRDY", &self.LSMCUDIVRDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LSMCUDIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LSMCUDIVR {{ LSMCUDIV: {=bool:?}, LSMCUDIVRDY: {=bool:?} }}",
            self.LSMCUDIV(),
            self.LSMCUDIVRDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LTDCCFGR(pub u32);
impl RCC_LTDCCFGR {
    #[doc = "LTDC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LTDCRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LTDC reset"]
    #[inline(always)]
    pub const fn set_LTDCRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LTDC peripheral clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LTDCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LTDC peripheral clock enable"]
    #[inline(always)]
    pub const fn set_LTDCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LTDC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LTDCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LTDC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LTDCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_LTDCCFGR {
    #[inline(always)]
    fn default() -> RCC_LTDCCFGR {
        RCC_LTDCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_LTDCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LTDCCFGR")
            .field("LTDCRST", &self.LTDCRST())
            .field("LTDCEN", &self.LTDCEN())
            .field("LTDCLPEN", &self.LTDCLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LTDCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LTDCCFGR {{ LTDCRST: {=bool:?}, LTDCEN: {=bool:?}, LTDCLPEN: {=bool:?} }}",
            self.LTDCRST(),
            self.LTDCEN(),
            self.LTDCLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_LVDSCFGR(pub u32);
impl RCC_LVDSCFGR {
    #[doc = "LVDS reset"]
    #[must_use]
    #[inline(always)]
    pub const fn LVDSRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LVDS reset"]
    #[inline(always)]
    pub const fn set_LVDSRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LVDSPHY clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn LVDSEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LVDSPHY clock enable"]
    #[inline(always)]
    pub const fn set_LVDSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LVDSPHY clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn LVDSLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LVDSPHY clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_LVDSLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LVDSPHY reference clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn LVDSPHYCKREFSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LVDSPHY reference clock select"]
    #[inline(always)]
    pub const fn set_LVDSPHYCKREFSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RCC_LVDSCFGR {
    #[inline(always)]
    fn default() -> RCC_LVDSCFGR {
        RCC_LVDSCFGR(0)
    }
}
impl core::fmt::Debug for RCC_LVDSCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_LVDSCFGR")
            .field("LVDSRST", &self.LVDSRST())
            .field("LVDSEN", &self.LVDSEN())
            .field("LVDSLPEN", &self.LVDSLPEN())
            .field("LVDSPHYCKREFSEL", &self.LVDSPHYCKREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_LVDSCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_LVDSCFGR {{ LVDSRST: {=bool:?}, LVDSEN: {=bool:?}, LVDSLPEN: {=bool:?}, LVDSPHYCKREFSEL: {=bool:?} }}",
            self.LVDSRST(),
            self.LVDSEN(),
            self.LVDSLPEN(),
            self.LVDSPHYCKREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_MCO1CFGR(pub u32);
impl RCC_MCO1CFGR {
    #[doc = "MCO1 clock output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MCO1SEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MCO1 clock output selection"]
    #[inline(always)]
    pub const fn set_MCO1SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control of MCO1 output"]
    #[must_use]
    #[inline(always)]
    pub const fn MCO1ON(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Control of MCO1 output"]
    #[inline(always)]
    pub const fn set_MCO1ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for RCC_MCO1CFGR {
    #[inline(always)]
    fn default() -> RCC_MCO1CFGR {
        RCC_MCO1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_MCO1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MCO1CFGR")
            .field("MCO1SEL", &self.MCO1SEL())
            .field("MCO1ON", &self.MCO1ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_MCO1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_MCO1CFGR {{ MCO1SEL: {=bool:?}, MCO1ON: {=bool:?} }}",
            self.MCO1SEL(),
            self.MCO1ON()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_MCO2CFGR(pub u32);
impl RCC_MCO2CFGR {
    #[doc = "MCO2 clock output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MCO2SEL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MCO2 clock output selection"]
    #[inline(always)]
    pub const fn set_MCO2SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control of the MCO2 output"]
    #[must_use]
    #[inline(always)]
    pub const fn MCO2ON(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Control of the MCO2 output"]
    #[inline(always)]
    pub const fn set_MCO2ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for RCC_MCO2CFGR {
    #[inline(always)]
    fn default() -> RCC_MCO2CFGR {
        RCC_MCO2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_MCO2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MCO2CFGR")
            .field("MCO2SEL", &self.MCO2SEL())
            .field("MCO2ON", &self.MCO2ON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_MCO2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_MCO2CFGR {{ MCO2SEL: {=bool:?}, MCO2ON: {=bool:?} }}",
            self.MCO2SEL(),
            self.MCO2ON()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_MDF1CFGR(pub u32);
impl RCC_MDF1CFGR {
    #[doc = "MDF1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn MDF1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MDF1 reset"]
    #[inline(always)]
    pub const fn set_MDF1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MDF1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn MDF1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MDF1 clock enable"]
    #[inline(always)]
    pub const fn set_MDF1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MDF1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn MDF1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MDF1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_MDF1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_MDF1CFGR {
    #[inline(always)]
    fn default() -> RCC_MDF1CFGR {
        RCC_MDF1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_MDF1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MDF1CFGR")
            .field("MDF1RST", &self.MDF1RST())
            .field("MDF1EN", &self.MDF1EN())
            .field("MDF1LPEN", &self.MDF1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_MDF1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_MDF1CFGR {{ MDF1RST: {=bool:?}, MDF1EN: {=bool:?}, MDF1LPEN: {=bool:?} }}",
            self.MDF1RST(),
            self.MDF1EN(),
            self.MDF1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_MSICFGR(pub u32);
impl RCC_MSICFGR {
    #[doc = "MSI clock trimming"]
    #[must_use]
    #[inline(always)]
    pub const fn MSITRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "MSI clock trimming"]
    #[inline(always)]
    pub const fn set_MSITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "MSI clock calibration"]
    #[must_use]
    #[inline(always)]
    pub const fn MSICAL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "MSI clock calibration"]
    #[inline(always)]
    pub const fn set_MSICAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RCC_MSICFGR {
    #[inline(always)]
    fn default() -> RCC_MSICFGR {
        RCC_MSICFGR(0)
    }
}
impl core::fmt::Debug for RCC_MSICFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MSICFGR")
            .field("MSITRIM", &self.MSITRIM())
            .field("MSICAL", &self.MSICAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_MSICFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_MSICFGR {{ MSITRIM: {=u8:?}, MSICAL: {=u8:?} }}",
            self.MSITRIM(),
            self.MSICAL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_MUXSELCFGR(pub u32);
impl RCC_MUXSELCFGR {
    #[doc = "PLL4 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "PLL4 reference clock selection"]
    #[inline(always)]
    pub const fn set_MUXSEL0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PLL5 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "PLL5 reference clock selection"]
    #[inline(always)]
    pub const fn set_MUXSEL1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "PLL6 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "PLL6 reference clock selection"]
    #[inline(always)]
    pub const fn set_MUXSEL2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "PLL7 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "PLL7 reference clock selection"]
    #[inline(always)]
    pub const fn set_MUXSEL3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "PLL8 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "PLL8 reference clock selection"]
    #[inline(always)]
    pub const fn set_MUXSEL4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "PLL1 reference clock selection (ck_pll1_ref)"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "PLL1 reference clock selection (ck_pll1_ref)"]
    #[inline(always)]
    pub const fn set_MUXSEL5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "PLL2 reference clock selection (ck_pll2_ref)"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "PLL2 reference clock selection (ck_pll2_ref)"]
    #[inline(always)]
    pub const fn set_MUXSEL6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "PLL3 reference clock selection (ck_pll3_ref)"]
    #[must_use]
    #[inline(always)]
    pub const fn MUXSEL7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "PLL3 reference clock selection (ck_pll3_ref)"]
    #[inline(always)]
    pub const fn set_MUXSEL7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for RCC_MUXSELCFGR {
    #[inline(always)]
    fn default() -> RCC_MUXSELCFGR {
        RCC_MUXSELCFGR(0)
    }
}
impl core::fmt::Debug for RCC_MUXSELCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_MUXSELCFGR")
            .field("MUXSEL0", &self.MUXSEL0())
            .field("MUXSEL1", &self.MUXSEL1())
            .field("MUXSEL2", &self.MUXSEL2())
            .field("MUXSEL3", &self.MUXSEL3())
            .field("MUXSEL4", &self.MUXSEL4())
            .field("MUXSEL5", &self.MUXSEL5())
            .field("MUXSEL6", &self.MUXSEL6())
            .field("MUXSEL7", &self.MUXSEL7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_MUXSELCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_MUXSELCFGR {{ MUXSEL0: {=u8:?}, MUXSEL1: {=u8:?}, MUXSEL2: {=u8:?}, MUXSEL3: {=u8:?}, MUXSEL4: {=u8:?}, MUXSEL5: {=u8:?}, MUXSEL6: {=u8:?}, MUXSEL7: {=u8:?} }}",
            self.MUXSEL0(),
            self.MUXSEL1(),
            self.MUXSEL2(),
            self.MUXSEL3(),
            self.MUXSEL4(),
            self.MUXSEL5(),
            self.MUXSEL6(),
            self.MUXSEL7()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OCENCLRR(pub u32);
impl RCC_OCENCLRR {
    #[doc = "HSION bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HSION bit clear"]
    #[inline(always)]
    pub const fn set_HSION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HSIKERON bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIKERON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HSIKERON bit clear"]
    #[inline(always)]
    pub const fn set_HSIKERON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSEDIV2ON bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIV2ON(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIV2ON bit clear"]
    #[inline(always)]
    pub const fn set_HSEDIV2ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HSEDIV2BYP bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIV2BYP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIV2BYP bit clear"]
    #[inline(always)]
    pub const fn set_HSEDIV2BYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "HSEDIGBYP bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIGBYP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIGBYP bit clear"]
    #[inline(always)]
    pub const fn set_HSEDIGBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "HSEON bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEON(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HSEON bit clear"]
    #[inline(always)]
    pub const fn set_HSEON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HSEKERON bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEKERON(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HSEKERON bit clear"]
    #[inline(always)]
    pub const fn set_HSEKERON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HSEBYP bit clear"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEBYP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HSEBYP bit clear"]
    #[inline(always)]
    pub const fn set_HSEBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_OCENCLRR {
    #[inline(always)]
    fn default() -> RCC_OCENCLRR {
        RCC_OCENCLRR(0)
    }
}
impl core::fmt::Debug for RCC_OCENCLRR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OCENCLRR")
            .field("HSION", &self.HSION())
            .field("HSIKERON", &self.HSIKERON())
            .field("HSEDIV2ON", &self.HSEDIV2ON())
            .field("HSEDIV2BYP", &self.HSEDIV2BYP())
            .field("HSEDIGBYP", &self.HSEDIGBYP())
            .field("HSEON", &self.HSEON())
            .field("HSEKERON", &self.HSEKERON())
            .field("HSEBYP", &self.HSEBYP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OCENCLRR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OCENCLRR {{ HSION: {=bool:?}, HSIKERON: {=bool:?}, HSEDIV2ON: {=bool:?}, HSEDIV2BYP: {=bool:?}, HSEDIGBYP: {=bool:?}, HSEON: {=bool:?}, HSEKERON: {=bool:?}, HSEBYP: {=bool:?} }}",
            self.HSION(),
            self.HSIKERON(),
            self.HSEDIV2ON(),
            self.HSEDIV2BYP(),
            self.HSEDIGBYP(),
            self.HSEON(),
            self.HSEKERON(),
            self.HSEBYP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OCENSETR(pub u32);
impl RCC_OCENSETR {
    #[doc = "HSION bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HSION bit set"]
    #[inline(always)]
    pub const fn set_HSION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HSIKERON bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIKERON(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HSIKERON bit set"]
    #[inline(always)]
    pub const fn set_HSIKERON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HSEDIV2ON bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIV2ON(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIV2ON bit set"]
    #[inline(always)]
    pub const fn set_HSEDIV2ON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HSEDIV2BYP bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIV2BYP(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIV2BYP bit set"]
    #[inline(always)]
    pub const fn set_HSEDIV2BYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "HSEDIGBYP bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEDIGBYP(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "HSEDIGBYP bit set"]
    #[inline(always)]
    pub const fn set_HSEDIGBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "HSEON bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEON(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HSEON bit set"]
    #[inline(always)]
    pub const fn set_HSEON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HSEKERON bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEKERON(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HSEKERON bit set"]
    #[inline(always)]
    pub const fn set_HSEKERON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HSEBYP bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSEBYP(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HSEBYP bit set"]
    #[inline(always)]
    pub const fn set_HSEBYP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HSECSSON bit set"]
    #[must_use]
    #[inline(always)]
    pub const fn HSECSSON(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "HSECSSON bit set"]
    #[inline(always)]
    pub const fn set_HSECSSON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for RCC_OCENSETR {
    #[inline(always)]
    fn default() -> RCC_OCENSETR {
        RCC_OCENSETR(0)
    }
}
impl core::fmt::Debug for RCC_OCENSETR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OCENSETR")
            .field("HSION", &self.HSION())
            .field("HSIKERON", &self.HSIKERON())
            .field("HSEDIV2ON", &self.HSEDIV2ON())
            .field("HSEDIV2BYP", &self.HSEDIV2BYP())
            .field("HSEDIGBYP", &self.HSEDIGBYP())
            .field("HSEON", &self.HSEON())
            .field("HSEKERON", &self.HSEKERON())
            .field("HSEBYP", &self.HSEBYP())
            .field("HSECSSON", &self.HSECSSON())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OCENSETR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OCENSETR {{ HSION: {=bool:?}, HSIKERON: {=bool:?}, HSEDIV2ON: {=bool:?}, HSEDIV2BYP: {=bool:?}, HSEDIGBYP: {=bool:?}, HSEON: {=bool:?}, HSEKERON: {=bool:?}, HSEBYP: {=bool:?}, HSECSSON: {=bool:?} }}",
            self.HSION(),
            self.HSIKERON(),
            self.HSEDIV2ON(),
            self.HSEDIV2BYP(),
            self.HSEDIGBYP(),
            self.HSEON(),
            self.HSEKERON(),
            self.HSEBYP(),
            self.HSECSSON()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OCRDYR(pub u32);
impl RCC_OCRDYR {
    #[doc = "HSI clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSIRDY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HSI clock ready flag"]
    #[inline(always)]
    pub const fn set_HSIRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HSE clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn HSERDY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "HSE clock ready flag"]
    #[inline(always)]
    pub const fn set_HSERDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock restore state machine status"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREST(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Clock restore state machine status"]
    #[inline(always)]
    pub const fn set_CKREST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for RCC_OCRDYR {
    #[inline(always)]
    fn default() -> RCC_OCRDYR {
        RCC_OCRDYR(0)
    }
}
impl core::fmt::Debug for RCC_OCRDYR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OCRDYR")
            .field("HSIRDY", &self.HSIRDY())
            .field("HSERDY", &self.HSERDY())
            .field("CKREST", &self.CKREST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OCRDYR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OCRDYR {{ HSIRDY: {=bool:?}, HSERDY: {=bool:?}, CKREST: {=bool:?} }}",
            self.HSIRDY(),
            self.HSERDY(),
            self.CKREST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OSPI1CFGR(pub u32);
impl RCC_OSPI1CFGR {
    #[doc = "OCTOSPI1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI1 reset"]
    #[inline(always)]
    pub const fn set_OSPI1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OCTOSPI1 and OTFDEC1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI1 and OTFDEC1 clock enable"]
    #[inline(always)]
    pub const fn set_OSPI1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCTOSPI1, OTFDEC1 and OCTOSPI1 delay (DLYBOSPI1) clock enable duringï¿½CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI1, OTFDEC1 and OCTOSPI1 delay (DLYBOSPI1) clock enable duringï¿½CSleep"]
    #[inline(always)]
    pub const fn set_OSPI1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OTFDEC1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OTFDEC1RST(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTFDEC1 reset"]
    #[inline(always)]
    pub const fn set_OTFDEC1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OCTOSPI1 DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI1DLLRST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI1 DLL reset"]
    #[inline(always)]
    pub const fn set_OSPI1DLLRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_OSPI1CFGR {
    #[inline(always)]
    fn default() -> RCC_OSPI1CFGR {
        RCC_OSPI1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_OSPI1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OSPI1CFGR")
            .field("OSPI1RST", &self.OSPI1RST())
            .field("OSPI1EN", &self.OSPI1EN())
            .field("OSPI1LPEN", &self.OSPI1LPEN())
            .field("OTFDEC1RST", &self.OTFDEC1RST())
            .field("OSPI1DLLRST", &self.OSPI1DLLRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OSPI1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OSPI1CFGR {{ OSPI1RST: {=bool:?}, OSPI1EN: {=bool:?}, OSPI1LPEN: {=bool:?}, OTFDEC1RST: {=bool:?}, OSPI1DLLRST: {=bool:?} }}",
            self.OSPI1RST(),
            self.OSPI1EN(),
            self.OSPI1LPEN(),
            self.OTFDEC1RST(),
            self.OSPI1DLLRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OSPI2CFGR(pub u32);
impl RCC_OSPI2CFGR {
    #[doc = "OCTOSPI2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI2 reset"]
    #[inline(always)]
    pub const fn set_OSPI2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OCTOSPI2 and OTFDEC2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI2 and OTFDEC2 clock enable"]
    #[inline(always)]
    pub const fn set_OSPI2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCTOSPI2 and OTFDEC2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI2 and OTFDEC2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_OSPI2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OTFDEC2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OTFDEC2RST(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OTFDEC2 reset"]
    #[inline(always)]
    pub const fn set_OTFDEC2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OCTOSPI2 DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPI2DLLRST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPI2 DLL reset"]
    #[inline(always)]
    pub const fn set_OSPI2DLLRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_OSPI2CFGR {
    #[inline(always)]
    fn default() -> RCC_OSPI2CFGR {
        RCC_OSPI2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_OSPI2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OSPI2CFGR")
            .field("OSPI2RST", &self.OSPI2RST())
            .field("OSPI2EN", &self.OSPI2EN())
            .field("OSPI2LPEN", &self.OSPI2LPEN())
            .field("OTFDEC2RST", &self.OTFDEC2RST())
            .field("OSPI2DLLRST", &self.OSPI2DLLRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OSPI2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OSPI2CFGR {{ OSPI2RST: {=bool:?}, OSPI2EN: {=bool:?}, OSPI2LPEN: {=bool:?}, OTFDEC2RST: {=bool:?}, OSPI2DLLRST: {=bool:?} }}",
            self.OSPI2RST(),
            self.OSPI2EN(),
            self.OSPI2LPEN(),
            self.OTFDEC2RST(),
            self.OSPI2DLLRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_OSPIIOMCFGR(pub u32);
impl RCC_OSPIIOMCFGR {
    #[doc = "OCTOSPIM reset"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPIIOMRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPIM reset"]
    #[inline(always)]
    pub const fn set_OSPIIOMRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OCTOSPIM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPIIOMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPIM clock enable"]
    #[inline(always)]
    pub const fn set_OSPIIOMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OCTOSPIM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn OSPIIOMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "OCTOSPIM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_OSPIIOMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_OSPIIOMCFGR {
    #[inline(always)]
    fn default() -> RCC_OSPIIOMCFGR {
        RCC_OSPIIOMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_OSPIIOMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_OSPIIOMCFGR")
            .field("OSPIIOMRST", &self.OSPIIOMRST())
            .field("OSPIIOMEN", &self.OSPIIOMEN())
            .field("OSPIIOMLPEN", &self.OSPIIOMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_OSPIIOMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_OSPIIOMCFGR {{ OSPIIOMRST: {=bool:?}, OSPIIOMEN: {=bool:?}, OSPIIOMLPEN: {=bool:?} }}",
            self.OSPIIOMRST(),
            self.OSPIIOMEN(),
            self.OSPIIOMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PCIECFGR(pub u32);
impl RCC_PCIECFGR {
    #[doc = "PCIE reset"]
    #[must_use]
    #[inline(always)]
    pub const fn PCIERST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PCIE reset"]
    #[inline(always)]
    pub const fn set_PCIERST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PCIE clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PCIEEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PCIE clock enable"]
    #[inline(always)]
    pub const fn set_PCIEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PCIE clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn PCIELPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PCIE clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_PCIELPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PCIE clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn PCIESTPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PCIE clock enable during CStop"]
    #[inline(always)]
    pub const fn set_PCIESTPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RCC_PCIECFGR {
    #[inline(always)]
    fn default() -> RCC_PCIECFGR {
        RCC_PCIECFGR(0)
    }
}
impl core::fmt::Debug for RCC_PCIECFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PCIECFGR")
            .field("PCIERST", &self.PCIERST())
            .field("PCIEEN", &self.PCIEEN())
            .field("PCIELPEN", &self.PCIELPEN())
            .field("PCIESTPEN", &self.PCIESTPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PCIECFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PCIECFGR {{ PCIERST: {=bool:?}, PCIEEN: {=bool:?}, PCIELPEN: {=bool:?}, PCIESTPEN: {=bool:?} }}",
            self.PCIERST(),
            self.PCIEEN(),
            self.PCIELPEN(),
            self.PCIESTPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PKACFGR(pub u32);
impl RCC_PKACFGR {
    #[doc = "PKA reset"]
    #[must_use]
    #[inline(always)]
    pub const fn PKARST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PKA reset"]
    #[inline(always)]
    pub const fn set_PKARST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PKA clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PKAEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PKA clock enable"]
    #[inline(always)]
    pub const fn set_PKAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PKA clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn PKALPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PKA clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_PKALPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_PKACFGR {
    #[inline(always)]
    fn default() -> RCC_PKACFGR {
        RCC_PKACFGR(0)
    }
}
impl core::fmt::Debug for RCC_PKACFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PKACFGR")
            .field("PKARST", &self.PKARST())
            .field("PKAEN", &self.PKAEN())
            .field("PKALPEN", &self.PKALPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PKACFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PKACFGR {{ PKARST: {=bool:?}, PKAEN: {=bool:?}, PKALPEN: {=bool:?} }}",
            self.PKARST(),
            self.PKAEN(),
            self.PKALPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR1(pub u32);
impl RCC_PLL2CFGR1 {
    #[doc = "PLLx spread-spectrum modulator reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx spread-spectrum modulator reset"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PLLx enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx enable"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLLx clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx clock ready flag"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PLLx reference clock state"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx reference clock state"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL2CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR1 {
        RCC_PLL2CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL2CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR2(pub u32);
impl RCC_PLL2CFGR2 {
    #[doc = "PLLx reference input clock divide frequency ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PLLx reference input clock divide frequency ratio"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PLLx VCO multiplication factor"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "PLLx VCO multiplication factor"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL2CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR2 {
        RCC_PLL2CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL2CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR3(pub u32);
impl RCC_PLL2CFGR3 {
    #[doc = "Fractional part of PLLx VCO multiplication factor"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Fractional part of PLLx VCO multiplication factor"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "PLLx VCO frequency modulation mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx VCO frequency modulation mode"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PLLx noise canceling DAC enable in fractional mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx noise canceling DAC enable in fractional mode"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "PLLx spread-spectrum modulator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx spread-spectrum modulator enable"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL2CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR3 {
        RCC_PLL2CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL2CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR4(pub u32);
impl RCC_PLL2CFGR4 {
    #[doc = "PLLx delta-sigma modulator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx delta-sigma modulator enable"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLLx output and post divider enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx output and post divider enable"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLLx FOUTPOSTDIV bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx FOUTPOSTDIV bypass"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL2CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR4 {
        RCC_PLL2CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL2CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR5(pub u32);
impl RCC_PLL2CFGR5 {
    #[doc = "Modulation frequency adjustment for PLLx"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Modulation frequency adjustment for PLLx"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Modulation depth adjustment for PLLx"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Modulation depth adjustment for PLLx"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL2CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR5 {
        RCC_PLL2CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL2CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR6(pub u32);
impl RCC_PLL2CFGR6 {
    #[doc = "PLLx VCO frequency divide level 1"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PLLx VCO frequency divide level 1"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL2CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR6 {
        RCC_PLL2CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL2CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL2CFGR7(pub u32);
impl RCC_PLL2CFGR7 {
    #[doc = "PLLx VCO frequency divide level 2"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PLLx VCO frequency divide level 2"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL2CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL2CFGR7 {
        RCC_PLL2CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL2CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL2CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL2CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL2CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR1(pub u32);
impl RCC_PLL3CFGR1 {
    #[doc = "PLLx spread-spectrum modulator reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx spread-spectrum modulator reset"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PLLx enable"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx enable"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLLx clock ready flag"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx clock ready flag"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PLLx reference clock state"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx reference clock state"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL3CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR1 {
        RCC_PLL3CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL3CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR2(pub u32);
impl RCC_PLL3CFGR2 {
    #[doc = "PLLx reference input clock divide frequency ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PLLx reference input clock divide frequency ratio"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PLLx VCO multiplication factor"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "PLLx VCO multiplication factor"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL3CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR2 {
        RCC_PLL3CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL3CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR3(pub u32);
impl RCC_PLL3CFGR3 {
    #[doc = "Fractional part of PLLx VCO multiplication factor"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Fractional part of PLLx VCO multiplication factor"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "PLLx VCO frequency modulation mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx VCO frequency modulation mode"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "PLLx noise canceling DAC enable in fractional mode"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx noise canceling DAC enable in fractional mode"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "PLLx spread-spectrum modulator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx spread-spectrum modulator enable"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL3CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR3 {
        RCC_PLL3CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL3CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR4(pub u32);
impl RCC_PLL3CFGR4 {
    #[doc = "PLLx delta-sigma modulator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx delta-sigma modulator enable"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLLx output and post divider enable"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx output and post divider enable"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PLLx FOUTPOSTDIV bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PLLx FOUTPOSTDIV bypass"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL3CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR4 {
        RCC_PLL3CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL3CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR5(pub u32);
impl RCC_PLL3CFGR5 {
    #[doc = "Modulation frequency adjustment for PLLx"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Modulation frequency adjustment for PLLx"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Modulation depth adjustment for PLLx"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Modulation depth adjustment for PLLx"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL3CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR5 {
        RCC_PLL3CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL3CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR6(pub u32);
impl RCC_PLL3CFGR6 {
    #[doc = "PLLx VCO frequency divide level 1"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PLLx VCO frequency divide level 1"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL3CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR6 {
        RCC_PLL3CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL3CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL3CFGR7(pub u32);
impl RCC_PLL3CFGR7 {
    #[doc = "PLLx VCO frequency divide level 2"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "PLLx VCO frequency divide level 2"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL3CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL3CFGR7 {
        RCC_PLL3CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL3CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL3CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL3CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL3CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR1(pub u32);
impl RCC_PLL4CFGR1 {
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR2 register"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL4CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR1 {
        RCC_PLL4CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL4CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR2(pub u32);
impl RCC_PLL4CFGR2 {
    #[doc = "Bit fields for RCC_PLL4CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL4CFGR3 register"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PLL4CFGR3 register"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL4CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR2 {
        RCC_PLL4CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL4CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR3(pub u32);
impl RCC_PLL4CFGR3 {
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR4 register"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL4CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR3 {
        RCC_PLL4CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL4CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR4(pub u32);
impl RCC_PLL4CFGR4 {
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL4CFGR5 register"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL4CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR4 {
        RCC_PLL4CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL4CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR5(pub u32);
impl RCC_PLL4CFGR5 {
    #[doc = "Bit fields for RCC_PLL4CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL4CFGR6 register"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL4CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL4CFGR6 register"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL4CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR5 {
        RCC_PLL4CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL4CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR6(pub u32);
impl RCC_PLL4CFGR6 {
    #[doc = "Bit fields for RCC_PLL4CFGR7 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL4CFGR7 register"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL4CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR6 {
        RCC_PLL4CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL4CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL4CFGR7(pub u32);
impl RCC_PLL4CFGR7 {
    #[doc = "Bit fields for RCC_PLL5CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL5CFGR1 register"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL4CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL4CFGR7 {
        RCC_PLL4CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL4CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL4CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL4CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL4CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR1(pub u32);
impl RCC_PLL5CFGR1 {
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR2 register"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL5CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR1 {
        RCC_PLL5CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL5CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR2(pub u32);
impl RCC_PLL5CFGR2 {
    #[doc = "Bit fields for RCC_PLL5CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL5CFGR3 register"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PLL5CFGR3 register"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL5CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR2 {
        RCC_PLL5CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL5CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR3(pub u32);
impl RCC_PLL5CFGR3 {
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR4 register"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL5CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR3 {
        RCC_PLL5CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL5CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR4(pub u32);
impl RCC_PLL5CFGR4 {
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL5CFGR5 register"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL5CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR4 {
        RCC_PLL5CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL5CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR5(pub u32);
impl RCC_PLL5CFGR5 {
    #[doc = "Bit fields for RCC_PLL5CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL5CFGR6 register"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL5CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL5CFGR6 register"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL5CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR5 {
        RCC_PLL5CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL5CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR6(pub u32);
impl RCC_PLL5CFGR6 {
    #[doc = "Bit fields for RCC_PLL5CFGR7 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL5CFGR7 register"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL5CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR6 {
        RCC_PLL5CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL5CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL5CFGR7(pub u32);
impl RCC_PLL5CFGR7 {
    #[doc = "Bit fields for RCC_PLL6CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL6CFGR1 register"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL5CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL5CFGR7 {
        RCC_PLL5CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL5CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL5CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL5CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL5CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR1(pub u32);
impl RCC_PLL6CFGR1 {
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR2 register"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL6CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR1 {
        RCC_PLL6CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL6CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR2(pub u32);
impl RCC_PLL6CFGR2 {
    #[doc = "Bit fields for RCC_PLL6CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL6CFGR3 register"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PLL6CFGR3 register"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL6CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR2 {
        RCC_PLL6CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL6CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR3(pub u32);
impl RCC_PLL6CFGR3 {
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR4 register"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL6CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR3 {
        RCC_PLL6CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL6CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR4(pub u32);
impl RCC_PLL6CFGR4 {
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL6CFGR5 register"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL6CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR4 {
        RCC_PLL6CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL6CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR5(pub u32);
impl RCC_PLL6CFGR5 {
    #[doc = "Bit fields for RCC_PLL6CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL6CFGR6 register"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL6CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL6CFGR6 register"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL6CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR5 {
        RCC_PLL6CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL6CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR6(pub u32);
impl RCC_PLL6CFGR6 {
    #[doc = "Bit fields for RCC_PLL6CFGR7 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL6CFGR7 register"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL6CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR6 {
        RCC_PLL6CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL6CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL6CFGR7(pub u32);
impl RCC_PLL6CFGR7 {
    #[doc = "Bit fields for RCC_PLL7CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL7CFGR1 register"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL6CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL6CFGR7 {
        RCC_PLL6CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL6CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL6CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL6CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL6CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR1(pub u32);
impl RCC_PLL7CFGR1 {
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR2 register"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL7CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR1 {
        RCC_PLL7CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL7CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR2(pub u32);
impl RCC_PLL7CFGR2 {
    #[doc = "Bit fields for RCC_PLL7CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL7CFGR3 register"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PLL7CFGR3 register"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL7CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR2 {
        RCC_PLL7CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL7CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR3(pub u32);
impl RCC_PLL7CFGR3 {
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR4 register"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL7CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR3 {
        RCC_PLL7CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL7CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR4(pub u32);
impl RCC_PLL7CFGR4 {
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL7CFGR5 register"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL7CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR4 {
        RCC_PLL7CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL7CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR5(pub u32);
impl RCC_PLL7CFGR5 {
    #[doc = "Bit fields for RCC_PLL7CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL7CFGR6 register"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL7CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL7CFGR6 register"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL7CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR5 {
        RCC_PLL7CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL7CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR6(pub u32);
impl RCC_PLL7CFGR6 {
    #[doc = "Bit fields for RCC_PLL7CFGR7 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL7CFGR7 register"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL7CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR6 {
        RCC_PLL7CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL7CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL7CFGR7(pub u32);
impl RCC_PLL7CFGR7 {
    #[doc = "Bit fields for RCC_PLL8CFGR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL8CFGR1 register"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL7CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL7CFGR7 {
        RCC_PLL7CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL7CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL7CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL7CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL7CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR1(pub u32);
impl RCC_PLL8CFGR1 {
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSMODRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[inline(always)]
    pub const fn set_SSMODRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PLLRDY(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[inline(always)]
    pub const fn set_PLLRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn CKREFST(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR2 register"]
    #[inline(always)]
    pub const fn set_CKREFST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for RCC_PLL8CFGR1 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR1 {
        RCC_PLL8CFGR1(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR1")
            .field("SSMODRST", &self.SSMODRST())
            .field("PLLEN", &self.PLLEN())
            .field("PLLRDY", &self.PLLRDY())
            .field("CKREFST", &self.CKREFST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL8CFGR1 {{ SSMODRST: {=bool:?}, PLLEN: {=bool:?}, PLLRDY: {=bool:?}, CKREFST: {=bool:?} }}",
            self.SSMODRST(),
            self.PLLEN(),
            self.PLLRDY(),
            self.CKREFST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR2(pub u32);
impl RCC_PLL8CFGR2 {
    #[doc = "Bit fields for RCC_PLL8CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FREFDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL8CFGR3 register"]
    #[inline(always)]
    pub const fn set_FREFDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR3 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FBDIV(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PLL8CFGR3 register"]
    #[inline(always)]
    pub const fn set_FBDIV(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for RCC_PLL8CFGR2 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR2 {
        RCC_PLL8CFGR2(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR2")
            .field("FREFDIV", &self.FREFDIV())
            .field("FBDIV", &self.FBDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL8CFGR2 {{ FREFDIV: {=u8:?}, FBDIV: {=u16:?} }}",
            self.FREFDIV(),
            self.FBDIV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR3(pub u32);
impl RCC_PLL8CFGR3 {
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FRACIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[inline(always)]
    pub const fn set_FRACIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DOWNSPREAD(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[inline(always)]
    pub const fn set_DOWNSPREAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DACEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[inline(always)]
    pub const fn set_DACEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SSCGDIS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR4 register"]
    #[inline(always)]
    pub const fn set_SSCGDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RCC_PLL8CFGR3 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR3 {
        RCC_PLL8CFGR3(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR3")
            .field("FRACIN", &self.FRACIN())
            .field("DOWNSPREAD", &self.DOWNSPREAD())
            .field("DACEN", &self.DACEN())
            .field("SSCGDIS", &self.SSCGDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL8CFGR3 {{ FRACIN: {=u32:?}, DOWNSPREAD: {=bool:?}, DACEN: {=bool:?}, SSCGDIS: {=bool:?} }}",
            self.FRACIN(),
            self.DOWNSPREAD(),
            self.DACEN(),
            self.SSCGDIS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR4(pub u32);
impl RCC_PLL8CFGR4 {
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DSMEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[inline(always)]
    pub const fn set_DSMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn FOUTPOSTDIVEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[inline(always)]
    pub const fn set_FOUTPOSTDIVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PLL8CFGR5 register"]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for RCC_PLL8CFGR4 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR4 {
        RCC_PLL8CFGR4(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR4")
            .field("DSMEN", &self.DSMEN())
            .field("FOUTPOSTDIVEN", &self.FOUTPOSTDIVEN())
            .field("BYPASS", &self.BYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL8CFGR4 {{ DSMEN: {=bool:?}, FOUTPOSTDIVEN: {=bool:?}, BYPASS: {=bool:?} }}",
            self.DSMEN(),
            self.FOUTPOSTDIVEN(),
            self.BYPASS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR5(pub u32);
impl RCC_PLL8CFGR5 {
    #[doc = "Bit fields for RCC_PLL8CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL8CFGR6 register"]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PLL8CFGR6 register"]
    #[must_use]
    #[inline(always)]
    pub const fn SPREAD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL8CFGR6 register"]
    #[inline(always)]
    pub const fn set_SPREAD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for RCC_PLL8CFGR5 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR5 {
        RCC_PLL8CFGR5(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR5")
            .field("DIVVAL", &self.DIVVAL())
            .field("SPREAD", &self.SPREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PLL8CFGR5 {{ DIVVAL: {=u8:?}, SPREAD: {=u8:?} }}",
            self.DIVVAL(),
            self.SPREAD()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR6(pub u32);
impl RCC_PLL8CFGR6 {
    #[doc = "Bit fields for RCC_PLL8CFGR7 register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_PLL8CFGR7 register"]
    #[inline(always)]
    pub const fn set_POSTDIV1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL8CFGR6 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR6 {
        RCC_PLL8CFGR6(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR6")
            .field("POSTDIV1", &self.POSTDIV1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL8CFGR6 {{ POSTDIV1: {=u8:?} }}", self.POSTDIV1())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PLL8CFGR7(pub u32);
impl RCC_PLL8CFGR7 {
    #[doc = "Bit fields for RCC_VERR register"]
    #[must_use]
    #[inline(always)]
    pub const fn POSTDIV2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Bit fields for RCC_VERR register"]
    #[inline(always)]
    pub const fn set_POSTDIV2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RCC_PLL8CFGR7 {
    #[inline(always)]
    fn default() -> RCC_PLL8CFGR7 {
        RCC_PLL8CFGR7(0)
    }
}
impl core::fmt::Debug for RCC_PLL8CFGR7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PLL8CFGR7")
            .field("POSTDIV2", &self.POSTDIV2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PLL8CFGR7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PLL8CFGR7 {{ POSTDIV2: {=u8:?} }}", self.POSTDIV2())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PREDIVSR1(pub u32);
impl RCC_PREDIVSR1 {
    #[doc = "Bit fields for RCC_PREDIVSR2 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PREDIVSTS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_PREDIVSR2 register"]
    #[inline(always)]
    pub const fn set_PREDIVSTS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_PREDIVSR1 {
    #[inline(always)]
    fn default() -> RCC_PREDIVSR1 {
        RCC_PREDIVSR1(0)
    }
}
impl core::fmt::Debug for RCC_PREDIVSR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PREDIVSR1")
            .field("PREDIVSTS", &self.PREDIVSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PREDIVSR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PREDIVSR1 {{ PREDIVSTS: {=u32:?} }}",
            self.PREDIVSTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PREDIVSR2(pub u32);
impl RCC_PREDIVSR2 {
    #[doc = "Bit fields for RCC_FINDIVxCFGR registers : x in \\[0;63\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn PREDIVSTS(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bit fields for RCC_FINDIVxCFGR registers : x in \\[0;63\\]"]
    #[inline(always)]
    pub const fn set_PREDIVSTS(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_PREDIVSR2 {
    #[inline(always)]
    fn default() -> RCC_PREDIVSR2 {
        RCC_PREDIVSR2(0)
    }
}
impl core::fmt::Debug for RCC_PREDIVSR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PREDIVSR2")
            .field("PREDIVSTS", &self.PREDIVSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PREDIVSR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PREDIVSR2 {{ PREDIVSTS: {=u32:?} }}",
            self.PREDIVSTS()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PREDIVxCFGR(pub u32);
impl RCC_PREDIVxCFGR {
    #[doc = "Bit fields for RCC_PREDIVSR1 register"]
    #[must_use]
    #[inline(always)]
    pub const fn PREDIVx(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Bit fields for RCC_PREDIVSR1 register"]
    #[inline(always)]
    pub const fn set_PREDIVx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for RCC_PREDIVxCFGR {
    #[inline(always)]
    fn default() -> RCC_PREDIVxCFGR {
        RCC_PREDIVxCFGR(0)
    }
}
impl core::fmt::Debug for RCC_PREDIVxCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PREDIVxCFGR")
            .field("PREDIVx", &self.PREDIVx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PREDIVxCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_PREDIVxCFGR {{ PREDIVx: {=u16:?} }}", self.PREDIVx())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_PWRLPDLYCR(pub u32);
impl RCC_PWRLPDLYCR {
    #[doc = "PWR_LP delay value"]
    #[must_use]
    #[inline(always)]
    pub const fn PWRLP_DLY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "PWR_LP delay value"]
    #[inline(always)]
    pub const fn set_PWRLP_DLY(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[doc = "Skip of PWR_LP delay for CPU2"]
    #[must_use]
    #[inline(always)]
    pub const fn CPU2TMPSKP(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Skip of PWR_LP delay for CPU2"]
    #[inline(always)]
    pub const fn set_CPU2TMPSKP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RCC_PWRLPDLYCR {
    #[inline(always)]
    fn default() -> RCC_PWRLPDLYCR {
        RCC_PWRLPDLYCR(0)
    }
}
impl core::fmt::Debug for RCC_PWRLPDLYCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_PWRLPDLYCR")
            .field("PWRLP_DLY", &self.PWRLP_DLY())
            .field("CPU2TMPSKP", &self.CPU2TMPSKP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_PWRLPDLYCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_PWRLPDLYCR {{ PWRLP_DLY: {=u32:?}, CPU2TMPSKP: {=bool:?} }}",
            self.PWRLP_DLY(),
            self.CPU2TMPSKP()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_RDCR(pub u32);
impl RCC_RDCR {
    #[doc = "Minimum reset duration"]
    #[must_use]
    #[inline(always)]
    pub const fn MRD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Minimum reset duration"]
    #[inline(always)]
    pub const fn set_MRD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "External access delays"]
    #[must_use]
    #[inline(always)]
    pub const fn EADLY(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "External access delays"]
    #[inline(always)]
    pub const fn set_EADLY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for RCC_RDCR {
    #[inline(always)]
    fn default() -> RCC_RDCR {
        RCC_RDCR(0)
    }
}
impl core::fmt::Debug for RCC_RDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RDCR")
            .field("MRD", &self.MRD())
            .field("EADLY", &self.EADLY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_RDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_RDCR {{ MRD: {=u8:?}, EADLY: {=u8:?} }}",
            self.MRD(),
            self.EADLY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_RETRAMCFGR(pub u32);
impl RCC_RETRAMCFGR {
    #[doc = "RETRAM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RETRAMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM clock enable"]
    #[inline(always)]
    pub const fn set_RETRAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RETRAM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn RETRAMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_RETRAMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_RETRAMCFGR {
    #[inline(always)]
    fn default() -> RCC_RETRAMCFGR {
        RCC_RETRAMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_RETRAMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RETRAMCFGR")
            .field("RETRAMEN", &self.RETRAMEN())
            .field("RETRAMLPEN", &self.RETRAMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_RETRAMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_RETRAMCFGR {{ RETRAMEN: {=bool:?}, RETRAMLPEN: {=bool:?} }}",
            self.RETRAMEN(),
            self.RETRAMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_RNGCFGR(pub u32);
impl RCC_RNGCFGR {
    #[doc = "RNG reset"]
    #[must_use]
    #[inline(always)]
    pub const fn RNGRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RNG reset"]
    #[inline(always)]
    pub const fn set_RNGRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RNG clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RNGEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RNG clock enable"]
    #[inline(always)]
    pub const fn set_RNGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RNG clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn RNGLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RNG clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_RNGLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_RNGCFGR {
    #[inline(always)]
    fn default() -> RCC_RNGCFGR {
        RCC_RNGCFGR(0)
    }
}
impl core::fmt::Debug for RCC_RNGCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RNGCFGR")
            .field("RNGRST", &self.RNGRST())
            .field("RNGEN", &self.RNGEN())
            .field("RNGLPEN", &self.RNGLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_RNGCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_RNGCFGR {{ RNGRST: {=bool:?}, RNGEN: {=bool:?}, RNGLPEN: {=bool:?} }}",
            self.RNGRST(),
            self.RNGEN(),
            self.RNGLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_RTCCFGR(pub u32);
impl RCC_RTCCFGR {
    #[doc = "RTC bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RTC bus clock enable"]
    #[inline(always)]
    pub const fn set_RTCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RTC bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RTC bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_RTCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RTC autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCAMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RTC autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_RTCAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_RTCCFGR {
    #[inline(always)]
    fn default() -> RCC_RTCCFGR {
        RCC_RTCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_RTCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RTCCFGR")
            .field("RTCEN", &self.RTCEN())
            .field("RTCLPEN", &self.RTCLPEN())
            .field("RTCAMEN", &self.RTCAMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_RTCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_RTCCFGR {{ RTCEN: {=bool:?}, RTCLPEN: {=bool:?}, RTCAMEN: {=bool:?} }}",
            self.RTCEN(),
            self.RTCLPEN(),
            self.RTCAMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_RTCDIVR(pub u32);
impl RCC_RTCDIVR {
    #[doc = "HSE division factor for RTC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn RTCDIV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "HSE division factor for RTC clock"]
    #[inline(always)]
    pub const fn set_RTCDIV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RCC_RTCDIVR {
    #[inline(always)]
    fn default() -> RCC_RTCDIVR {
        RCC_RTCDIVR(0)
    }
}
impl core::fmt::Debug for RCC_RTCDIVR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_RTCDIVR")
            .field("RTCDIV", &self.RTCDIV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_RTCDIVR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_RTCDIVR {{ RTCDIV: {=u8:?} }}", self.RTCDIV())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SAESCFGR(pub u32);
impl RCC_SAESCFGR {
    #[doc = "SAES reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SAESRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SAES reset"]
    #[inline(always)]
    pub const fn set_SAESRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAES clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SAESEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SAES clock enable"]
    #[inline(always)]
    pub const fn set_SAESEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SAES clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SAESLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SAES clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SAESLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SAESCFGR {
    #[inline(always)]
    fn default() -> RCC_SAESCFGR {
        RCC_SAESCFGR(0)
    }
}
impl core::fmt::Debug for RCC_SAESCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SAESCFGR")
            .field("SAESRST", &self.SAESRST())
            .field("SAESEN", &self.SAESEN())
            .field("SAESLPEN", &self.SAESLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SAESCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SAESCFGR {{ SAESRST: {=bool:?}, SAESEN: {=bool:?}, SAESLPEN: {=bool:?} }}",
            self.SAESRST(),
            self.SAESEN(),
            self.SAESLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SAI1CFGR(pub u32);
impl RCC_SAI1CFGR {
    #[doc = "SAI1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 reset"]
    #[inline(always)]
    pub const fn set_SAI1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAI1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 clock enable"]
    #[inline(always)]
    pub const fn set_SAI1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SAI1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SAI1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SAI1CFGR {
    #[inline(always)]
    fn default() -> RCC_SAI1CFGR {
        RCC_SAI1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SAI1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SAI1CFGR")
            .field("SAI1RST", &self.SAI1RST())
            .field("SAI1EN", &self.SAI1EN())
            .field("SAI1LPEN", &self.SAI1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SAI1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SAI1CFGR {{ SAI1RST: {=bool:?}, SAI1EN: {=bool:?}, SAI1LPEN: {=bool:?} }}",
            self.SAI1RST(),
            self.SAI1EN(),
            self.SAI1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SAI2CFGR(pub u32);
impl RCC_SAI2CFGR {
    #[doc = "SAI2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SAI2 reset"]
    #[inline(always)]
    pub const fn set_SAI2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAI2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SAI2 clock enable"]
    #[inline(always)]
    pub const fn set_SAI2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SAI2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SAI2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SAI2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SAI2CFGR {
    #[inline(always)]
    fn default() -> RCC_SAI2CFGR {
        RCC_SAI2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SAI2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SAI2CFGR")
            .field("SAI2RST", &self.SAI2RST())
            .field("SAI2EN", &self.SAI2EN())
            .field("SAI2LPEN", &self.SAI2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SAI2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SAI2CFGR {{ SAI2RST: {=bool:?}, SAI2EN: {=bool:?}, SAI2LPEN: {=bool:?} }}",
            self.SAI2RST(),
            self.SAI2EN(),
            self.SAI2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SAI3CFGR(pub u32);
impl RCC_SAI3CFGR {
    #[doc = "SAI3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 reset"]
    #[inline(always)]
    pub const fn set_SAI3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAI3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 clock enable"]
    #[inline(always)]
    pub const fn set_SAI3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SAI3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SAI3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SAI3CFGR {
    #[inline(always)]
    fn default() -> RCC_SAI3CFGR {
        RCC_SAI3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SAI3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SAI3CFGR")
            .field("SAI3RST", &self.SAI3RST())
            .field("SAI3EN", &self.SAI3EN())
            .field("SAI3LPEN", &self.SAI3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SAI3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SAI3CFGR {{ SAI3RST: {=bool:?}, SAI3EN: {=bool:?}, SAI3LPEN: {=bool:?} }}",
            self.SAI3RST(),
            self.SAI3EN(),
            self.SAI3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SAI4CFGR(pub u32);
impl RCC_SAI4CFGR {
    #[doc = "SAI4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SAI4 reset"]
    #[inline(always)]
    pub const fn set_SAI4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAI4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SAI4 clock enable"]
    #[inline(always)]
    pub const fn set_SAI4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SAI4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SAI4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SAI4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SAI4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SAI4CFGR {
    #[inline(always)]
    fn default() -> RCC_SAI4CFGR {
        RCC_SAI4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SAI4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SAI4CFGR")
            .field("SAI4RST", &self.SAI4RST())
            .field("SAI4EN", &self.SAI4EN())
            .field("SAI4LPEN", &self.SAI4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SAI4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SAI4CFGR {{ SAI4RST: {=bool:?}, SAI4EN: {=bool:?}, SAI4LPEN: {=bool:?} }}",
            self.SAI4RST(),
            self.SAI4EN(),
            self.SAI4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SDMMC1CFGR(pub u32);
impl RCC_SDMMC1CFGR {
    #[doc = "SDMMC1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC1 reset"]
    #[inline(always)]
    pub const fn set_SDMMC1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDMMC1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC1 clock enable"]
    #[inline(always)]
    pub const fn set_SDMMC1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SDMMC1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SDMMC1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SDMMC1 DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC1DLLRST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC1 DLL reset"]
    #[inline(always)]
    pub const fn set_SDMMC1DLLRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_SDMMC1CFGR {
    #[inline(always)]
    fn default() -> RCC_SDMMC1CFGR {
        RCC_SDMMC1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SDMMC1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SDMMC1CFGR")
            .field("SDMMC1RST", &self.SDMMC1RST())
            .field("SDMMC1EN", &self.SDMMC1EN())
            .field("SDMMC1LPEN", &self.SDMMC1LPEN())
            .field("SDMMC1DLLRST", &self.SDMMC1DLLRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SDMMC1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SDMMC1CFGR {{ SDMMC1RST: {=bool:?}, SDMMC1EN: {=bool:?}, SDMMC1LPEN: {=bool:?}, SDMMC1DLLRST: {=bool:?} }}",
            self.SDMMC1RST(),
            self.SDMMC1EN(),
            self.SDMMC1LPEN(),
            self.SDMMC1DLLRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SDMMC2CFGR(pub u32);
impl RCC_SDMMC2CFGR {
    #[doc = "SDMMC2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC2 reset"]
    #[inline(always)]
    pub const fn set_SDMMC2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDMMC2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC2 clock enable"]
    #[inline(always)]
    pub const fn set_SDMMC2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SDMMC2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SDMMC2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SDMMC2 DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC2DLLRST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC2 DLL reset"]
    #[inline(always)]
    pub const fn set_SDMMC2DLLRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_SDMMC2CFGR {
    #[inline(always)]
    fn default() -> RCC_SDMMC2CFGR {
        RCC_SDMMC2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SDMMC2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SDMMC2CFGR")
            .field("SDMMC2RST", &self.SDMMC2RST())
            .field("SDMMC2EN", &self.SDMMC2EN())
            .field("SDMMC2LPEN", &self.SDMMC2LPEN())
            .field("SDMMC2DLLRST", &self.SDMMC2DLLRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SDMMC2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SDMMC2CFGR {{ SDMMC2RST: {=bool:?}, SDMMC2EN: {=bool:?}, SDMMC2LPEN: {=bool:?}, SDMMC2DLLRST: {=bool:?} }}",
            self.SDMMC2RST(),
            self.SDMMC2EN(),
            self.SDMMC2LPEN(),
            self.SDMMC2DLLRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SDMMC3CFGR(pub u32);
impl RCC_SDMMC3CFGR {
    #[doc = "SDMMC3 block reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC3 block reset"]
    #[inline(always)]
    pub const fn set_SDMMC3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDMMC3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC3 clock enable"]
    #[inline(always)]
    pub const fn set_SDMMC3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SDMMC3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SDMMC3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SDMMC3 DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SDMMC3DLLRST(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SDMMC3 DLL reset"]
    #[inline(always)]
    pub const fn set_SDMMC3DLLRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RCC_SDMMC3CFGR {
    #[inline(always)]
    fn default() -> RCC_SDMMC3CFGR {
        RCC_SDMMC3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SDMMC3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SDMMC3CFGR")
            .field("SDMMC3RST", &self.SDMMC3RST())
            .field("SDMMC3EN", &self.SDMMC3EN())
            .field("SDMMC3LPEN", &self.SDMMC3LPEN())
            .field("SDMMC3DLLRST", &self.SDMMC3DLLRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SDMMC3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SDMMC3CFGR {{ SDMMC3RST: {=bool:?}, SDMMC3EN: {=bool:?}, SDMMC3LPEN: {=bool:?}, SDMMC3DLLRST: {=bool:?} }}",
            self.SDMMC3RST(),
            self.SDMMC3EN(),
            self.SDMMC3LPEN(),
            self.SDMMC3DLLRST()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SERCCFGR(pub u32);
impl RCC_SERCCFGR {
    #[doc = "SERC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SERCRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SERC reset"]
    #[inline(always)]
    pub const fn set_SERCRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SERC clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SERCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SERC clocks enable"]
    #[inline(always)]
    pub const fn set_SERCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SERC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SERCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SERC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SERCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SERCCFGR {
    #[inline(always)]
    fn default() -> RCC_SERCCFGR {
        RCC_SERCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_SERCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SERCCFGR")
            .field("SERCRST", &self.SERCRST())
            .field("SERCEN", &self.SERCEN())
            .field("SERCLPEN", &self.SERCLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SERCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SERCCFGR {{ SERCRST: {=bool:?}, SERCEN: {=bool:?}, SERCLPEN: {=bool:?} }}",
            self.SERCRST(),
            self.SERCEN(),
            self.SERCLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SIDR(pub u32);
impl RCC_SIDR {
    #[must_use]
    #[inline(always)]
    pub const fn SID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_SID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCC_SIDR {
    #[inline(always)]
    fn default() -> RCC_SIDR {
        RCC_SIDR(0)
    }
}
impl core::fmt::Debug for RCC_SIDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SIDR")
            .field("SID", &self.SID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SIDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCC_SIDR {{ SID: {=u32:?} }}", self.SID())
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPDIFRXCFGR(pub u32);
impl RCC_SPDIFRXCFGR {
    #[doc = "SPDIFRX reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPDIFRXRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIFRX reset"]
    #[inline(always)]
    pub const fn set_SPDIFRXRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPDIFRX clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPDIFRXEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIFRX clock enable"]
    #[inline(always)]
    pub const fn set_SPDIFRXEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPDIFRX clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPDIFRXLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPDIFRX clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPDIFRXLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPDIFRXCFGR {
    #[inline(always)]
    fn default() -> RCC_SPDIFRXCFGR {
        RCC_SPDIFRXCFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPDIFRXCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPDIFRXCFGR")
            .field("SPDIFRXRST", &self.SPDIFRXRST())
            .field("SPDIFRXEN", &self.SPDIFRXEN())
            .field("SPDIFRXLPEN", &self.SPDIFRXLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPDIFRXCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPDIFRXCFGR {{ SPDIFRXRST: {=bool:?}, SPDIFRXEN: {=bool:?}, SPDIFRXLPEN: {=bool:?} }}",
            self.SPDIFRXRST(),
            self.SPDIFRXEN(),
            self.SPDIFRXLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI1CFGR(pub u32);
impl RCC_SPI1CFGR {
    #[doc = "SPI1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI1 reset"]
    #[inline(always)]
    pub const fn set_SPI1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI1 clock enable"]
    #[inline(always)]
    pub const fn set_SPI1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI1CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI1CFGR {
        RCC_SPI1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI1CFGR")
            .field("SPI1RST", &self.SPI1RST())
            .field("SPI1EN", &self.SPI1EN())
            .field("SPI1LPEN", &self.SPI1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI1CFGR {{ SPI1RST: {=bool:?}, SPI1EN: {=bool:?}, SPI1LPEN: {=bool:?} }}",
            self.SPI1RST(),
            self.SPI1EN(),
            self.SPI1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI2CFGR(pub u32);
impl RCC_SPI2CFGR {
    #[doc = "SPI2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI2 reset"]
    #[inline(always)]
    pub const fn set_SPI2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI2 clock enable"]
    #[inline(always)]
    pub const fn set_SPI2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI2CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI2CFGR {
        RCC_SPI2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI2CFGR")
            .field("SPI2RST", &self.SPI2RST())
            .field("SPI2EN", &self.SPI2EN())
            .field("SPI2LPEN", &self.SPI2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI2CFGR {{ SPI2RST: {=bool:?}, SPI2EN: {=bool:?}, SPI2LPEN: {=bool:?} }}",
            self.SPI2RST(),
            self.SPI2EN(),
            self.SPI2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI3CFGR(pub u32);
impl RCC_SPI3CFGR {
    #[doc = "SPI3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI3 reset"]
    #[inline(always)]
    pub const fn set_SPI3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI3 clock enable"]
    #[inline(always)]
    pub const fn set_SPI3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI3CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI3CFGR {
        RCC_SPI3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI3CFGR")
            .field("SPI3RST", &self.SPI3RST())
            .field("SPI3EN", &self.SPI3EN())
            .field("SPI3LPEN", &self.SPI3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI3CFGR {{ SPI3RST: {=bool:?}, SPI3EN: {=bool:?}, SPI3LPEN: {=bool:?} }}",
            self.SPI3RST(),
            self.SPI3EN(),
            self.SPI3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI4CFGR(pub u32);
impl RCC_SPI4CFGR {
    #[doc = "SPI4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI4 reset"]
    #[inline(always)]
    pub const fn set_SPI4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI4 clock enable"]
    #[inline(always)]
    pub const fn set_SPI4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI4CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI4CFGR {
        RCC_SPI4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI4CFGR")
            .field("SPI4RST", &self.SPI4RST())
            .field("SPI4EN", &self.SPI4EN())
            .field("SPI4LPEN", &self.SPI4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI4CFGR {{ SPI4RST: {=bool:?}, SPI4EN: {=bool:?}, SPI4LPEN: {=bool:?} }}",
            self.SPI4RST(),
            self.SPI4EN(),
            self.SPI4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI5CFGR(pub u32);
impl RCC_SPI5CFGR {
    #[doc = "SPI5 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI5RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI5 reset"]
    #[inline(always)]
    pub const fn set_SPI5RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI5 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI5 clock enable"]
    #[inline(always)]
    pub const fn set_SPI5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI5 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI5 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI5CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI5CFGR {
        RCC_SPI5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI5CFGR")
            .field("SPI5RST", &self.SPI5RST())
            .field("SPI5EN", &self.SPI5EN())
            .field("SPI5LPEN", &self.SPI5LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI5CFGR {{ SPI5RST: {=bool:?}, SPI5EN: {=bool:?}, SPI5LPEN: {=bool:?} }}",
            self.SPI5RST(),
            self.SPI5EN(),
            self.SPI5LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI6CFGR(pub u32);
impl RCC_SPI6CFGR {
    #[doc = "SPI6 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI6RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI6 reset"]
    #[inline(always)]
    pub const fn set_SPI6RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI6 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI6EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI6 clock enable"]
    #[inline(always)]
    pub const fn set_SPI6EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI6 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI6LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI6 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI6LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI6CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI6CFGR {
        RCC_SPI6CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI6CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI6CFGR")
            .field("SPI6RST", &self.SPI6RST())
            .field("SPI6EN", &self.SPI6EN())
            .field("SPI6LPEN", &self.SPI6LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI6CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI6CFGR {{ SPI6RST: {=bool:?}, SPI6EN: {=bool:?}, SPI6LPEN: {=bool:?} }}",
            self.SPI6RST(),
            self.SPI6EN(),
            self.SPI6LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI7CFGR(pub u32);
impl RCC_SPI7CFGR {
    #[doc = "SPI7 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI7RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI7 reset"]
    #[inline(always)]
    pub const fn set_SPI7RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI7 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI7EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI7 clock enable"]
    #[inline(always)]
    pub const fn set_SPI7EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI7 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI7LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI7 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI7LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SPI7CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI7CFGR {
        RCC_SPI7CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI7CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI7CFGR")
            .field("SPI7RST", &self.SPI7RST())
            .field("SPI7EN", &self.SPI7EN())
            .field("SPI7LPEN", &self.SPI7LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI7CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI7CFGR {{ SPI7RST: {=bool:?}, SPI7EN: {=bool:?}, SPI7LPEN: {=bool:?} }}",
            self.SPI7RST(),
            self.SPI7EN(),
            self.SPI7LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SPI8CFGR(pub u32);
impl RCC_SPI8CFGR {
    #[doc = "SPI8 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI8RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI8 reset"]
    #[inline(always)]
    pub const fn set_SPI8RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI8 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI8EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI8 clock enable"]
    #[inline(always)]
    pub const fn set_SPI8EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI8 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI8LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI8 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SPI8LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SPI8 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SPI8AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SPI8 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_SPI8AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_SPI8CFGR {
    #[inline(always)]
    fn default() -> RCC_SPI8CFGR {
        RCC_SPI8CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SPI8CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SPI8CFGR")
            .field("SPI8RST", &self.SPI8RST())
            .field("SPI8EN", &self.SPI8EN())
            .field("SPI8LPEN", &self.SPI8LPEN())
            .field("SPI8AMEN", &self.SPI8AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SPI8CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SPI8CFGR {{ SPI8RST: {=bool:?}, SPI8EN: {=bool:?}, SPI8LPEN: {=bool:?}, SPI8AMEN: {=bool:?} }}",
            self.SPI8RST(),
            self.SPI8EN(),
            self.SPI8LPEN(),
            self.SPI8AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SRAM1CFGR(pub u32);
impl RCC_SRAM1CFGR {
    #[doc = "SRAM1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM1 clock enable"]
    #[inline(always)]
    pub const fn set_SRAM1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SRAM1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SRAM1CFGR {
    #[inline(always)]
    fn default() -> RCC_SRAM1CFGR {
        RCC_SRAM1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SRAM1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SRAM1CFGR")
            .field("SRAM1EN", &self.SRAM1EN())
            .field("SRAM1LPEN", &self.SRAM1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SRAM1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SRAM1CFGR {{ SRAM1EN: {=bool:?}, SRAM1LPEN: {=bool:?} }}",
            self.SRAM1EN(),
            self.SRAM1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SRAM2CFGR(pub u32);
impl RCC_SRAM2CFGR {
    #[doc = "SRAM2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM2 clock enable"]
    #[inline(always)]
    pub const fn set_SRAM2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SRAM2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SRAM2CFGR {
    #[inline(always)]
    fn default() -> RCC_SRAM2CFGR {
        RCC_SRAM2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SRAM2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SRAM2CFGR")
            .field("SRAM2EN", &self.SRAM2EN())
            .field("SRAM2LPEN", &self.SRAM2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SRAM2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SRAM2CFGR {{ SRAM2EN: {=bool:?}, SRAM2LPEN: {=bool:?} }}",
            self.SRAM2EN(),
            self.SRAM2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_STBYBOOTCR(pub u32);
impl RCC_STBYBOOTCR {
    #[doc = "CPU boot select"]
    #[must_use]
    #[inline(always)]
    pub const fn CPU_BEN_SEL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CPU boot select"]
    #[inline(always)]
    pub const fn set_CPU_BEN_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU2 cold boot"]
    #[must_use]
    #[inline(always)]
    pub const fn COLD_CPU2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 cold boot"]
    #[inline(always)]
    pub const fn set_COLD_CPU2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CPU2 hardware boot enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CPU2_HW_BEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CPU2 hardware boot enable"]
    #[inline(always)]
    pub const fn set_CPU2_HW_BEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 hardware boot enable"]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_HW_BEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 hardware boot enable"]
    #[inline(always)]
    pub const fn set_CPU1_HW_BEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RETRAM CRC error system reset enable"]
    #[must_use]
    #[inline(always)]
    pub const fn RET_CRCERR_RSTEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RETRAM CRC error system reset enable"]
    #[inline(always)]
    pub const fn set_RET_CRCERR_RSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for RCC_STBYBOOTCR {
    #[inline(always)]
    fn default() -> RCC_STBYBOOTCR {
        RCC_STBYBOOTCR(0)
    }
}
impl core::fmt::Debug for RCC_STBYBOOTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_STBYBOOTCR")
            .field("CPU_BEN_SEL", &self.CPU_BEN_SEL())
            .field("COLD_CPU2", &self.COLD_CPU2())
            .field("CPU2_HW_BEN", &self.CPU2_HW_BEN())
            .field("CPU1_HW_BEN", &self.CPU1_HW_BEN())
            .field("RET_CRCERR_RSTEN", &self.RET_CRCERR_RSTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_STBYBOOTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_STBYBOOTCR {{ CPU_BEN_SEL: {=bool:?}, COLD_CPU2: {=bool:?}, CPU2_HW_BEN: {=bool:?}, CPU1_HW_BEN: {=bool:?}, RET_CRCERR_RSTEN: {=bool:?} }}",
            self.CPU_BEN_SEL(),
            self.COLD_CPU2(),
            self.CPU2_HW_BEN(),
            self.CPU1_HW_BEN(),
            self.RET_CRCERR_RSTEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_STGENCFGR(pub u32);
impl RCC_STGENCFGR {
    #[doc = "STGENRW and STGENRO clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STGENEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "STGENRW and STGENRO clock enable"]
    #[inline(always)]
    pub const fn set_STGENEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "STGENRW and STGENRO clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn STGENLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "STGENRW and STGENRO clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_STGENLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "STGENRW and STGENRO kernel clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn STGENSTPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "STGENRW and STGENRO kernel clock enable during CStop"]
    #[inline(always)]
    pub const fn set_STGENSTPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RCC_STGENCFGR {
    #[inline(always)]
    fn default() -> RCC_STGENCFGR {
        RCC_STGENCFGR(0)
    }
}
impl core::fmt::Debug for RCC_STGENCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_STGENCFGR")
            .field("STGENEN", &self.STGENEN())
            .field("STGENLPEN", &self.STGENLPEN())
            .field("STGENSTPEN", &self.STGENSTPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_STGENCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_STGENCFGR {{ STGENEN: {=bool:?}, STGENLPEN: {=bool:?}, STGENSTPEN: {=bool:?} }}",
            self.STGENEN(),
            self.STGENLPEN(),
            self.STGENSTPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_STMCFGR(pub u32);
impl RCC_STMCFGR {
    #[doc = "STM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn STMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "STM clock enable"]
    #[inline(always)]
    pub const fn set_STMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "STM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn STMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "STM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_STMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_STMCFGR {
    #[inline(always)]
    fn default() -> RCC_STMCFGR {
        RCC_STMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_STMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_STMCFGR")
            .field("STMEN", &self.STMEN())
            .field("STMLPEN", &self.STMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_STMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_STMCFGR {{ STMEN: {=bool:?}, STMLPEN: {=bool:?} }}",
            self.STMEN(),
            self.STMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SYSCPU1CFGR(pub u32);
impl RCC_SYSCPU1CFGR {
    #[doc = "CPU1 system configuration clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCPU1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system configuration clock enable"]
    #[inline(always)]
    pub const fn set_SYSCPU1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU1 system configuration clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSCPU1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 system configuration clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SYSCPU1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SYSCPU1CFGR {
    #[inline(always)]
    fn default() -> RCC_SYSCPU1CFGR {
        RCC_SYSCPU1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_SYSCPU1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SYSCPU1CFGR")
            .field("SYSCPU1EN", &self.SYSCPU1EN())
            .field("SYSCPU1LPEN", &self.SYSCPU1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SYSCPU1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SYSCPU1CFGR {{ SYSCPU1EN: {=bool:?}, SYSCPU1LPEN: {=bool:?} }}",
            self.SYSCPU1EN(),
            self.SYSCPU1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_SYSRAMCFGR(pub u32);
impl RCC_SYSRAMCFGR {
    #[doc = "SYSRAM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRAMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SYSRAM clock enable"]
    #[inline(always)]
    pub const fn set_SYSRAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SYSRAM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRAMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SYSRAM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_SYSRAMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_SYSRAMCFGR {
    #[inline(always)]
    fn default() -> RCC_SYSRAMCFGR {
        RCC_SYSRAMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_SYSRAMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SYSRAMCFGR")
            .field("SYSRAMEN", &self.SYSRAMEN())
            .field("SYSRAMLPEN", &self.SYSRAMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_SYSRAMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_SYSRAMCFGR {{ SYSRAMEN: {=bool:?}, SYSRAMLPEN: {=bool:?} }}",
            self.SYSRAMEN(),
            self.SYSRAMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM10CFGR(pub u32);
impl RCC_TIM10CFGR {
    #[doc = "TIM10 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM10RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM10 reset"]
    #[inline(always)]
    pub const fn set_TIM10RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM10 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM10EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM10 clock enable"]
    #[inline(always)]
    pub const fn set_TIM10EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM10 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM10LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM10 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM10LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM10CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM10CFGR {
        RCC_TIM10CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM10CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM10CFGR")
            .field("TIM10RST", &self.TIM10RST())
            .field("TIM10EN", &self.TIM10EN())
            .field("TIM10LPEN", &self.TIM10LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM10CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM10CFGR {{ TIM10RST: {=bool:?}, TIM10EN: {=bool:?}, TIM10LPEN: {=bool:?} }}",
            self.TIM10RST(),
            self.TIM10EN(),
            self.TIM10LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM11CFGR(pub u32);
impl RCC_TIM11CFGR {
    #[doc = "TIM11 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM11RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM11 reset"]
    #[inline(always)]
    pub const fn set_TIM11RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM11 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM11EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM11 clock enable"]
    #[inline(always)]
    pub const fn set_TIM11EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM11 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM11LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM11 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM11LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM11CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM11CFGR {
        RCC_TIM11CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM11CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM11CFGR")
            .field("TIM11RST", &self.TIM11RST())
            .field("TIM11EN", &self.TIM11EN())
            .field("TIM11LPEN", &self.TIM11LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM11CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM11CFGR {{ TIM11RST: {=bool:?}, TIM11EN: {=bool:?}, TIM11LPEN: {=bool:?} }}",
            self.TIM11RST(),
            self.TIM11EN(),
            self.TIM11LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM12CFGR(pub u32);
impl RCC_TIM12CFGR {
    #[doc = "TIM12 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM12RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM12 reset"]
    #[inline(always)]
    pub const fn set_TIM12RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM12 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM12EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM12 clock enable"]
    #[inline(always)]
    pub const fn set_TIM12EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM12 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM12LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM12 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM12LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM12CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM12CFGR {
        RCC_TIM12CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM12CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM12CFGR")
            .field("TIM12RST", &self.TIM12RST())
            .field("TIM12EN", &self.TIM12EN())
            .field("TIM12LPEN", &self.TIM12LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM12CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM12CFGR {{ TIM12RST: {=bool:?}, TIM12EN: {=bool:?}, TIM12LPEN: {=bool:?} }}",
            self.TIM12RST(),
            self.TIM12EN(),
            self.TIM12LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM13CFGR(pub u32);
impl RCC_TIM13CFGR {
    #[doc = "TIM13 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM13RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM13 reset"]
    #[inline(always)]
    pub const fn set_TIM13RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM13 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM13EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM13 clock enable"]
    #[inline(always)]
    pub const fn set_TIM13EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM13 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM13LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM13 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM13LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM13CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM13CFGR {
        RCC_TIM13CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM13CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM13CFGR")
            .field("TIM13RST", &self.TIM13RST())
            .field("TIM13EN", &self.TIM13EN())
            .field("TIM13LPEN", &self.TIM13LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM13CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM13CFGR {{ TIM13RST: {=bool:?}, TIM13EN: {=bool:?}, TIM13LPEN: {=bool:?} }}",
            self.TIM13RST(),
            self.TIM13EN(),
            self.TIM13LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM14CFGR(pub u32);
impl RCC_TIM14CFGR {
    #[doc = "TIM14 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM14RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM14 reset"]
    #[inline(always)]
    pub const fn set_TIM14RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM14 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM14EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM14 clock enable"]
    #[inline(always)]
    pub const fn set_TIM14EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM14 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM14LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM14 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM14LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM14CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM14CFGR {
        RCC_TIM14CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM14CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM14CFGR")
            .field("TIM14RST", &self.TIM14RST())
            .field("TIM14EN", &self.TIM14EN())
            .field("TIM14LPEN", &self.TIM14LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM14CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM14CFGR {{ TIM14RST: {=bool:?}, TIM14EN: {=bool:?}, TIM14LPEN: {=bool:?} }}",
            self.TIM14RST(),
            self.TIM14EN(),
            self.TIM14LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM15CFGR(pub u32);
impl RCC_TIM15CFGR {
    #[doc = "TIM15 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM15RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM15 reset"]
    #[inline(always)]
    pub const fn set_TIM15RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM15 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM15EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM15 clock enable"]
    #[inline(always)]
    pub const fn set_TIM15EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM15 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM15LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM15 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM15LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM15CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM15CFGR {
        RCC_TIM15CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM15CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM15CFGR")
            .field("TIM15RST", &self.TIM15RST())
            .field("TIM15EN", &self.TIM15EN())
            .field("TIM15LPEN", &self.TIM15LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM15CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM15CFGR {{ TIM15RST: {=bool:?}, TIM15EN: {=bool:?}, TIM15LPEN: {=bool:?} }}",
            self.TIM15RST(),
            self.TIM15EN(),
            self.TIM15LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM16CFGR(pub u32);
impl RCC_TIM16CFGR {
    #[doc = "TIM16 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM16RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 reset"]
    #[inline(always)]
    pub const fn set_TIM16RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM16 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM16EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 clock enable"]
    #[inline(always)]
    pub const fn set_TIM16EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM16 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM16LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM16 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM16LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM16CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM16CFGR {
        RCC_TIM16CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM16CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM16CFGR")
            .field("TIM16RST", &self.TIM16RST())
            .field("TIM16EN", &self.TIM16EN())
            .field("TIM16LPEN", &self.TIM16LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM16CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM16CFGR {{ TIM16RST: {=bool:?}, TIM16EN: {=bool:?}, TIM16LPEN: {=bool:?} }}",
            self.TIM16RST(),
            self.TIM16EN(),
            self.TIM16LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM17CFGR(pub u32);
impl RCC_TIM17CFGR {
    #[doc = "TIM17 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM17RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 reset"]
    #[inline(always)]
    pub const fn set_TIM17RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM17 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM17EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 clock enable"]
    #[inline(always)]
    pub const fn set_TIM17EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM17 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM17LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM17 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM17LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM17CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM17CFGR {
        RCC_TIM17CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM17CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM17CFGR")
            .field("TIM17RST", &self.TIM17RST())
            .field("TIM17EN", &self.TIM17EN())
            .field("TIM17LPEN", &self.TIM17LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM17CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM17CFGR {{ TIM17RST: {=bool:?}, TIM17EN: {=bool:?}, TIM17LPEN: {=bool:?} }}",
            self.TIM17RST(),
            self.TIM17EN(),
            self.TIM17LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM1CFGR(pub u32);
impl RCC_TIM1CFGR {
    #[doc = "TIM1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 reset"]
    #[inline(always)]
    pub const fn set_TIM1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 clock enable"]
    #[inline(always)]
    pub const fn set_TIM1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM1CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM1CFGR {
        RCC_TIM1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM1CFGR")
            .field("TIM1RST", &self.TIM1RST())
            .field("TIM1EN", &self.TIM1EN())
            .field("TIM1LPEN", &self.TIM1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM1CFGR {{ TIM1RST: {=bool:?}, TIM1EN: {=bool:?}, TIM1LPEN: {=bool:?} }}",
            self.TIM1RST(),
            self.TIM1EN(),
            self.TIM1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM20CFGR(pub u32);
impl RCC_TIM20CFGR {
    #[doc = "TIM20 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM20RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM20 reset"]
    #[inline(always)]
    pub const fn set_TIM20RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM20 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM20EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM20 clock enable"]
    #[inline(always)]
    pub const fn set_TIM20EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM20 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM20LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM20 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM20LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM20CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM20CFGR {
        RCC_TIM20CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM20CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM20CFGR")
            .field("TIM20RST", &self.TIM20RST())
            .field("TIM20EN", &self.TIM20EN())
            .field("TIM20LPEN", &self.TIM20LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM20CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM20CFGR {{ TIM20RST: {=bool:?}, TIM20EN: {=bool:?}, TIM20LPEN: {=bool:?} }}",
            self.TIM20RST(),
            self.TIM20EN(),
            self.TIM20LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM2CFGR(pub u32);
impl RCC_TIM2CFGR {
    #[doc = "TIM2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM2 reset"]
    #[inline(always)]
    pub const fn set_TIM2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM2 clock enable"]
    #[inline(always)]
    pub const fn set_TIM2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM2CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM2CFGR {
        RCC_TIM2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM2CFGR")
            .field("TIM2RST", &self.TIM2RST())
            .field("TIM2EN", &self.TIM2EN())
            .field("TIM2LPEN", &self.TIM2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM2CFGR {{ TIM2RST: {=bool:?}, TIM2EN: {=bool:?}, TIM2LPEN: {=bool:?} }}",
            self.TIM2RST(),
            self.TIM2EN(),
            self.TIM2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM3CFGR(pub u32);
impl RCC_TIM3CFGR {
    #[doc = "TIM3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 reset"]
    #[inline(always)]
    pub const fn set_TIM3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 clock enable"]
    #[inline(always)]
    pub const fn set_TIM3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM3CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM3CFGR {
        RCC_TIM3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM3CFGR")
            .field("TIM3RST", &self.TIM3RST())
            .field("TIM3EN", &self.TIM3EN())
            .field("TIM3LPEN", &self.TIM3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM3CFGR {{ TIM3RST: {=bool:?}, TIM3EN: {=bool:?}, TIM3LPEN: {=bool:?} }}",
            self.TIM3RST(),
            self.TIM3EN(),
            self.TIM3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM4CFGR(pub u32);
impl RCC_TIM4CFGR {
    #[doc = "TIM4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM4 reset"]
    #[inline(always)]
    pub const fn set_TIM4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM4 clock enable"]
    #[inline(always)]
    pub const fn set_TIM4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM4CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM4CFGR {
        RCC_TIM4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM4CFGR")
            .field("TIM4RST", &self.TIM4RST())
            .field("TIM4EN", &self.TIM4EN())
            .field("TIM4LPEN", &self.TIM4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM4CFGR {{ TIM4RST: {=bool:?}, TIM4EN: {=bool:?}, TIM4LPEN: {=bool:?} }}",
            self.TIM4RST(),
            self.TIM4EN(),
            self.TIM4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM5CFGR(pub u32);
impl RCC_TIM5CFGR {
    #[doc = "TIM5 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM5RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM5 reset"]
    #[inline(always)]
    pub const fn set_TIM5RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM5 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM5 clock enable"]
    #[inline(always)]
    pub const fn set_TIM5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM5 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM5 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM5CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM5CFGR {
        RCC_TIM5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM5CFGR")
            .field("TIM5RST", &self.TIM5RST())
            .field("TIM5EN", &self.TIM5EN())
            .field("TIM5LPEN", &self.TIM5LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM5CFGR {{ TIM5RST: {=bool:?}, TIM5EN: {=bool:?}, TIM5LPEN: {=bool:?} }}",
            self.TIM5RST(),
            self.TIM5EN(),
            self.TIM5LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM6CFGR(pub u32);
impl RCC_TIM6CFGR {
    #[doc = "TIM6 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM6RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM6 reset"]
    #[inline(always)]
    pub const fn set_TIM6RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM6 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM6EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM6 clock enable"]
    #[inline(always)]
    pub const fn set_TIM6EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM6 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM6LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM6 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM6LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM6CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM6CFGR {
        RCC_TIM6CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM6CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM6CFGR")
            .field("TIM6RST", &self.TIM6RST())
            .field("TIM6EN", &self.TIM6EN())
            .field("TIM6LPEN", &self.TIM6LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM6CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM6CFGR {{ TIM6RST: {=bool:?}, TIM6EN: {=bool:?}, TIM6LPEN: {=bool:?} }}",
            self.TIM6RST(),
            self.TIM6EN(),
            self.TIM6LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM7CFGR(pub u32);
impl RCC_TIM7CFGR {
    #[doc = "TIM7 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM7RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM7 reset"]
    #[inline(always)]
    pub const fn set_TIM7RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM7 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM7EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM7 clock enable"]
    #[inline(always)]
    pub const fn set_TIM7EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM7 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM7LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM7 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM7LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM7CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM7CFGR {
        RCC_TIM7CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM7CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM7CFGR")
            .field("TIM7RST", &self.TIM7RST())
            .field("TIM7EN", &self.TIM7EN())
            .field("TIM7LPEN", &self.TIM7LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM7CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM7CFGR {{ TIM7RST: {=bool:?}, TIM7EN: {=bool:?}, TIM7LPEN: {=bool:?} }}",
            self.TIM7RST(),
            self.TIM7EN(),
            self.TIM7LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIM8CFGR(pub u32);
impl RCC_TIM8CFGR {
    #[doc = "TIM8 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM8RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TIM8 reset"]
    #[inline(always)]
    pub const fn set_TIM8RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TIM8 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM8EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TIM8 clock enable"]
    #[inline(always)]
    pub const fn set_TIM8EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TIM8 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn TIM8LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TIM8 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_TIM8LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_TIM8CFGR {
    #[inline(always)]
    fn default() -> RCC_TIM8CFGR {
        RCC_TIM8CFGR(0)
    }
}
impl core::fmt::Debug for RCC_TIM8CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIM8CFGR")
            .field("TIM8RST", &self.TIM8RST())
            .field("TIM8EN", &self.TIM8EN())
            .field("TIM8LPEN", &self.TIM8LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIM8CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIM8CFGR {{ TIM8RST: {=bool:?}, TIM8EN: {=bool:?}, TIM8LPEN: {=bool:?} }}",
            self.TIM8RST(),
            self.TIM8EN(),
            self.TIM8LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIMG1PRER(pub u32);
impl RCC_TIMG1PRER {
    #[doc = "Clock prescaler selection for timers of group 1"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMG1PRE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock prescaler selection for timers of group 1"]
    #[inline(always)]
    pub const fn set_TIMG1PRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock prescaler status for timers of group 1"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMG1PRERDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock prescaler status for timers of group 1"]
    #[inline(always)]
    pub const fn set_TIMG1PRERDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_TIMG1PRER {
    #[inline(always)]
    fn default() -> RCC_TIMG1PRER {
        RCC_TIMG1PRER(0)
    }
}
impl core::fmt::Debug for RCC_TIMG1PRER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIMG1PRER")
            .field("TIMG1PRE", &self.TIMG1PRE())
            .field("TIMG1PRERDY", &self.TIMG1PRERDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIMG1PRER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIMG1PRER {{ TIMG1PRE: {=bool:?}, TIMG1PRERDY: {=bool:?} }}",
            self.TIMG1PRE(),
            self.TIMG1PRERDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_TIMG2PRER(pub u32);
impl RCC_TIMG2PRER {
    #[doc = "Clock prescaler selection for timers of group 2"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMG2PRE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock prescaler selection for timers of group 2"]
    #[inline(always)]
    pub const fn set_TIMG2PRE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock prescaler status for timers of group 2"]
    #[must_use]
    #[inline(always)]
    pub const fn TIMG2PRERDY(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock prescaler status for timers of group 2"]
    #[inline(always)]
    pub const fn set_TIMG2PRERDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RCC_TIMG2PRER {
    #[inline(always)]
    fn default() -> RCC_TIMG2PRER {
        RCC_TIMG2PRER(0)
    }
}
impl core::fmt::Debug for RCC_TIMG2PRER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_TIMG2PRER")
            .field("TIMG2PRE", &self.TIMG2PRE())
            .field("TIMG2PRERDY", &self.TIMG2PRERDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_TIMG2PRER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_TIMG2PRER {{ TIMG2PRE: {=bool:?}, TIMG2PRERDY: {=bool:?} }}",
            self.TIMG2PRE(),
            self.TIMG2PRERDY()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UART4CFGR(pub u32);
impl RCC_UART4CFGR {
    #[doc = "UART4 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UART4RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART4 reset"]
    #[inline(always)]
    pub const fn set_UART4RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UART4 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UART4EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UART4 clock enable"]
    #[inline(always)]
    pub const fn set_UART4EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART4 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UART4LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART4 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UART4LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UART4CFGR {
    #[inline(always)]
    fn default() -> RCC_UART4CFGR {
        RCC_UART4CFGR(0)
    }
}
impl core::fmt::Debug for RCC_UART4CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART4CFGR")
            .field("UART4RST", &self.UART4RST())
            .field("UART4EN", &self.UART4EN())
            .field("UART4LPEN", &self.UART4LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UART4CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UART4CFGR {{ UART4RST: {=bool:?}, UART4EN: {=bool:?}, UART4LPEN: {=bool:?} }}",
            self.UART4RST(),
            self.UART4EN(),
            self.UART4LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UART5CFGR(pub u32);
impl RCC_UART5CFGR {
    #[doc = "UART5 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UART5RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART5 reset"]
    #[inline(always)]
    pub const fn set_UART5RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UART5 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UART5EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UART5 clock enable"]
    #[inline(always)]
    pub const fn set_UART5EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART5 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UART5LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART5 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UART5LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UART5CFGR {
    #[inline(always)]
    fn default() -> RCC_UART5CFGR {
        RCC_UART5CFGR(0)
    }
}
impl core::fmt::Debug for RCC_UART5CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART5CFGR")
            .field("UART5RST", &self.UART5RST())
            .field("UART5EN", &self.UART5EN())
            .field("UART5LPEN", &self.UART5LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UART5CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UART5CFGR {{ UART5RST: {=bool:?}, UART5EN: {=bool:?}, UART5LPEN: {=bool:?} }}",
            self.UART5RST(),
            self.UART5EN(),
            self.UART5LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UART7CFGR(pub u32);
impl RCC_UART7CFGR {
    #[doc = "UART7 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UART7RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART7 reset"]
    #[inline(always)]
    pub const fn set_UART7RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UART7 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UART7EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UART7 clock enable"]
    #[inline(always)]
    pub const fn set_UART7EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART7 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UART7LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART7 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UART7LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UART7CFGR {
    #[inline(always)]
    fn default() -> RCC_UART7CFGR {
        RCC_UART7CFGR(0)
    }
}
impl core::fmt::Debug for RCC_UART7CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART7CFGR")
            .field("UART7RST", &self.UART7RST())
            .field("UART7EN", &self.UART7EN())
            .field("UART7LPEN", &self.UART7LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UART7CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UART7CFGR {{ UART7RST: {=bool:?}, UART7EN: {=bool:?}, UART7LPEN: {=bool:?} }}",
            self.UART7RST(),
            self.UART7EN(),
            self.UART7LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UART8CFGR(pub u32);
impl RCC_UART8CFGR {
    #[doc = "UART8 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UART8RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART8 reset"]
    #[inline(always)]
    pub const fn set_UART8RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UART8 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UART8EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UART8 clock enable"]
    #[inline(always)]
    pub const fn set_UART8EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART8 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UART8LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART8 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UART8LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UART8CFGR {
    #[inline(always)]
    fn default() -> RCC_UART8CFGR {
        RCC_UART8CFGR(0)
    }
}
impl core::fmt::Debug for RCC_UART8CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART8CFGR")
            .field("UART8RST", &self.UART8RST())
            .field("UART8EN", &self.UART8EN())
            .field("UART8LPEN", &self.UART8LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UART8CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UART8CFGR {{ UART8RST: {=bool:?}, UART8EN: {=bool:?}, UART8LPEN: {=bool:?} }}",
            self.UART8RST(),
            self.UART8EN(),
            self.UART8LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UART9CFGR(pub u32);
impl RCC_UART9CFGR {
    #[doc = "UART9 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UART9RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART9 reset"]
    #[inline(always)]
    pub const fn set_UART9RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UART9 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UART9EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UART9 clock enable"]
    #[inline(always)]
    pub const fn set_UART9EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART9 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UART9LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART9 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UART9LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UART9CFGR {
    #[inline(always)]
    fn default() -> RCC_UART9CFGR {
        RCC_UART9CFGR(0)
    }
}
impl core::fmt::Debug for RCC_UART9CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UART9CFGR")
            .field("UART9RST", &self.UART9RST())
            .field("UART9EN", &self.UART9EN())
            .field("UART9LPEN", &self.UART9LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UART9CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UART9CFGR {{ UART9RST: {=bool:?}, UART9EN: {=bool:?}, UART9LPEN: {=bool:?} }}",
            self.UART9RST(),
            self.UART9EN(),
            self.UART9LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_UCPDCFGR(pub u32);
impl RCC_UCPDCFGR {
    #[doc = "UCPD reset"]
    #[must_use]
    #[inline(always)]
    pub const fn UCPDRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UCPD reset"]
    #[inline(always)]
    pub const fn set_UCPDRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "UCPD clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn UCPDEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "UCPD clock enable"]
    #[inline(always)]
    pub const fn set_UCPDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UCPD clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn UCPDLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UCPD clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_UCPDLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_UCPDCFGR {
    #[inline(always)]
    fn default() -> RCC_UCPDCFGR {
        RCC_UCPDCFGR(0)
    }
}
impl core::fmt::Debug for RCC_UCPDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_UCPDCFGR")
            .field("UCPDRST", &self.UCPDRST())
            .field("UCPDEN", &self.UCPDEN())
            .field("UCPDLPEN", &self.UCPDLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_UCPDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_UCPDCFGR {{ UCPDRST: {=bool:?}, UCPDEN: {=bool:?}, UCPDLPEN: {=bool:?} }}",
            self.UCPDRST(),
            self.UCPDEN(),
            self.UCPDLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USART1CFGR(pub u32);
impl RCC_USART1CFGR {
    #[doc = "USART1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USART1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART1 reset"]
    #[inline(always)]
    pub const fn set_USART1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USART1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART1 clock enable"]
    #[inline(always)]
    pub const fn set_USART1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USART1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USART1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USART1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USART1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_USART1CFGR {
    #[inline(always)]
    fn default() -> RCC_USART1CFGR {
        RCC_USART1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USART1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USART1CFGR")
            .field("USART1RST", &self.USART1RST())
            .field("USART1EN", &self.USART1EN())
            .field("USART1LPEN", &self.USART1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USART1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USART1CFGR {{ USART1RST: {=bool:?}, USART1EN: {=bool:?}, USART1LPEN: {=bool:?} }}",
            self.USART1RST(),
            self.USART1EN(),
            self.USART1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USART2CFGR(pub u32);
impl RCC_USART2CFGR {
    #[doc = "USART2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USART2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART2 reset"]
    #[inline(always)]
    pub const fn set_USART2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USART2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART2 clock enable"]
    #[inline(always)]
    pub const fn set_USART2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USART2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USART2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USART2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USART2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_USART2CFGR {
    #[inline(always)]
    fn default() -> RCC_USART2CFGR {
        RCC_USART2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USART2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USART2CFGR")
            .field("USART2RST", &self.USART2RST())
            .field("USART2EN", &self.USART2EN())
            .field("USART2LPEN", &self.USART2LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USART2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USART2CFGR {{ USART2RST: {=bool:?}, USART2EN: {=bool:?}, USART2LPEN: {=bool:?} }}",
            self.USART2RST(),
            self.USART2EN(),
            self.USART2LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USART3CFGR(pub u32);
impl RCC_USART3CFGR {
    #[doc = "USART3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USART3RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART3 reset"]
    #[inline(always)]
    pub const fn set_USART3RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART3 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USART3EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART3 clock enable"]
    #[inline(always)]
    pub const fn set_USART3EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USART3 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USART3LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USART3 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USART3LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_USART3CFGR {
    #[inline(always)]
    fn default() -> RCC_USART3CFGR {
        RCC_USART3CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USART3CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USART3CFGR")
            .field("USART3RST", &self.USART3RST())
            .field("USART3EN", &self.USART3EN())
            .field("USART3LPEN", &self.USART3LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USART3CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USART3CFGR {{ USART3RST: {=bool:?}, USART3EN: {=bool:?}, USART3LPEN: {=bool:?} }}",
            self.USART3RST(),
            self.USART3EN(),
            self.USART3LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USART6CFGR(pub u32);
impl RCC_USART6CFGR {
    #[doc = "USART6 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USART6RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USART6 reset"]
    #[inline(always)]
    pub const fn set_USART6RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USART6 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USART6EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USART6 clock enable"]
    #[inline(always)]
    pub const fn set_USART6EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USART6 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USART6LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USART6 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USART6LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_USART6CFGR {
    #[inline(always)]
    fn default() -> RCC_USART6CFGR {
        RCC_USART6CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USART6CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USART6CFGR")
            .field("USART6RST", &self.USART6RST())
            .field("USART6EN", &self.USART6EN())
            .field("USART6LPEN", &self.USART6LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USART6CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USART6CFGR {{ USART6RST: {=bool:?}, USART6EN: {=bool:?}, USART6LPEN: {=bool:?} }}",
            self.USART6RST(),
            self.USART6EN(),
            self.USART6LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USB2PHY1CFGR(pub u32);
impl RCC_USB2PHY1CFGR {
    #[doc = "USB2PHY1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY1 reset"]
    #[inline(always)]
    pub const fn set_USB2PHY1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB2PHY1 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY1 clock enable"]
    #[inline(always)]
    pub const fn set_USB2PHY1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USB2PHY1 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY1 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USB2PHY1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB2PHY1 reference clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY1STPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY1 reference clock enable during CStop"]
    #[inline(always)]
    pub const fn set_USB2PHY1STPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "USB2PHY1 reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY1CKREFSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY1 reference clock selection"]
    #[inline(always)]
    pub const fn set_USB2PHY1CKREFSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RCC_USB2PHY1CFGR {
    #[inline(always)]
    fn default() -> RCC_USB2PHY1CFGR {
        RCC_USB2PHY1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USB2PHY1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USB2PHY1CFGR")
            .field("USB2PHY1RST", &self.USB2PHY1RST())
            .field("USB2PHY1EN", &self.USB2PHY1EN())
            .field("USB2PHY1LPEN", &self.USB2PHY1LPEN())
            .field("USB2PHY1STPEN", &self.USB2PHY1STPEN())
            .field("USB2PHY1CKREFSEL", &self.USB2PHY1CKREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USB2PHY1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USB2PHY1CFGR {{ USB2PHY1RST: {=bool:?}, USB2PHY1EN: {=bool:?}, USB2PHY1LPEN: {=bool:?}, USB2PHY1STPEN: {=bool:?}, USB2PHY1CKREFSEL: {=bool:?} }}",
            self.USB2PHY1RST(),
            self.USB2PHY1EN(),
            self.USB2PHY1LPEN(),
            self.USB2PHY1STPEN(),
            self.USB2PHY1CKREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USB2PHY2CFGR(pub u32);
impl RCC_USB2PHY2CFGR {
    #[doc = "USB2PHY2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY2 reset"]
    #[inline(always)]
    pub const fn set_USB2PHY2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB2PHY2 clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY2 clock enable"]
    #[inline(always)]
    pub const fn set_USB2PHY2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USB2PHY2 clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY2 clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USB2PHY2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB2PHY2 reference clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY2STPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY2 reference clock enable during CStop"]
    #[inline(always)]
    pub const fn set_USB2PHY2STPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "USB2PHY2 reference clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn USB2PHY2CKREFSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB2PHY2 reference clock select"]
    #[inline(always)]
    pub const fn set_USB2PHY2CKREFSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RCC_USB2PHY2CFGR {
    #[inline(always)]
    fn default() -> RCC_USB2PHY2CFGR {
        RCC_USB2PHY2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_USB2PHY2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USB2PHY2CFGR")
            .field("USB2PHY2RST", &self.USB2PHY2RST())
            .field("USB2PHY2EN", &self.USB2PHY2EN())
            .field("USB2PHY2LPEN", &self.USB2PHY2LPEN())
            .field("USB2PHY2STPEN", &self.USB2PHY2STPEN())
            .field("USB2PHY2CKREFSEL", &self.USB2PHY2CKREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USB2PHY2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USB2PHY2CFGR {{ USB2PHY2RST: {=bool:?}, USB2PHY2EN: {=bool:?}, USB2PHY2LPEN: {=bool:?}, USB2PHY2STPEN: {=bool:?}, USB2PHY2CKREFSEL: {=bool:?} }}",
            self.USB2PHY2RST(),
            self.USB2PHY2EN(),
            self.USB2PHY2LPEN(),
            self.USB2PHY2STPEN(),
            self.USB2PHY2CKREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USB3DRDCFGR(pub u32);
impl RCC_USB3DRDCFGR {
    #[doc = "USB3DRD reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3DRDRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB3DRD reset"]
    #[inline(always)]
    pub const fn set_USB3DRDRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB3DRD peripheral clocks enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3DRDEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB3DRD peripheral clocks enable"]
    #[inline(always)]
    pub const fn set_USB3DRDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USB3DRD clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3DRDLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USB3DRD clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USB3DRDLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB3DRD clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3DRDSTPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB3DRD clock enable during CStop"]
    #[inline(always)]
    pub const fn set_USB3DRDSTPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RCC_USB3DRDCFGR {
    #[inline(always)]
    fn default() -> RCC_USB3DRDCFGR {
        RCC_USB3DRDCFGR(0)
    }
}
impl core::fmt::Debug for RCC_USB3DRDCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USB3DRDCFGR")
            .field("USB3DRDRST", &self.USB3DRDRST())
            .field("USB3DRDEN", &self.USB3DRDEN())
            .field("USB3DRDLPEN", &self.USB3DRDLPEN())
            .field("USB3DRDSTPEN", &self.USB3DRDSTPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USB3DRDCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USB3DRDCFGR {{ USB3DRDRST: {=bool:?}, USB3DRDEN: {=bool:?}, USB3DRDLPEN: {=bool:?}, USB3DRDSTPEN: {=bool:?} }}",
            self.USB3DRDRST(),
            self.USB3DRDEN(),
            self.USB3DRDLPEN(),
            self.USB3DRDSTPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USB3PCIEPHYCFGR(pub u32);
impl RCC_USB3PCIEPHYCFGR {
    #[doc = "USB3PCIEPHY reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3PCIEPHYRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB3PCIEPHY reset"]
    #[inline(always)]
    pub const fn set_USB3PCIEPHYRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB3PCIEPHY clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3PCIEPHYEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB3PCIEPHY clock enable"]
    #[inline(always)]
    pub const fn set_USB3PCIEPHYEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USB3PCIEPHY clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3PCIEPHYLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USB3PCIEPHY clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USB3PCIEPHYLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB3PCIEPHY reference clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3PCIEPHYSTPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB3PCIEPHY reference clock enable during CStop"]
    #[inline(always)]
    pub const fn set_USB3PCIEPHYSTPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "USB3PCIEPHY reference clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn USB3PCIEPHYCKREFSEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "USB3PCIEPHY reference clock selection"]
    #[inline(always)]
    pub const fn set_USB3PCIEPHYCKREFSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for RCC_USB3PCIEPHYCFGR {
    #[inline(always)]
    fn default() -> RCC_USB3PCIEPHYCFGR {
        RCC_USB3PCIEPHYCFGR(0)
    }
}
impl core::fmt::Debug for RCC_USB3PCIEPHYCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USB3PCIEPHYCFGR")
            .field("USB3PCIEPHYRST", &self.USB3PCIEPHYRST())
            .field("USB3PCIEPHYEN", &self.USB3PCIEPHYEN())
            .field("USB3PCIEPHYLPEN", &self.USB3PCIEPHYLPEN())
            .field("USB3PCIEPHYSTPEN", &self.USB3PCIEPHYSTPEN())
            .field("USB3PCIEPHYCKREFSEL", &self.USB3PCIEPHYCKREFSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USB3PCIEPHYCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USB3PCIEPHYCFGR {{ USB3PCIEPHYRST: {=bool:?}, USB3PCIEPHYEN: {=bool:?}, USB3PCIEPHYLPEN: {=bool:?}, USB3PCIEPHYSTPEN: {=bool:?}, USB3PCIEPHYCKREFSEL: {=bool:?} }}",
            self.USB3PCIEPHYRST(),
            self.USB3PCIEPHYEN(),
            self.USB3PCIEPHYLPEN(),
            self.USB3PCIEPHYSTPEN(),
            self.USB3PCIEPHYCKREFSEL()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_USBHCFGR(pub u32);
impl RCC_USBHCFGR {
    #[doc = "USBH reset"]
    #[must_use]
    #[inline(always)]
    pub const fn USBHRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USBH reset"]
    #[inline(always)]
    pub const fn set_USBHRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USBH peripheral clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn USBHEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USBH peripheral clock enable"]
    #[inline(always)]
    pub const fn set_USBHEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "USBH clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn USBHLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "USBH clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_USBHLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USBH clock enable during CStop"]
    #[must_use]
    #[inline(always)]
    pub const fn USBHSTPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USBH clock enable during CStop"]
    #[inline(always)]
    pub const fn set_USBHSTPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RCC_USBHCFGR {
    #[inline(always)]
    fn default() -> RCC_USBHCFGR {
        RCC_USBHCFGR(0)
    }
}
impl core::fmt::Debug for RCC_USBHCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_USBHCFGR")
            .field("USBHRST", &self.USBHRST())
            .field("USBHEN", &self.USBHEN())
            .field("USBHLPEN", &self.USBHLPEN())
            .field("USBHSTPEN", &self.USBHSTPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_USBHCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_USBHCFGR {{ USBHRST: {=bool:?}, USBHEN: {=bool:?}, USBHLPEN: {=bool:?}, USBHSTPEN: {=bool:?} }}",
            self.USBHRST(),
            self.USBHEN(),
            self.USBHLPEN(),
            self.USBHSTPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_VDECCFGR(pub u32);
impl RCC_VDECCFGR {
    #[doc = "VDEC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn VDECRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VDEC reset"]
    #[inline(always)]
    pub const fn set_VDECRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VDEC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn VDECEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VDEC clock enable"]
    #[inline(always)]
    pub const fn set_VDECEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VDEC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn VDECLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VDEC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_VDECLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_VDECCFGR {
    #[inline(always)]
    fn default() -> RCC_VDECCFGR {
        RCC_VDECCFGR(0)
    }
}
impl core::fmt::Debug for RCC_VDECCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_VDECCFGR")
            .field("VDECRST", &self.VDECRST())
            .field("VDECEN", &self.VDECEN())
            .field("VDECLPEN", &self.VDECLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_VDECCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_VDECCFGR {{ VDECRST: {=bool:?}, VDECEN: {=bool:?}, VDECLPEN: {=bool:?} }}",
            self.VDECRST(),
            self.VDECEN(),
            self.VDECLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_VDERAMCFGR(pub u32);
impl RCC_VDERAMCFGR {
    #[doc = "VDERAM clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn VDERAMEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VDERAM clock enable"]
    #[inline(always)]
    pub const fn set_VDERAMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VDERAM clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn VDERAMLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VDERAM clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_VDERAMLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_VDERAMCFGR {
    #[inline(always)]
    fn default() -> RCC_VDERAMCFGR {
        RCC_VDERAMCFGR(0)
    }
}
impl core::fmt::Debug for RCC_VDERAMCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_VDERAMCFGR")
            .field("VDERAMEN", &self.VDERAMEN())
            .field("VDERAMLPEN", &self.VDERAMLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_VDERAMCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_VDERAMCFGR {{ VDERAMEN: {=bool:?}, VDERAMLPEN: {=bool:?} }}",
            self.VDERAMEN(),
            self.VDERAMLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_VENCCFGR(pub u32);
impl RCC_VENCCFGR {
    #[doc = "VENC reset"]
    #[must_use]
    #[inline(always)]
    pub const fn VENCRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VENC reset"]
    #[inline(always)]
    pub const fn set_VENCRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VENC clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn VENCEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VENC clock enable"]
    #[inline(always)]
    pub const fn set_VENCEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VENC clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn VENCLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VENC clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_VENCLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_VENCCFGR {
    #[inline(always)]
    fn default() -> RCC_VENCCFGR {
        RCC_VENCCFGR(0)
    }
}
impl core::fmt::Debug for RCC_VENCCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_VENCCFGR")
            .field("VENCRST", &self.VENCRST())
            .field("VENCEN", &self.VENCEN())
            .field("VENCLPEN", &self.VENCLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_VENCCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_VENCCFGR {{ VENCRST: {=bool:?}, VENCEN: {=bool:?}, VENCLPEN: {=bool:?} }}",
            self.VENCRST(),
            self.VENCEN(),
            self.VENCLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_VERR(pub u32);
impl RCC_VERR {
    #[doc = "Bit fields for RCC_IDR register"]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_IDR register"]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_IDR register"]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_IDR register"]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for RCC_VERR {
    #[inline(always)]
    fn default() -> RCC_VERR {
        RCC_VERR(0)
    }
}
impl core::fmt::Debug for RCC_VERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_VERR")
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_VERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_VERR {{ MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.MINREV(),
            self.MAJREV()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_VREFCFGR(pub u32);
impl RCC_VREFCFGR {
    #[doc = "VREF reset"]
    #[must_use]
    #[inline(always)]
    pub const fn VREFRST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VREF reset"]
    #[inline(always)]
    pub const fn set_VREFRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VREF clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn VREFEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VREF clock enable"]
    #[inline(always)]
    pub const fn set_VREFEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VREF clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn VREFLPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VREF clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_VREFLPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_VREFCFGR {
    #[inline(always)]
    fn default() -> RCC_VREFCFGR {
        RCC_VREFCFGR(0)
    }
}
impl core::fmt::Debug for RCC_VREFCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_VREFCFGR")
            .field("VREFRST", &self.VREFRST())
            .field("VREFEN", &self.VREFEN())
            .field("VREFLPEN", &self.VREFLPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_VREFCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_VREFCFGR {{ VREFRST: {=bool:?}, VREFEN: {=bool:?}, VREFLPEN: {=bool:?} }}",
            self.VREFRST(),
            self.VREFEN(),
            self.VREFLPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_WWDG1CFGR(pub u32);
impl RCC_WWDG1CFGR {
    #[doc = "WWDG1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG1RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG1 reset"]
    #[inline(always)]
    pub const fn set_WWDG1RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WWDG1 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG1EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG1 bus clock enable"]
    #[inline(always)]
    pub const fn set_WWDG1EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WWDG1 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG1LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG1 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_WWDG1LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for RCC_WWDG1CFGR {
    #[inline(always)]
    fn default() -> RCC_WWDG1CFGR {
        RCC_WWDG1CFGR(0)
    }
}
impl core::fmt::Debug for RCC_WWDG1CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_WWDG1CFGR")
            .field("WWDG1RST", &self.WWDG1RST())
            .field("WWDG1EN", &self.WWDG1EN())
            .field("WWDG1LPEN", &self.WWDG1LPEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_WWDG1CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_WWDG1CFGR {{ WWDG1RST: {=bool:?}, WWDG1EN: {=bool:?}, WWDG1LPEN: {=bool:?} }}",
            self.WWDG1RST(),
            self.WWDG1EN(),
            self.WWDG1LPEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_WWDG2CFGR(pub u32);
impl RCC_WWDG2CFGR {
    #[doc = "WWDG2 block reset"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG2RST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG2 block reset"]
    #[inline(always)]
    pub const fn set_WWDG2RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WWDG2 bus clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG2EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG2 bus clock enable"]
    #[inline(always)]
    pub const fn set_WWDG2EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WWDG2 bus clock enable during CSleep"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG2LPEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG2 bus clock enable during CSleep"]
    #[inline(always)]
    pub const fn set_WWDG2LPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WWDG2 autonomous clock mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn WWDG2AMEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WWDG2 autonomous clock mode enable"]
    #[inline(always)]
    pub const fn set_WWDG2AMEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for RCC_WWDG2CFGR {
    #[inline(always)]
    fn default() -> RCC_WWDG2CFGR {
        RCC_WWDG2CFGR(0)
    }
}
impl core::fmt::Debug for RCC_WWDG2CFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_WWDG2CFGR")
            .field("WWDG2RST", &self.WWDG2RST())
            .field("WWDG2EN", &self.WWDG2EN())
            .field("WWDG2LPEN", &self.WWDG2LPEN())
            .field("WWDG2AMEN", &self.WWDG2AMEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_WWDG2CFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_WWDG2CFGR {{ WWDG2RST: {=bool:?}, WWDG2EN: {=bool:?}, WWDG2LPEN: {=bool:?}, WWDG2AMEN: {=bool:?} }}",
            self.WWDG2RST(),
            self.WWDG2EN(),
            self.WWDG2LPEN(),
            self.WWDG2AMEN()
        )
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCC_XBARxCFGR(pub u32);
impl RCC_XBARxCFGR {
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn XBARxSEL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[inline(always)]
    pub const fn set_XBARxSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn XBARxEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[inline(always)]
    pub const fn set_XBARxEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn XBARxSTS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bit fields for RCC_PREDIVxCFGR registers : x in \\[0;63\\]"]
    #[inline(always)]
    pub const fn set_XBARxSTS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for RCC_XBARxCFGR {
    #[inline(always)]
    fn default() -> RCC_XBARxCFGR {
        RCC_XBARxCFGR(0)
    }
}
impl core::fmt::Debug for RCC_XBARxCFGR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_XBARxCFGR")
            .field("XBARxSEL", &self.XBARxSEL())
            .field("XBARxEN", &self.XBARxEN())
            .field("XBARxSTS", &self.XBARxSTS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCC_XBARxCFGR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCC_XBARxCFGR {{ XBARxSEL: {=u8:?}, XBARxEN: {=bool:?}, XBARxSTS: {=bool:?} }}",
            self.XBARxSEL(),
            self.XBARxEN(),
            self.XBARxSTS()
        )
    }
}

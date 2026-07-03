#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API for i.MX9 eDMA (hand-maintained)."]
//! Register widths follow the vendor `PERI_DMA.h` CMSIS header: the TCD
//! `SOFF`/`ATTR`/`DOFF`/`CITER`/`CSR`/`BITER` registers are 16 bits wide and
//! must be accessed with halfword operations, which the chiptool renderer
//! currently models incorrectly, so this block is maintained by hand.

#[doc = "eDMA management page (instance base)."]
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
    #[doc = "Management Page Control"]
    #[inline(always)]
    pub const fn MP_CSR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x00usize) as _) }
    }
    #[doc = "Management Page Error Status"]
    #[inline(always)]
    pub const fn MP_ES(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status"]
    #[inline(always)]
    pub const fn MP_INT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status"]
    #[inline(always)]
    pub const fn MP_HRS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Arbitration Group"]
    #[inline(always)]
    pub const fn CH_GRPRI(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 31usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Channel page for channel `n` (channel control + TCD)."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> DMA_CH {
        assert!(n < 31usize);
        DMA_CH {
            ptr: self
                .ptr
                .wrapping_add(0x0001_0000usize + n * 0x0001_0000usize),
        }
    }
}

#[doc = "eDMA channel page: channel control/status registers and the TCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH {
    ptr: *mut u8,
}
unsafe impl Send for DMA_CH {}
unsafe impl Sync for DMA_CH {}
impl DMA_CH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel Control and Status"]
    #[inline(always)]
    pub const fn CH_CSR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x00usize) as _) }
    }
    #[doc = "Channel Error Status"]
    #[inline(always)]
    pub const fn CH_ES(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel Interrupt Status"]
    #[inline(always)]
    pub const fn CH_INT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel System Bus"]
    #[inline(always)]
    pub const fn CH_SBR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn CH_PRI(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn TCD_SADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset (halfword register)"]
    #[inline(always)]
    pub const fn TCD_SOFF(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "TCD Transfer Attributes (halfword register)"]
    #[inline(always)]
    pub const fn TCD_ATTR(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
    #[inline(always)]
    pub const fn TCD_NBYTES_MLOFFNO(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
    #[inline(always)]
    pub const fn TCD_SLAST_SDA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn TCD_DADDR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset (halfword register)"]
    #[inline(always)]
    pub const fn TCD_DOFF(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "TCD Current Major Loop Count, no channel linking (halfword register)"]
    #[inline(always)]
    pub const fn TCD_CITER_ELINKNO(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
    #[inline(always)]
    pub const fn TCD_DLAST_SGA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "TCD Control and Status (halfword register)"]
    #[inline(always)]
    pub const fn TCD_CSR(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TCD Beginning Major Loop Count, no channel linking (halfword register)"]
    #[inline(always)]
    pub const fn TCD_BITER_ELINKNO(self) -> crate::common::Reg<u16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
}

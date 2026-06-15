// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::{
    chan::{IPCC_CHAN_COUNT, IPCC_CHAN_RESERVED},
    error::IpccError,
    registers::{
        ChannelRegister, channel_clear_flag, channel_set_flag, cr::ControlRegister, offsets,
    },
};

/// IPCC-backed doorbell.
///
/// `PROC` is the local processor index used by the IPCC register bank:
/// - `1`: ring processor 2 through `C1SCR.CHnS`, wait on `C2TOC1SR`.
/// - `2`: ring processor 1 through `C2SCR.CHnS`, wait on `C1TOC2SR`.
///
/// The IPCC base address is supplied by board code. On Linux/std it must be a
/// virtual address returned by `mmap`; on bare metal it is normally the
/// peripheral physical address.
pub struct IpccDoorbell<const PROC: u8> {
    base: usize,
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

impl<const PROC: u8> IpccDoorbell<PROC> {
    #[inline]
    const fn valid_proc() -> bool {
        PROC == 1 || PROC == 2
    }

    #[inline]
    fn reg_addr(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    #[inline]
    const fn cr_offset() -> Result<usize, IpccError> {
        match PROC {
            1 => Ok(offsets::C1CR),
            2 => Ok(offsets::C2CR),
            _ => Err(IpccError::InvalidProcessor),
        }
    }

    #[inline]
    const fn mr_offset() -> Result<usize, IpccError> {
        match PROC {
            1 => Ok(offsets::C1MR),
            2 => Ok(offsets::C2MR),
            _ => Err(IpccError::InvalidProcessor),
        }
    }

    #[inline]
    const fn scr_offset() -> Result<usize, IpccError> {
        match PROC {
            1 => Ok(offsets::C1SCR),
            2 => Ok(offsets::C2SCR),
            _ => Err(IpccError::InvalidProcessor),
        }
    }

    #[inline]
    const fn outgoing_status_offset() -> Result<usize, IpccError> {
        match PROC {
            1 => Ok(offsets::C1TOC2SR),
            2 => Ok(offsets::C2TOC1SR),
            _ => Err(IpccError::InvalidProcessor),
        }
    }

    #[inline]
    const fn incoming_status_offset() -> Result<usize, IpccError> {
        match PROC {
            1 => Ok(offsets::C2TOC1SR),
            2 => Ok(offsets::C1TOC2SR),
            _ => Err(IpccError::InvalidProcessor),
        }
    }

    #[inline]
    fn validate_ch(ch: Chan) -> Result<u8, IpccError> {
        match ch.id() {
            i if i < IPCC_CHAN_RESERVED => Err(IpccError::ChanReserved),
            i if i >= IPCC_CHAN_COUNT => Err(IpccError::ChanOutOfRange),
            i => Ok(i),
        }
    }

    #[inline]
    fn read_reg(&self, offset: usize) -> u32 {
        // SAFETY: caller supplied a valid mapped IPCC base address; offset is a valid IPCC register offset.
        unsafe { self.reg_addr(offset).read_volatile() }
    }

    #[inline]
    unsafe fn write_reg(&self, offset: usize, value: u32) {
        // SAFETY: caller supplied a valid mapped IPCC base address; offset is a valid IPCC register offset.
        unsafe { self.reg_addr(offset).write_volatile(value) };
    }

    #[inline]
    unsafe fn update_reg(&self, offset: usize, f: impl FnOnce(u32) -> u32) {
        let reg = self.reg_addr(offset);
        // SAFETY: caller guarantees exclusive access to this read-modify-write register update.
        let value = unsafe { reg.read_volatile() };
        // SAFETY: caller guarantees exclusive access to this read-modify-write register update.
        unsafe { reg.write_volatile(f(value)) };
    }

    #[inline]
    fn outgoing_occupied(&self, ch: u8) -> Result<bool, IpccError> {
        let register = ChannelRegister::from_bits(self.read_reg(Self::outgoing_status_offset()?));
        Ok(register.contains_channel(ch))
    }

    #[inline]
    fn incoming_occupied(&self, ch: u8) -> Result<bool, IpccError> {
        let register = ChannelRegister::from_bits(self.read_reg(Self::incoming_status_offset()?));
        Ok(register.contains_channel(ch))
    }

    #[inline]
    unsafe fn set_outgoing_occupied(&self, ch: u8) -> Result<(), IpccError> {
        // SAFETY: SCR uses write-one action fields; writing only CHnS avoids disturbing other channels.
        unsafe { self.write_reg(Self::scr_offset()?, channel_set_flag(ch).bits()) };
        Ok(())
    }

    #[inline]
    unsafe fn clear_incoming_occupied(&self, ch: u8) -> Result<(), IpccError> {
        // SAFETY: SCR uses write-one action fields; writing only CHnC avoids disturbing other channels.
        unsafe { self.write_reg(Self::scr_offset()?, channel_clear_flag(ch).bits()) };
        Ok(())
    }

    #[inline]
    unsafe fn enable_rx_occupied_interrupt(&self, ch: u8) -> Result<(), IpccError> {
        // SAFETY: caller coordinates access to CR/MR interrupt configuration.
        unsafe {
            self.update_reg(Self::cr_offset()?, |value| {
                let mut register = ControlRegister::from_bits(value);
                register.set_rxoie(true);
                register.bits()
            })
        };
        // CHnOM is active-high mask: clear bit to unmask RX occupied for this channel.
        // SAFETY: caller coordinates access to CR/MR interrupt configuration.
        unsafe {
            self.update_reg(Self::mr_offset()?, |value| {
                ChannelRegister::from_bits(value)
                    .with_channel_unmasked(ch)
                    .bits()
            })
        };
        Ok(())
    }

    #[inline]
    unsafe fn mask_rx_occupied_interrupt(&self, ch: u8) -> Result<(), IpccError> {
        // CHnOM is active-high mask: set bit to mask RX occupied for this channel.
        // SAFETY: caller coordinates access to MR interrupt configuration.
        unsafe {
            self.update_reg(Self::mr_offset()?, |value| {
                ChannelRegister::from_bits(value)
                    .with_channel_masked(ch)
                    .bits()
            })
        };
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<const PROC: u8> IpccDoorbell<PROC> {
    /// Create an IPCC doorbell from an mmap'd virtual IPCC base address.
    ///
    /// # Panics
    ///
    /// Panics if `PROC` is not 1 or 2.
    pub fn new(base: usize) -> Self {
        assert!(Self::valid_proc(), "PROC must be 1 or 2");
        Self {
            base,
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`.
    ///
    /// Pass this to the UIO IRQ loop or platform IRQ adapter so it can wake
    /// waiting [`wait()`](Doorbell::wait) futures when IPCC RX occupied fires.
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

#[cfg(not(feature = "std"))]
impl<const PROC: u8> IpccDoorbell<PROC> {
    /// Create an IPCC doorbell from a board-provided IPCC base address.
    ///
    /// # Panics
    ///
    /// Panics if `PROC` is not 1 or 2.
    pub const fn new(base: usize) -> Self {
        assert!(Self::valid_proc(), "PROC must be 1 or 2");
        Self { base }
    }

    /// Create a bare-metal doorbell for the main IPCC1 instance.
    ///
    /// IPCC1 is present at `0x4049_0000` on STM32MP21/23/25.
    #[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
    pub const fn ipcc1() -> Self {
        Self::new(crate::registers::base::IPCC1_BASE)
    }

    /// Create a bare-metal doorbell for the SmartRun IPCC2 instance.
    ///
    /// IPCC2 is present at `0x4625_0000` only on STM32MP23/25.
    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    pub const fn ipcc2() -> Self {
        Self::new(crate::registers::base::IPCC2_BASE)
    }
}

impl<const PROC: u8> Doorbell for IpccDoorbell<PROC> {
    type Error = IpccError;
    type WaitFut<'a>
        = IpccWaitFut<'a, PROC>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), Self::Error> {
        let id = Self::validate_ch(ch)?;
        if self.outgoing_occupied(id)? {
            return Err(IpccError::ChannelOccupied);
        }

        compiler_fence(Ordering::Release);

        // SAFETY: id is validated and SCR offset is selected from the local processor bank.
        unsafe { self.set_outgoing_occupied(id)? };
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        IpccWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        let Ok(id) = Self::validate_ch(ch) else {
            return false;
        };
        self.incoming_occupied(id).unwrap_or(false)
    }
}

/// Future returned by [`IpccDoorbell::wait()`].
pub struct IpccWaitFut<'a, const PROC: u8> {
    doorbell: &'a IpccDoorbell<PROC>,
    ch: Chan,
    interrupt_armed: bool,
    #[cfg(feature = "std")]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

impl<'a, const PROC: u8> IpccWaitFut<'a, PROC> {
    fn new(doorbell: &'a IpccDoorbell<PROC>, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            interrupt_armed: false,
            #[cfg(feature = "std")]
            notified: None,
        }
    }
}

impl<'a, const PROC: u8> core::future::Future for IpccWaitFut<'a, PROC> {
    type Output = Result<Chan, IpccError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: the future does not move pinned fields after `notified` is stored.
        let this = unsafe { self.get_unchecked_mut() };
        let id = IpccDoorbell::<PROC>::validate_ch(this.ch)?;

        if !this.interrupt_armed {
            // SAFETY: id is validated; caller/platform coordinates interrupt register updates.
            unsafe { this.doorbell.enable_rx_occupied_interrupt(id)? };
            this.interrupt_armed = true;

            #[cfg(feature = "std")]
            {
                let notified = this.doorbell.notify.notified();
                // SAFETY: `doorbell` is borrowed for `'a`, and its Arc-backed Notify outlives this future.
                let notified = unsafe {
                    core::mem::transmute::<
                        tokio::sync::futures::Notified<'_>,
                        tokio::sync::futures::Notified<'static>,
                    >(notified)
                };
                this.notified = Some(notified);
            }

            #[cfg(not(feature = "std"))]
            {
                IPCC_WAKER.register(cx.waker());
            }
        }

        if this.doorbell.incoming_occupied(id)? {
            compiler_fence(Ordering::Acquire);

            // SAFETY: id is validated and the incoming status bit is set.
            unsafe { this.doorbell.clear_incoming_occupied(id)? };
            // SAFETY: id is validated; masking after completion avoids repeated wakeups.
            unsafe { this.doorbell.mask_rx_occupied_interrupt(id)? };

            return Poll::Ready(Ok(this.ch));
        }

        #[cfg(feature = "std")]
        if let Some(notified) = this.notified.as_mut() {
            // SAFETY: `notified` lives inside the pinned future.
            let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
        }

        #[cfg(not(feature = "std"))]
        {
            IPCC_WAKER.register(cx.waker());
        }

        Poll::Pending
    }
}

#[cfg(not(feature = "std"))]
static IPCC_WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Wake tasks waiting on an IPCC RX occupied interrupt.
///
/// Board support code should call this from the verified STM32MP2 IPCC RX
/// occupied interrupt vector. The exact NVIC line is intentionally not encoded
/// here because it differs by family and interrupt routing configuration.
#[cfg(not(feature = "std"))]
pub fn notify_rx_occupied_interrupt() {
    IPCC_WAKER.wake();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ch(id: u8) -> Chan {
        // SAFETY: tests choose ids deliberately to exercise validation paths.
        unsafe { Chan::new_unchecked(id) }
    }

    #[test]
    fn ring_sets_processor_1_channel_occupied() {
        let mut regs = [0u32; 8];
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);

        doorbell.ring(ch(2)).unwrap();

        assert_eq!(regs[offsets::C1SCR / 4], channel_set_flag(2).bits());
    }

    #[test]
    fn ring_rejects_occupied_outgoing_channel() {
        let mut regs = [0u32; 8];
        regs[offsets::C1TOC2SR / 4] = crate::registers::channel_flag(2).bits();
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);

        assert_eq!(doorbell.ring(ch(2)), Err(IpccError::ChannelOccupied));
    }

    #[test]
    fn pending_reads_incoming_status() {
        let mut regs = [0u32; 8];
        regs[offsets::C2TOC1SR / 4] = crate::registers::channel_flag(3).bits();
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);

        assert!(doorbell.pending(ch(3)));
        assert!(!doorbell.pending(ch(4)));
    }

    #[test]
    fn wait_clears_incoming_channel() {
        let mut regs = [0u32; 8];
        regs[offsets::C2TOC1SR / 4] = crate::registers::channel_flag(2).bits();
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);
        let mut wait = std::pin::pin!(doorbell.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert!(ControlRegister::from_bits(regs[offsets::C1CR / 4]).rxoie());
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).contains_channel(2));
        assert_eq!(regs[offsets::C1SCR / 4], channel_clear_flag(2).bits());
    }

    #[cfg(feature = "std")]
    #[test]
    fn wait_registers_waker_then_ready_after_notify_and_incoming_status() {
        let mut regs = [0u32; 8];
        regs[offsets::C1MR / 4] = u32::MAX;
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);
        let mut wait = std::pin::pin!(doorbell.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(ControlRegister::from_bits(regs[offsets::C1CR / 4]).rxoie());
        assert!(!ChannelRegister::from_bits(regs[offsets::C1MR / 4]).contains_channel(2));

        regs[offsets::C2TOC1SR / 4] = crate::registers::channel_flag(2).bits();
        doorbell.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert_eq!(regs[offsets::C1SCR / 4], channel_clear_flag(2).bits());
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).contains_channel(2));
    }

    #[test]
    fn validation_rejects_reserved_and_out_of_range_channels() {
        let mut regs = [0u32; 8];
        let doorbell = IpccDoorbell::<1>::new(regs.as_mut_ptr() as usize);

        assert_eq!(doorbell.ring(ch(0)), Err(IpccError::ChanReserved));
        assert_eq!(doorbell.ring(ch(16)), Err(IpccError::ChanOutOfRange));
    }
}

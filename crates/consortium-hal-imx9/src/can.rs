//! Classic CAN driver backed by an i.MX9 FlexCAN instance.
//!
//! Select an instance by passing its MMIO base to [`Can::new`] together with a
//! `&'static` [`CanState`]. Blocking transfers ([`embedded_can::nb::Can`]) work
//! without interrupts. Async [`Can::transmit`]/[`Can::receive`] require the
//! caller to route the instance's interrupt to [`CanState::on_interrupt`].
//!
//! The driver uses a fixed two-mailbox layout: message buffer 0 transmits and
//! message buffer 1 receives every frame (the receive global mask is cleared,
//! so all identifiers are accepted). This covers classic 2.0A/2.0B frames up to
//! eight data bytes; FlexCAN's FD, RX-FIFO, and multi-mailbox filtering modes
//! are intentionally out of scope for this first cut. The primitives read and
//! write the mailbox words directly, so a larger mailbox pool or an RX FIFO can
//! be layered on without changing the public shape.

use core::future::poll_fn;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embedded_can::{ErrorKind, Id, StandardId};
use pac::peripherals::can::{CAN, CAN_IFLAG1, CAN_IMASK1};

use crate::pac;

/// Transmit mailbox index.
const TX_MB: usize = 0;
/// Receive mailbox index.
const RX_MB: usize = 1;

/// Mailbox `CODE` field values (bits 24..28 of the CS word).
mod code {
    /// Mailbox is inactive.
    pub const RX_INACTIVE: u32 = 0b0000;
    /// Mailbox is empty and ready to receive.
    pub const RX_EMPTY: u32 = 0b0100;
    /// Mailbox holds an unread frame that overwrote a previous one.
    pub const RX_OVERRUN: u32 = 0b0110;
    /// Transmit mailbox is inactive.
    pub const TX_INACTIVE: u32 = 0b1000;
    /// Transmit the mailbox contents (data frame).
    pub const TX_DATA: u32 = 0b1100;
}

/// CS-word bit positions shared by transmit and receive mailboxes.
mod cs {
    pub const CODE_SHIFT: u32 = 24;
    pub const SRR: u32 = 1 << 22;
    pub const IDE: u32 = 1 << 21;
    pub const RTR: u32 = 1 << 20;
    pub const DLC_SHIFT: u32 = 16;
    pub const DLC_MASK: u32 = 0xF;
}

/// Standard identifiers occupy bits 18..29 of the ID word.
const STD_ID_SHIFT: u32 = 18;
/// Extended identifiers occupy bits 0..29 of the ID word.
const EXT_ID_MASK: u32 = 0x1FFF_FFFF;

/// CAN transfer/bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The controller entered the bus-off state.
    BusOff,
    /// A received frame overran an unread mailbox.
    Overrun,
}

impl embedded_can::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::BusOff => ErrorKind::Other,
            Error::Overrun => ErrorKind::Overrun,
        }
    }
}

/// A classic CAN frame (up to eight data bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    id: Id,
    remote: bool,
    dlc: u8,
    data: [u8; 8],
}

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if data.len() > 8 {
            return None;
        }
        let mut buf = [0u8; 8];
        buf[..data.len()].copy_from_slice(data);
        Some(Self {
            id: id.into(),
            remote: false,
            dlc: data.len() as u8,
            data: buf,
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        if dlc > 8 {
            return None;
        }
        Some(Self {
            id: id.into(),
            remote: true,
            dlc: dlc as u8,
            data: [0u8; 8],
        })
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        self.remote
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.dlc as usize]
    }
}

/// Compute `CTRL1.PRESDIV` for a target bit rate.
///
/// The driver uses a fixed 16-quantum bit time (`SYNC 1 + PROPSEG 7 + PSEG1 4 +
/// PSEG2 4`), so `PRESDIV = f_can / (16 * bitrate) - 1`, clamped to the 8-bit
/// field. Sample point sits at 12/16 = 75%.
const fn presdiv(clock_hz: u32, bitrate: u32) -> u8 {
    assert!(bitrate > 0);
    let div = clock_hz / (bitrate * 16);
    if div < 1 {
        0
    } else {
        let d = div - 1;
        if d > 255 { 255 } else { d as u8 }
    }
}

/// Per-instance interrupt state shared between a [`Can`] and its IRQ handler.
pub struct CanState {
    base: AtomicPtr<()>,
    waker: AtomicWaker,
}

impl CanState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            base: AtomicPtr::new(core::ptr::null_mut()),
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the FlexCAN instance's interrupt.
    ///
    /// Masks every mailbox interrupt that is both enabled and asserted, then
    /// wakes the pending transfer, which re-arms the mask it still needs and
    /// clears the mailbox flag once it has drained the frame.
    pub fn on_interrupt(&self) {
        let ptr = self.base.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `Can::new` installs the MMIO base for the selected FlexCAN
        // instance before any interrupt is routed here.
        let r = unsafe { CAN::from_ptr(ptr) };
        let flag = r.IFLAG1().read().0;
        let mask = r.IMASK1().read().BUF31TO0M();
        let pending = flag & mask;
        if pending != 0 {
            let mut next = CAN_IMASK1(0);
            next.set_BUF31TO0M(mask & !pending);
            r.IMASK1().write_value(next);
            self.waker.wake();
        }
    }
}

impl Default for CanState {
    fn default() -> Self {
        Self::new()
    }
}

/// FlexCAN configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Target nominal bit rate in bits per second.
    pub bitrate: u32,
    /// Enable internal loopback (and self-reception) for self-test.
    pub loopback: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bitrate: 500_000,
            loopback: false,
        }
    }
}

/// Classic CAN controller over one FlexCAN instance.
pub struct Can {
    regs: CAN,
    state: &'static CanState,
}

impl Can {
    /// Initialize a FlexCAN instance for classic CAN.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid FlexCAN register block, clocked and pinned
    /// out by the caller, that is exclusively owned by this driver for the
    /// lifetime of the program. `clock_hz` is the CAN protocol-engine clock.
    /// `state` must be dedicated to this instance; for async use, route the
    /// instance's interrupt to [`CanState::on_interrupt`] on the same state.
    pub unsafe fn new(
        base: *mut (),
        clock_hz: u32,
        config: Config,
        state: &'static CanState,
    ) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned FlexCAN
        // register block.
        let r = unsafe { CAN::from_ptr(base) };
        let this = Self { regs: r, state };

        // Enable the module and wait for it to leave low-power mode.
        r.MCR().modify(|w| w.set_MDIS(false));
        while r.MCR().read().LPMACK() {}

        // Soft-reset the logic, then wait for the reset to complete.
        r.MCR().modify(|w| w.set_SOFTRST(true));
        while r.MCR().read().SOFTRST() {}

        // Enter freeze mode to program bit timing and the mailboxes.
        r.MCR().modify(|w| {
            w.set_FRZ(true);
            w.set_HALT(true);
        });
        while !r.MCR().read().FRZACK() {}

        // Fixed 16-quantum bit timing (75% sample point) at the requested rate.
        r.CTRL1().modify(|w| {
            w.set_PRESDIV(presdiv(clock_hz, config.bitrate));
            w.set_PROPSEG(6);
            w.set_PSEG1(3);
            w.set_PSEG2(3);
            w.set_RJW(3);
            w.set_LPB(config.loopback);
        });

        // Two mailboxes; accept every identifier on the receive mailbox. Disable
        // self-reception unless loopback wants the frames echoed back.
        r.MCR().modify(|w| {
            w.set_MAXMB(RX_MB as u8);
            w.set_SRXDIS(!config.loopback);
            w.set_IRMQ(false);
        });
        r.RXMGMASK()
            .write_value(pac::peripherals::can::CAN_RXMGMASK(0));

        // Park both mailboxes, then arm the receive mailbox as empty.
        this.write_cs(TX_MB, code::TX_INACTIVE << cs::CODE_SHIFT);
        this.write_cs(RX_MB, code::RX_INACTIVE << cs::CODE_SHIFT);
        this.write_id(RX_MB, 0);
        this.write_cs(RX_MB, code::RX_EMPTY << cs::CODE_SHIFT);

        // Clear any latched flags for our mailboxes and leave freeze mode.
        r.IFLAG1()
            .write_value(CAN_IFLAG1((1 << TX_MB) | (1 << RX_MB)));
        r.MCR().modify(|w| {
            w.set_HALT(false);
            w.set_FRZ(false);
        });
        while r.MCR().read().NOTRDY() {}

        state.base.store(base, Ordering::Release);
        this
    }

    /// Base pointer of mailbox `n` (the CS word; ID/data words follow).
    fn mb_ptr(&self, n: usize) -> *mut u32 {
        self.regs.MB_8B_CS(n).as_ptr()
    }

    fn write_cs(&self, n: usize, value: u32) {
        self.regs.MB_8B_CS(n).write_value(value);
    }

    fn read_cs(&self, n: usize) -> u32 {
        self.regs.MB_8B_CS(n).read()
    }

    fn write_id(&self, n: usize, value: u32) {
        let p = self.mb_ptr(n).wrapping_add(1);
        // SAFETY: the ID word sits one 32-bit word after the CS word of a valid,
        // exclusively owned mailbox.
        unsafe { p.write_volatile(value) };
    }

    fn read_id(&self, n: usize) -> u32 {
        let p = self.mb_ptr(n).wrapping_add(1);
        // SAFETY: see `write_id`.
        unsafe { p.read_volatile() }
    }

    fn write_data(&self, n: usize, data: &[u8; 8]) {
        let w0 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let w1 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let d0 = self.mb_ptr(n).wrapping_add(2);
        let d1 = self.mb_ptr(n).wrapping_add(3);
        // SAFETY: `d0` is the first data word, two 32-bit words after the CS word
        // of a valid, exclusively owned mailbox.
        unsafe { d0.write_volatile(w0) };
        // SAFETY: `d1` is the second data word, three words after the CS word.
        unsafe { d1.write_volatile(w1) };
    }

    fn read_data(&self, n: usize) -> [u8; 8] {
        let d0 = self.mb_ptr(n).wrapping_add(2);
        let d1 = self.mb_ptr(n).wrapping_add(3);
        // SAFETY: `d0` is the first data word of a valid, exclusively owned
        // mailbox.
        let w0 = unsafe { d0.read_volatile() };
        // SAFETY: `d1` is the second data word of the same mailbox.
        let w1 = unsafe { d1.read_volatile() };
        let mut out = [0u8; 8];
        out[..4].copy_from_slice(&w0.to_be_bytes());
        out[4..].copy_from_slice(&w1.to_be_bytes());
        out
    }

    /// True once the transmit mailbox has finished its previous frame (or has
    /// never been loaded).
    fn tx_ready(&self) -> bool {
        (self.regs.IFLAG1().read().0 & (1 << TX_MB)) != 0
            || (self.read_cs(TX_MB) >> cs::CODE_SHIFT) & 0xF == code::TX_INACTIVE
    }

    /// Load and start transmission of `frame` on the transmit mailbox.
    fn load_tx(&mut self, frame: &Frame) {
        let (ide, id_word) = encode_id(frame.id);
        // Inactivate first so the engine does not read a half-written mailbox.
        self.write_cs(TX_MB, code::TX_INACTIVE << cs::CODE_SHIFT);
        self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << TX_MB));
        self.write_id(TX_MB, id_word);
        self.write_data(TX_MB, &frame.data);
        let mut word = code::TX_DATA << cs::CODE_SHIFT;
        word |= (u32::from(frame.dlc) & cs::DLC_MASK) << cs::DLC_SHIFT;
        if ide {
            word |= cs::IDE | cs::SRR;
        }
        if frame.remote {
            word |= cs::RTR;
        }
        self.write_cs(TX_MB, word);
    }

    /// Read the received frame out of the receive mailbox and re-arm it.
    fn drain_rx(&mut self) -> Result<Frame, Error> {
        // Reading the CS word locks the mailbox against engine updates.
        let cs = self.read_cs(RX_MB);
        let overrun = (cs >> cs::CODE_SHIFT) & 0xF == code::RX_OVERRUN;
        let dlc = ((cs >> cs::DLC_SHIFT) & cs::DLC_MASK).min(8) as u8;
        let ide = cs & cs::IDE != 0;
        let remote = cs & cs::RTR != 0;
        let id_word = self.read_id(RX_MB);
        let data = self.read_data(RX_MB);
        // Read the free-running timer to unlock the mailbox.
        let _ = self.regs.TIMER().read();
        // Acknowledge the mailbox; it returns to the empty state automatically.
        self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << RX_MB));

        let id = decode_id(id_word, ide);
        if overrun {
            return Err(Error::Overrun);
        }
        Ok(Frame {
            id,
            remote,
            dlc,
            data,
        })
    }

    /// True when the receive mailbox holds an unread frame.
    fn rx_pending(&self) -> bool {
        (self.regs.IFLAG1().read().0 & (1 << RX_MB)) != 0
    }

    /// Surface a bus-off condition as an error.
    fn check_bus(&self) -> Result<(), Error> {
        if self.regs.ESR1().read().FLTCONF() == 0b10 {
            Err(Error::BusOff)
        } else {
            Ok(())
        }
    }

    // --- async primitives ---------------------------------------------------

    async fn wait_flag(
        &self,
        mb: usize,
        ready: impl Fn(&Self) -> Option<Result<(), Error>>,
    ) -> Result<(), Error> {
        poll_fn(|cx| {
            if let Some(res) = ready(self) {
                return Poll::Ready(res);
            }
            self.state.waker.register(cx.waker());
            critical_section::with(|_| {
                self.regs.IMASK1().modify(|w| {
                    w.set_BUF31TO0M(w.BUF31TO0M() | (1 << mb));
                });
            });
            if let Some(res) = ready(self) {
                return Poll::Ready(res);
            }
            Poll::Pending
        })
        .await
    }

    /// Transmit `frame`, awaiting completion of the transfer.
    pub async fn transmit(&mut self, frame: &Frame) -> Result<(), Error> {
        self.check_bus()?;
        // Wait for the transmit mailbox to be free, then load the frame.
        self.wait_flag(TX_MB, |s| s.tx_ready().then_some(Ok(())))
            .await?;
        self.load_tx(frame);
        // Await the transmit-complete flag.
        self.wait_flag(TX_MB, |s| {
            ((s.regs.IFLAG1().read().0 & (1 << TX_MB)) != 0).then_some(Ok(()))
        })
        .await?;
        self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << TX_MB));
        Ok(())
    }

    /// Await and return the next received frame.
    pub async fn receive(&mut self) -> Result<Frame, Error> {
        self.wait_flag(RX_MB, |s| s.rx_pending().then_some(Ok(())))
            .await?;
        self.drain_rx()
    }
}

/// Encode an identifier into `(is_extended, id_word)`.
fn encode_id(id: Id) -> (bool, u32) {
    match id {
        Id::Standard(s) => (false, u32::from(s.as_raw()) << STD_ID_SHIFT),
        Id::Extended(e) => (true, e.as_raw() & EXT_ID_MASK),
    }
}

/// Decode an identifier from the mailbox ID word.
fn decode_id(id_word: u32, extended: bool) -> Id {
    if extended {
        // SAFETY-free: mask keeps the value within the 29-bit range, so the
        // constructor cannot fail.
        embedded_can::ExtendedId::new(id_word & EXT_ID_MASK)
            .map(Id::Extended)
            .unwrap_or(Id::Standard(StandardId::ZERO))
    } else {
        StandardId::new(((id_word >> STD_ID_SHIFT) & 0x7FF) as u16)
            .map(Id::Standard)
            .unwrap_or(Id::Standard(StandardId::ZERO))
    }
}

impl embedded_can::nb::Can for Can {
    type Frame = Frame;
    type Error = Error;

    fn transmit(&mut self, frame: &Frame) -> nb::Result<Option<Frame>, Error> {
        self.check_bus()?;
        if !self.tx_ready() {
            return Err(nb::Error::WouldBlock);
        }
        self.load_tx(frame);
        Ok(None)
    }

    fn receive(&mut self) -> nb::Result<Frame, Error> {
        if !self.rx_pending() {
            return Err(nb::Error::WouldBlock);
        }
        self.drain_rx().map_err(nb::Error::Other)
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_id, encode_id, presdiv};
    use embedded_can::{ExtendedId, Id, StandardId};

    #[test]
    fn presdiv_matches_bit_rate() {
        // 40 MHz / (16 * 500 kbit) = 5 => PRESDIV 4.
        assert_eq!(presdiv(40_000_000, 500_000), 4);
    }

    #[test]
    fn standard_id_round_trips() {
        let id = Id::Standard(StandardId::new(0x123).unwrap());
        let (ext, word) = encode_id(id);
        assert!(!ext);
        assert_eq!(decode_id(word, ext), id);
    }

    #[test]
    fn extended_id_round_trips() {
        let id = Id::Extended(ExtendedId::new(0x1AB_CDEF).unwrap());
        let (ext, word) = encode_id(id);
        assert!(ext);
        assert_eq!(decode_id(word, ext), id);
    }
}

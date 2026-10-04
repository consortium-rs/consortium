//! Classic CAN driver backed by an i.MX9 FlexCAN instance.
//!
//! Select an instance by handing its peripheral singleton to [`Can::new`]:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let can = Can::new(p.CAN1, 80_000_000, Config::default());
//! ```
//!
//! Blocking transfers ([`embedded_can::nb::Can`]) work without interrupts.
//! Async [`Can::transmit`]/[`Can::receive`] require the application to forward
//! the instance's interrupt to [`on_interrupt`] parameterized with the same
//! singleton type.
//!
//! [`Can::split`] hands out independent [`CanTx`] and [`CanRx`] halves for code
//! that transmits and receives concurrently, such as a full-duplex IPC
//! transport. The halves drive disjoint mailboxes and park on separate wakers,
//! so neither starves the other.
//!
//! The driver uses a fixed two-mailbox layout: message buffer 0 transmits and
//! message buffer 1 receives every frame (the receive global mask is cleared,
//! so all identifiers are accepted). This covers classic 2.0A/2.0B frames up to
//! eight data bytes; FlexCAN's FD, RX-FIFO, and multi-mailbox filtering modes
//! are intentionally out of scope for this first cut. The primitives read and
//! write the mailbox words directly, so a larger mailbox pool or an RX FIFO can
//! be layered on without changing the public shape.
//!
//! One consequence of the single receive mailbox is worth knowing before
//! layering a segmenting protocol on top: a frame that arrives while the
//! previous one is still unread sets the overrun code and is reported as
//! [`Error::Overrun`]. A receiver that is not already parked in
//! [`CanRx::receive`] will therefore lose frames from a back-to-back burst. An
//! RX FIFO would remove the constraint; until then, keep the bit rate and
//! message size within what the receiving task can drain.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
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
///
/// One `static` per FlexCAN instance is created by `impl_can` and reached
/// through the instance type, so callers never supply or name it.
/// The transmit and receive paths park on separate wakers so the [`CanTx`] and
/// [`CanRx`] halves can await concurrently. A single waker would let whichever
/// half registered last clobber the other's, stranding it forever.
pub struct State {
    tx_waker: AtomicWaker,
    rx_waker: AtomicWaker,
}

impl State {
    /// Create an empty state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            tx_waker: AtomicWaker::new(),
            rx_waker: AtomicWaker::new(),
        }
    }

    /// Handle the interrupt of the FlexCAN instance at `r`.
    ///
    /// Masks every mailbox interrupt that is both enabled and asserted, then
    /// wakes the transfer owning that mailbox, which re-arms the mask it still
    /// needs and clears the mailbox flag once it has drained the frame.
    fn on_interrupt(&self, r: CAN) {
        let flag = r.IFLAG1().read().0;
        let mask = r.IMASK1().read().BUF31TO0M();
        let pending = flag & mask;
        if pending != 0 {
            let mut next = CAN_IMASK1(0);
            next.set_BUF31TO0M(mask & !pending);
            r.IMASK1().write_value(next);
            if pending & (1 << TX_MB) != 0 {
                self.tx_waker.wake();
            }
            if pending & (1 << RX_MB) != 0 {
                self.rx_waker.wake();
            }
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> CAN;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// A FlexCAN instance the [`Can`] driver can drive.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address and interrupt state stay single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_can {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::can::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::can::CAN {
                // SAFETY: `$base` is this instance's FlexCAN base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::can::CAN::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::can::State {
                static STATE: crate::can::State = crate::can::State::new();
                &STATE
            }
        }

        impl crate::can::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_can;

/// Handle the FlexCAN interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
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

/// Transmit half of a FlexCAN instance, produced by [`Can::split`].
///
/// Drives the transmit mailbox and parks on the instance's transmit waker, so
/// it can run concurrently with a [`CanRx`] for the same instance.
pub struct CanTx {
    regs: CAN,
    state: &'static State,
}

/// Receive half of a FlexCAN instance, produced by [`Can::split`].
///
/// Drives the receive mailbox and parks on the instance's receive waker.
pub struct CanRx {
    regs: CAN,
    state: &'static State,
}

/// Classic CAN controller over one FlexCAN instance.
pub struct Can {
    tx: CanTx,
    rx: CanRx,
}

impl Can {
    /// Initialize a FlexCAN instance for classic CAN.
    ///
    /// `clock_hz` is the CAN protocol-engine clock; the platform must leave the
    /// instance clocked and pinned out. For async transfers, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, config: Config) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();
        let this = Self {
            tx: CanTx {
                regs: r,
                state: T::state(),
            },
            rx: CanRx {
                regs: r,
                state: T::state(),
            },
        };

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
        write_cs(r, TX_MB, code::TX_INACTIVE << cs::CODE_SHIFT);
        write_cs(r, RX_MB, code::RX_INACTIVE << cs::CODE_SHIFT);
        write_id(r, RX_MB, 0);
        write_cs(r, RX_MB, code::RX_EMPTY << cs::CODE_SHIFT);

        // Clear any latched flags for our mailboxes and leave freeze mode.
        r.IFLAG1()
            .write_value(CAN_IFLAG1((1 << TX_MB) | (1 << RX_MB)));
        r.MCR().modify(|w| {
            w.set_HALT(false);
            w.set_FRZ(false);
        });
        while r.MCR().read().NOTRDY() {}

        this
    }

    /// Split into independent transmit and receive halves.
    ///
    /// The halves drive disjoint mailboxes — buffer 0 versus buffer 1 — and park
    /// on separate wakers, so a transport can await a frame and send one at the
    /// same time.
    pub fn split(self) -> (CanTx, CanRx) {
        (self.tx, self.rx)
    }

    /// Transmit `frame`, awaiting completion of the transfer.
    pub async fn transmit(&mut self, frame: &Frame) -> Result<(), Error> {
        self.tx.transmit(frame).await
    }

    /// Await and return the next received frame.
    pub async fn receive(&mut self) -> Result<Frame, Error> {
        self.rx.receive().await
    }
}

impl CanTx {
    /// True once the transmit mailbox has finished its previous frame (or has
    /// never been loaded).
    fn tx_ready(&self) -> bool {
        (self.regs.IFLAG1().read().0 & (1 << TX_MB)) != 0
            || (read_cs(self.regs, TX_MB) >> cs::CODE_SHIFT) & 0xF == code::TX_INACTIVE
    }

    /// Load and start transmission of `frame` on the transmit mailbox.
    fn load_tx(&mut self, frame: &Frame) {
        let (ide, id_word) = encode_id(frame.id);
        // Inactivate first so the engine does not read a half-written mailbox.
        write_cs(self.regs, TX_MB, code::TX_INACTIVE << cs::CODE_SHIFT);
        self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << TX_MB));
        write_id(self.regs, TX_MB, id_word);
        write_data(self.regs, TX_MB, &frame.data);
        let mut word = code::TX_DATA << cs::CODE_SHIFT;
        word |= (u32::from(frame.dlc) & cs::DLC_MASK) << cs::DLC_SHIFT;
        if ide {
            word |= cs::IDE | cs::SRR;
        }
        if frame.remote {
            word |= cs::RTR;
        }
        write_cs(self.regs, TX_MB, word);
    }

    /// Transmit `frame`, awaiting completion of the transfer.
    pub async fn transmit(&mut self, frame: &Frame) -> Result<(), Error> {
        check_bus(self.regs)?;
        // Wait for the transmit mailbox to be free, then load the frame.
        wait_flag(&self.state.tx_waker, self.regs, TX_MB, || {
            self.tx_ready().then_some(Ok(()))
        })
        .await?;
        self.load_tx(frame);
        // A frame no node acknowledges is retransmitted forever and would hold
        // the mailbox against every later `transmit`; withdraw it if this future
        // is dropped before it completes.
        let guard = TxCancelGuard {
            regs: self.regs,
            armed: true,
        };
        // Await the transmit-complete flag.
        wait_flag(&self.state.tx_waker, self.regs, TX_MB, || {
            ((self.regs.IFLAG1().read().0 & (1 << TX_MB)) != 0).then_some(Ok(()))
        })
        .await?;
        guard.defuse();
        self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << TX_MB));
        Ok(())
    }
}

/// Deactivates the transmit mailbox when dropped unless
/// [`defuse`](Self::defuse)d. An inactive code is what [`CanTx::tx_ready`]
/// accepts, so the next `transmit` can load the mailbox again. A frame already
/// in arbitration may still complete; that is the documented FlexCAN outcome
/// of deactivating a pending mailbox.
struct TxCancelGuard {
    regs: CAN,
    armed: bool,
}

impl TxCancelGuard {
    /// The frame was sent; nothing to cancel.
    fn defuse(mut self) {
        self.armed = false;
    }
}

impl Drop for TxCancelGuard {
    fn drop(&mut self) {
        if self.armed {
            write_cs(self.regs, TX_MB, code::TX_INACTIVE << cs::CODE_SHIFT);
            self.regs.IFLAG1().write_value(CAN_IFLAG1(1 << TX_MB));
        }
    }
}

impl CanRx {
    /// Read the received frame out of the receive mailbox and re-arm it.
    fn drain_rx(&mut self) -> Result<Frame, Error> {
        // Reading the CS word locks the mailbox against engine updates.
        let cs = read_cs(self.regs, RX_MB);
        let overrun = (cs >> cs::CODE_SHIFT) & 0xF == code::RX_OVERRUN;
        let dlc = ((cs >> cs::DLC_SHIFT) & cs::DLC_MASK).min(8) as u8;
        let ide = cs & cs::IDE != 0;
        let remote = cs & cs::RTR != 0;
        let id_word = read_id(self.regs, RX_MB);
        let data = read_data(self.regs, RX_MB);
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

    /// Await and return the next received frame.
    pub async fn receive(&mut self) -> Result<Frame, Error> {
        wait_flag(&self.state.rx_waker, self.regs, RX_MB, || {
            self.rx_pending().then_some(Ok(()))
        })
        .await?;
        self.drain_rx()
    }
}

/// Base pointer of mailbox `n` (the CS word; ID/data words follow).
fn mb_ptr(regs: CAN, n: usize) -> *mut u32 {
    regs.MB_8B_CS(n).as_ptr()
}

fn write_cs(regs: CAN, n: usize, value: u32) {
    regs.MB_8B_CS(n).write_value(value);
}

fn read_cs(regs: CAN, n: usize) -> u32 {
    regs.MB_8B_CS(n).read()
}

fn write_id(regs: CAN, n: usize, value: u32) {
    let p = mb_ptr(regs, n).wrapping_add(1);
    // SAFETY: the ID word sits one 32-bit word after the CS word of a valid,
    // exclusively owned mailbox.
    unsafe { p.write_volatile(value) };
}

fn read_id(regs: CAN, n: usize) -> u32 {
    let p = mb_ptr(regs, n).wrapping_add(1);
    // SAFETY: see `write_id`.
    unsafe { p.read_volatile() }
}

fn write_data(regs: CAN, n: usize, data: &[u8; 8]) {
    let w0 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let w1 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    let d0 = mb_ptr(regs, n).wrapping_add(2);
    let d1 = mb_ptr(regs, n).wrapping_add(3);
    // SAFETY: `d0` is the first data word, two 32-bit words after the CS word
    // of a valid, exclusively owned mailbox.
    unsafe { d0.write_volatile(w0) };
    // SAFETY: `d1` is the second data word, three words after the CS word.
    unsafe { d1.write_volatile(w1) };
}

fn read_data(regs: CAN, n: usize) -> [u8; 8] {
    let d0 = mb_ptr(regs, n).wrapping_add(2);
    let d1 = mb_ptr(regs, n).wrapping_add(3);
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

/// Surface a bus-off condition as an error.
fn check_bus(regs: CAN) -> Result<(), Error> {
    if regs.ESR1().read().FLTCONF() == 0b10 {
        Err(Error::BusOff)
    } else {
        Ok(())
    }
}

/// Park on `waker` until `ready` reports a result, unmasking mailbox `mb`'s
/// interrupt each time the future is polled to `Pending`.
///
/// `ready` is re-checked after registering the waker: the interrupt can fire in
/// the window between the first check and the registration, and an already-set
/// mailbox flag would otherwise never wake anyone.
async fn wait_flag(
    waker: &AtomicWaker,
    regs: CAN,
    mb: usize,
    ready: impl Fn() -> Option<Result<(), Error>>,
) -> Result<(), Error> {
    poll_fn(|cx| {
        if let Some(res) = ready() {
            return Poll::Ready(res);
        }
        waker.register(cx.waker());
        critical_section::with(|_| {
            regs.IMASK1().modify(|w| {
                w.set_BUF31TO0M(w.BUF31TO0M() | (1 << mb));
            });
        });
        if let Some(res) = ready() {
            return Poll::Ready(res);
        }
        Poll::Pending
    })
    .await
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
        check_bus(self.tx.regs)?;
        if !self.tx.tx_ready() {
            return Err(nb::Error::WouldBlock);
        }
        self.tx.load_tx(frame);
        Ok(None)
    }

    fn receive(&mut self) -> nb::Result<Frame, Error> {
        if !self.rx.rx_pending() {
            return Err(nb::Error::WouldBlock);
        }
        self.rx.drain_rx().map_err(nb::Error::Other)
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

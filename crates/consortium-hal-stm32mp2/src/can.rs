//! Classic CAN driver backed by an STM32MP2 FDCAN (Bosch M_CAN) instance.
//!
//! Select an instance by handing its peripheral singleton to [`Can::new`],
//! along with the shared CAN message-RAM base and this instance's byte offset
//! within it. The singleton carries the register base and the per-instance
//! interrupt [`State`]; the message-RAM window has no singleton yet, which is
//! why the constructor is still `unsafe`. Blocking transfers
//! ([`embedded_can::nb::Can`]) work without interrupts. Async
//! [`Can::transmit`]/[`Can::receive`] require the application to forward one of
//! the instance's interrupt lines to [`on_interrupt::<T>`](on_interrupt) (the
//! driver enables interrupt line 0, `FDCANx_IT0`).
//!
//! [`Can::split`] hands out independent [`CanTx`] and [`CanRx`] halves for code
//! that transmits and receives concurrently, such as a full-duplex IPC
//! transport. The halves drive disjoint registers and message-RAM regions and
//! park on separate wakers, so neither starves the other.
//!
//! The message RAM is the SoC's shared `CAN_SRAM` window (`0x4031_0000`, 10 KiB
//! split across the FDCAN instances). Each instance's start-address registers
//! are programmed relative to that window, so the caller partitions the window
//! by passing a distinct `msg_ram_offset` per instance. The driver lays out a
//! [`REGION_BYTES`]-byte region for that offset: an eight-element receive FIFO 0
//! followed by a single dedicated transmit buffer, both 8-byte (classic)
//! elements, with the global filter set to accept every identifier into FIFO 0.
//! The FIFO is deep enough that a burst of back-to-back frames — what a
//! segmenting transport emits for one large message — survives a receiver that
//! is not already parked in `receive`. FD frames, extra transmit buffers, and
//! hardware acceptance filters are intentionally out of scope for this first
//! cut; the element accessors leave room to add them later.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_can::{ErrorKind, Id, StandardId};
use pac::peripherals::fdcan::{FDCAN, FDCAN_IR, FDCAN_RXF0A, FDCAN_TXBAR, FDCAN_TXBCR};

use crate::pac;

/// Receive FIFO 0 depth, in elements.
const RX_FIFO0_ELEMS: usize = 8;
/// Words per 8-byte-payload element: two header words plus two data words.
const ELEM_WORDS: usize = 4;
/// Bytes per element.
const ELEM_BYTES: usize = ELEM_WORDS * 4;
/// Local byte offset of receive FIFO 0 within this instance's region.
const RX_FIFO0_LOCAL: usize = 0;
/// Local byte offset of the transmit buffer within this instance's region.
const TX_BUF_LOCAL: usize = RX_FIFO0_ELEMS * ELEM_BYTES;

/// Bytes of message RAM one instance's layout occupies.
///
/// Callers partitioning the shared `CAN_SRAM` window must leave at least this
/// much room between the `msg_ram_offset` values they hand to [`Can::new`].
pub const REGION_BYTES: usize = TX_BUF_LOCAL + ELEM_BYTES;

/// `RXESC`/`TXESC` data-field-size code for an 8-byte payload.
const DATA_SIZE_8B: u8 = 0;

/// Element header bit positions (shared by the RX and TX element layouts).
mod elem {
    /// Remote-transmission-request flag in the first header word.
    pub const RTR: u32 = 1 << 29;
    /// Extended-identifier flag in the first header word.
    pub const XTD: u32 = 1 << 30;
    /// Data-length-code shift in the second header word.
    pub const DLC_SHIFT: u32 = 16;
    pub const DLC_MASK: u32 = 0xF;
}

/// Standard identifiers occupy bits 18..29 of the first header word.
const STD_ID_SHIFT: u32 = 18;
/// Extended identifiers occupy bits 0..29 of the first header word.
const EXT_ID_MASK: u32 = 0x1FFF_FFFF;

/// CAN transfer/bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The controller entered the bus-off state.
    BusOff,
    /// Receive FIFO 0 overflowed (a frame was lost).
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

/// Compute `NBTP.NBRP` for a target nominal bit rate.
///
/// The driver uses a fixed 16-quantum bit time (`SYNC 1 + NTSEG1 12 + NTSEG2
/// 3`), so `NBRP = f_can / (16 * bitrate) - 1`, clamped to the 9-bit field.
/// Sample point sits at 13/16 ≈ 81%.
const fn nbrp(clock_hz: u32, bitrate: u32) -> u16 {
    assert!(bitrate > 0);
    let div = clock_hz / (bitrate * 16);
    if div < 1 {
        0
    } else {
        let d = div - 1;
        if d > 511 { 511 } else { d as u16 }
    }
}

/// Per-instance interrupt state shared between a [`Can`] and its IRQ handler.
///
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

    /// Handle the FDCAN instance's interrupt.
    ///
    /// Masks every enabled interrupt source that is asserted, then wakes the
    /// transfer waiting on it, which re-arms the source it still needs and
    /// clears the underlying flag once it has drained the frame. Bus-off is not
    /// owned by either direction, so it wakes both.
    fn on_interrupt(&self, r: FDCAN) {
        let ir = r.IR().read();
        let ie = r.IE().read();
        let (mut tx, mut rx) = (false, false);
        r.IE().modify(|w| {
            if ie.TCE() && ir.TC() {
                w.set_TCE(false);
                tx = true;
            }
            if ie.RF0NE() && ir.RF0N() {
                w.set_RF0NE(false);
                rx = true;
            }
            if ie.BOE() && ir.BO() {
                w.set_BOE(false);
                tx = true;
                rx = true;
            }
        });
        if tx {
            self.tx_waker.wake();
        }
        if rx {
            self.rx_waker.wake();
        }
    }
}

/// FDCAN configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Target nominal bit rate in bits per second.
    pub bitrate: u32,
    /// Enable internal loopback (bus-monitoring + loopback) for self-test.
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

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> FDCAN;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An FDCAN instance the [`Can`] driver can drive.
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
            fn regs() -> crate::pac::peripherals::fdcan::FDCAN {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::fdcan::FDCAN::from_ptr($base as *mut ()) }
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

/// Handle the interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// Transmit half of an FDCAN instance, produced by [`Can::split`].
///
/// Drives the dedicated transmit buffer and parks on the instance's transmit
/// waker, so it can run concurrently with a [`CanRx`] for the same instance.
pub struct CanTx {
    regs: FDCAN,
    state: &'static State,
    msg_ram: *mut u8,
    ram_offset: usize,
}

/// Receive half of an FDCAN instance, produced by [`Can::split`].
///
/// Drives receive FIFO 0 and parks on the instance's receive waker.
pub struct CanRx {
    regs: FDCAN,
    state: &'static State,
    msg_ram: *mut u8,
    ram_offset: usize,
}

/// Classic CAN controller over one FDCAN instance.
pub struct Can {
    tx: CanTx,
    rx: CanRx,
}

impl Can {
    /// Initialize an FDCAN instance for classic CAN.
    ///
    /// # Safety
    ///
    /// `msg_ram_base` must be the CPU address of the shared CAN message RAM,
    /// and `[msg_ram_offset, msg_ram_offset + `[`REGION_BYTES`]`)` must be a
    /// region within it reserved for this instance and no other. The register
    /// block and interrupt state come from the consumed singleton; the platform
    /// must leave the instance clocked and pinned out.
    ///
    /// `clock_hz` is the CAN kernel clock. For async use, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub unsafe fn new<T: Instance>(
        peri: Peri<'static, T>,
        msg_ram_base: *mut (),
        msg_ram_offset: usize,
        clock_hz: u32,
        config: Config,
    ) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();
        let this = Self {
            tx: CanTx {
                regs: r,
                state: T::state(),
                msg_ram: msg_ram_base as *mut u8,
                ram_offset: msg_ram_offset,
            },
            rx: CanRx {
                regs: r,
                state: T::state(),
                msg_ram: msg_ram_base as *mut u8,
                ram_offset: msg_ram_offset,
            },
        };

        // Request initialization and enable configuration changes.
        r.CCCR().modify(|w| w.set_INIT(true));
        while !r.CCCR().read().INIT() {}
        r.CCCR().modify(|w| {
            w.set_CCE(true);
            // Classic CAN only: no FD, no bit-rate switching.
            w.set_FDOE(false);
            w.set_BRSE(false);
            w.set_TEST(config.loopback);
            w.set_MON(config.loopback);
        });
        if config.loopback {
            r.TEST().modify(|w| w.set_LBCK(true));
        }

        // Nominal bit timing: fixed 16-quantum bit, ~81% sample point.
        r.NBTP().write(|w| {
            w.set_NBRP(nbrp(clock_hz, config.bitrate));
            w.set_NTSEG1(11);
            w.set_NTSEG2(2);
            w.set_NSJW(2);
        });

        // Accept every identifier into FIFO 0; no hardware filter elements.
        r.SIDFC().modify(|w| w.set_LSS(0));
        r.XIDFC().modify(|w| w.set_LSE(0));
        r.GFC().write(|w| {
            w.set_ANFS(0);
            w.set_ANFE(0);
            w.set_RRFS(false);
            w.set_RRFE(false);
        });

        // 8-byte payload elements for FIFO 0 and the transmit buffer.
        r.RXESC().modify(|w| w.set_F0DS(DATA_SIZE_8B));
        r.TXESC().modify(|w| w.set_TBDS(DATA_SIZE_8B));
        r.RXF0C().write(|w| {
            w.set_F0SA(((msg_ram_offset + RX_FIFO0_LOCAL) >> 2) as u16);
            w.set_F0S(RX_FIFO0_ELEMS as u8);
            w.set_F0WM(0);
        });
        r.TXBC().write(|w| {
            w.set_TBSA(((msg_ram_offset + TX_BUF_LOCAL) >> 2) as u16);
            w.set_NDTB(1);
            w.set_TFQS(0);
        });

        // Route all interrupts to line 0 (`ILS` reset default) and enable it.
        r.ILE().write(|w| w.set_EINT0(true));

        // Leave initialization; the core starts synchronizing to the bus.
        r.CCCR().modify(|w| {
            w.set_CCE(false);
            w.set_INIT(false);
        });
        while r.CCCR().read().INIT() {}

        this
    }

    /// Split into independent transmit and receive halves.
    ///
    /// The halves touch disjoint registers and disjoint message-RAM regions —
    /// the transmit buffer versus receive FIFO 0 — and park on separate wakers,
    /// so a transport can await a frame and send one at the same time.
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
    /// True while the transmit buffer still has a pending request.
    fn tx_busy(&self) -> bool {
        self.regs.TXBRP().read().0 & 1 != 0
    }

    /// Load `frame` into the transmit buffer and request transmission.
    fn load_tx(&mut self, frame: &Frame) {
        let (xtd, id_word) = encode_id(frame.id);
        let mut t0 = id_word;
        if xtd {
            t0 |= elem::XTD;
        }
        if frame.remote {
            t0 |= elem::RTR;
        }
        let t1 = (u32::from(frame.dlc) & elem::DLC_MASK) << elem::DLC_SHIFT;
        let w0 = u32::from_le_bytes([frame.data[0], frame.data[1], frame.data[2], frame.data[3]]);
        let w1 = u32::from_le_bytes([frame.data[4], frame.data[5], frame.data[6], frame.data[7]]);
        let p = ram_word(self.msg_ram, self.ram_offset, TX_BUF_LOCAL);
        // SAFETY: `p` is header word T0 of this instance's transmit buffer
        // element within the reserved message-RAM region.
        unsafe { p.write_volatile(t0) };
        // SAFETY: header word T1, one word after T0 in the same element.
        unsafe { p.wrapping_add(1).write_volatile(t1) };
        // SAFETY: data word 0, two words after T0 in the same element.
        unsafe { p.wrapping_add(2).write_volatile(w0) };
        // SAFETY: data word 1, three words after T0 in the same element.
        unsafe { p.wrapping_add(3).write_volatile(w1) };
        // Request transmission of buffer 0.
        self.regs.TXBAR().write_value(FDCAN_TXBAR(1));
    }

    /// Transmit `frame`, awaiting completion of the transfer.
    pub async fn transmit(&mut self, frame: &Frame) -> Result<(), Error> {
        check_bus(self.regs)?;
        // Wait for the transmit buffer to be free.
        wait_flag(
            &self.state.tx_waker,
            self.regs,
            |r| r.IE().modify(|w| w.set_TCE(true)),
            || (!self.tx_busy()).then_some(Ok(())),
        )
        .await?;
        self.load_tx(frame);
        // A frame no node acknowledges is retransmitted forever and would hold
        // the buffer against every later `transmit`; withdraw it if this future
        // is dropped before it completes.
        let guard = TxCancelGuard {
            regs: self.regs,
            armed: true,
        };
        // Await transmission completion.
        wait_flag(
            &self.state.tx_waker,
            self.regs,
            |r| r.IE().modify(|w| w.set_TCE(true)),
            || (!self.tx_busy()).then_some(Ok(())),
        )
        .await?;
        guard.defuse();
        self.regs.IR().write_value(ir_tc());
        Ok(())
    }
}

/// Requests cancellation of transmit buffer 0 when dropped unless
/// [`defuse`](Self::defuse)d. `TXBRP` clears once the controller has withdrawn
/// the frame (or finished sending it), which is what the next `transmit` waits
/// on.
struct TxCancelGuard {
    regs: FDCAN,
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
            self.regs.TXBCR().write_value(FDCAN_TXBCR(1));
        }
    }
}

impl CanRx {
    /// True while receive FIFO 0 holds at least one frame.
    fn rx_pending(&self) -> bool {
        self.regs.RXF0S().read().F0FL() > 0
    }

    /// Read the oldest frame from receive FIFO 0 and acknowledge it.
    fn drain_rx(&mut self) -> Result<Frame, Error> {
        let status = self.regs.RXF0S().read();
        let overrun = status.RF0L();
        let gi = status.F0GI() as usize;
        let p = ram_word(
            self.msg_ram,
            self.ram_offset,
            RX_FIFO0_LOCAL + gi * ELEM_BYTES,
        );
        // SAFETY: `p` is header word R0 of FIFO 0 element `gi` within the
        // reserved message-RAM region (`gi < F0S`).
        let r0 = unsafe { p.read_volatile() };
        // SAFETY: header word R1, one word after R0 in the same element.
        let r1 = unsafe { p.wrapping_add(1).read_volatile() };
        // SAFETY: data word 0, two words after R0 in the same element.
        let w0 = unsafe { p.wrapping_add(2).read_volatile() };
        // SAFETY: data word 1, three words after R0 in the same element.
        let w1 = unsafe { p.wrapping_add(3).read_volatile() };
        // Acknowledge the element so the FIFO advances.
        self.regs.RXF0A().write_value(FDCAN_RXF0A(gi as u32 & 0x3F));

        if overrun {
            return Err(Error::Overrun);
        }
        let extended = r0 & elem::XTD != 0;
        let remote = r0 & elem::RTR != 0;
        let dlc = ((r1 >> elem::DLC_SHIFT) & elem::DLC_MASK).min(8) as u8;
        let mut data = [0u8; 8];
        data[..4].copy_from_slice(&w0.to_le_bytes());
        data[4..].copy_from_slice(&w1.to_le_bytes());
        Ok(Frame {
            id: decode_id(r0, extended),
            remote,
            dlc,
            data,
        })
    }

    /// Await and return the next received frame.
    pub async fn receive(&mut self) -> Result<Frame, Error> {
        wait_flag(
            &self.state.rx_waker,
            self.regs,
            |r| r.IE().modify(|w| w.set_RF0NE(true)),
            || self.rx_pending().then_some(Ok(())),
        )
        .await?;
        let frame = self.drain_rx();
        self.regs.IR().write_value(ir_rf0n());
        frame
    }
}

/// Pointer to a 32-bit word at `byte_off` inside an instance's message-RAM
/// region.
///
/// `wrapping_add` keeps this safe; the resulting pointer is only dereferenced
/// through the volatile accessors in the halves, which carry their own
/// reasoning.
fn ram_word(msg_ram: *mut u8, ram_offset: usize, byte_off: usize) -> *mut u32 {
    msg_ram.wrapping_add(ram_offset + byte_off) as *mut u32
}

/// Surface a bus-off condition as an error.
fn check_bus(regs: FDCAN) -> Result<(), Error> {
    if regs.PSR().read().BO() {
        Err(Error::BusOff)
    } else {
        Ok(())
    }
}

/// Park on `waker` until `ready` reports a result, arming the interrupt source
/// `arm` selects each time the future is polled to `Pending`.
///
/// `ready` is re-checked after registering the waker: the interrupt can fire in
/// the window between the first check and the registration, and a level that is
/// already asserted would otherwise never wake anyone.
async fn wait_flag(
    waker: &AtomicWaker,
    regs: FDCAN,
    arm: fn(&FDCAN),
    ready: impl Fn() -> Option<Result<(), Error>>,
) -> Result<(), Error> {
    poll_fn(|cx| {
        if let Some(res) = ready() {
            return Poll::Ready(res);
        }
        waker.register(cx.waker());
        critical_section::with(|_| arm(&regs));
        if let Some(res) = ready() {
            return Poll::Ready(res);
        }
        Poll::Pending
    })
    .await
}

// SAFETY: the message-RAM pointer names a fixed MMIO-like region exclusively
// owned by this instance; `CanTx` carries no thread-local state, so it is safe
// to move across execution contexts like the other single-owner peripheral
// drivers.
unsafe impl Send for CanTx {}

// SAFETY: as `CanTx`, for the disjoint receive-FIFO region.
unsafe impl Send for CanRx {}

// SAFETY: both halves are `Send` for the reasons above, and `Can` is only their
// container.
unsafe impl Send for Can {}

/// `IR` value that clears only the transmission-complete flag (write-1-to-clear).
fn ir_tc() -> FDCAN_IR {
    let mut v = FDCAN_IR(0);
    v.set_TC(true);
    v
}

/// `IR` value that clears only the FIFO-0-new-message flag (write-1-to-clear).
fn ir_rf0n() -> FDCAN_IR {
    let mut v = FDCAN_IR(0);
    v.set_RF0N(true);
    v
}

/// Encode an identifier into `(is_extended, first_header_word)`.
fn encode_id(id: Id) -> (bool, u32) {
    match id {
        Id::Standard(s) => (false, u32::from(s.as_raw()) << STD_ID_SHIFT),
        Id::Extended(e) => (true, e.as_raw() & EXT_ID_MASK),
    }
}

/// Decode an identifier from an element's first header word.
fn decode_id(r0: u32, extended: bool) -> Id {
    if extended {
        embedded_can::ExtendedId::new(r0 & EXT_ID_MASK)
            .map(Id::Extended)
            .unwrap_or(Id::Standard(StandardId::ZERO))
    } else {
        StandardId::new(((r0 >> STD_ID_SHIFT) & 0x7FF) as u16)
            .map(Id::Standard)
            .unwrap_or(Id::Standard(StandardId::ZERO))
    }
}

impl embedded_can::nb::Can for Can {
    type Frame = Frame;
    type Error = Error;

    fn transmit(&mut self, frame: &Frame) -> nb::Result<Option<Frame>, Error> {
        check_bus(self.tx.regs)?;
        if self.tx.tx_busy() {
            return Err(nb::Error::WouldBlock);
        }
        self.tx.load_tx(frame);
        Ok(None)
    }

    fn receive(&mut self) -> nb::Result<Frame, Error> {
        if !self.rx.rx_pending() {
            return Err(nb::Error::WouldBlock);
        }
        let frame = self.rx.drain_rx().map_err(nb::Error::Other)?;
        self.rx.regs.IR().write_value(ir_rf0n());
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_id, encode_id, nbrp};
    use embedded_can::{ExtendedId, Id, StandardId};

    #[test]
    fn nbrp_matches_bit_rate() {
        // 80 MHz / (16 * 500 kbit) = 10 => NBRP 9.
        assert_eq!(nbrp(80_000_000, 500_000), 9);
    }

    #[test]
    fn standard_id_round_trips() {
        let id = Id::Standard(StandardId::new(0x2AB).unwrap());
        let (ext, word) = encode_id(id);
        assert!(!ext);
        assert_eq!(decode_id(word, ext), id);
    }

    #[test]
    fn extended_id_round_trips() {
        let id = Id::Extended(ExtendedId::new(0x0C0F_FEE1).unwrap());
        let (ext, word) = encode_id(id);
        assert!(ext);
        assert_eq!(decode_id(word, ext), id);
    }
}

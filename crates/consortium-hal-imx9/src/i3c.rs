//! I3C controller driver for the NXP I3C block used by i.MX 9.
//!
//! The controller engine follows Embassy MCXA's NXP I3C implementation. It
//! supports controller-mode I3C SDR and legacy I2C private transfers, repeated
//! starts, broadcast CCCs, and dynamic address assignment. Blocking calls need
//! no interrupt. Async calls require the application to route the instance IRQ
//! to `State::on_interrupt`.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::i3c::{I3C, I3C_MWDATAB, I3C_MWDATABE};

use crate::pac;

const REQUEST_START: u8 = 1;
const REQUEST_STOP: u8 = 2;
const REQUEST_DAA: u8 = 4;
const TYPE_I3C: u8 = 0;
const TYPE_I2C: u8 = 1;
const MASTER_ON: u8 = 1;
const MAX_READ_CHUNK: usize = 256;

/// Protocol used for a private transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BusMode {
    /// I3C single-data-rate transfer to a dynamic address.
    #[default]
    I3cSdr,
    /// Legacy I2C transfer to a static address.
    I2c,
}

impl BusMode {
    const fn type_bits(self) -> u8 {
        match self {
            Self::I3cSdr => TYPE_I3C,
            Self::I2c => TYPE_I2C,
        }
    }
}

/// One operation in a repeated-start transaction.
pub enum Operation<'a> {
    /// Read into the supplied buffer.
    Read(&'a mut [u8]),
    /// Write the supplied bytes.
    Write(&'a [u8]),
}

/// Information returned by dynamic address assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeviceInfo {
    /// 15-bit MIPI manufacturer identifier.
    pub manufacturer_id: u16,
    /// 32-bit part number portion of the provisional ID.
    pub part_number: u32,
    /// Bus Characteristics Register.
    pub bcr: u8,
    /// Device Characteristics Register.
    pub dcr: u8,
    /// Assigned dynamic address.
    pub dynamic_address: u8,
}

/// I3C setup error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    /// A requested frequency is zero or cannot fit the hardware dividers.
    InvalidFrequency,
}

/// I3C transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A target did not acknowledge.
    Nack,
    /// Transmit underrun.
    Underrun,
    /// A write was aborted.
    WriteAbort,
    /// A target terminated a transfer.
    Terminated,
    /// FIFO over-read or overwrite.
    Fifo,
    /// A malformed message or invalid controller request.
    Protocol,
    /// The hardware timeout expired.
    Timeout,
    /// Address or buffer arguments are invalid.
    InvalidArgument,
}

/// Controller clock configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Functional clock feeding the I3C instance.
    pub clock_hz: u32,
    /// I3C push-pull SCL frequency.
    pub push_pull_hz: u32,
    /// I3C open-drain SCL frequency.
    pub open_drain_hz: u32,
    /// Legacy I2C SCL frequency.
    pub i2c_hz: u32,
}

impl Config {
    /// A conservative configuration based on a 24 MHz functional clock.
    pub const fn new(clock_hz: u32) -> Self {
        Self {
            clock_hz,
            push_pull_hz: 4_000_000,
            open_drain_hz: 1_000_000,
            i2c_hz: 400_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Baud {
    pp: u8,
    od: u8,
    i2c: u8,
}

fn baud(config: Config) -> Result<Baud, ConfigError> {
    if config.clock_hz == 0
        || config.push_pull_hz == 0
        || config.open_drain_hz == 0
        || config.i2c_hz == 0
        || config.push_pull_hz > config.clock_hz / 2
    {
        return Err(ConfigError::InvalidFrequency);
    }

    // NXP: push-pull frequency is fclk / (2 * (PPBAUD + 1)).
    let pp_div = config
        .clock_hz
        .div_ceil(config.push_pull_hz.saturating_mul(2));
    if !(1..=16).contains(&pp_div) {
        return Err(ConfigError::InvalidFrequency);
    }
    let pp = pp_div - 1;
    let actual_pp = config.clock_hz / (2 * (pp + 1));
    if config.open_drain_hz > actual_pp {
        return Err(ConfigError::InvalidFrequency);
    }

    // Use high-speed open-drain mode, matching Embassy MCXA: OD frequency is
    // 2 * PP / (ODBAUD + 2).
    let od_div = actual_pp
        .saturating_mul(2)
        .div_ceil(config.open_drain_hz)
        .max(2);
    if od_div > 257 {
        return Err(ConfigError::InvalidFrequency);
    }
    let od = od_div - 2;

    // The I2C divider is a four-bit multiplier on the OD timing. Select the
    // first value that does not exceed the requested rate.
    let od_rate = actual_pp.saturating_mul(2) / od_div;
    if config.i2c_hz > od_rate {
        return Err(ConfigError::InvalidFrequency);
    }
    let i2c_div = od_rate.div_ceil(config.i2c_hz).clamp(1, 16);

    Ok(Baud {
        pp: pp as u8,
        od: od as u8,
        i2c: (i2c_div - 1) as u8,
    })
}

/// Per-instance interrupt state.
pub struct State {
    waker: AtomicWaker,
}

impl State {
    /// Create state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
        }
    }

    /// Mask asserted controller interrupts and wake the pending async transfer.
    fn on_interrupt(&self, r: I3C) {
        let pending = r.MINTMASKED().read().0;
        if pending != 0 {
            r.MINTCLR()
                .write_value(pac::peripherals::i3c::I3C_MINTCLR(pending));
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> I3C;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An I3C instance the [`I3c`] driver can drive.
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
macro_rules! impl_i3c {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::i3c::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::i3c::I3C {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::i3c::I3C::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::i3c::State {
                static STATE: crate::i3c::State = crate::i3c::State::new();
                &STATE
            }
        }

        impl crate::i3c::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_i3c;

/// Handle the interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// Controller over one i.MX9 I3C instance.
pub struct I3c {
    regs: I3C,
    state: &'static State,
}

impl I3c {
    /// Configure an instance from its peripheral singleton.
    ///
    /// The platform must leave the instance clocked and pinned out. For async
    /// transfers, forward its interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, config: Config) -> Result<Self, ConfigError> {
        let b = baud(config)?;
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let regs = T::regs();

        regs.MCONFIG().write(|w| w.set_MSTENA(0));
        regs.MDATACTRL().write(|w| {
            w.set_FLUSHTB(true);
            w.set_FLUSHFB(true);
            w.set_UNLOCK(true);
            w.set_TXTRIG(0);
            w.set_RXTRIG(0);
        });
        regs.MCONFIG().write(|w| {
            w.set_PPBAUD(b.pp);
            w.set_ODBAUD(b.od);
            w.set_I2CBAUD(b.i2c);
            w.set_ODHPP(true);
            // `DISTO = 0` keeps the hardware transfer timeout enabled.
            w.set_DISTO(false);
            w.set_MSTENA(MASTER_ON);
        });
        Ok(Self {
            regs,
            state: T::state(),
        })
    }

    fn clear_status(&self) {
        self.regs
            .MSTATUS()
            .write_value(pac::peripherals::i3c::I3C_MSTATUS(
                (1 << 8) | (1 << 9) | (1 << 10) | (1 << 15),
            ));
        self.regs
            .MERRWARN()
            .write_value(pac::peripherals::i3c::I3C_MERRWARN(u32::MAX));
    }

    fn check_error(&self) -> Result<(), Error> {
        let e = self.regs.MERRWARN().read();
        if e.NACK() {
            return Err(Error::Nack);
        }
        if e.URUN() {
            return Err(Error::Underrun);
        }
        if e.WRABT() {
            return Err(Error::WriteAbort);
        }
        if e.TERM() {
            return Err(Error::Terminated);
        }
        if e.OREAD() || e.OWRITE() {
            return Err(Error::Fifo);
        }
        if e.MSGERR() || e.INVREQ() || e.HPAR() || e.HCRC() {
            return Err(Error::Protocol);
        }
        if e.TIMEOUT() {
            return Err(Error::Timeout);
        }
        Ok(())
    }

    fn start(&self, address: u8, read: bool, len: usize, mode: BusMode) -> Result<(), Error> {
        if address >= 0x80 || len > MAX_READ_CHUNK {
            return Err(Error::InvalidArgument);
        }
        self.regs.MCTRL().write(|w| {
            w.set_REQUEST(REQUEST_START);
            w.set_TYPE(mode.type_bits());
            w.set_DIR(read);
            w.set_ADDR(address);
            // Zero encodes 256 bytes for reads.
            w.set_RDTERM((len & 0xff) as u8);
        });
        self.wait_control()
    }

    fn wait_control(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.MSTATUS().read().MCTRLDONE() {
                self.regs.MSTATUS().write(|w| w.set_MCTRLDONE(true));
                return Ok(());
            }
        }
    }

    fn wait_tx(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.MSTATUS().read().TXNOTFULL() {
                return Ok(());
            }
        }
    }

    fn wait_rx(&self) -> Result<u8, Error> {
        loop {
            self.check_error()?;
            if self.regs.MSTATUS().read().RXPEND() {
                return Ok(self.regs.MRDATAB().read().VALUE());
            }
        }
    }

    fn wait_complete(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.MSTATUS().read().COMPLETE() {
                self.regs.MSTATUS().write(|w| w.set_COMPLETE(true));
                return Ok(());
            }
        }
    }

    fn stop(&self, mode: BusMode) -> Result<(), Error> {
        self.regs
            .MCONFIG()
            .modify(|w| w.set_ODSTOP(mode == BusMode::I2c));
        self.regs.MCTRL().write(|w| {
            w.set_REQUEST(REQUEST_STOP);
            w.set_TYPE(mode.type_bits());
        });
        self.wait_control()
    }

    fn write_part(&self, address: u8, bytes: &[u8], mode: BusMode) -> Result<(), Error> {
        self.start(address, false, 0, mode)?;
        if let Some((last, rest)) = bytes.split_last() {
            for &byte in rest {
                self.wait_tx()?;
                self.regs
                    .MWDATAB()
                    .write_value(I3C_MWDATAB(u32::from(byte)));
            }
            self.wait_tx()?;
            self.regs
                .MWDATABE()
                .write_value(I3C_MWDATABE(u32::from(*last)));
            self.wait_complete()?;
        }
        Ok(())
    }

    fn read_part(&self, address: u8, bytes: &mut [u8], mode: BusMode) -> Result<(), Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidArgument);
        }
        for chunk in bytes.chunks_mut(MAX_READ_CHUNK) {
            self.start(address, true, chunk.len(), mode)?;
            for byte in chunk {
                *byte = self.wait_rx()?;
            }
        }
        Ok(())
    }

    /// Blocking private write followed by STOP.
    pub fn blocking_write(
        &mut self,
        address: u8,
        bytes: &[u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_status();
        self.write_part(address, bytes, mode)?;
        self.stop(mode)
    }

    /// Blocking private read followed by STOP.
    pub fn blocking_read(
        &mut self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_status();
        self.read_part(address, bytes, mode)?;
        self.stop(mode)
    }

    /// Blocking write/read with a repeated START.
    pub fn blocking_write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_status();
        self.write_part(address, write, mode)?;
        self.read_part(address, read, mode)?;
        self.stop(mode)
    }

    /// Execute a repeated-start transaction and emit one final STOP.
    pub fn blocking_transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
        mode: BusMode,
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        self.clear_status();
        for operation in operations {
            match operation {
                Operation::Read(bytes) => self.read_part(address, bytes, mode)?,
                Operation::Write(bytes) => self.write_part(address, bytes, mode)?,
            }
        }
        self.stop(mode)
    }

    /// Send one broadcast CCC with no defining byte.
    pub fn broadcast_ccc(&mut self, command: u8) -> Result<(), Error> {
        self.blocking_write(0x7e, &[command], BusMode::I3cSdr)
    }

    /// Reset target dynamic addresses with the broadcast RSTDAA CCC.
    pub fn reset_daa(&mut self) -> Result<(), Error> {
        self.broadcast_ccc(0x06)
    }

    /// Run ENTDAA and assign consecutive dynamic addresses.
    ///
    /// The slice bounds the number of devices accepted. The returned count can
    /// be smaller when fewer targets participate.
    pub fn daa(&mut self, devices: &mut [DeviceInfo], first_address: u8) -> Result<usize, Error> {
        if first_address == 0
            || first_address >= 0x7e
            || first_address as usize + devices.len() >= 0x7e
        {
            return Err(Error::InvalidArgument);
        }
        self.clear_status();
        self.regs.MCTRL().write(|w| w.set_REQUEST(REQUEST_DAA));

        let mut assigned = 0;
        for device in devices {
            loop {
                self.check_error()?;
                let status = self.regs.MSTATUS().read();
                if status.COMPLETE() {
                    return Ok(assigned);
                }
                // MSTATUS.STATE = 0b101 while the controller is in DAA.
                if status.MCTRLDONE() && status.STATE() == 5 {
                    break;
                }
            }

            let mut payload = [0u8; 8];
            for byte in &mut payload {
                while self.regs.MDATACTRL().read().RXEMPTY() {
                    self.check_error()?;
                }
                *byte = self.regs.MRDATAB().read().VALUE();
            }
            let address = first_address + assigned as u8;
            *device = DeviceInfo {
                manufacturer_id: u16::from_be_bytes([payload[0], payload[1]]) >> 1,
                part_number: u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]),
                bcr: payload[6],
                dcr: payload[7],
                dynamic_address: address,
            };
            self.regs
                .MWDATAB()
                .write_value(I3C_MWDATAB(u32::from(address)));
            self.regs.MCTRL().write(|w| w.set_REQUEST(REQUEST_DAA));
            assigned += 1;
        }
        Ok(assigned)
    }

    async fn wait_async(
        &self,
        mask: u32,
        ready: impl Fn(&Self) -> Option<Result<(), Error>>,
    ) -> Result<(), Error> {
        poll_fn(|cx| {
            if let Some(result) = ready(self) {
                return Poll::Ready(result);
            }
            self.state.waker.register(cx.waker());
            self.regs
                .MINTSET()
                .write_value(pac::peripherals::i3c::I3C_MINTSET(mask));
            if let Some(result) = ready(self) {
                return Poll::Ready(result);
            }
            Poll::Pending
        })
        .await
    }

    async fn start_async(
        &self,
        address: u8,
        read: bool,
        len: usize,
        mode: BusMode,
    ) -> Result<(), Error> {
        if address >= 0x80 || len > MAX_READ_CHUNK {
            return Err(Error::InvalidArgument);
        }
        self.regs.MCTRL().write(|w| {
            w.set_REQUEST(REQUEST_START);
            w.set_TYPE(mode.type_bits());
            w.set_DIR(read);
            w.set_ADDR(address);
            w.set_RDTERM((len & 0xff) as u8);
        });
        self.wait_async((1 << 9) | (1 << 15), |s| match s.check_error() {
            Err(e) => Some(Err(e)),
            Ok(()) if s.regs.MSTATUS().read().MCTRLDONE() => Some(Ok(())),
            Ok(()) => None,
        })
        .await
    }

    async fn write_part_async(
        &self,
        address: u8,
        bytes: &[u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.start_async(address, false, 0, mode).await?;
        if let Some((last, rest)) = bytes.split_last() {
            for &byte in rest {
                self.wait_async((1 << 12) | (1 << 15), |s| match s.check_error() {
                    Err(e) => Some(Err(e)),
                    Ok(()) if s.regs.MSTATUS().read().TXNOTFULL() => Some(Ok(())),
                    Ok(()) => None,
                })
                .await?;
                self.regs
                    .MWDATAB()
                    .write_value(I3C_MWDATAB(u32::from(byte)));
            }
            self.wait_async((1 << 12) | (1 << 15), |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.MSTATUS().read().TXNOTFULL() => Some(Ok(())),
                Ok(()) => None,
            })
            .await?;
            self.regs
                .MWDATABE()
                .write_value(I3C_MWDATABE(u32::from(*last)));
            self.wait_async((1 << 10) | (1 << 15), |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.MSTATUS().read().COMPLETE() => Some(Ok(())),
                Ok(()) => None,
            })
            .await?;
        }
        Ok(())
    }

    async fn read_part_async(
        &self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidArgument);
        }
        for chunk in bytes.chunks_mut(MAX_READ_CHUNK) {
            self.start_async(address, true, chunk.len(), mode).await?;
            for byte in chunk {
                self.wait_async((1 << 11) | (1 << 15), |s| match s.check_error() {
                    Err(e) => Some(Err(e)),
                    Ok(()) if s.regs.MSTATUS().read().RXPEND() => Some(Ok(())),
                    Ok(()) => None,
                })
                .await?;
                *byte = self.regs.MRDATAB().read().VALUE();
            }
        }
        Ok(())
    }

    async fn stop_async(&self, mode: BusMode) -> Result<(), Error> {
        self.regs
            .MCONFIG()
            .modify(|w| w.set_ODSTOP(mode == BusMode::I2c));
        self.regs.MCTRL().write(|w| {
            w.set_REQUEST(REQUEST_STOP);
            w.set_TYPE(mode.type_bits());
        });
        self.wait_async((1 << 9) | (1 << 15), |s| match s.check_error() {
            Err(e) => Some(Err(e)),
            Ok(()) if s.regs.MSTATUS().read().MCTRLDONE() => Some(Ok(())),
            Ok(()) => None,
        })
        .await
    }

    /// Async private write followed by STOP.
    pub async fn write(&mut self, address: u8, bytes: &[u8], mode: BusMode) -> Result<(), Error> {
        self.clear_status();
        self.write_part_async(address, bytes, mode).await?;
        self.stop_async(mode).await
    }

    /// Async private read followed by STOP.
    pub async fn read(
        &mut self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_status();
        self.read_part_async(address, bytes, mode).await?;
        self.stop_async(mode).await
    }

    /// Async write/read with a repeated START.
    pub async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_status();
        self.write_part_async(address, write, mode).await?;
        self.read_part_async(address, read, mode).await?;
        self.stop_async(mode).await
    }

    /// Async repeated-start transaction.
    pub async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
        mode: BusMode,
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        self.clear_status();
        for operation in operations {
            match operation {
                Operation::Read(bytes) => self.read_part_async(address, bytes, mode).await?,
                Operation::Write(bytes) => self.write_part_async(address, bytes, mode).await?,
            }
        }
        self.stop_async(mode).await
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, baud};

    #[test]
    fn common_clock_plan_fits() {
        let b = baud(Config::new(24_000_000)).unwrap();
        assert!(b.pp <= 15);
    }

    #[test]
    fn impossible_push_pull_rate_is_rejected() {
        let mut config = Config::new(1_000_000);
        config.push_pull_hz = 10_000_000;
        assert!(baud(config).is_err());
    }

    #[test]
    fn open_drain_cannot_exceed_push_pull_clock() {
        let mut config = Config::new(24_000_000);
        config.open_drain_hz = 20_000_000;
        assert!(baud(config).is_err());
    }
}

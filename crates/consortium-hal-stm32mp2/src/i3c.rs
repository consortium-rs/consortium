//! I3C controller driver for the STM32MP2 I3C peripheral.
//!
//! The register sequencing follows STM32CubeMP2's LL/HAL implementation while
//! the interrupt contract follows the other Consortium/Embassy-style drivers:
//! blocking transfers poll, and async transfers wake through [`State`].
//! Controller-mode I3C SDR, legacy I2C, broadcast CCC, repeated starts, and
//! dynamic address assignment are supported.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::i3c::{I3C, I3C_CR, I3C_DEVRX, I3C_TDR};

use crate::pac;

const MTYPE_I3C_PRIVATE: u8 = 2;
const MTYPE_I2C_PRIVATE: u8 = 4;
const MTYPE_BROADCAST_CCC: u8 = 6;
const MAX_TRANSFER: usize = u16::MAX as usize;

/// Protocol used for a private transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BusMode {
    /// I3C single-data-rate private transfer.
    #[default]
    I3cSdr,
    /// Legacy I2C private transfer.
    I2c,
}

impl BusMode {
    const fn message_type(self) -> u8 {
        match self {
            Self::I3cSdr => MTYPE_I3C_PRIVATE,
            Self::I2c => MTYPE_I2C_PRIVATE,
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
    /// A requested frequency is zero or outside the eight-bit timing range.
    InvalidFrequency,
}

/// I3C transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Target address was not acknowledged.
    AddressNack,
    /// Data was not acknowledged.
    DataNack,
    /// FIFO overrun or underrun.
    Fifo,
    /// A protocol or parity error occurred.
    Protocol,
    /// A target ended a read before the requested length.
    Terminated,
    /// Address or buffer arguments are invalid.
    InvalidArgument,
}

/// Controller timing configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Kernel clock feeding the I3C instance.
    pub clock_hz: u32,
    /// I3C push-pull SCL frequency.
    pub push_pull_hz: u32,
    /// I3C open-drain SCL frequency.
    pub open_drain_hz: u32,
    /// Legacy I2C SCL frequency.
    pub i2c_hz: u32,
}

impl Config {
    /// Conservative defaults for the supplied kernel clock.
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
struct Timing {
    pp_low: u8,
    i3c_high: u8,
    od_low: u8,
    i2c_high: u8,
}

fn half_period(clock_hz: u32, frequency: u32) -> Result<u8, ConfigError> {
    if clock_hz == 0 || frequency == 0 || frequency > clock_hz / 2 {
        return Err(ConfigError::InvalidFrequency);
    }
    let divisor = frequency
        .checked_mul(2)
        .ok_or(ConfigError::InvalidFrequency)?;
    let clocks = clock_hz.div_ceil(divisor);
    if !(1..=255).contains(&clocks) {
        return Err(ConfigError::InvalidFrequency);
    }
    Ok(clocks as u8)
}

fn timing(config: Config) -> Result<Timing, ConfigError> {
    Ok(Timing {
        pp_low: half_period(config.clock_hz, config.push_pull_hz)?,
        i3c_high: half_period(config.clock_hz, config.push_pull_hz)?,
        od_low: half_period(config.clock_hz, config.open_drain_hz)?,
        i2c_high: half_period(config.clock_hz, config.i2c_hz)?,
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

    /// Mask active transfer interrupts and wake the pending operation.
    fn on_interrupt(&self, r: I3C) {
        let events = r.EVR().read();
        let enables = r.IER().read();
        let fire = (enables.TXFNFIE() && events.TXFNFF())
            || (enables.RXFNEIE() && events.RXFNEF())
            || (enables.FCIE() && events.FCF())
            || (enables.RXTGTENDIE() && events.RXTGTENDF())
            || (enables.ERRIE() && events.ERRF());
        if fire {
            r.IER().modify(|w| {
                w.set_TXFNFIE(false);
                w.set_RXFNEIE(false);
                w.set_FCIE(false);
                w.set_RXTGTENDIE(false);
                w.set_ERRIE(false);
            });
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

/// Controller over one STM32MP2 I3C instance.
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
        let t = timing(config)?;
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let regs = T::regs();
        regs.CFGR().modify(|w| w.set_EN(false));
        regs.TIMINGR0().write(|w| {
            w.set_SCLL_PP(t.pp_low);
            w.set_SCLH_I3C(t.i3c_high);
            w.set_SCLL_OD(t.od_low);
            w.set_SCLH_I2C(t.i2c_high);
        });
        regs.TIMINGR1().write(|w| {
            // Conservative controller bus-free/available timings.
            w.set_AVAL(t.od_low);
            w.set_FREE(t.od_low);
        });
        regs.TIMINGR2().write(|w| {
            w.set_STALLT(true);
            w.set_STALLD(true);
            w.set_STALLC(true);
            w.set_STALLA(true);
            w.set_STALL(0xff);
        });
        regs.CFGR().write(|w| {
            w.set_CRINIT(true);
            w.set_EN(true);
        });
        Ok(Self {
            regs,
            state: T::state(),
        })
    }

    fn clear_events(&self) {
        self.regs.CEVR().write(|w| {
            w.set_CFCF(true);
            w.set_CRXTGTENDF(true);
            w.set_CERRF(true);
        });
        self.regs.CFGR().modify(|w| {
            w.set_RXFLUSH(true);
            w.set_TXFLUSH(true);
            w.set_CFLUSH(true);
            w.set_SFLUSH(true);
        });
    }

    fn check_error(&self) -> Result<(), Error> {
        if !self.regs.EVR().read().ERRF() {
            return Ok(());
        }
        let e = self.regs.SER().read();
        self.regs.CEVR().write(|w| w.set_CERRF(true));
        if e.ANACK() {
            Err(Error::AddressNack)
        } else if e.DNACK() {
            Err(Error::DataNack)
        } else if e.DOVR() || e.COVR() {
            Err(Error::Fifo)
        } else {
            Err(Error::Protocol)
        }
    }

    fn descriptor(
        address: u8,
        read: bool,
        len: usize,
        mode: BusMode,
        stop: bool,
    ) -> Result<I3C_CR, Error> {
        if address >= 0x80 || len > MAX_TRANSFER {
            return Err(Error::InvalidArgument);
        }
        let mut cr = I3C_CR(0);
        cr.set_DCNT(len as u16);
        cr.set_RNW(read);
        cr.set_ADD(address);
        cr.set_MTYPE(mode.message_type());
        cr.set_MEND(stop);
        Ok(cr)
    }

    fn wait_tx(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.EVR().read().TXFNFF() {
                return Ok(());
            }
        }
    }

    fn wait_rx(&self) -> Result<u8, Error> {
        loop {
            self.check_error()?;
            let e = self.regs.EVR().read();
            if e.RXFNEF() {
                return Ok(self.regs.RDR().read().RDB0());
            }
            if e.RXTGTENDF() {
                return Err(Error::Terminated);
            }
        }
    }

    fn wait_complete(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.EVR().read().FCF() {
                self.regs.CEVR().write(|w| w.set_CFCF(true));
                return Ok(());
            }
        }
    }

    fn write_part(
        &self,
        address: u8,
        bytes: &[u8],
        mode: BusMode,
        stop: bool,
    ) -> Result<(), Error> {
        self.regs
            .CR()
            .write_value(Self::descriptor(address, false, bytes.len(), mode, stop)?);
        for &byte in bytes {
            self.wait_tx()?;
            self.regs.TDR().write_value(I3C_TDR(u32::from(byte)));
        }
        self.wait_complete()
    }

    fn read_part(
        &self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
        stop: bool,
    ) -> Result<(), Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidArgument);
        }
        self.regs
            .CR()
            .write_value(Self::descriptor(address, true, bytes.len(), mode, stop)?);
        for byte in bytes {
            *byte = self.wait_rx()?;
        }
        self.wait_complete()
    }

    /// Blocking private write followed by STOP.
    pub fn blocking_write(
        &mut self,
        address: u8,
        bytes: &[u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        self.write_part(address, bytes, mode, true)
    }

    /// Blocking private read followed by STOP.
    pub fn blocking_read(
        &mut self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        self.read_part(address, bytes, mode, true)
    }

    /// Blocking write/read with a repeated START.
    pub fn blocking_write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        self.write_part(address, write, mode, false)?;
        self.read_part(address, read, mode, true)
    }

    /// Execute a repeated-start transaction.
    pub fn blocking_transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        let count = operations.len();
        for (index, operation) in operations.iter_mut().enumerate() {
            let stop = index + 1 == count;
            match operation {
                Operation::Read(bytes) => self.read_part(address, bytes, mode, stop)?,
                Operation::Write(bytes) => self.write_part(address, bytes, mode, stop)?,
            }
        }
        Ok(())
    }

    fn ccc_descriptor(command: u8, stop: bool) -> I3C_CR {
        I3C_CR(
            (u32::from(command) << 16)
                | (u32::from(MTYPE_BROADCAST_CCC) << 27)
                | (u32::from(stop) << 31),
        )
    }

    /// Send a broadcast CCC without a defining byte.
    pub fn broadcast_ccc(&mut self, command: u8) -> Result<(), Error> {
        self.clear_events();
        self.regs
            .CR()
            .write_value(Self::ccc_descriptor(command, true));
        self.wait_complete()
    }

    /// Reset all target dynamic addresses.
    pub fn reset_daa(&mut self) -> Result<(), Error> {
        self.broadcast_ccc(0x06)
    }

    /// Run ENTDAA and assign consecutive dynamic addresses.
    ///
    /// STM32MP2 exposes four controller device-table slots, so at most four
    /// devices can be retained by this driver.
    pub fn daa(&mut self, devices: &mut [DeviceInfo], first_address: u8) -> Result<usize, Error> {
        if devices.len() > 4
            || first_address == 0
            || first_address >= 0x7e
            || first_address as usize + devices.len() >= 0x7e
        {
            return Err(Error::InvalidArgument);
        }
        self.clear_events();
        self.regs.CFGR().modify(|w| w.set_NOARBH(false));
        self.regs.CR().write_value(Self::ccc_descriptor(0x07, true));

        let mut assigned = 0;
        for (slot, device) in devices.iter_mut().enumerate() {
            loop {
                self.check_error()?;
                let events = self.regs.EVR().read();
                if events.FCF() {
                    self.regs.CEVR().write(|w| w.set_CFCF(true));
                    return Ok(assigned);
                }
                if events.TXFNFF() {
                    break;
                }
            }

            let mut payload = [0u8; 8];
            for byte in &mut payload {
                *byte = self.wait_rx()?;
            }
            let address = first_address + assigned as u8;
            *device = DeviceInfo {
                manufacturer_id: u16::from_be_bytes([payload[0], payload[1]]) >> 1,
                part_number: u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]),
                bcr: payload[6],
                dcr: payload[7],
                dynamic_address: address,
            };
            self.regs.TDR().write_value(I3C_TDR(u32::from(address)));
            self.regs.DEVRX(slot).write_value({
                let mut entry = I3C_DEVRX(0);
                entry.set_DA(address);
                entry
            });
            assigned += 1;
        }
        Ok(assigned)
    }

    async fn wait_async(
        &self,
        arm: fn(&I3C),
        ready: impl Fn(&Self) -> Option<Result<(), Error>>,
    ) -> Result<(), Error> {
        poll_fn(|cx| {
            if let Some(result) = ready(self) {
                return Poll::Ready(result);
            }
            self.state.waker.register(cx.waker());
            critical_section::with(|_| arm(&self.regs));
            if let Some(result) = ready(self) {
                return Poll::Ready(result);
            }
            Poll::Pending
        })
        .await
    }

    async fn wait_tx_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| {
                r.IER().modify(|w| {
                    w.set_TXFNFIE(true);
                    w.set_ERRIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.EVR().read().TXFNFF() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_rx_async(&self) -> Result<u8, Error> {
        self.wait_async(
            |r| {
                r.IER().modify(|w| {
                    w.set_RXFNEIE(true);
                    w.set_RXTGTENDIE(true);
                    w.set_ERRIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.EVR().read().RXFNEF() => Some(Ok(())),
                Ok(()) if s.regs.EVR().read().RXTGTENDF() => Some(Err(Error::Terminated)),
                Ok(()) => None,
            },
        )
        .await?;
        Ok(self.regs.RDR().read().RDB0())
    }

    async fn wait_complete_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| {
                r.IER().modify(|w| {
                    w.set_FCIE(true);
                    w.set_ERRIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.EVR().read().FCF() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await?;
        self.regs.CEVR().write(|w| w.set_CFCF(true));
        Ok(())
    }

    async fn write_part_async(
        &self,
        address: u8,
        bytes: &[u8],
        mode: BusMode,
        stop: bool,
    ) -> Result<(), Error> {
        self.regs
            .CR()
            .write_value(Self::descriptor(address, false, bytes.len(), mode, stop)?);
        for &byte in bytes {
            self.wait_tx_async().await?;
            self.regs.TDR().write_value(I3C_TDR(u32::from(byte)));
        }
        self.wait_complete_async().await
    }

    async fn read_part_async(
        &self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
        stop: bool,
    ) -> Result<(), Error> {
        if bytes.is_empty() {
            return Err(Error::InvalidArgument);
        }
        self.regs
            .CR()
            .write_value(Self::descriptor(address, true, bytes.len(), mode, stop)?);
        for byte in bytes {
            *byte = self.wait_rx_async().await?;
        }
        self.wait_complete_async().await
    }

    /// Async private write followed by STOP.
    pub async fn write(&mut self, address: u8, bytes: &[u8], mode: BusMode) -> Result<(), Error> {
        self.clear_events();
        self.write_part_async(address, bytes, mode, true).await
    }

    /// Async private read followed by STOP.
    pub async fn read(
        &mut self,
        address: u8,
        bytes: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        self.read_part_async(address, bytes, mode, true).await
    }

    /// Async write/read with a repeated START.
    pub async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        self.write_part_async(address, write, mode, false).await?;
        self.read_part_async(address, read, mode, true).await
    }

    /// Async repeated-start transaction.
    pub async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
        mode: BusMode,
    ) -> Result<(), Error> {
        self.clear_events();
        let count = operations.len();
        for (index, operation) in operations.iter_mut().enumerate() {
            let stop = index + 1 == count;
            match operation {
                Operation::Read(bytes) => self.read_part_async(address, bytes, mode, stop).await?,
                Operation::Write(bytes) => {
                    self.write_part_async(address, bytes, mode, stop).await?
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BusMode, Config, I3c, timing};

    #[test]
    fn common_timing_fits() {
        let timing = timing(Config::new(64_000_000)).unwrap();
        assert!(timing.pp_low > 0);
        assert!(timing.od_low >= timing.pp_low);
    }

    #[test]
    fn descriptor_matches_st_encoding() {
        let cr = I3c::descriptor(0x30, true, 16, BusMode::I3cSdr, true).unwrap();
        assert_eq!(cr.DCNT(), 16);
        assert_eq!(cr.ADD(), 0x30);
        assert!(cr.RNW());
        assert!(cr.MEND());
    }
}

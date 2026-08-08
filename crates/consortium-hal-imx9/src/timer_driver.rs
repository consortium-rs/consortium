//! Embassy time driver backed by an i.MX9 LPIT instance.
//!
//! Any LPIT instance can back the driver; select it by handing its peripheral
//! singleton to [`init_timer`]. The LPIT has no prescaler, so the tick rate
//! always equals the instance's functional clock: the final binary must select
//! the `embassy-time` `tick-hz-*` feature matching the clock the integrator
//! routes to the LPIT (default 1 MHz), and [`init_timer`] panics on mismatch.

use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicPtr, AtomicU32, Ordering, compiler_fence};

use critical_section::CriticalSection;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use pac::peripherals::lpit::LPIT;

use crate::pac;

const PERIOD_CH: usize = 0;
const ALARM_CH: usize = 1;
const HALF_PERIOD: u64 = 0x8000_0000;
const LPIT_MCR_M_CEN: u32 = 1 << 0;
const LPIT_TCTRL_T_EN: u32 = 1 << 0;
const LPIT_TCTRL_MODE_PERIODIC: u32 = 0 << 2;
const LPIT_TCTRL_TSOI: u32 = 1 << 17;

struct AlarmState {
    timestamp: Cell<u64>,
}

// SAFETY: the `Cell` is only accessed through the driver's
// `Mutex<CriticalSectionRawMutex, _>`, which serializes access across threads
// and interrupts, so moving an `AlarmState` between contexts is sound.
unsafe impl Send for AlarmState {}

impl AlarmState {
    const fn new() -> Self {
        Self {
            timestamp: Cell::new(u64::MAX),
        }
    }
}

pub struct LpitTimeDriver {
    base: AtomicPtr<()>,
    period: AtomicU32,
    alarm: Mutex<CriticalSectionRawMutex, AlarmState>,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: LpitTimeDriver = LpitTimeDriver {
    base: AtomicPtr::new(core::ptr::null_mut()),
    period: AtomicU32::new(0),
    alarm: Mutex::const_new(CriticalSectionRawMutex::new(), AlarmState::new()),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

fn lpit() -> Option<LPIT> {
    let ptr = DRIVER.base.load(Ordering::Acquire);
    if ptr.is_null() {
        None
    } else {
        // SAFETY: `init_timer` installs the MMIO base for the selected LPIT instance.
        Some(unsafe { LPIT::from_ptr(ptr) })
    }
}

// LPIT timeout is `TVAL + 1` clock cycles, so a HALF_PERIOD-tick period needs
// TVAL = HALF_PERIOD - 1 and CVAL counts down from TVAL to 0.
const PERIOD_TVAL: u32 = (HALF_PERIOD - 1) as u32;

const fn calc_now(period: u32, elapsed: u32) -> u64 {
    ((period as u64) * HALF_PERIOD) + elapsed as u64
}

fn elapsed_in_period(r: LPIT) -> u32 {
    let remaining = r.CHANNEL_CVAL(PERIOD_CH).read().VAL();
    PERIOD_TVAL.saturating_sub(remaining)
}

const fn lpit_alarm_ticks(now: u64, timestamp: u64) -> Option<u32> {
    if timestamp <= now {
        None
    } else {
        let diff = timestamp - now;
        if diff > u32::MAX as u64 {
            Some(u32::MAX)
        } else {
            Some(diff as u32)
        }
    }
}

impl LpitTimeDriver {
    unsafe fn init(&'static self, base: *mut (), clock_hz: u32) {
        // The LPIT has no prescaler; the embassy tick rate must equal the
        // functional clock or all timekeeping would be silently scaled.
        assert!(clock_hz as u64 == TICK_HZ);

        self.base.store(base, Ordering::Release);
        self.period.store(0, Ordering::Release);

        // SAFETY: Caller guarantees `base` is a valid LPIT register block for exclusive
        // timer-driver use.
        let r = unsafe { LPIT::from_ptr(base) };

        r.MCR().write_value(LPIT_MCR_M_CEN);
        r.CLRTEN().write_value((1 << PERIOD_CH) | (1 << ALARM_CH));
        r.MIER().write(|w| {
            w.set_TIE0(true);
            w.set_TIE1(false);
        });
        r.MSR().write(|w| {
            w.set_TIF0(true);
            w.set_TIF1(true);
        });
        r.CHANNEL_TVAL(PERIOD_CH).write_value(PERIOD_TVAL);
        r.CHANNEL_TCTRL(PERIOD_CH)
            .write_value(LPIT_TCTRL_T_EN | LPIT_TCTRL_MODE_PERIODIC);
        r.SETTEN().write_value(1 << PERIOD_CH);
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let Some(r) = lpit() else {
            return false;
        };

        self.alarm.borrow(cs).timestamp.set(timestamp);

        let now = self.now();
        if timestamp <= now {
            r.MIER().modify(|w| w.set_TIE1(false));
            r.CLRTEN().write_value(1 << ALARM_CH);
            self.alarm.borrow(cs).timestamp.set(u64::MAX);
            return false;
        }

        let ticks = lpit_alarm_ticks(now, timestamp).unwrap_or(0);
        if ticks == u32::MAX {
            r.MIER().modify(|w| w.set_TIE1(false));
            r.CLRTEN().write_value(1 << ALARM_CH);
            return true;
        }

        r.CLRTEN().write_value(1 << ALARM_CH);
        r.MSR().write(|w| w.set_TIF1(true));
        // Timeout is TVAL + 1 cycles; `ticks >= 1`, so this fires at most one
        // tick late, never early. TSOI makes the channel one-shot; TSOT must
        // stay clear so the channel starts on SETTEN rather than waiting for
        // a hardware trigger that is never routed.
        r.CHANNEL_TVAL(ALARM_CH).write_value(ticks);
        r.CHANNEL_TCTRL(ALARM_CH)
            .write_value(LPIT_TCTRL_T_EN | LPIT_TCTRL_MODE_PERIODIC | LPIT_TCTRL_TSOI);
        r.MIER().modify(|w| w.set_TIE1(true));
        r.SETTEN().write_value(1 << ALARM_CH);

        true
    }

    fn trigger_alarm(&self, cs: CriticalSection) {
        let mut next = self
            .queue
            .borrow(cs)
            .borrow_mut()
            .next_expiration(self.now());
        while !self.set_alarm(cs, next) {
            next = self
                .queue
                .borrow(cs)
                .borrow_mut()
                .next_expiration(self.now());
        }
    }

    fn on_interrupt(&self) {
        let Some(r) = lpit() else {
            return;
        };

        critical_section::with(|cs| {
            let sr = r.MSR().read();
            let ier = r.MIER().read();

            if sr.TIF0() {
                self.period.fetch_add(1, Ordering::Relaxed);
                r.MSR().write(|w| w.set_TIF0(true));
                self.trigger_alarm(cs);
            }

            if sr.TIF1() && ier.TIE1() {
                r.MIER().modify(|w| w.set_TIE1(false));
                r.MSR().write(|w| w.set_TIF1(true));
                self.trigger_alarm(cs);
            }
        });
    }
}

impl Driver for LpitTimeDriver {
    fn now(&self) -> u64 {
        // The counter reloads at the end of each period, but `period` is only
        // incremented by the interrupt handler, so a wrap racing these reads
        // (or observed with interrupts masked) must be detected here or time
        // would jump backwards by a whole period.
        loop {
            let period = self.period.load(Ordering::Relaxed);
            compiler_fence(Ordering::Acquire);
            let Some(r) = lpit() else {
                return calc_now(period, 0);
            };
            let elapsed = elapsed_in_period(r);
            compiler_fence(Ordering::Acquire);
            let wrapped = r.MSR().read().TIF0();
            compiler_fence(Ordering::Acquire);
            if self.period.load(Ordering::Relaxed) != period {
                // The period interrupt ran mid-sequence; retry.
                continue;
            }
            return if wrapped {
                // Wrap not yet serviced: the counter has reloaded, so re-read
                // the post-wrap elapsed value and account the period here.
                calc_now(period + 1, elapsed_in_period(r))
            } else {
                calc_now(period, elapsed)
            };
        }
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> LPIT;
}

/// An LPIT instance that can back the Embassy time driver.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address stays single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_lpit {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::timer_driver::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::lpit::LPIT {
                // SAFETY: `$base` is this instance's LPIT base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::lpit::LPIT::from_ptr($base as *mut ()) }
            }
        }

        impl crate::timer_driver::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_lpit;

/// Initialize the global Embassy time driver from an LPIT peripheral singleton.
///
/// `clock_hz` is the LPIT functional clock and must equal
/// `embassy_time_driver::TICK_HZ`, or initialization panics. The caller must
/// also route and enable the instance's interrupt to [`on_timer_interrupt`].
pub fn init_timer<T: Instance>(peri: Peri<'static, T>, clock_hz: u32) {
    // Owning the singleton for the program's lifetime is what makes the
    // driver's exclusive register access safe; it is consumed here.
    let _ = peri;
    // SAFETY: `T::regs()` is the instance's database-resolved LPIT block, and
    // the consumed singleton guarantees nothing else drives it.
    unsafe { DRIVER.init(T::regs().as_ptr(), clock_hz) }
}

/// Handle the selected LPIT interrupt.
pub fn on_timer_interrupt() {
    DRIVER.on_interrupt();
}

/// Bring up the conventional i.MX9 Embassy time driver (LPIT1) and route its
/// interrupt line.
///
/// This encodes the reference-board wiring for the CM7: LPIT1 clocked from the
/// 24 MHz AON/root timer domain, at NVIC priority `0x80`. The LPIT has no
/// prescaler, so the firmware must select the `embassy-time`
/// `tick-hz-24_000_000` feature and the platform must leave LPIT1 clocked and
/// accessible to the CM7. The caller (the generated `init`) still owns the
/// `#[interrupt] fn LPIT1()` vector that forwards to [`on_timer_interrupt`]. Firmware
/// that manages its own time base should disable the generated time driver
/// (`time_driver = false`) and call [`init_timer`] directly instead.
///
/// Takes the LPIT1 singleton so the reservation is visible in the type system:
/// nothing else can build a driver over it afterwards.
///
/// # Panics
///
/// Panics if the `embassy-time` tick rate does not match the LPIT clock, or if
/// the instance has no NVIC line on this core.
pub fn bringup(lpit1: Peri<'static, crate::peripherals::LPIT1>) {
    const LPIT1_CLOCK_HZ: u32 = 24_000_000;
    let irq =
        <crate::peripherals::LPIT1 as Instance>::IRQ.expect("LPIT1 has no NVIC line on this core");

    init_timer(lpit1, LPIT1_CLOCK_HZ);

    // SAFETY: `irq` is LPIT1's vector from the chip database; the handler
    // forwarding to the time driver is installed by the generated code.
    unsafe { crate::hal::nvic::set_priority(crate::hal::nvic::Irq(irq), 0x80) };
    // SAFETY: see above; enabling the same reserved LPIT1 vector.
    unsafe { crate::hal::nvic::enable(crate::hal::nvic::Irq(irq)) };
}

#[cfg(test)]
mod tests {
    use super::{HALF_PERIOD, calc_now, lpit_alarm_ticks};

    #[test]
    fn calc_now_extends_half_periods() {
        assert_eq!(calc_now(0, 7), 7);
        assert_eq!(calc_now(1, 0), HALF_PERIOD);
        assert_eq!(calc_now(2, 3), HALF_PERIOD * 2 + 3);
    }

    #[test]
    fn alarm_ticks_reject_past_deadlines() {
        assert_eq!(lpit_alarm_ticks(10, 10), None);
        assert_eq!(lpit_alarm_ticks(10, 9), None);
    }

    #[test]
    fn alarm_ticks_program_near_deadlines() {
        assert_eq!(lpit_alarm_ticks(10, 11), Some(1));
        assert_eq!(lpit_alarm_ticks(10, 42), Some(32));
    }

    #[test]
    fn alarm_ticks_clamp_far_deadlines() {
        assert_eq!(
            lpit_alarm_ticks(10, 10 + u32::MAX as u64 + 1),
            Some(u32::MAX)
        );
    }
}

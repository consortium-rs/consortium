//! Embassy time driver backed by an STM32MP2 general-purpose timer.
//!
//! Any general-purpose timer with at least two capture/compare channels on a
//! single interrupt vector can back the driver (TIM2/3/4/5/12/15); select it by
//! passing its singleton to [`init_timer`]. Only those instances implement
//! [`Instance`], so the basic, single-channel, and advanced timers are rejected
//! at compile time.
//!
//! Following `embassy-stm32`'s `time_driver/tim.rs`, the driver works with
//! 16- and 32-bit counters alike: the counter free-runs over its full width,
//! capture/compare channel 1 fires at the half-way point, and every overflow or
//! half-way event advances a software `period`. The current time is
//! `period · 2^(BITS-1)` plus the counter with its top bit folded by the
//! period's parity, which stays monotonic even when the counter has wrapped but
//! the overflow interrupt has not yet run. The timer prescaler is derived from the instance clock and
//! `embassy_time_driver::TICK_HZ`, so the tick rate is chosen by the final
//! binary through the `embassy-time` `tick-hz-*` features (default 1 MHz).
//! The instance clock must be an integer multiple of the tick rate and the
//! divider must fit the 16-bit PSC register.

use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicPtr, AtomicU8, AtomicU32, Ordering, compiler_fence};

use critical_section::CriticalSection;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use pac::peripherals::timer::{TIM_GP, TIM_SR};

use crate::pac;

const PERIOD_CCR: usize = 0;
const ALARM_CCR: usize = 1;

/// Ticks in half a counter period for a `bits`-wide counter.
const fn half_period(bits: u8) -> u64 {
    1 << (bits - 1)
}

/// Largest value a `bits`-wide counter holds, which is also its `ARR`.
const fn counter_max(bits: u8) -> u32 {
    (((1u64) << bits) - 1) as u32
}

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

pub struct GpTimerTimeDriver {
    base: AtomicPtr<()>,
    /// Counter width in bits (16 or 32) of the installed timer.
    bits: AtomicU8,
    period: AtomicU32,
    alarm: Mutex<CriticalSectionRawMutex, AlarmState>,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: GpTimerTimeDriver = GpTimerTimeDriver {
    base: AtomicPtr::new(core::ptr::null_mut()),
    bits: AtomicU8::new(32),
    period: AtomicU32::new(0),
    alarm: Mutex::const_new(CriticalSectionRawMutex::new(), AlarmState::new()),
    queue: Mutex::new(RefCell::new(Queue::new())),
});

fn tim() -> Option<TIM_GP> {
    let ptr = DRIVER.base.load(Ordering::Acquire);
    if ptr.is_null() {
        None
    } else {
        // SAFETY: `init_timer` installs the MMIO base for the selected TIM instance.
        Some(unsafe { TIM_GP::from_ptr(ptr) })
    }
}

/// Current time from the software `period` and the hardware counter.
///
/// An odd period means the counter has passed its half-way point, so its top
/// bit is already accounted for in `period`; flipping that bit makes the sum
/// continuous (and keeps it monotonic when the counter wraps before the
/// overflow interrupt bumps `period`).
const fn calc_now(bits: u8, period: u32, counter: u32) -> u64 {
    let top = bits as u32 - 1;
    ((period as u64) << top) + ((counter ^ ((period & 1) << top)) as u64)
}

const fn compute_psc(clock_hz: u32, tick_hz: u64) -> u32 {
    assert!(tick_hz > 0 && (clock_hz as u64).is_multiple_of(tick_hz));
    let div = clock_hz as u64 / tick_hz;
    assert!(div > 0);
    // The PSC register is 16 bits wide even on 32-bit counters.
    assert!(div - 1 <= u16::MAX as u64);
    (div - 1) as u32
}

/// Whether the alarm compare may be armed now: only when the deadline falls
/// within the next 1.5 half-periods, so the truncated compare value cannot
/// match an earlier counter lap. `None` for a deadline already passed.
const fn tim_alarm_enabled(bits: u8, now: u64, timestamp: u64) -> Option<bool> {
    let half = half_period(bits);
    if timestamp <= now {
        None
    } else {
        Some(timestamp - now < half + (half / 2))
    }
}

impl GpTimerTimeDriver {
    unsafe fn init(&'static self, base: *mut (), bits: u8, clock_hz: u32) {
        assert!(
            bits == 16 || bits == 32,
            "timer counter must be 16 or 32 bits"
        );
        self.bits.store(bits, Ordering::Release);
        self.base.store(base, Ordering::Release);
        self.period.store(0, Ordering::Release);

        // SAFETY: Caller guarantees `base` is a valid TIM register block for exclusive
        // timer-driver use.
        let r = unsafe { TIM_GP::from_ptr(base) };
        let psc = compute_psc(clock_hz, TICK_HZ);

        r.CR1().modify(|w| w.set_CEN(false));
        r.CNT().write(|w| w.set_CNT(0));
        r.PSC().write(|w| w.set_PSC(psc));
        r.ARR().write(|w| w.set_ARR(counter_max(bits)));
        r.CCR(PERIOD_CCR)
            .write(|w| w.set_CCR(half_period(bits) as u32));
        r.EGR().write(|w| w.set_UG(true));
        r.SR().write_value(Default::default());
        r.DIER().write(|w| {
            w.set_UIE(true);
            w.set_CCIE(PERIOD_CCR, true);
            w.set_CCIE(ALARM_CCR, false);
        });
        r.CR1().modify(|w| w.set_CEN(true));
    }

    fn set_alarm(&self, cs: CriticalSection, timestamp: u64) -> bool {
        let Some(r) = tim() else {
            return false;
        };

        self.alarm.borrow(cs).timestamp.set(timestamp);

        let now = self.now();
        if timestamp <= now {
            r.DIER().modify(|w| w.set_CCIE(ALARM_CCR, false));
            self.alarm.borrow(cs).timestamp.set(u64::MAX);
            return false;
        }

        let bits = self.bits.load(Ordering::Relaxed);
        r.CCR(ALARM_CCR)
            .write(|w| w.set_CCR(timestamp as u32 & counter_max(bits)));
        r.DIER().modify(|w| {
            w.set_CCIE(
                ALARM_CCR,
                tim_alarm_enabled(bits, now, timestamp).unwrap_or(false),
            )
        });

        let now = self.now();
        if timestamp <= now {
            r.DIER().modify(|w| w.set_CCIE(ALARM_CCR, false));
            self.alarm.borrow(cs).timestamp.set(u64::MAX);
            return false;
        }

        true
    }

    fn next_period(&self, cs: CriticalSection) {
        let Some(r) = tim() else {
            return;
        };

        let half = half_period(self.bits.load(Ordering::Relaxed));
        let period = self.period.load(Ordering::Relaxed) + 1;
        self.period.store(period, Ordering::Relaxed);
        let now = (period as u64) * half;

        let alarm = self.alarm.borrow(cs);
        if alarm.timestamp.get() < now + half + (half / 2) {
            r.DIER().modify(|w| w.set_CCIE(ALARM_CCR, true));
        }
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
        consortium_log::trace!("on_timer_interrupt");
        let Some(r) = tim() else {
            return;
        };

        critical_section::with(|cs| {
            consortium_log::trace!("on_timer_interrupt: critical section");
            let sr = r.SR().read();
            let dier = r.DIER().read();

            // SR flags are rc_w0: writing the bitwise NOT clears exactly the
            // observed flags while leaving flags raised after the read intact.
            r.SR().write_value(TIM_SR(!sr.0));

            if sr.UIF() {
                self.next_period(cs);
            }
            if sr.CCIF(PERIOD_CCR) {
                self.next_period(cs);
            }
            if sr.CCIF(ALARM_CCR) && dier.CCIE(ALARM_CCR) {
                self.trigger_alarm(cs);
            }
        });
    }
}

impl Driver for GpTimerTimeDriver {
    fn now(&self) -> u64 {
        let period = self.period.load(Ordering::Relaxed);
        compiler_fence(Ordering::Acquire);
        let counter = tim().map_or(0, |r| r.CNT().read().CNT());
        calc_now(self.bits.load(Ordering::Relaxed), period, counter)
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

/// Initialize the global Embassy time driver from a TIM peripheral singleton.
///
/// `clock_hz` is the timer instance's kernel clock; it must be an integer
/// multiple of `embassy_time_driver::TICK_HZ` with a divider that fits the
/// 16-bit prescaler, or initialization panics. The caller must also route and
/// enable the instance's interrupt to [`on_timer_interrupt`].
pub fn init_timer<T: Instance>(peri: Peri<'static, T>, clock_hz: u32) {
    consortium_log::trace!("init_timer, clock hz={}", clock_hz);
    // Owning the singleton for the program's lifetime is what makes the
    // driver's exclusive register access safe; it is consumed here.
    let _ = peri;
    // SAFETY: `T::regs()` is the instance's database-resolved timer block, and
    // the consumed singleton guarantees nothing else drives it.
    unsafe { DRIVER.init(T::regs().as_ptr(), T::BITS, clock_hz) }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> TIM_GP;
}

/// A timer that can back the Embassy time driver.
///
/// Implemented by the generated peripheral singletons for the timers with two
/// capture/compare channels on one vector; sealed so the base address and
/// counter width stay single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
    /// Counter width in bits: 32 for TIM2/TIM5, 16 otherwise.
    const BITS: u8;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_tim_driver {
    ($instance:ident, $base:expr, Bits16, $irq:expr) => {
        crate::timer_driver::impl_tim_driver!(@impl $instance, $base, 16, $irq);
    };
    ($instance:ident, $base:expr, Bits32, $irq:expr) => {
        crate::timer_driver::impl_tim_driver!(@impl $instance, $base, 32, $irq);
    };
    (@impl $instance:ident, $base:expr, $bits:literal, $irq:expr) => {
        impl crate::timer_driver::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::timer::TIM_GP {
                // SAFETY: `$base` is this instance's timer base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::timer::TIM_GP::from_ptr($base as *mut ()) }
            }
        }

        impl crate::timer_driver::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
            const BITS: u8 = $bits;
        }
    };
}
pub(crate) use impl_tim_driver;

/// Handle the selected TIM interrupt.
pub fn on_timer_interrupt() {
    consortium_log::trace!("on_timer_interrupt triggered");
    DRIVER.on_interrupt();
}

/// Bring up the conventional STM32MP2 non-secure Embassy time driver (TIM2) and
/// route its interrupt line.
///
/// This encodes the reference-board wiring for the CM33 non-secure context:
/// TIM2 clocked from the 64 MHz kernel clock, at NVIC priority `0x80`. The
/// boot/runtime handoff must leave TIM2 clocked at that rate. The caller (the
/// generated `init`) still owns the `#[interrupt] fn TIM2()` vector that
/// forwards to [`on_timer_interrupt`]. Firmware that manages its own time base
/// should disable the generated time driver (`time_driver = false`) and call
/// [`init_timer`] directly instead.
///
/// Takes the TIM2 singleton so the reservation is visible in the type system:
/// nothing else can build a driver over it afterwards.
///
/// # Panics
///
/// Panics if TIM2 has no NVIC line on this core.
pub fn bringup(tim2: Peri<'static, crate::peripherals::TIM2>) {
    const RCC_BASE_NS: *mut () = 0x4420_0000 as *mut ();
    const TIM2_CLOCK_HZ: u32 = 64_000_000;
    let irq =
        <crate::peripherals::TIM2 as Instance>::IRQ.expect("TIM2 has no NVIC line on this core");

    // SAFETY: `RCC_BASE_NS` is the STM32MP257F non-secure RCC register block;
    // enabling TIM2's kernel/low-power clock is the documented boot handoff for
    // this board.
    let rcc = unsafe { crate::pac::peripherals::rcc::RCC::from_ptr(RCC_BASE_NS) };
    rcc.TIM2CFGR().modify(|w| {
        w.set_TIM2RST(false);
        w.set_TIM2EN(true);
        w.set_TIM2LPEN(true);
    });

    init_timer(tim2, TIM2_CLOCK_HZ);

    // SAFETY: `irq` is TIM2's vector from the chip database; the handler
    // forwarding to the time driver is installed by the generated code.
    unsafe { crate::hal::nvic::set_priority(crate::hal::nvic::Irq(irq), 0x80) };
    // SAFETY: see above; enabling the same reserved TIM2 vector.
    unsafe { crate::hal::nvic::enable(crate::hal::nvic::Irq(irq)) };
}

#[cfg(test)]
mod tests {
    use super::{calc_now, compute_psc, counter_max, half_period, tim_alarm_enabled};

    const HALF_PERIOD: u64 = half_period(32);

    #[test]
    fn calc_now_tracks_period_parity() {
        assert_eq!(calc_now(32, 0, 7), 7);
        assert_eq!(calc_now(32, 1, 0x8000_0000), HALF_PERIOD);
        assert_eq!(calc_now(32, 2, 3), HALF_PERIOD * 2 + 3);
        assert_eq!(calc_now(16, 1, 0x8000), 0x8000);
        assert_eq!(calc_now(16, 2, 3), 0x1_0003);
    }

    #[test]
    fn calc_now_remains_monotonic_across_rollover_race() {
        assert!(calc_now(32, 1, 0) > calc_now(32, 1, 0xffff_fffe));
        assert_eq!(calc_now(32, 1, 0), HALF_PERIOD * 2);
        // The 16-bit counter wrapped but the overflow interrupt has not run.
        assert!(calc_now(16, 1, 0) > calc_now(16, 1, 0xfffe));
        assert_eq!(calc_now(16, 1, 0), 0x1_0000);
    }

    #[test]
    fn counter_geometry_matches_width() {
        assert_eq!(counter_max(16), 0xffff);
        assert_eq!(counter_max(32), u32::MAX);
        assert_eq!(half_period(16), 0x8000);
        assert_eq!(half_period(32), 0x8000_0000);
    }

    /// Replay a 16-bit counter through several wraps, advancing `period`
    /// exactly where the hardware raises the half-way compare and update
    /// events, and check `now` equals elapsed ticks at every step. This is the
    /// regression for driving a 16-bit timer with 32-bit constants, which lost
    /// time at every 16-bit wrap.
    #[test]
    fn sixteen_bit_counter_keeps_time_across_wraps() {
        let bits = 16;
        let max = counter_max(bits);
        let mut period = 0u32;
        let mut counter = 0u32;
        for elapsed in 0..(5 * (max as u64 + 1) + 123) {
            assert_eq!(calc_now(bits, period, counter), elapsed);
            counter = if counter == max { 0 } else { counter + 1 };
            // CC1 at the half-way point, UIF on wrap to zero.
            if counter == half_period(bits) as u32 || counter == 0 {
                period += 1;
            }
        }
    }

    /// The same replay with the interrupt that bumps `period` late by a few
    /// ticks: `now` must still never go backwards.
    #[test]
    fn sixteen_bit_counter_is_monotonic_with_late_interrupts() {
        let bits = 16;
        let max = counter_max(bits);
        let mut period = 0u32;
        let mut counter = 0u32;
        let mut pending = None::<u32>;
        let mut last = 0;
        for _ in 0..(3 * (max as u64 + 1)) {
            let now = calc_now(bits, period, counter);
            assert!(now >= last, "time went backwards: {last} -> {now}");
            last = now;
            counter = if counter == max { 0 } else { counter + 1 };
            if counter == half_period(bits) as u32 || counter == 0 {
                pending = Some(7);
            }
            if let Some(delay) = pending.as_mut() {
                if *delay == 0 {
                    period += 1;
                    pending = None;
                } else {
                    *delay -= 1;
                }
            }
        }
    }

    #[test]
    fn alarm_enable_rejects_past_deadlines() {
        assert_eq!(tim_alarm_enabled(32, 10, 10), None);
        assert_eq!(tim_alarm_enabled(32, 10, 9), None);
    }

    #[test]
    fn alarm_enable_handles_near_and_far_deadlines() {
        assert_eq!(tim_alarm_enabled(32, 10, 42), Some(true));
        assert_eq!(tim_alarm_enabled(32, 10, 10 + HALF_PERIOD * 2), Some(false));
        // 1.5 half-periods is 0xC000 ticks on a 16-bit counter.
        assert_eq!(tim_alarm_enabled(16, 0, 0xbfff), Some(true));
        assert_eq!(tim_alarm_enabled(16, 0, 0xc000), Some(false));
    }

    #[test]
    fn psc_divides_clock_to_tick_rate() {
        assert_eq!(compute_psc(1_000_000, 1_000_000), 0);
        assert_eq!(compute_psc(200_000_000, 1_000_000), 199);
        assert_eq!(compute_psc(32_768, 32_768), 0);
    }

    #[test]
    #[should_panic]
    fn psc_rejects_non_divisible_clock() {
        compute_psc(3_000_001, 1_000_000);
    }

    #[test]
    #[should_panic]
    fn psc_rejects_clock_slower_than_tick() {
        compute_psc(500_000, 1_000_000);
    }

    #[test]
    #[should_panic]
    fn psc_rejects_divider_overflow() {
        compute_psc(200_000_000, 1_000);
    }
}

//! Embassy time driver backed by an STM32MP2 general-purpose timer.
//!
//! Any general-purpose timer instance with at least two capture/compare
//! channels can back the driver; select it by passing its MMIO base to
//! [`init_timer`]. The timer prescaler is derived from the instance clock and
//! `embassy_time_driver::TICK_HZ`, so the tick rate is chosen by the final
//! binary through the `embassy-time` `tick-hz-*` features (default 1 MHz).
//! The instance clock must be an integer multiple of the tick rate and the
//! divider must fit the 16-bit PSC register.

use core::cell::{Cell, RefCell};
use core::sync::atomic::{AtomicPtr, AtomicU32, Ordering, compiler_fence};

use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_driver::{Driver, TICK_HZ};
use embassy_time_queue_utils::Queue;
use pac::peripherals::timer::{TIM_GP, TIM_SR};

use crate::pac;

const PERIOD_CCR: usize = 0;
const ALARM_CCR: usize = 1;
const HALF_PERIOD: u64 = 0x8000_0000;

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
    period: AtomicU32,
    alarm: Mutex<CriticalSectionRawMutex, AlarmState>,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

embassy_time_driver::time_driver_impl!(static DRIVER: GpTimerTimeDriver = GpTimerTimeDriver {
    base: AtomicPtr::new(core::ptr::null_mut()),
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

const fn calc_now(period: u32, counter: u32) -> u64 {
    ((period as u64) * HALF_PERIOD) + ((counter ^ ((period & 1) << 31)) as u64)
}

const fn compute_psc(clock_hz: u32, tick_hz: u64) -> u32 {
    assert!(tick_hz > 0 && (clock_hz as u64).is_multiple_of(tick_hz));
    let div = clock_hz as u64 / tick_hz;
    assert!(div > 0);
    // The PSC register is 16 bits wide even on 32-bit counters.
    assert!(div - 1 <= u16::MAX as u64);
    (div - 1) as u32
}

const fn tim_alarm_enabled(now: u64, timestamp: u64) -> Option<bool> {
    if timestamp <= now {
        None
    } else {
        Some(timestamp - now < HALF_PERIOD + (HALF_PERIOD / 2))
    }
}

impl GpTimerTimeDriver {
    unsafe fn init(&'static self, base: *mut (), clock_hz: u32) {
        self.base.store(base, Ordering::Release);
        self.period.store(0, Ordering::Release);

        // SAFETY: Caller guarantees `base` is a valid TIM register block for exclusive
        // timer-driver use.
        let r = unsafe { TIM_GP::from_ptr(base) };
        let psc = compute_psc(clock_hz, TICK_HZ);

        r.CR1().modify(|w| w.set_CEN(false));
        r.CNT().write(|w| w.set_CNT(0));
        r.PSC().write(|w| w.set_PSC(psc));
        r.ARR().write(|w| w.set_ARR(u32::MAX));
        r.CCR(PERIOD_CCR).write(|w| w.set_CCR(HALF_PERIOD as u32));
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

        r.CCR(ALARM_CCR).write(|w| w.set_CCR(timestamp as u32));
        r.DIER().modify(|w| {
            w.set_CCIE(
                ALARM_CCR,
                tim_alarm_enabled(now, timestamp).unwrap_or(false),
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

        let period = self.period.load(Ordering::Relaxed) + 1;
        self.period.store(period, Ordering::Relaxed);
        let now = (period as u64) * HALF_PERIOD;

        let alarm = self.alarm.borrow(cs);
        if alarm.timestamp.get() < now + HALF_PERIOD + (HALF_PERIOD / 2) {
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
        calc_now(period, counter)
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

/// Initialize the global Embassy time driver from a TIM register base.
///
/// # Safety
///
/// `base` must point to a valid STM32 general-purpose timer register block that
/// is exclusively owned by the time driver for the lifetime of the program.
/// `clock_hz` is the timer instance's kernel clock; it must be an integer
/// multiple of `embassy_time_driver::TICK_HZ` with a divider that fits the
/// 16-bit prescaler, or initialization panics. The caller must also route and
/// enable the instance's interrupt to [`on_timer_interrupt`].
pub unsafe fn init_timer(base: *mut (), clock_hz: u32) {
    consortium_log::trace!("init_timer, clock hz={}", clock_hz);
    // SAFETY: Propagates the caller's MMIO validity and exclusivity contract.
    unsafe { DRIVER.init(base, clock_hz) }
}

/// Handle the selected TIM interrupt.
pub fn on_timer_interrupt() {
    consortium_log::trace!("on_timer_interrupt triggered");
    DRIVER.on_interrupt();
}

/// Bring up the conventional STM32MP2 non-secure Embassy time driver (TIM2) and
/// route its interrupt line.
///
/// This encodes the reference-board wiring for the CM33 non-secure context:
/// TIM2 at `0x4000_0000` clocked from the 64 MHz kernel clock, IRQ 105 at NVIC
/// priority `0x80`. The boot/runtime handoff must leave TIM2 clocked at that
/// rate. The caller (the generated `init`) still owns the `#[interrupt] fn
/// TIM2()` vector that forwards to [`on_timer_interrupt`]. Firmware that manages
/// its own time base should disable the generated time driver (`time_driver =
/// false`) and call [`init_timer`] directly instead.
///
/// # Safety
///
/// Must be called once during controller bring-up, in privileged mode, with
/// exclusive access to the RCC, TIM2, and NVIC and before any `embassy-time`
/// use.
pub unsafe fn bringup() {
    const RCC_BASE_NS: *mut () = 0x4420_0000 as *mut ();
    const TIM2_BASE_NS: *mut () = 0x4000_0000 as *mut ();
    const TIM2_IRQ: u16 = 105;
    const TIM2_CLOCK_HZ: u32 = 64_000_000;

    // SAFETY: `RCC_BASE_NS` is the STM32MP257F non-secure RCC register block;
    // enabling TIM2's kernel/low-power clock is the documented boot handoff for
    // this board.
    let rcc = unsafe { crate::pac::peripherals::rcc::RCC::from_ptr(RCC_BASE_NS) };
    rcc.TIM2CFGR().modify(|w| {
        w.set_TIM2RST(false);
        w.set_TIM2EN(true);
        w.set_TIM2LPEN(true);
    });

    // SAFETY: TIM2 is reserved for the time driver and clocked above; its IRQ is
    // routed to `on_timer_interrupt` through the generated vector.
    unsafe { init_timer(TIM2_BASE_NS, TIM2_CLOCK_HZ) };

    // SAFETY: `TIM2_IRQ` is the STM32MP257F non-secure TIM2 vector; the handler
    // forwarding to the time driver is installed by the generated code.
    unsafe {
        crate::hal::nvic::set_priority(TIM2_IRQ, 0x80);
        crate::hal::nvic::enable(TIM2_IRQ);
    }
}

#[cfg(test)]
mod tests {
    use super::{HALF_PERIOD, calc_now, compute_psc, tim_alarm_enabled};

    #[test]
    fn calc_now_tracks_period_parity() {
        assert_eq!(calc_now(0, 7), 7);
        assert_eq!(calc_now(1, 0x8000_0000), HALF_PERIOD);
        assert_eq!(calc_now(2, 3), HALF_PERIOD * 2 + 3);
    }

    #[test]
    fn calc_now_remains_monotonic_across_rollover_race() {
        assert!(calc_now(1, 0) > calc_now(1, 0xffff_fffe));
        assert_eq!(calc_now(1, 0), HALF_PERIOD * 2);
    }

    #[test]
    fn alarm_enable_rejects_past_deadlines() {
        assert_eq!(tim_alarm_enabled(10, 10), None);
        assert_eq!(tim_alarm_enabled(10, 9), None);
    }

    #[test]
    fn alarm_enable_handles_near_and_far_deadlines() {
        assert_eq!(tim_alarm_enabled(10, 42), Some(true));
        assert_eq!(tim_alarm_enabled(10, 10 + HALF_PERIOD * 2), Some(false));
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

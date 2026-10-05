//! Bounded busy-waits for blocking driver paths.
//!
//! A blocking transfer that polls a status flag spins forever if the flag never
//! comes: a target holding SCL low, a peripheral whose kernel clock was gated
//! behind the driver's back, a DMA channel that never reaches idle. `embassy-stm32`
//! bounds those loops with a `Timeout` that reads `embassy-time` when it can and
//! never expires when it cannot. Consortium's drivers also spin in `Drop` guards
//! and before any time driver is running, so this `Timeout` counts polls instead
//! of reading a clock: it works everywhere, and it always expires.
//!
//! The cost is that a poll budget is not a duration. One poll is one status read
//! plus loop overhead, so the wall-clock bound scales inversely with the core
//! clock; [`Timeout::DEFAULT`] is generous enough to cover a clock-stretched I2C
//! byte at 10 kHz on a 1 GHz core and still bounded on a slow one.
//!
//! ```
//! use consortium_hal::timeout::Timeout;
//!
//! let mut ready = 3;
//! let mut wait = Timeout::polls(10).start();
//! let result = loop {
//!     ready -= 1;
//!     if ready == 0 {
//!         break Ok(());
//!     }
//!     if let Err(e) = wait.tick() {
//!         break Err(e);
//!     }
//! };
//! assert_eq!(result, Ok(()));
//! ```

/// The poll budget a blocking wait may spend before giving up.
///
/// Each wait creates a fresh [`Deadline`] with [`start`](Self::start), so the
/// budget bounds one wait step (one byte, one FIFO slot, one idle flag), not
/// the whole transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeout {
    polls: u32,
}

impl Timeout {
    /// The budget drivers use unless configured otherwise: ten million polls.
    pub const DEFAULT: Self = Self::polls(10_000_000);

    /// A budget of `polls` status reads. Zero expires on the first tick.
    #[must_use]
    pub const fn polls(polls: u32) -> Self {
        Self { polls }
    }

    /// The configured poll count.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.polls
    }

    /// Begin one bounded wait.
    #[must_use]
    pub const fn start(self) -> Deadline {
        Deadline {
            remaining: self.polls,
        }
    }

    /// Poll `ready` until it yields a value or the budget runs out.
    ///
    /// `ready` runs at least once, so a condition that already holds succeeds
    /// even with a zero budget.
    pub fn spin<T>(self, mut ready: impl FnMut() -> Option<T>) -> Result<T, TimedOut> {
        let mut deadline = self.start();
        loop {
            if let Some(value) = ready() {
                return Ok(value);
            }
            deadline.tick()?;
        }
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// One wait in progress; see [`Timeout::start`].
#[derive(Debug)]
pub struct Deadline {
    remaining: u32,
}

impl Deadline {
    /// Spend one poll, failing once the budget is exhausted.
    pub fn tick(&mut self) -> Result<(), TimedOut> {
        match self.remaining.checked_sub(1) {
            Some(rest) => {
                self.remaining = rest;
                Ok(())
            }
            None => Err(TimedOut),
        }
    }
}

/// A bounded wait ran out of polls. Drivers map it to their own
/// `Error::Timeout`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimedOut;

#[cfg(test)]
mod tests {
    use super::{TimedOut, Timeout};

    #[test]
    fn zero_budget_expires_on_first_tick() {
        assert_eq!(Timeout::polls(0).start().tick(), Err(TimedOut));
    }

    #[test]
    fn budget_allows_exactly_n_ticks() {
        let mut d = Timeout::polls(3).start();
        assert!(d.tick().is_ok());
        assert!(d.tick().is_ok());
        assert!(d.tick().is_ok());
        assert_eq!(d.tick(), Err(TimedOut));
    }

    #[test]
    fn spin_checks_condition_before_spending() {
        assert_eq!(Timeout::polls(0).spin(|| Some(7)), Ok(7));
    }

    #[test]
    fn spin_gives_up_on_a_stuck_condition() {
        let mut polls = 0u32;
        let r: Result<(), _> = Timeout::polls(100).spin(|| {
            polls += 1;
            None
        });
        assert_eq!(r, Err(TimedOut));
        assert_eq!(polls, 101);
    }
}

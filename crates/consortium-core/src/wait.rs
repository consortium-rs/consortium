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

//! Waiting for a transport to become ready: async, by parking on a doorbell, or
//! blocking, by spinning against a deadline.
//!
//! Both helpers drive the same thing — a `check` closure that inspects shared
//! state once and reports a [`Readiness`] — so a transport writes its readiness
//! test once and gets both forms. The usual `check` is a
//! [`TrySendTransport::try_send`](crate::TrySendTransport::try_send) or
//! [`TryRecvTransport::try_recv`](crate::TryRecvTransport::try_recv) folded through
//! [`Readiness::from_try`].
//!
//! Readiness is always decided from shared state, never from a doorbell edge, so a
//! spurious wake is harmless and a check may be repeated at will.

use core::task::Poll;

use crate::{Chan, Doorbell, TryError};

/// What one readiness check found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readiness<T> {
    /// The condition holds; resolve with this value.
    Ready(T),
    /// Nothing yet; wait for the peer to ring the doorbell line.
    Park,
    /// State is mid-update, or the transport has no completion signal: check again
    /// soon without waiting for a doorbell edge.
    Yield,
}

impl<T> Readiness<T> {
    /// Fold a try-transport result into a readiness step: `Ok` is
    /// [`Ready`](Self::Ready), [`TryError::WouldBlock`] is [`Park`](Self::Park), and
    /// [`TryError::Fail`] is the error.
    pub fn from_try<E>(result: Result<T, TryError<E>>) -> Result<Self, E> {
        match result {
            Ok(value) => Ok(Self::Ready(value)),
            Err(TryError::WouldBlock) => Ok(Self::Park),
            Err(TryError::Fail(error)) => Err(error),
        }
    }
}

/// Hand control back to the executor once, then resume.
///
/// Used where a wait loop must not park — it has work to redo immediately — but
/// must not starve other tasks by spinning.
pub fn yield_now() -> impl Future<Output = ()> {
    let mut yielded = false;
    core::future::poll_fn(move |cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
}

/// Wait on doorbell line `ch` until `check` reports [`Readiness::Ready`].
///
/// This is the one implementation of the doorbell wait protocol, and its ordering
/// is load-bearing: `check` runs, then the wait future is created and polled
/// **once** to register the waker, then `check` runs again, and only then does the
/// caller park. A doorbell whose fan-out stores no permit (`notify_waiters`, and
/// every hardware doorbell) drops an edge delivered between a check and the
/// registration, so re-checking after arming is what turns that race into a
/// guaranteed delivery. Awaiting the wait future directly would register and park
/// in one step, leaving no point at which to re-check.
///
/// The wait future lives for one iteration and no longer, so a later stray edge
/// cannot resolve a stale wait; dropping it runs the doorbell's `Drop`, which for
/// the MMIO bells disarms the line **without** acknowledging its status.
///
/// [`Readiness::Yield`] skips the doorbell entirely and re-checks after one
/// executor yield, so a `check` that always yields turns this into a cooperative
/// poll loop for transports with no completion line.
///
/// # Arguments
///
/// The context `cx` is borrowed mutably rather than shared purely so the returned
/// future is `Send` whenever `C` is: transport state typically holds a raw window
/// pointer and is not `Sync`. For the same reason `doorbell` and `check` are handed
/// `&C` on each pass instead of closing over it, and nothing derived from it is
/// held across an `await` except the doorbell's own wait future.
/// `doorbell_error` lifts the doorbell's error into the caller's.
///
/// No timeout: callers own the deadline, as with
/// [`Connect::connect`](crate::Connect::connect).
pub async fn park_until<C: ?Sized, D: Doorbell, T, E>(
    cx: &mut C,
    doorbell: impl Fn(&C) -> &D,
    ch: Chan,
    doorbell_error: impl Fn(D::Error) -> E,
    mut check: impl FnMut(&C) -> Result<Readiness<T>, E>,
) -> Result<T, E> {
    loop {
        match check(&*cx)? {
            Readiness::Ready(value) => return Ok(value),
            Readiness::Yield => {
                yield_now().await;
                continue;
            }
            Readiness::Park => {}
        }

        consortium_log::trace!("park: nothing ready, arming doorbell wait on {:?}", ch);
        let mut wait = core::pin::pin!(doorbell(&*cx).wait(ch));
        let armed = core::future::poll_fn(|w| Poll::Ready(wait.as_mut().poll(w))).await;

        match check(&*cx)? {
            Readiness::Ready(value) => return Ok(value),
            Readiness::Yield => {
                yield_now().await;
                continue;
            }
            Readiness::Park => {}
        }

        match armed {
            // The peer rang before we subscribed and the re-check still found nothing.
            // The future is spent, so start over — after a yield, so a doorbell that
            // keeps firing without changing memory cannot starve the executor.
            Poll::Ready(fired) => {
                consortium_log::trace!("park: doorbell had already fired on {:?}", ch);
                fired.map_err(&doorbell_error)?;
                yield_now().await;
            }
            // Waker is registered; park until the peer rings.
            Poll::Pending => {
                consortium_log::trace!("park: parking on {:?}", ch);
                wait.as_mut().await.map_err(&doorbell_error)?;
                consortium_log::trace!("park: doorbell fired on {:?}", ch);
            }
        }
    }
}

/// Why [`spin_until`] gave up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BlockingError<E> {
    /// `expired` reported the deadline passed before the check succeeded.
    TimedOut,
    /// The check itself failed.
    Fail(E),
}

impl<E: core::fmt::Display> core::fmt::Display for BlockingError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TimedOut => f.write_str("timed out waiting for the transport"),
            Self::Fail(error) => error.fmt(f),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for BlockingError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::TimedOut => None,
            Self::Fail(error) => Some(error),
        }
    }
}

/// Spin until `check` reports [`Readiness::Ready`], or `expired` reports the
/// deadline has passed.
///
/// The blocking counterpart of [`park_until`], for code with no executor: a
/// synchronous `init()`, a bootloader, a panic path. [`Readiness::Park`] and
/// [`Readiness::Yield`] both mean "check again".
///
/// Unlike the async form this **carries a deadline**: a blocking call cannot be
/// wrapped in an outer timeout, so a peer that never answers would otherwise hang
/// the core. `expired` is clock-agnostic — pass an `embassy_time` check, a cycle
/// counter, or a plain spin budget — and is consulted after every unsuccessful
/// check, so a check that succeeds first time never calls it.
///
/// Spinning only works when the peer makes progress independently of this core:
/// another processor, or hardware. Do not use it to wait on work an interrupt
/// handler of this same core would perform while that interrupt is masked.
pub fn spin_until<T, E>(
    mut check: impl FnMut() -> Result<Readiness<T>, E>,
    mut expired: impl FnMut() -> bool,
) -> Result<T, BlockingError<E>> {
    loop {
        match check().map_err(BlockingError::Fail)? {
            Readiness::Ready(value) => return Ok(value),
            Readiness::Park | Readiness::Yield => {}
        }
        if expired() {
            return Err(BlockingError::TimedOut);
        }
        core::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    use core::{
        pin::pin,
        sync::atomic::{AtomicBool, AtomicUsize, Ordering},
        task::{Context, Poll, Waker},
    };

    use super::*;
    use crate::MaybeSend;

    #[derive(Debug)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct BellFault;

    impl core::fmt::Display for BellFault {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("bell fault")
        }
    }

    impl core::error::Error for BellFault {}

    /// A doorbell whose `wait` runs a side effect when it is *created* — standing in
    /// for a peer that publishes and rings in the window between a check and the
    /// waker registration — and then either never fires or fails at once.
    struct Bell {
        ready: AtomicBool,
        publish_on_arm: bool,
        fail: bool,
        arms: AtomicUsize,
    }

    impl Bell {
        fn new(publish_on_arm: bool, fail: bool) -> Self {
            Self {
                ready: AtomicBool::new(false),
                publish_on_arm,
                fail,
                arms: AtomicUsize::new(0),
            }
        }
    }

    impl Doorbell for Bell {
        type Error = BellFault;

        fn ring(&self, _ch: Chan) -> Result<(), BellFault> {
            Ok(())
        }

        fn wait(&self, _ch: Chan) -> impl Future<Output = Result<Chan, BellFault>> + MaybeSend {
            self.arms.fetch_add(1, Ordering::Relaxed);
            if self.publish_on_arm {
                // The edge this models is lost: the future below never resolves.
                self.ready.store(true, Ordering::Release);
            }
            let fail = self.fail;
            core::future::poll_fn(move |_| {
                if fail {
                    Poll::Ready(Err(BellFault))
                } else {
                    Poll::Pending
                }
            })
        }
    }

    #[derive(Debug, PartialEq)]
    enum TestError {
        Bell,
    }

    fn poll_once<F: Future>(future: F) -> Poll<F::Output> {
        let mut future = pin!(future);
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
    }

    fn line() -> Chan {
        // SAFETY: the fake bell accepts any channel id.
        unsafe { Chan::new_unchecked(0) }
    }

    #[test]
    fn rechecks_after_arming_so_an_edge_before_registration_is_not_lost() {
        let mut bell = Bell::new(true, false);
        let outcome = poll_once(park_until(
            &mut bell,
            |bell| bell,
            line(),
            |_| TestError::Bell,
            |bell: &Bell| {
                Ok::<_, TestError>(if bell.ready.load(Ordering::Acquire) {
                    Readiness::Ready(7)
                } else {
                    Readiness::Park
                })
            },
        ));
        // The wait future never resolves, so only the post-arm re-check can see it.
        assert_eq!(outcome, Poll::Ready(Ok(7)));
    }

    #[test]
    fn parks_when_the_recheck_still_finds_nothing() {
        let mut bell = Bell::new(false, false);
        let outcome = poll_once(park_until(
            &mut bell,
            |bell| bell,
            line(),
            |_| TestError::Bell,
            |_: &Bell| Ok::<Readiness<()>, TestError>(Readiness::Park),
        ));
        assert!(outcome.is_pending());
        assert_eq!(bell.arms.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn yield_rechecks_without_arming_the_doorbell() {
        let mut bell = Bell::new(false, false);
        let mut checks = 0;
        {
            let mut future = pin!(park_until(
                &mut bell,
                |bell| bell,
                line(),
                |_| TestError::Bell,
                |_: &Bell| {
                    checks += 1;
                    Ok::<_, TestError>(if checks == 1 {
                        Readiness::Yield
                    } else {
                        Readiness::Ready(())
                    })
                },
            ));
            let mut cx = Context::from_waker(Waker::noop());
            assert!(future.as_mut().poll(&mut cx).is_pending());
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(Ok(())));
        }
        assert_eq!(bell.arms.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn doorbell_errors_are_lifted_into_the_callers_error() {
        let mut bell = Bell::new(false, true);
        let outcome = poll_once(park_until(
            &mut bell,
            |bell| bell,
            line(),
            |_| TestError::Bell,
            |_: &Bell| Ok::<Readiness<()>, TestError>(Readiness::Park),
        ));
        assert_eq!(outcome, Poll::Ready(Err(TestError::Bell)));
    }

    #[test]
    fn from_try_maps_would_block_to_park() {
        assert_eq!(Readiness::from_try::<()>(Ok(1)), Ok(Readiness::Ready(1)));
        assert_eq!(
            Readiness::<u8>::from_try::<()>(Err(TryError::WouldBlock)),
            Ok(Readiness::Park)
        );
        assert_eq!(
            Readiness::<u8>::from_try(Err(TryError::Fail("boom"))),
            Err("boom")
        );
    }

    #[test]
    fn spin_until_retries_park_and_yield_until_ready() {
        let mut steps = [Readiness::Park, Readiness::Yield, Readiness::Ready(3)].into_iter();
        let value = spin_until(
            || Ok::<_, ()>(steps.next().expect("ready before the steps run out")),
            || false,
        );
        assert_eq!(value, Ok(3));
    }

    #[test]
    fn spin_until_does_not_consult_the_deadline_on_first_success() {
        let value = spin_until(
            || Ok::<_, ()>(Readiness::Ready(())),
            || panic!("deadline consulted"),
        );
        assert_eq!(value, Ok(()));
    }

    #[test]
    fn spin_until_surfaces_check_failures() {
        let value: Result<(), _> = spin_until(|| Err("broken"), || false);
        assert_eq!(value, Err(BlockingError::Fail("broken")));
    }
}

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

//! The Type-A wait protocol, tested once against an in-memory register model. Each
//! block's own tests then only need to check its register mapping.

use core::{
    future::Future,
    task::{Context, Poll},
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering},
};
use std::task::{RawWaker, RawWakerVTable, Waker};

use consortium_ipc::{Chan, Doorbell};

use super::*;

/// Fake register block: raw status, an enable mask, and a log of every write.
#[derive(Default)]
struct Regs {
    status: AtomicU32,
    enabled: AtomicU32,
    busy: AtomicU32,
    log: Mutex<Vec<(&'static str, u32)>>,
    /// Stretch every register read-modify-write so concurrent ones would overlap.
    slow_rmw: AtomicBool,
    /// Set while a read-modify-write is in flight.
    in_rmw: AtomicBool,
    /// Read-modify-writes that started while another was still in flight.
    overlaps: AtomicUsize,
    /// This block's waker table. Per register model rather than static, so tests
    /// running in parallel cannot overwrite each other's registrations.
    wakers: [atomic_waker::AtomicWaker; LINES as usize],
}

impl Regs {
    fn record(&self, op: &'static str, bits: u32) {
        self.log.lock().unwrap().push((op, bits));
    }

    fn ops(&self) -> Vec<(&'static str, u32)> {
        self.log.lock().unwrap().clone()
    }

    fn enabled(&self, line: u8) -> bool {
        self.enabled.load(Ordering::SeqCst) & (1 << line) != 0
    }

    fn signal(&self, line: u8) {
        self.status.fetch_or(1 << line, Ordering::SeqCst);
    }

    /// Model a register read-modify-write, counting any that run concurrently.
    fn rmw(&self, f: impl FnOnce()) {
        if !self.slow_rmw.load(Ordering::SeqCst) {
            return f();
        }
        if self.in_rmw.swap(true, Ordering::SeqCst) {
            self.overlaps.fetch_add(1, Ordering::SeqCst);
        }
        std::thread::sleep(std::time::Duration::from_micros(50));
        f();
        self.in_rmw.store(false, Ordering::SeqCst);
    }
}

#[derive(Clone)]
struct FakeBell(Arc<Regs>);

const LINES: u8 = 8;

// SAFETY: an in-memory model; status bit `n` is channel `n`, `< 8`.
unsafe impl BellHw for FakeBell {
    type Line = u8;

    const PHYS_BASE: usize = 0;

    const CHANNELS: u8 = LINES;

    unsafe fn at(base: usize) -> Self {
        // SAFETY: tests pass a pointer leaked from `Arc::into_raw` (see `bell`).
        let regs = unsafe { Arc::from_raw(base as *const Regs) };
        let this = Self(Arc::clone(&regs));
        // Keep the test's own reference alive.
        let _ = Arc::into_raw(regs);
        this
    }

    fn line(id: u8) -> u8 {
        id
    }

    fn ring(&self, line: u8) -> Result<(), BellError> {
        if self.0.busy.load(Ordering::SeqCst) & (1 << line) != 0 {
            return Err(BellError::Busy);
        }
        self.0.rmw(|| self.0.record("ring", 1 << line));
        Ok(())
    }

    fn raw_pending(&self, line: u8) -> bool {
        self.0.status.load(Ordering::SeqCst) & (1 << line) != 0
    }

    unsafe fn ack(&self, line: u8) {
        self.0.record("ack", 1 << line);
        self.0.status.fetch_and(!(1 << line), Ordering::SeqCst);
    }

    unsafe fn arm(&self, line: u8) {
        self.0.rmw(|| {
            self.0.record("arm", 1 << line);
            self.0.enabled.fetch_or(1 << line, Ordering::SeqCst);
        });
    }

    unsafe fn disarm(&self, line: u8) {
        self.0.rmw(|| {
            self.0.record("disarm", 1 << line);
            self.0.enabled.fetch_and(!(1 << line), Ordering::SeqCst);
        });
    }

    fn asserted(&self) -> LineSet {
        self.0.status.load(Ordering::SeqCst)
    }

    fn waker(&self, id: u8) -> &atomic_waker::AtomicWaker {
        &self.0.wakers[id as usize]
    }

    unsafe fn disarm_all(&self, lines: LineSet) {
        self.0.rmw(|| {
            self.0.record("disarm_all", lines);
            self.0.enabled.fetch_and(!lines, Ordering::SeqCst);
        });
    }
}

fn bell() -> (BellA<FakeBell>, Arc<Regs>) {
    let regs = Arc::new(Regs::default());
    let raw = Arc::into_raw(Arc::clone(&regs)) as usize;
    // SAFETY: `raw` is a live `Arc<Regs>` pointer, which is what `FakeBell::at`
    // expects; the extra count it holds is reclaimed below.
    let bell = unsafe { BellA::<FakeBell>::new_mapped(raw) };
    // SAFETY: balances the `into_raw` above.
    drop(unsafe { Arc::from_raw(raw as *const Regs) });
    (bell, regs)
}

fn ch(id: u8) -> Chan {
    // SAFETY: tests pick ids deliberately to exercise validation paths.
    unsafe { Chan::new_unchecked(id) }
}

fn noop_cx() -> Context<'static> {
    Context::from_waker(Waker::noop())
}

/// A waker that counts its wakes, so a test can assert the future is actually
/// *registered* rather than relying on being re-polled by hand.
fn counting_waker() -> (Waker, Arc<AtomicUsize>) {
    unsafe fn clone(p: *const ()) -> RawWaker {
        // SAFETY: `p` came from `Arc::into_raw` below and is still alive.
        unsafe { Arc::increment_strong_count(p as *const AtomicUsize) };
        RawWaker::new(p, &VTABLE)
    }
    unsafe fn wake(p: *const ()) {
        // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
        let count = unsafe { Arc::from_raw(p as *const AtomicUsize) };
        count.fetch_add(1, Ordering::SeqCst);
    }
    unsafe fn wake_by_ref(p: *const ()) {
        // SAFETY: `p` came from `Arc::into_raw` and is still alive.
        unsafe { &*(p as *const AtomicUsize) }.fetch_add(1, Ordering::SeqCst);
    }
    unsafe fn drop_fn(p: *const ()) {
        // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
        unsafe { Arc::from_raw(p as *const AtomicUsize) };
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_fn);

    let count = Arc::new(AtomicUsize::new(0));
    let raw = RawWaker::new(Arc::into_raw(Arc::clone(&count)) as *const (), &VTABLE);
    // SAFETY: `VTABLE` upholds the `RawWaker` contract for an `Arc<AtomicUsize>`.
    (unsafe { Waker::from_raw(raw) }, count)
}

#[test]
fn ring_delegates_to_the_block() {
    let (bell, regs) = bell();

    bell.ring(ch(3)).unwrap();
    assert_eq!(regs.ops(), [("ring", 1 << 3)]);

    regs.busy.store(1 << 3, Ordering::SeqCst);
    assert_eq!(bell.ring(ch(3)), Err(BellError::Busy));
}

#[test]
fn invalid_channel_is_reported_everywhere() {
    let (bell, regs) = bell();

    assert_eq!(bell.ring(ch(LINES)), Err(BellError::ChanOutOfRange));
    let mut cx = noop_cx();
    assert_eq!(
        std::pin::pin!(bell.wait(ch(LINES))).poll(&mut cx),
        Poll::Ready(Err(BellError::ChanOutOfRange))
    );
    assert!(regs.ops().is_empty());
}

#[test]
fn completion_acknowledges_then_disarms() {
    let (bell, regs) = bell();
    let mut cx = noop_cx();
    let mut wait = std::pin::pin!(bell.wait(ch(1)));

    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
    regs.signal(1);
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(1))));

    assert_eq!(
        regs.ops(),
        [("arm", 1 << 1), ("ack", 1 << 1), ("disarm", 1 << 1)]
    );
    assert!(!regs.enabled(1));
}

#[test]
fn line_is_armed_once_across_polls() {
    let (bell, regs) = bell();
    let mut cx = noop_cx();
    let mut wait = std::pin::pin!(bell.wait(ch(1)));

    for _ in 0..3 {
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
    }
    assert_eq!(regs.ops(), [("arm", 1 << 1)]);
}

/// A signal that landed before the wait began is a notification the caller has not
/// consumed. Arming must not acknowledge it: the remote cannot signal again while it
/// waits for a reply.
#[test]
fn a_signal_from_before_the_wait_is_delivered_not_dropped() {
    let (bell, regs) = bell();
    regs.signal(4);
    let mut cx = noop_cx();

    assert_eq!(
        std::pin::pin!(bell.wait(ch(4))).poll(&mut cx),
        Poll::Ready(Ok(ch(4)))
    );
    assert_eq!(
        regs.ops(),
        [("arm", 1 << 4), ("ack", 1 << 4), ("disarm", 1 << 4)]
    );
}

/// A cancelled wait must leave its line masked, so nothing re-fires a status no one
/// is left to clear, and must not acknowledge it, so the signal survives for the next
/// wait.
#[test]
fn cancelling_a_wait_disarms_without_acknowledging() {
    let (bell, regs) = bell();
    let mut cx = noop_cx();
    {
        let mut wait = std::pin::pin!(bell.wait(ch(2)));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        regs.signal(2);
    }

    assert_eq!(regs.ops(), [("arm", 1 << 2), ("disarm", 1 << 2)]);
    assert!(!regs.enabled(2));
    assert!(
        bell.hw.raw_pending(2),
        "the signal must outlive the cancelled wait"
    );
}

#[test]
fn completed_or_unpolled_waits_do_not_disarm_on_drop() {
    let (bell, regs) = bell();
    drop(bell.wait(ch(2)));
    assert!(regs.ops().is_empty());

    regs.signal(2);
    let mut cx = noop_cx();
    {
        let mut wait = std::pin::pin!(bell.wait(ch(2)));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
    }
    // arm, ack, disarm — and no second disarm from `Drop`.
    assert_eq!(regs.ops().len(), 3);
}

#[test]
fn service_masks_every_asserted_line_before_waking() {
    let (bell, regs) = bell();
    let (waker, wakes) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut wait = std::pin::pin!(bell.wait(ch(2)));
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);

    regs.signal(2);
    regs.signal(5);
    bell.service_interrupt();

    assert_eq!(
        regs.ops(),
        [("arm", 1 << 2), ("disarm_all", (1 << 2) | (1 << 5))]
    );
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
}

/// Wakes are per line: servicing another line must not wake this waiter, and its own
/// line must still wake it afterwards.
#[test]
fn a_foreign_line_neither_wakes_nor_strands_the_waiter() {
    let (bell, regs) = bell();
    let (waker, wakes) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut wait = std::pin::pin!(bell.wait(ch(2)));

    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);

    regs.signal(5);
    bell.service_interrupt();
    assert_eq!(wakes.load(Ordering::SeqCst), 0);
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);

    regs.signal(2);
    bell.service_interrupt();
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
}

/// Two waiters on two lines of one instance is the ordinary duplex case. Each must
/// be woken by its own line and neither may steal the other's completion.
#[test]
fn two_lines_of_one_instance_complete_independently() {
    let (bell, regs) = bell();
    let (waker, wakes) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut a = std::pin::pin!(bell.wait(ch(1)));
    let mut b = std::pin::pin!(bell.wait(ch(6)));

    assert_eq!(a.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(b.as_mut().poll(&mut cx), Poll::Pending);

    regs.signal(6);
    bell.service_interrupt();
    // Only line 6's waiter is woken.
    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    assert_eq!(a.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(b.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(6))));

    regs.signal(1);
    bell.service_interrupt();
    assert_eq!(a.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(1))));
    assert_eq!(regs.status.load(Ordering::SeqCst), 0);
}

#[test]
fn clones_share_one_waker_table() {
    let (bell, regs) = bell();
    let other = bell.clone();
    let (waker, wakes) = counting_waker();
    let mut cx = Context::from_waker(&waker);
    let mut wait = std::pin::pin!(bell.wait(ch(3)));
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);

    regs.signal(3);
    other.service_interrupt();

    assert_eq!(wakes.load(Ordering::SeqCst), 1);
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));
}

#[test]
fn chan_validate_uses_the_block_bounds() {
    type Db = BellA<FakeBell>;
    assert_eq!(Chan::new::<Db>(0).map(|c| c.id()), Ok(0));
    assert_eq!(Chan::new::<Db>(1).map(|c| c.id()), Ok(1));
    assert_eq!(Chan::new::<Db>(LINES), Err(BellError::ChanOutOfRange));
}

/// The UIO IRQ task services the bell on one thread while waiters arm and disarm,
/// and the halves ring, on others. Every one of those is a register
/// read-modify-write, so none may run concurrently with another.
#[test]
fn register_writes_from_waiters_rings_and_service_never_overlap() {
    let (bell, regs) = bell();
    regs.slow_rmw.store(true, Ordering::SeqCst);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            for _ in 0..100 {
                let mut cx = noop_cx();
                // Arm on the first poll, disarm on drop.
                let mut wait = std::pin::pin!(bell.wait(ch(1)));
                assert!(wait.as_mut().poll(&mut cx).is_pending());
            }
        });
        scope.spawn(|| {
            for _ in 0..100 {
                regs.signal(2);
                bell.service_interrupt();
                regs.status.fetch_and(!(1 << 2), Ordering::SeqCst);
            }
        });
        scope.spawn(|| {
            for _ in 0..100 {
                bell.ring(ch(3)).unwrap();
            }
        });
    });

    assert_eq!(regs.overlaps.load(Ordering::SeqCst), 0);
}

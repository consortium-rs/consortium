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

//! MU register mapping. The wait protocol itself is covered once, generically, in
//! `bell::tests`; these check that each `BellHw` operation hits the right register
//! for both line kinds.

use consortium_ipc::{Chan, Primary, Secondary};

use super::registers::offsets;
use super::*;

// MU7 is the i.MX95 CA55 <-> CM7 default route; i.MX93 has only MU1/MU2.
const N: usize = if cfg!(feature = "mimx95") { 7 } else { 1 };

/// A zeroed fake window large enough to reach `RR3`.
const WINDOW_WORDS: usize = 0x300 / 4;

fn ch(id: u8) -> Chan {
    // SAFETY: tests pick ids deliberately to exercise validation paths.
    unsafe { Chan::new_unchecked(id) }
}

fn word(regs: &[u32], offset: u32) -> u32 {
    regs[offset as usize / 4]
}

#[test]
fn channels_split_into_gi_then_rf_lines() {
    type M = Mu<Primary, N>;
    assert_eq!(M::line_of(ch(0)), Ok(MuLine::Gi(0)));
    assert_eq!(M::line_of(ch(3)), Ok(MuLine::Gi(3)));
    assert_eq!(M::line_of(ch(4)), Ok(MuLine::Rf(0)));
    assert_eq!(M::line_of(ch(7)), Ok(MuLine::Rf(3)));
    assert_eq!(M::line_of(ch(8)), Err(BellError::ChanOutOfRange));

    for id in 0..MU_CHAN_COUNT {
        assert_eq!(M::slot(M::line_of(ch(id)).unwrap()), id);
    }
}

#[test]
fn chan_validate_accepts_every_line() {
    for id in 0..MU_CHAN_COUNT {
        assert_eq!(Chan::new::<MuChanValidate>(id).map(|c| c.id()), Ok(id));
    }
    assert_eq!(
        Chan::new::<MuChanValidate>(MU_CHAN_COUNT),
        Err(BellError::ChanOutOfRange)
    );
}

#[test]
fn side_b_window_sits_one_length_above_side_a() {
    assert_eq!(Mu::<Primary, N>::PHYS_BASE, base::BASE[N - 1]);
    assert_eq!(
        Mu::<Secondary, N>::PHYS_BASE,
        base::BASE[N - 1] + base::LENGTH
    );
}

#[test]
fn gi_lines_map_to_gcr_gsr_gier() {
    let mut regs = [0u32; WINDOW_WORDS];
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let mu = unsafe { Mu::<Primary, N>::at(regs.as_mut_ptr() as usize) };
    let line = MuLine::Gi(2);

    mu.ring(line).unwrap();
    assert_eq!(word(&regs, offsets::GCR), 1 << 2);

    assert!(!mu.raw_pending(line));
    regs[offsets::GSR as usize / 4] = 1 << 2;
    assert!(mu.raw_pending(line));

    // SAFETY: fake window; the test is single-threaded.
    unsafe { mu.arm(line) };
    assert_eq!(word(&regs, offsets::GIER), 1 << 2);
    // SAFETY: as above; the status is set.
    unsafe { mu.ack(line) };
    // W1C: exactly this line's bit is written.
    assert_eq!(word(&regs, offsets::GSR), 1 << 2);
    // SAFETY: as above.
    unsafe { mu.disarm(line) };
    assert_eq!(word(&regs, offsets::GIER), 0);
}

#[test]
fn rf_lines_map_to_transmit_and_receive_registers() {
    let mut regs = [0u32; WINDOW_WORDS];
    // TSR reset: every transmit register empty.
    regs[offsets::TSR as usize / 4] = registers::reset::TSR;
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let mu = unsafe { Mu::<Primary, N>::at(regs.as_mut_ptr() as usize) };
    let line = MuLine::Rf(1);

    mu.ring(line).unwrap();
    assert_ne!(word(&regs, offsets::TR1), 0);
    assert_eq!(word(&regs, offsets::TR0), 0);

    regs[offsets::RSR as usize / 4] = 1 << 1;
    assert!(mu.raw_pending(line));
    assert!(!mu.raw_pending(MuLine::Rf(2)));

    // SAFETY: fake window; the test is single-threaded.
    unsafe { mu.arm(line) };
    assert_eq!(word(&regs, offsets::RCR), 1 << 1);
    assert_eq!(word(&regs, offsets::GIER), 0, "RF must not touch GIER");
    // SAFETY: as above.
    unsafe { mu.disarm(line) };
    assert_eq!(word(&regs, offsets::RCR), 0);
}

#[test]
fn rf_ring_is_skipped_while_the_previous_token_is_unread() {
    let mut regs = [0u32; WINDOW_WORDS];
    // TE1 clear: TR1 still holds an unread token.
    regs[offsets::TSR as usize / 4] = registers::reset::TSR & !(1 << 1);
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let mu = unsafe { Mu::<Primary, N>::at(regs.as_mut_ptr() as usize) };

    assert_eq!(mu.ring(MuLine::Rf(1)), Ok(()));
    assert_eq!(word(&regs, offsets::TR1), 0);
}

#[test]
fn asserted_packs_gi_below_rf_and_disarm_all_splits_them() {
    let mut regs = [0u32; WINDOW_WORDS];
    regs[offsets::GSR as usize / 4] = 1 << 1;
    regs[offsets::RSR as usize / 4] = 1 << 2;
    regs[offsets::GIER as usize / 4] = 0b1111;
    regs[offsets::RCR as usize / 4] = 0b1111;
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let mu = unsafe { Mu::<Primary, N>::at(regs.as_mut_ptr() as usize) };

    let lines = mu.asserted();
    assert_eq!(lines, (1 << 1) | (1 << (MU_GI_CHAN_COUNT + 2)));

    // SAFETY: fake window; the set came from `asserted`.
    unsafe { mu.disarm_all(lines) };
    assert_eq!(word(&regs, offsets::GIER), 0b1101);
    assert_eq!(word(&regs, offsets::RCR), 0b1011);
}

#[cfg(feature = "std")]
#[test]
fn service_masks_gi_and_rf_lines_before_the_wait_acknowledges() {
    use consortium_ipc::Doorbell;
    use core::future::Future;
    use core::task::{Context, Poll};

    let mut regs = [0u32; WINDOW_WORDS];
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { MuDoorbell::<Primary, N>::new_mapped(regs.as_mut_ptr() as usize) };
    let mut cx = Context::from_waker(std::task::Waker::noop());
    let mut gi = std::pin::pin!(db.wait(ch(1)));
    let mut rf = std::pin::pin!(db.wait(ch(6)));
    assert_eq!(gi.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(rf.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(word(&regs, offsets::GIER), 1 << 1);
    assert_eq!(word(&regs, offsets::RCR), 1 << 2);

    regs[offsets::GSR as usize / 4] = 1 << 1;
    regs[offsets::RSR as usize / 4] = 1 << 2;
    db.service_interrupt();
    assert_eq!(word(&regs, offsets::GIER), 0);
    assert_eq!(word(&regs, offsets::RCR), 0);

    assert_eq!(gi.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(1))));
    assert_eq!(rf.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(6))));
}

/// On `no_std`, two lines of one instance must keep two wakers: a single shared
/// `AtomicWaker` would let the second registration overwrite the first.
#[cfg(not(feature = "std"))]
#[test]
fn distinct_lines_keep_distinct_wakers() {
    extern crate std;

    use consortium_ipc::Doorbell;
    use core::future::Future;
    use core::sync::atomic::{AtomicUsize, Ordering};
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use std::sync::Arc;

    fn counting_waker() -> (Waker, Arc<AtomicUsize>) {
        unsafe fn clone(p: *const ()) -> RawWaker {
            // SAFETY: `p` came from `Arc::into_raw` and remains live.
            unsafe { Arc::increment_strong_count(p as *const AtomicUsize) };
            RawWaker::new(p, &VTABLE)
        }
        unsafe fn wake(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            let count = unsafe { Arc::from_raw(p as *const AtomicUsize) };
            count.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn wake_by_ref(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw` and remains live.
            unsafe { &*(p as *const AtomicUsize) }.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn drop_fn(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            unsafe { drop(Arc::from_raw(p as *const AtomicUsize)) };
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_fn);

        let count = Arc::new(AtomicUsize::new(0));
        let raw = RawWaker::new(Arc::into_raw(Arc::clone(&count)) as *const (), &VTABLE);
        // SAFETY: `VTABLE` implements the `RawWaker` contract for the Arc payload.
        (unsafe { Waker::from_raw(raw) }, count)
    }

    let mut regs = [0u32; WINDOW_WORDS];
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { MuDoorbell::<Primary, N>::new_mapped(regs.as_mut_ptr() as usize) };
    let mut first = core::pin::pin!(db.wait(ch(1)));
    let mut second = core::pin::pin!(db.wait(ch(5)));
    let (first_waker, first_wakes) = counting_waker();
    let (second_waker, second_wakes) = counting_waker();

    assert_eq!(
        first.as_mut().poll(&mut Context::from_waker(&first_waker)),
        Poll::Pending
    );
    assert_eq!(
        second
            .as_mut()
            .poll(&mut Context::from_waker(&second_waker)),
        Poll::Pending
    );

    // Raise the lines through the window pointer the doorbell holds.
    // SAFETY: both offsets are in-bounds registers of the fake window.
    unsafe {
        let window = regs.as_mut_ptr();
        window.add(offsets::GSR as usize / 4).write_volatile(1 << 1);
        window.add(offsets::RSR as usize / 4).write_volatile(1 << 1);
    }
    // The ISR core, without the real `PHYS_BASE` that `service_interrupt` uses.
    super::super::service(db.hw(), |slot| Mu::<Primary, N>::waker(slot).wake());

    assert_eq!(first_wakes.load(Ordering::SeqCst), 1);
    assert_eq!(second_wakes.load(Ordering::SeqCst), 1);
}

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

//! HSEM register mapping. The wait protocol itself is covered once, generically, in
//! `bell::tests`; these check the lock/unlock ring and that each interrupt operation
//! hits this bank's registers.

use consortium_ipc::{Chan, Doorbell, Primary, Secondary, Tertiary};

use super::registers::{cr::SemaphoreRegister, offsets};
use super::*;

/// A zeroed fake window covering the whole HSEM block.
const WINDOW_WORDS: usize = 0x400 / 4;

fn ch(id: u8) -> Chan {
    // SAFETY: tests pick ids deliberately to exercise validation paths.
    unsafe { Chan::new_unchecked(id) }
}

fn word(regs: &[u32], offset: u32) -> u32 {
    regs[offset as usize / 4]
}

fn set(regs: &mut [u32], offset: u32, value: u32) {
    regs[offset as usize / 4] = value;
}

/// What `RLRn` returns once `lockid` holds the semaphore.
fn held_by(lockid: u8) -> u32 {
    let mut reg = SemaphoreRegister::from_bits(0);
    reg.set_lock(true);
    reg.set_lockid(lockid.into());
    reg.bits()
}

#[test]
fn ring_unlocks_the_semaphore_it_just_locked() {
    let mut regs = [0u32; WINDOW_WORDS];
    let sem = 3;
    set(
        &mut regs,
        offsets::RLR + 4 * sem,
        held_by(Hsem::<Primary, 1>::LOCKID),
    );
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };

    db.ring(ch(sem as u8)).unwrap();

    let written = SemaphoreRegister::from_bits(word(&regs, offsets::R + 4 * sem));
    assert!(
        !written.lock(),
        "ring must write LOCK = 0 to raise the interrupt"
    );
    assert_eq!(written.lockid() as u8, Hsem::<Primary, 1>::LOCKID);
}

#[test]
fn ring_reports_busy_when_another_core_holds_the_semaphore() {
    let mut regs = [0u32; WINDOW_WORDS];
    set(
        &mut regs,
        offsets::RLR + 4 * 3,
        held_by(Hsem::<Secondary, 1>::LOCKID),
    );
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };

    assert_eq!(db.ring(ch(3)), Err(BellError::Busy));
    assert_eq!(word(&regs, offsets::R + 4 * 3), 0);
}

#[test]
fn each_side_drives_its_own_interrupt_bank() {
    fn armed_ier<S: Side>() -> [u32; 3] {
        let mut regs = [0u32; WINDOW_WORDS];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let hsem = unsafe { Hsem::<S, 1>::at(regs.as_mut_ptr() as usize) };
        // SAFETY: fake window; the test is single-threaded.
        unsafe { hsem.arm(5) };
        [
            word(&regs, offsets::C1IER),
            word(&regs, offsets::C2IER),
            word(&regs, offsets::C3IER),
        ]
    }

    assert_eq!(armed_ier::<Primary>(), [1 << 5, 0, 0]);
    assert_eq!(armed_ier::<Secondary>(), [0, 1 << 5, 0]);
    assert_eq!(armed_ier::<Tertiary>(), [0, 0, 1 << 5]);
    assert_eq!(Hsem::<Tertiary, 1>::LOCKID, 3);
}

#[test]
fn status_is_read_raw_and_acknowledged_through_icr() {
    let mut regs = [0u32; WINDOW_WORDS];
    // Masked: CnMISR is clear, but the raw CnISR still holds the release.
    set(&mut regs, offsets::C2ISR, 1 << 4);
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let hsem = unsafe { Hsem::<Secondary, 1>::at(regs.as_mut_ptr() as usize) };

    assert!(hsem.raw_pending(4));
    assert!(!hsem.raw_pending(5));
    assert_eq!(hsem.asserted(), 1 << 4);

    // SAFETY: fake window; the status is set.
    unsafe { hsem.ack(4) };
    assert_eq!(word(&regs, offsets::C2ICR), 1 << 4);
    assert_eq!(word(&regs, offsets::C1ICR), 0);
}

#[test]
fn disarm_all_clears_only_the_given_semaphores() {
    let mut regs = [0u32; WINDOW_WORDS];
    set(&mut regs, offsets::C1IER, 0b1111_0000);
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let hsem = unsafe { Hsem::<Primary, 1>::at(regs.as_mut_ptr() as usize) };

    // SAFETY: fake window; the test is single-threaded.
    unsafe { hsem.disarm_all((1 << 4) | (1 << 6)) };
    assert_eq!(word(&regs, offsets::C1IER), 0b1010_0000);
}

#[test]
fn chan_validate_rejects_out_of_range() {
    type Db = HsemDoorbell<Primary, 1>;
    assert_eq!(Chan::new::<Db>(0).map(|c| c.id()), Ok(0));
    assert_eq!(
        Chan::new::<Db>(HSEM_CHAN_COUNT),
        Err(BellError::ChanOutOfRange)
    );
}

#[test]
fn phys_base_is_the_single_instance_window() {
    assert_eq!(Hsem::<Secondary, 1>::PHYS_BASE, base::BASE[0]);
}

#[cfg(feature = "std")]
#[test]
fn service_masks_released_semaphores_before_the_wait_acknowledges() {
    use core::future::Future;
    use core::task::{Context, Poll};

    let mut regs = [0u32; WINDOW_WORDS];
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { HsemDoorbell::<Secondary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
    let mut cx = Context::from_waker(std::task::Waker::noop());
    let mut wait = std::pin::pin!(db.wait(ch(6)));
    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
    assert_eq!(word(&regs, offsets::C2IER), 1 << 6);

    set(&mut regs, offsets::C2ISR, 1 << 6);
    db.service_interrupt();
    assert_eq!(
        word(&regs, offsets::C2IER),
        0,
        "the ISR masks before waking"
    );
    assert_eq!(word(&regs, offsets::C2ICR), 0, "the ISR never acknowledges");

    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(6))));
    assert_eq!(word(&regs, offsets::C2ICR), 1 << 6);
}

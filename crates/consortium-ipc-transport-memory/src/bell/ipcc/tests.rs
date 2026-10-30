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

//! IPCC register mapping. The wait protocol itself is covered once, generically, in
//! `bell::tests`; these check that each `BellHw` operation hits the right bank and
//! bits.

use core::{
    future::Future,
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, Primary, Secondary};

use super::registers::{
    cr::{ChannelRegister, ControlRegister, StatusSetClearRegister},
    offsets,
};
use super::*;

fn ch(id: u8) -> Chan {
    // SAFETY: tests pick ids deliberately to exercise validation paths.
    unsafe { Chan::new_unchecked(id) }
}

fn set_flag(ch: u8) -> u32 {
    let mut reg = StatusSetClearRegister::from_bits(0);
    reg.set_channel_set(1 << ch);
    reg.bits()
}

fn clear_flag(ch: u8) -> u32 {
    let mut reg = StatusSetClearRegister::from_bits(0);
    reg.set_channel_clear(1 << ch);
    reg.bits()
}

fn occupied(ch: u8) -> u32 {
    ChannelRegister::from_bits(1 << ch).bits()
}

fn masked(regs: &[u32], mr: usize, ch: u8) -> bool {
    ChannelRegister::from_bits(regs[mr / 4]).channels() & (1 << ch) != 0
}

// Primary is processor 1; it writes C1SCR and reads C2TOC1SR.
fn doorbell(regs: &mut [u32]) -> IpccDoorbell<Primary, 1> {
    // SAFETY: `regs` backs a fake MMIO window for the test.
    unsafe { IpccDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) }
}

#[test]
fn ring_sets_processor_1_channel_occupied() {
    let mut regs = [0u32; 8];
    let db = doorbell(&mut regs);

    db.ring(ch(2)).unwrap();

    assert_eq!(regs[offsets::C1SCR / 4], set_flag(2));
}

#[test]
fn secondary_rings_through_processor_2_bank() {
    let mut regs = [0u32; 8];
    // SAFETY: `regs` backs a fake MMIO window for the test.
    let db = unsafe { IpccDoorbell::<Secondary, 1>::new_mapped(regs.as_mut_ptr() as usize) };

    db.ring(ch(3)).unwrap();

    assert_eq!(regs[offsets::C2SCR / 4], set_flag(3));
    assert_eq!(regs[offsets::C1SCR / 4], 0);
}

#[test]
fn ring_reports_busy_while_outgoing_channel_is_occupied() {
    let mut regs = [0u32; 8];
    regs[offsets::C1TOC2SR / 4] = occupied(2);
    let db = doorbell(&mut regs);

    assert_eq!(db.ring(ch(2)), Err(BellError::Busy));
    assert_eq!(regs[offsets::C1SCR / 4], 0);
}

#[test]
fn pending_reads_incoming_status() {
    let mut regs = [0u32; 8];
    regs[offsets::C2TOC1SR / 4] = occupied(3);
    let db = doorbell(&mut regs);

    assert!(db.pending(ch(3)));
    assert!(!db.pending(ch(4)));
}

#[test]
fn wait_arms_then_acknowledges_and_masks() {
    let mut regs = [0u32; 8];
    regs[offsets::C1MR / 4] = u32::MAX;
    let db = doorbell(&mut regs);
    let mut wait = std::pin::pin!(db.wait(ch(2)));
    let mut cx = Context::from_waker(std::task::Waker::noop());

    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
    assert!(ControlRegister::from_bits(regs[offsets::C1CR / 4]).rxoie());
    assert!(!masked(&regs, offsets::C1MR, 2));

    regs[offsets::C2TOC1SR / 4] = occupied(2);
    db.notifier().notify_waiters();

    assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
    assert_eq!(regs[offsets::C1SCR / 4], clear_flag(2));
    assert!(masked(&regs, offsets::C1MR, 2));
}

#[test]
fn service_masks_every_occupied_channel_in_one_update() {
    let mut regs = [0u32; 8];
    regs[offsets::C2TOC1SR / 4] = occupied(2) | occupied(5);
    regs[offsets::C1MR / 4] = 1 << 7;
    let db = doorbell(&mut regs);

    db.service_interrupt();

    assert_eq!(
        ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels(),
        (1 << 2) | (1 << 5) | (1 << 7)
    );
    // Servicing never acknowledges; the waiting future does.
    assert_eq!(regs[offsets::C1SCR / 4], 0);
}

#[test]
fn service_reads_raw_status_not_the_masked_view() {
    let mut regs = [0u32; 8];
    regs[offsets::C2TOC1SR / 4] = occupied(4);
    regs[offsets::C1MR / 4] = 1 << 4;
    let db = doorbell(&mut regs);

    assert_eq!(db.hw().asserted(), 1 << 4);
}

#[test]
fn dropping_pending_wait_masks_its_channel() {
    let mut regs = [0u32; 8];
    regs[offsets::C1MR / 4] = u32::MAX;
    let db = doorbell(&mut regs);
    let mut cx = Context::from_waker(std::task::Waker::noop());

    {
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(!masked(&regs, offsets::C1MR, 2));
    }

    assert!(masked(&regs, offsets::C1MR, 2));
}

#[test]
fn out_of_range_channels_are_rejected() {
    let mut regs = [0u32; 8];
    let db = doorbell(&mut regs);

    assert_eq!(db.ring(ch(16)), Err(BellError::ChanOutOfRange));
    assert!(!db.pending(ch(16)));
}

#[test]
fn chan_validate_rejects_reserved_and_out_of_range() {
    assert_eq!(
        Chan::new::<IpccDoorbell<Primary, 1>>(0),
        Err(BellError::ChanReserved)
    );
    assert_eq!(
        Chan::new::<IpccDoorbell<Primary, 1>>(IPCC_CHAN_COUNT),
        Err(BellError::ChanOutOfRange)
    );
    assert_eq!(
        Chan::new::<IpccDoorbell<Primary, 1>>(1).map(|c| c.id()),
        Ok(1)
    );
}

#[test]
fn phys_base_is_the_instance_window() {
    assert_eq!(Ipcc::<Secondary, 1>::PHYS_BASE, base::BASE[0]);
}

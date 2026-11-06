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

//! The SCMI agent against a scripted platform.
//!
//! The channel is a heap-backed `UncachedRegion`. The fake doorbell plays the
//! platform: ringing it decodes the request straight out of the window, asks the
//! test's script for a reply, and writes that reply back the way a platform
//! would — immediately, or from another thread after a delay so the polled path
//! genuinely waits.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};

use consortium_core::{Chan, Doorbell, MaybeSend, UncachedRegion};
use consortium_pmi::{
    ClockId, ClockProvider, FunctionId, PinConfig, PinId, PinctrlProvider, PowerDomainId,
    PowerProvider, PowerState, ResetId, ResetProvider,
};
use consortium_pmi_scmi::{
    Client, Completion, Error, MessageHeader, MessageType, ProtocolError, ProtocolId, Smt,
    SmtError, Status,
    smt::{
        CHANNEL_STATUS, FLAG_COMPLETION_IRQ, FLAGS, LENGTH, MSG_HEADER, MSG_PAYLOAD, STATUS_ERROR,
        STATUS_FREE,
    },
};

// ── the shared window ───────────────────────────────────────────────────────

/// Heap memory standing in for the mailbox window; `u32`s for alignment.
struct Window {
    words: Box<[u32]>,
}

impl Window {
    fn new(len: usize) -> Self {
        let mut w = Self {
            words: vec![0u32; len.div_ceil(4)].into_boxed_slice(),
        };
        w.set(CHANNEL_STATUS, STATUS_FREE);
        w
    }

    fn len(&self) -> usize {
        self.words.len() * 4
    }

    fn ptr(&self) -> *mut u8 {
        self.words.as_ptr() as *mut u8
    }

    fn region(&self) -> UncachedRegion {
        // SAFETY: the window outlives every client a test builds on it, and the
        // fake platform only touches it between a ring and the client's next read.
        unsafe { UncachedRegion::new(self.ptr(), self.len()) }
    }

    fn set(&mut self, offset: usize, value: u32) {
        self.words[offset / 4] = value.to_le();
    }
}

/// A raw window pointer the fake platform may carry to another thread.
#[derive(Clone, Copy)]
struct Raw(usize);

impl Raw {
    fn get(self, offset: usize) -> u32 {
        // SAFETY: `offset` is a 4-aligned field offset inside the window.
        u32::from_le(unsafe { ((self.0 + offset) as *const u32).read_volatile() })
    }

    fn put(self, offset: usize, value: u32) {
        // SAFETY: as in `get`.
        unsafe { ((self.0 + offset) as *mut u32).write_volatile(value.to_le()) }
    }

    fn byte(self, offset: usize) -> u8 {
        // SAFETY: `offset` is inside the window.
        unsafe { ((self.0 + offset) as *const u8).read_volatile() }
    }

    fn put_byte(self, offset: usize, value: u8) {
        // SAFETY: `offset` is inside the window.
        unsafe { ((self.0 + offset) as *mut u8).write_volatile(value) }
    }
}

// ── the scripted platform ───────────────────────────────────────────────────

/// One request as the platform saw it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Seen {
    header: MessageHeader,
    payload: Vec<u8>,
    completion_irq: bool,
}

impl Seen {
    fn words(&self) -> Vec<u32> {
        self.payload
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }
}

/// How the platform answers one request.
#[derive(Clone, Debug)]
enum Reply {
    /// Status plus return values, echoing the request header.
    Ok(i32, Vec<u8>),
    /// As `Ok(0, ..)`, but with the token off by one.
    WrongToken,
    /// Set the channel's error bit.
    ChannelError,
}

fn ok(ret: &[u32]) -> Reply {
    Reply::Ok(0, ret.iter().flat_map(|w| w.to_le_bytes()).collect())
}

#[derive(Default)]
struct Platform {
    replies: VecDeque<Reply>,
    seen: Vec<Seen>,
    /// Answer from another thread after this long, instead of inside `ring`.
    delay: Option<Duration>,
    /// Completion lines rung back to the agent and not yet consumed.
    completions: u32,
}

#[derive(Clone)]
struct FakeBell {
    window: Raw,
    platform: Arc<Mutex<Platform>>,
}

#[derive(Debug)]
struct FakeError;

impl core::fmt::Display for FakeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("fake doorbell")
    }
}

impl core::error::Error for FakeError {}

fn answer(window: Raw, platform: &Mutex<Platform>) {
    let mut p = platform.lock().unwrap();
    let header = MessageHeader::from_raw(window.get(MSG_HEADER));
    let len = window.get(LENGTH) as usize - 4;
    let payload = (0..len).map(|i| window.byte(MSG_PAYLOAD + i)).collect();
    let completion_irq = window.get(FLAGS) & FLAG_COMPLETION_IRQ != 0;
    p.seen.push(Seen {
        header,
        payload,
        completion_irq,
    });
    let reply = p.replies.pop_front().expect("unscripted request");
    match reply {
        Reply::Ok(status, ret) => {
            write_reply(window, header.raw(), status, &ret);
            window.put(CHANNEL_STATUS, STATUS_FREE);
        }
        Reply::WrongToken => {
            let bad = MessageHeader::new(
                header.protocol(),
                header.message(),
                MessageType::Command,
                header.token().wrapping_add(1),
            );
            write_reply(window, bad.raw(), 0, &[]);
            window.put(CHANNEL_STATUS, STATUS_FREE);
        }
        Reply::ChannelError => window.put(CHANNEL_STATUS, STATUS_FREE | STATUS_ERROR),
    }
    if completion_irq {
        p.completions += 1;
    }
}

fn write_reply(window: Raw, header: u32, status: i32, ret: &[u8]) {
    window.put(MSG_HEADER, header);
    window.put(LENGTH, 8 + ret.len() as u32);
    window.put(MSG_PAYLOAD, status as u32);
    for (i, b) in ret.iter().enumerate() {
        window.put_byte(MSG_PAYLOAD + 4 + i, *b);
    }
}

impl Doorbell for FakeBell {
    type Error = FakeError;

    fn ring(&self, _ch: Chan) -> Result<(), FakeError> {
        let delay = self.platform.lock().unwrap().delay;
        match delay {
            None => answer(self.window, &self.platform),
            Some(delay) => {
                let (window, platform) = (self.window, Arc::clone(&self.platform));
                std::thread::spawn(move || {
                    std::thread::sleep(delay);
                    answer(window, &platform);
                });
            }
        }
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, FakeError>> + MaybeSend {
        let platform = Arc::clone(&self.platform);
        core::future::poll_fn(move |_cx| {
            let mut p = platform.lock().unwrap();
            if p.completions > 0 {
                p.completions -= 1;
                Poll::Ready(Ok(ch))
            } else {
                Poll::Pending
            }
        })
    }
}

// ── harness ─────────────────────────────────────────────────────────────────

/// Drive `fut` to completion, spinning; the fakes need no real wakeups.
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    for _ in 0..1_000_000 {
        if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
            return out;
        }
        std::thread::yield_now();
    }
    panic!("future never completed");
}

const A2P: u8 = 0;
const P2A: u8 = 1;

fn chan(id: u8) -> Chan {
    // SAFETY: the fake doorbell accepts any line.
    unsafe { Chan::new_unchecked(id) }
}

struct Rig {
    window: Window,
    platform: Arc<Mutex<Platform>>,
    client: Client<UncachedRegion, FakeBell>,
}

fn rig_with(len: usize, completion: Completion, replies: Vec<Reply>) -> Rig {
    let window = Window::new(len);
    let platform = Arc::new(Mutex::new(Platform {
        replies: replies.into(),
        ..Platform::default()
    }));
    let bell = FakeBell {
        window: Raw(window.ptr() as usize),
        platform: Arc::clone(&platform),
    };
    let smt = Smt::new(window.region()).unwrap();
    let client = Client::new(smt, bell, chan(A2P), completion);
    Rig {
        window,
        platform,
        client,
    }
}

fn rig(replies: Vec<Reply>) -> Rig {
    rig_with(128, Completion::Doorbell(chan(P2A)), replies)
}

impl Rig {
    fn seen(&self) -> Vec<Seen> {
        self.platform.lock().unwrap().seen.clone()
    }
}

// ── channel and exchange ────────────────────────────────────────────────────

#[test]
fn smt_rejects_short_and_misaligned_windows() {
    let w = Window::new(64);
    // SAFETY: `w` outlives both regions and only the length/alignment are inspected.
    let short = unsafe { UncachedRegion::new(w.ptr(), 31) };
    assert!(matches!(
        Smt::new(short),
        Err(SmtError::TooSmall { len: 31 })
    ));
    // SAFETY: as above; one byte past the start stays inside the allocation.
    let skewed = unsafe { UncachedRegion::new(w.ptr().wrapping_add(1), 40) };
    assert!(matches!(Smt::new(skewed), Err(SmtError::Misaligned)));
}

#[test]
fn base_version_round_trips_and_tokens_advance() {
    let mut r = rig(vec![ok(&[0x0002_0001]), ok(&[0x0003_0000])]);
    let v = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap();
    assert_eq!((v.major(), v.minor()), (2, 1));
    block_on(r.client.protocol_version(ProtocolId::CLOCK)).unwrap();

    let seen = r.seen();
    assert_eq!(seen[0].header.protocol(), ProtocolId::BASE);
    assert_eq!(seen[0].header.message(), 0);
    assert_eq!(seen[0].header.kind(), Some(MessageType::Command));
    assert!(seen[0].payload.is_empty());
    assert!(
        seen[0].completion_irq,
        "doorbell completion asks for the interrupt"
    );
    assert_eq!(seen[1].header.token(), seen[0].header.token() + 1);
}

#[test]
fn discover_vendor_reads_sixteen_bytes() {
    let mut name = *b"NXP\0\0\0\0\0\0\0\0\0\0\0\0\0";
    name[3] = 0;
    let mut r = rig(vec![Reply::Ok(0, name.to_vec())]);
    assert_eq!(&block_on(r.client.discover_vendor()).unwrap()[..3], b"NXP");
}

#[test]
fn error_status_is_reported() {
    let mut r = rig(vec![Reply::Ok(-4, vec![])]);
    let err = block_on(r.client.rate(ClockId(9))).unwrap_err();
    assert!(matches!(err, Error::Status(Status::NotFound)), "{err:?}");
}

#[test]
fn busy_channel_is_refused_without_ringing() {
    let mut r = rig(vec![]);
    r.window.set(CHANNEL_STATUS, 0);
    let err = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap_err();
    assert!(matches!(err, Error::ChannelBusy), "{err:?}");
    assert!(r.seen().is_empty());
}

#[test]
fn channel_error_bit_is_reported() {
    let mut r = rig(vec![Reply::ChannelError]);
    let err = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap_err();
    assert!(matches!(err, Error::ChannelError), "{err:?}");
}

#[test]
fn mismatched_token_is_a_protocol_error() {
    let mut r = rig(vec![Reply::WrongToken]);
    let err = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap_err();
    assert!(
        matches!(err, Error::Protocol(ProtocolError::HeaderMismatch { .. })),
        "{err:?}"
    );
}

#[test]
fn short_response_is_a_protocol_error() {
    let mut r = rig(vec![ok(&[])]);
    let err = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap_err();
    assert!(
        matches!(
            err,
            Error::Protocol(ProtocolError::ShortResponse { len: 0 })
        ),
        "{err:?}"
    );
}

#[test]
fn oversized_request_is_refused() {
    let mut r = rig_with(40, Completion::Poll, vec![]);
    let request = [0u8; 16]; // capacity is 40 - 0x1c = 12
    let err = block_on(r.client.call(ProtocolId::BASE, 0, &request, &mut [])).unwrap_err();
    assert!(
        matches!(
            err,
            Error::Protocol(ProtocolError::RequestTooLarge {
                len: 16,
                capacity: 12
            })
        ),
        "{err:?}"
    );
}

#[test]
fn polled_completion_waits_for_a_late_platform() {
    let mut r = rig_with(128, Completion::Poll, vec![ok(&[0x0001_0000])]);
    r.platform.lock().unwrap().delay = Some(Duration::from_millis(20));
    let v = block_on(r.client.protocol_version(ProtocolId::BASE)).unwrap();
    assert_eq!(v.major(), 1);
    assert!(
        !r.seen()[0].completion_irq,
        "polling does not ask for the interrupt"
    );
}

#[test]
fn doorbell_completion_waits_for_a_late_platform() {
    let mut r = rig(vec![ok(&[0x0001_0000])]);
    r.platform.lock().unwrap().delay = Some(Duration::from_millis(20));
    assert_eq!(
        block_on(r.client.protocol_version(ProtocolId::BASE))
            .unwrap()
            .major(),
        1
    );
}

// ── providers ───────────────────────────────────────────────────────────────

#[test]
fn clock_config_set_follows_the_platform_clock_version() {
    // v3: query once, then the 12-byte layout for both calls.
    let mut r = rig(vec![ok(&[0x0003_0000]), ok(&[]), ok(&[])]);
    block_on(r.client.enable(ClockId(5))).unwrap();
    block_on(r.client.disable(ClockId(5))).unwrap();
    let seen = r.seen();
    assert_eq!(seen.len(), 3, "version is asked once");
    assert_eq!(
        (seen[0].header.protocol(), seen[0].header.message()),
        (ProtocolId::CLOCK, 0)
    );
    assert_eq!(seen[1].header.message(), 0x7);
    assert_eq!(seen[1].words(), [5, 1, 0]);
    assert_eq!(seen[2].words(), [5, 0, 0]);

    // v2: the 8-byte layout.
    let mut r = rig(vec![ok(&[0x0002_0000]), ok(&[])]);
    block_on(r.client.enable(ClockId(5))).unwrap();
    assert_eq!(r.seen()[1].words(), [5, 1]);
}

#[test]
fn clock_rates_are_split_into_words() {
    let hz: u64 = 0x1_2345_6789;
    let mut r = rig(vec![ok(&[]), ok(&[hz as u32, (hz >> 32) as u32])]);
    block_on(r.client.set_rate(ClockId(3), hz)).unwrap();
    assert_eq!(block_on(r.client.rate(ClockId(3))).unwrap(), hz);
    let seen = r.seen();
    assert_eq!(seen[0].header.message(), 0x5);
    assert_eq!(seen[0].words(), [0, 3, hz as u32, (hz >> 32) as u32]);
    assert_eq!((seen[1].header.message(), seen[1].words()), (0x6, vec![3]));
}

#[test]
fn pinctrl_function_select_sets_the_function_bit() {
    let mut r = rig(vec![ok(&[])]);
    block_on(r.client.set_function(PinId(40), FunctionId(2))).unwrap();
    let seen = &r.seen()[0];
    assert_eq!(
        (seen.header.protocol(), seen.header.message()),
        (ProtocolId::PINCTRL, 0x6)
    );
    assert_eq!(seen.words(), [40, 2, 1 << 10]);
}

#[test]
fn pinctrl_configs_are_paired_and_chunked_to_the_channel() {
    // capacity 0x40 - 0x1c = 36 bytes: 3 fixed words + 3 pairs would be 36 → 3 per message.
    let configs = [
        PinConfig::PullUp,
        PinConfig::DriveStrength(6),
        PinConfig::InputSchmitt(true),
        PinConfig::SlewRate(1),
        PinConfig::Vendor {
            kind: 200,
            value: 7,
        },
    ];
    let mut r = rig_with(0x40, Completion::Poll, vec![ok(&[]), ok(&[])]);
    block_on(r.client.set_config(PinId(8), &configs)).unwrap();
    let seen = r.seen();
    assert_eq!(seen.len(), 2);
    assert_eq!(
        seen[0].words(),
        [8, 0xffff_ffff, 3 << 2, 4, 1, 10, 6, 15, 1]
    );
    assert_eq!(seen[1].words(), [8, 0xffff_ffff, 2 << 2, 20, 1, 200, 7]);
}

#[test]
fn power_states_map_to_the_generic_encodings() {
    let mut r = rig(vec![ok(&[]), ok(&[1 << 30]), ok(&[0x42])]);
    block_on(r.client.set_state(PowerDomainId(4), PowerState::On)).unwrap();
    assert_eq!(
        block_on(r.client.state(PowerDomainId(4))).unwrap(),
        PowerState::Off
    );
    assert_eq!(
        block_on(r.client.state(PowerDomainId(4))).unwrap(),
        PowerState::Other(0x42)
    );
    let seen = r.seen();
    assert_eq!(
        (seen[0].header.protocol(), seen[0].header.message()),
        (ProtocolId::POWER, 0x4)
    );
    assert_eq!(seen[0].words(), [0, 4, 0]);
    assert_eq!((seen[1].header.message(), seen[1].words()), (0x5, vec![4]));
}

#[test]
fn reset_flags_select_assert_release_and_pulse() {
    let mut r = rig(vec![ok(&[]), ok(&[]), ok(&[])]);
    block_on(r.client.assert(ResetId(1))).unwrap();
    block_on(r.client.deassert(ResetId(1))).unwrap();
    block_on(r.client.reset(ResetId(1))).unwrap();
    let seen = r.seen();
    assert_eq!(
        (seen[0].header.protocol(), seen[0].header.message()),
        (ProtocolId::RESET, 0x4)
    );
    assert_eq!(seen[0].words(), [1, 1 << 1, 0]);
    assert_eq!(seen[1].words(), [1, 0, 0]);
    assert_eq!(seen[2].words(), [1, 1 << 0, 0]);
}

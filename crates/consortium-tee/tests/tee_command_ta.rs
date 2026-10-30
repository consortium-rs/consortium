//! Runs the TA-side code `#[tee_command]` generates against a host mock of the
//! `optee_utee` 0.10 typed parameter API (`tests/support/optee_utee_mock.rs`).
//!
//! The snapshots in `consortium-tee-macros-impl` pin what the expansion *says*; this
//! checks what it *does* at the slot level — the parts a CA observes: the size it reads
//! back, the slot layout it must send, and what happens when it sends something else.
//!
//! Gated to `ta` without `ca`: the CA half references Linux-only `optee-teec`.
#![cfg(all(feature = "ta", not(feature = "ca")))]

extern crate alloc;
extern crate self as optee_utee;

include!("support/optee_utee_mock.rs");

use std::sync::atomic::{AtomicU32, Ordering};

use alloc::vec::Vec;
use consortium_codec::{Codec, CodecFor};
use consortium_tee::{TeeParam, inout, tee_command, tee_service};

/// Encodes into exactly three bytes and, like protobuf, refuses any trailing byte, so a
/// slot that reports the padded buffer instead of the written length fails to decode.
struct StrictCodec;

#[derive(Debug)]
struct CodecError;

impl Codec for StrictCodec {
    type Error = CodecError;
}

#[derive(Debug, Clone, PartialEq, TeeParam)]
#[tee(max_size = 8)]
struct Config {
    id: u8,
    limit: u16,
}

impl CodecFor<Config> for StrictCodec {
    type Decoded<'buf>
        = Config
    where
        Config: 'buf;

    fn encode(msg: &Config, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        let out = buf.get_mut(..3).ok_or(CodecError)?;
        out[0] = msg.id;
        out[1..].copy_from_slice(&msg.limit.to_le_bytes());
        Ok(3)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> core::result::Result<Self::Decoded<'buf>, Self::Error>
    where
        Config: 'buf,
    {
        let [id, lo, hi] = *buf else {
            return Err(CodecError);
        };
        Ok(Config {
            id,
            limit: u16::from_le_bytes([lo, hi]),
        })
    }
}

fn encode(config: &Config) -> Vec<u8> {
    let mut buf = [0u8; 8];
    let len = StrictCodec::encode(config, &mut buf).unwrap();
    buf[..len].to_vec()
}

static CALLS: AtomicU32 = AtomicU32::new(0);

fn calls() -> u32 {
    CALLS.load(Ordering::SeqCst)
}

#[tee_command(codec = StrictCodec)]
fn read_config(slot: u32) -> Config {
    CALLS.fetch_add(1, Ordering::SeqCst);
    Config {
        id: slot as u8,
        limit: 500,
    }
}

#[tee_command(codec = StrictCodec)]
fn adjust_config(config: &mut Config, extra: Config) {
    config.limit += extra.limit;
}

#[tee_command]
fn scalars(a: &mut u32, b: &mut i32, c: &mut u64) -> bool {
    *a += 1;
    *b -= 1;
    *c <<= 4;
    true
}

#[tee_command]
fn bytes(input: &[u8], scratch: &mut [u8]) -> Vec<u8> {
    scratch.reverse();
    input.iter().rev().copied().collect()
}

tee_service! {
    commands { read_config, adjust_config, scalars, bytes }
}

fn invoke(cmd: Command, raw: &mut [Raw; 4]) -> Result<()> {
    invoke_command(cmd as u32, &mut params(raw))
}

#[test]
fn serialized_return_reports_exactly_the_written_length() {
    let mut raw = [Raw::ValueInput(7, 0), Raw::output(8), Raw::None, Raw::None];
    invoke(Command::ReadConfig, &mut raw).unwrap();
    // The deprecated-API expansion left the size at the full 8-byte buffer, which a
    // strict codec rejects. The typed `set_output` reports the 3 bytes written.
    assert_eq!(raw[1].written().len(), 3);
    assert_eq!(
        StrictCodec::decode(raw[1].written()).unwrap(),
        Config { id: 7, limit: 500 }
    );
}

#[test]
fn oversized_return_is_short_buffer_not_a_panic() {
    let mut raw = [Raw::ValueInput(7, 0), Raw::output(2), Raw::None, Raw::None];
    let err = invoke(Command::ReadConfig, &mut raw).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ShortBuffer);
}

#[test]
fn mismatched_layout_is_rejected_before_the_handler_runs() {
    let before = calls();
    let layouts = [
        // Input value sent as an output value.
        [Raw::ValueOutput(7, 0), Raw::output(8), Raw::None, Raw::None],
        // Output memref sent as an input memref.
        [
            Raw::ValueInput(7, 0),
            Raw::MemrefInput(std::vec![0; 8]),
            Raw::None,
            Raw::None,
        ],
        // An unused slot that is not `None`.
        [
            Raw::ValueInput(7, 0),
            Raw::output(8),
            Raw::None,
            Raw::ValueInput(1, 2),
        ],
    ];
    for mut raw in layouts {
        let err = invoke(Command::ReadConfig, &mut raw).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::BadParameters);
    }
    assert_eq!(calls(), before, "handler ran despite a bad layout");
}

#[test]
fn unknown_command_is_bad_parameters() {
    let mut raw = [Raw::None, Raw::None, Raw::None, Raw::None];
    let err = invoke_command(u32::MAX, &mut params(&mut raw)).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadParameters);
}

#[test]
fn inout_serialized_round_trips_through_a_framed_padded_buffer() {
    // What the generated CA sends: a worst-case buffer holding a length-framed payload.
    let mut buf = std::vec![0u8; inout::capacity(<Config as TeeParam<StrictCodec>>::MAX_SIZE)];
    inout::write(&mut buf, &encode(&Config { id: 1, limit: 10 })).unwrap();
    let extra = encode(&Config { id: 0, limit: 5 });

    let mut raw = [
        Raw::inout(buf),
        Raw::MemrefInput(extra),
        Raw::None,
        Raw::None,
    ];
    invoke(Command::AdjustConfig, &mut raw).unwrap();

    let payload = inout::read(raw[0].written(), 8).unwrap();
    assert_eq!(
        StrictCodec::decode(payload).unwrap(),
        Config { id: 1, limit: 15 }
    );
}

#[test]
fn serialized_input_larger_than_max_size_is_rejected() {
    let oversized = std::vec![0u8; 9];
    let mut frame = std::vec![0u8; inout::capacity(8)];
    inout::write(&mut frame, &encode(&Config { id: 1, limit: 10 })).unwrap();

    let mut raw = [
        Raw::inout(frame),
        Raw::MemrefInput(oversized),
        Raw::None,
        Raw::None,
    ];
    let err = invoke(Command::AdjustConfig, &mut raw).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadParameters);
}

#[test]
fn inout_frame_declaring_more_than_max_size_is_rejected() {
    let mut buf = std::vec![0u8; inout::capacity(8)];
    buf[..4].copy_from_slice(&9u32.to_le_bytes());
    let extra = encode(&Config { id: 0, limit: 5 });

    let mut raw = [
        Raw::inout(buf),
        Raw::MemrefInput(extra),
        Raw::None,
        Raw::None,
    ];
    let err = invoke(Command::AdjustConfig, &mut raw).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadParameters);
}

#[test]
fn value_inout_scalars_are_written_back() {
    let c: u64 = 0x0000_0001_0000_0002;
    let mut raw = [
        Raw::ValueInout(41, 0),
        Raw::ValueInout((-3i32) as u32, 0),
        Raw::ValueInout(c as u32, (c >> 32) as u32),
        Raw::ValueOutput(0, 0),
    ];
    invoke(Command::Scalars, &mut raw).unwrap();
    assert_eq!(raw[0].value().0, 42);
    assert_eq!(raw[1].value().0 as i32, -4);
    let (a, b) = raw[2].value();
    assert_eq!((b as u64) << 32 | a as u64, c << 4);
    assert_eq!(raw[3].value(), (1, 0));
}

#[test]
fn byte_slices_are_copied_in_and_written_back() {
    let mut raw = [
        Raw::MemrefInput(std::vec![1, 2, 3]),
        Raw::inout(std::vec![4, 5, 6]),
        Raw::output(16),
        Raw::None,
    ];
    invoke(Command::Bytes, &mut raw).unwrap();
    assert_eq!(raw[1].written(), [6, 5, 4]);
    assert_eq!(raw[2].written(), [3, 2, 1]);
}

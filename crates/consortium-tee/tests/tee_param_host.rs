extern crate alloc;

use alloc::vec::Vec;
use consortium_codec::{Codec, CodecFor};
use consortium_tee::{TeeParam, TeeParamError, TeeParamSlot};

#[derive(Debug, Clone, PartialEq, TeeParam)]
struct Config {
    id: u8,
    limit: u16,
}

#[derive(Debug, Clone, PartialEq, TeeParam)]
struct SmallBufferPayload {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Debug, Clone, PartialEq, TeeParam)]
#[tee(max_size = 2)]
struct TooSmallMaxSize {
    a: u8,
    b: u8,
    c: u8,
}

#[derive(Debug)]
struct CodecError;

struct TestCodec;

impl Codec for TestCodec {
    type Error = CodecError;
}

impl CodecFor<Config> for TestCodec {
    type Decoded<'buf>
        = Config
    where
        Config: 'buf;

    fn encode(msg: &Config, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if msg.id == 0xFF || buf.len() < 3 {
            return Err(CodecError);
        }
        buf[0] = msg.id;
        buf[1..3].copy_from_slice(&msg.limit.to_le_bytes());
        Ok(3)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        Config: 'buf,
    {
        if buf.len() < 3 {
            return Err(CodecError);
        }
        Ok(Config {
            id: buf[0],
            limit: u16::from_le_bytes(buf[1..3].try_into().expect("limit bytes")),
        })
    }
}

impl CodecFor<SmallBufferPayload> for TestCodec {
    type Decoded<'buf>
        = SmallBufferPayload
    where
        SmallBufferPayload: 'buf;

    fn encode(msg: &SmallBufferPayload, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if msg.a == 0xFF || buf.len() < 3 {
            return Err(CodecError);
        }
        buf[..3].copy_from_slice(&[msg.a, msg.b, msg.c]);
        Ok(3)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        SmallBufferPayload: 'buf,
    {
        if buf.len() < 3 {
            return Err(CodecError);
        }
        Ok(SmallBufferPayload {
            a: buf[0],
            b: buf[1],
            c: buf[2],
        })
    }
}

impl CodecFor<TooSmallMaxSize> for TestCodec {
    type Decoded<'buf>
        = TooSmallMaxSize
    where
        TooSmallMaxSize: 'buf;

    fn encode(msg: &TooSmallMaxSize, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.len() < 3 {
            return Err(CodecError);
        }
        buf[..3].copy_from_slice(&[msg.a, msg.b, msg.c]);
        Ok(3)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        TooSmallMaxSize: 'buf,
    {
        if buf.len() < 3 {
            return Err(CodecError);
        }
        Ok(TooSmallMaxSize {
            a: buf[0],
            b: buf[1],
            c: buf[2],
        })
    }
}

#[derive(Default)]
struct FakeFrame {
    slots: [Option<TeeParamSlot<Vec<u8>>>; 4],
}

impl FakeFrame {
    fn set(&mut self, index: usize, slot: TeeParamSlot<Vec<u8>>) {
        self.slots[index] = Some(slot);
    }

    fn borrowed(&self, index: usize) -> TeeParamSlot<&[u8]> {
        match self.slots[index].as_ref().expect("slot populated") {
            TeeParamSlot::Primitive { a, b } => TeeParamSlot::Primitive { a: *a, b: *b },
            TeeParamSlot::Serialized(bytes) => TeeParamSlot::Serialized(bytes),
        }
    }
}

#[test]
fn fake_frame_round_trips_primitive_command_shape() {
    let mut frame = FakeFrame::default();
    frame.set(
        0,
        <u32 as TeeParam>::into_tee_param(&41).expect("pack ca input"),
    );

    let input = <u32 as TeeParam>::from_tee_param(frame.borrowed(0)).expect("ta input");
    let output = input + 1;
    frame.set(
        1,
        <u32 as TeeParam>::into_tee_param(&output).expect("pack ta output"),
    );

    let ca_output = <u32 as TeeParam>::from_tee_param(frame.borrowed(1)).expect("ca output");
    assert_eq!(ca_output, 42);
}

#[test]
fn fake_frame_round_trips_serialized_inout_command_shape() {
    let mut frame = FakeFrame::default();
    let input = Config { id: 7, limit: 900 };
    let inout = Config { id: 9, limit: 100 };

    frame.set(
        0,
        <Config as TeeParam<TestCodec>>::into_tee_param(&input).expect("pack input"),
    );
    frame.set(
        1,
        <Config as TeeParam<TestCodec>>::into_tee_param(&inout).expect("pack inout"),
    );

    let ta_input =
        <Config as TeeParam<TestCodec>>::from_tee_param(frame.borrowed(0)).expect("ta input");
    let mut ta_inout =
        <Config as TeeParam<TestCodec>>::from_tee_param(frame.borrowed(1)).expect("ta inout");
    ta_inout.limit += ta_input.limit;
    frame.set(
        1,
        <Config as TeeParam<TestCodec>>::into_tee_param(&ta_inout).expect("flush inout"),
    );

    let ca_inout =
        <Config as TeeParam<TestCodec>>::from_tee_param(frame.borrowed(1)).expect("ca inout");
    assert_eq!(ca_inout, Config { id: 9, limit: 1000 });
}

#[test]
fn derived_tee_param_maps_codec_encode_failure() {
    let value = SmallBufferPayload {
        a: 0xFF,
        b: 2,
        c: 3,
    };

    assert!(matches!(
        <SmallBufferPayload as TeeParam<TestCodec>>::into_tee_param(&value),
        Err(TeeParamError::EncodeFailed)
    ));
}

#[test]
fn derived_tee_param_maps_codec_decode_failure() {
    assert!(matches!(
        <SmallBufferPayload as TeeParam<TestCodec>>::from_tee_param(TeeParamSlot::Serialized(&[
            1, 2
        ])),
        Err(TeeParamError::DecodeFailed)
    ));
}

#[test]
fn derived_tee_param_max_size_bounds_encode_buffer() {
    let value = TooSmallMaxSize { a: 1, b: 2, c: 3 };

    assert_eq!(<TooSmallMaxSize as TeeParam<TestCodec>>::MAX_SIZE, 2);
    assert!(matches!(
        <TooSmallMaxSize as TeeParam<TestCodec>>::into_tee_param(&value),
        Err(TeeParamError::EncodeFailed)
    ));
}

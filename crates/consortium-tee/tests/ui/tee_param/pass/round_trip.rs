#![allow(deprecated)]
extern crate alloc;

use consortium_common::codec::{Codec, CodecFor};
use consortium_tee::{TeeParam, TeeParamSlot};

#[derive(Debug, Clone, PartialEq, TeeParam)]
struct Config {
    id: u8,
    limit: u16,
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
        if buf.len() < 3 {
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
            limit: u16::from_le_bytes([buf[1], buf[2]]),
        })
    }
}

fn main() {
    let value = Config { id: 7, limit: 900 };
    let repr = <Config as TeeParam<TestCodec>>::into_tee_param(&value).unwrap();
    let TeeParamSlot::Serialized(bytes) = repr else {
        panic!("derived TeeParam should use serialized slot");
    };
    assert_eq!(bytes, alloc::vec![7, 132, 3]);

    let decoded =
        <Config as TeeParam<TestCodec>>::from_tee_param(TeeParamSlot::Serialized(&bytes))
            .unwrap();
    assert_eq!(decoded, value);
}

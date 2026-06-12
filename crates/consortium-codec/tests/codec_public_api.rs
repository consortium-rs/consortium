#[cfg(any(
    feature = "postcard",
    feature = "prost",
    all(feature = "rkyv", feature = "alloc")
))]
use consortium_codec::CodecFor;

#[cfg(any(
    feature = "postcard",
    feature = "prost",
    all(feature = "rkyv", feature = "alloc")
))]
fn round_trip<'buf, T, C>(msg: &'buf T, buf: &'buf mut [u8]) -> C::Decoded<'buf>
where
    C: CodecFor<T>,
{
    let len = C::encode(msg, buf).expect("encode should succeed");
    C::decode(&buf[..len]).expect("decode should succeed")
}

#[cfg(feature = "postcard")]
mod postcard {
    use super::round_trip;
    use consortium_codec::postcard::PostcardCodec;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Command {
        opcode: u8,
        argument: u16,
    }

    #[test]
    fn typed_message_round_trips_through_public_trait() {
        let msg = Command {
            opcode: 3,
            argument: 0x1204,
        };
        let mut buf = [0u8; 16];

        let decoded = round_trip::<_, PostcardCodec>(&msg, &mut buf);

        assert_eq!(decoded, msg);
    }
}

#[cfg(feature = "prost")]
mod prost {
    use super::round_trip;
    use consortium_codec::prost::ProstCodec;
    use prost::Message;
    use prost::bytes::{Buf, BufMut};
    use prost::encoding::{self, DecodeContext, WireType};

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Command {
        opcode: u32,
        argument: u32,
    }

    impl Message for Command {
        fn encode_raw(&self, buf: &mut impl BufMut)
        where
            Self: Sized,
        {
            encoding::uint32::encode(1, &self.opcode, buf);
            encoding::uint32::encode(2, &self.argument, buf);
        }

        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: WireType,
            buf: &mut impl Buf,
            ctx: DecodeContext,
        ) -> Result<(), prost::DecodeError>
        where
            Self: Sized,
        {
            match tag {
                1 => encoding::uint32::merge(wire_type, &mut self.opcode, buf, ctx),
                2 => encoding::uint32::merge(wire_type, &mut self.argument, buf, ctx),
                _ => encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }

        fn encoded_len(&self) -> usize {
            encoding::uint32::encoded_len(1, &self.opcode)
                + encoding::uint32::encoded_len(2, &self.argument)
        }

        fn clear(&mut self) {
            *self = Self::default();
        }
    }

    #[test]
    fn typed_message_round_trips_through_public_trait() {
        let msg = Command {
            opcode: 3,
            argument: 0x1204,
        };
        let mut buf = [0u8; 16];

        let decoded = round_trip::<_, ProstCodec>(&msg, &mut buf);

        assert_eq!(decoded, msg);
    }
}

#[cfg(all(feature = "rkyv", feature = "alloc"))]
mod rkyv {
    use super::round_trip;
    use consortium_codec::rkyv::RkyvCodec;

    #[test]
    fn typed_message_round_trips_through_public_trait() {
        let msg = 0x1204_0003u32;
        let mut buf = [0u8; 16];

        let decoded = round_trip::<_, RkyvCodec<64>>(&msg, &mut buf);

        assert_eq!(*decoded, msg);
    }
}

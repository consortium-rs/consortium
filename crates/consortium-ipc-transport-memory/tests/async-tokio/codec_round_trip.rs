use crate::codec_support::{Message, round_trip_between_threads};
use consortium_codec::{PostcardCodec, ProstCodec, RkyvCodec};

#[test]
fn postcard_channels_exchange_messages_between_threads() {
    round_trip_between_threads::<PostcardCodec>(
        Message {
            sequence: 11,
            ready: true,
        },
        Message {
            sequence: 12,
            ready: false,
        },
    );
}

#[test]
fn prost_channels_exchange_messages_between_threads() {
    round_trip_between_threads::<ProstCodec>(
        Message {
            sequence: 21,
            ready: false,
        },
        Message {
            sequence: 22,
            ready: true,
        },
    );
}

#[test]
fn rkyv_channels_exchange_messages_between_threads() {
    round_trip_between_threads::<RkyvCodec<128>>(
        Message {
            sequence: 31,
            ready: true,
        },
        Message {
            sequence: 32,
            ready: true,
        },
    );
}

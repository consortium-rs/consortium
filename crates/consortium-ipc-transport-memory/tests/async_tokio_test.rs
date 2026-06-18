mod common;

#[path = "async-tokio/codec_support.rs"]
mod codec_support;
#[path = "async-tokio/transport_support.rs"]
mod transport_support;

#[path = "async-tokio/codec_round_trip.rs"]
mod codec_round_trip;
#[path = "async-tokio/end_to_end.rs"]
mod end_to_end;
#[path = "async-tokio/postcard_channel.rs"]
mod postcard_channel;
#[path = "async-tokio/recv.rs"]
mod recv;
#[path = "async-tokio/rkyv_channel.rs"]
mod rkyv_channel;
#[path = "async-tokio/send.rs"]
mod send;
#[path = "async-tokio/split.rs"]
mod split;

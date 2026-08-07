#![allow(
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks
)]

mod common;

#[path = "async_tokio/codec_support.rs"]
mod codec_support;
#[path = "async_tokio/transport_support.rs"]
mod transport_support;

#[path = "async_tokio/codec_round_trip.rs"]
mod codec_round_trip;
#[path = "async_tokio/connect.rs"]
mod connect;
#[path = "async_tokio/end_to_end.rs"]
mod end_to_end;
#[path = "async_tokio/endpoint.rs"]
mod endpoint;
#[path = "async_tokio/postcard_channel.rs"]
mod postcard_channel;
#[path = "async_tokio/recv.rs"]
mod recv;
#[path = "async_tokio/rkyv_channel.rs"]
mod rkyv_channel;
#[path = "async_tokio/send.rs"]
mod send;
#[path = "async_tokio/split.rs"]
mod split;

use crate::common::doorbell::HostDoorbell;
use crate::transport_support::REGION_LEN;
use consortium_codec::PostcardCodec;
use consortium_ipc::Chan;
use consortium_ipc_transport_memory::{
    Endpoint, Primary, Secondary, SharedMemoryTransport, Transceiver, channels, impl_channel,
    install,
};
use consortium_shared_memory::UncachedRegion;
use serde::{Deserialize, Serialize};

const MTU: usize = 64;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ping(u32);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pong(u32);

channels!(PRIMARY_ENDPOINT, SECONDARY_ENDPOINT);

impl_channel!(
    PRIMARY_ENDPOINT,
    Primary,
    HostDoorbell,
    UncachedRegion,
    Ping,
    Pong,
    PostcardCodec,
    MTU,
);

impl_channel!(
    SECONDARY_ENDPOINT,
    Secondary,
    HostDoorbell,
    UncachedRegion,
    Pong,
    Ping,
    PostcardCodec,
    MTU,
);

#[test]
fn endpoint_connects_before_split_and_round_trips_typed_messages() {
    let mut backing = vec![0u8; REGION_LEN].into_boxed_slice();
    let base_addr = backing.as_mut_ptr() as usize;
    let (primary_doorbell, secondary_doorbell) = HostDoorbell::pair();

    let primary = unsafe {
        SharedMemoryTransport::<Primary, _, _>::new(
            Chan::new_unchecked(0),
            Chan::new_unchecked(1),
            primary_doorbell,
            UncachedRegion::new(base_addr as *mut u8, REGION_LEN),
        )
    };
    let secondary = unsafe {
        SharedMemoryTransport::<Secondary, _, _>::new(
            Chan::new_unchecked(1),
            Chan::new_unchecked(0),
            secondary_doorbell,
            UncachedRegion::new(base_addr as *mut u8, REGION_LEN),
        )
    };

    install::<channels::PRIMARY_ENDPOINT>(primary);
    install::<channels::SECONDARY_ENDPOINT>(secondary);
    let c = Channels::take();

    std::thread::scope(|scope| {
        let primary_thread = scope.spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create primary runtime");
            runtime.block_on(async move {
                // The one-line shorthand owns the same sequence as
                // `Endpoint::new(...).connect().await` below.
                let mut endpoint = Transceiver::connect(c.PRIMARY_ENDPOINT)
                    .await
                    .expect("connect primary endpoint");
                endpoint.send(&Ping(7)).await.expect("send ping");
                let pong = endpoint.recv().await.expect("receive pong");
                assert_eq!(pong.into_inner(), Pong(8));
            });
        });

        let secondary_thread = scope.spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create secondary runtime");
            runtime.block_on(async move {
                let mut endpoint = Endpoint::new(c.SECONDARY_ENDPOINT)
                    .connect()
                    .await
                    .expect("connect secondary endpoint");
                let ping = endpoint.recv().await.expect("receive ping");
                assert_eq!(ping.into_inner(), Ping(7));
                endpoint.send(&Pong(8)).await.expect("send pong");
            });
        });

        primary_thread.join().expect("primary thread completes");
        secondary_thread.join().expect("secondary thread completes");
    });
}

#![allow(dead_code, unused, unused_unsafe, non_snake_case)]

mod shared {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub struct LowPowerData {
        pub _v: u32,
    }
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct MotionControl {
        pub _v: u32,
    }
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub struct PowerControl {
        pub _v: u32,
    }
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct SensorData {
        pub _v: u32,
    }
}

/// Descriptor-handshake failure, with the doorbell-specific payload
/// erased so a single error type spans transceivers that use
/// different doorbells. `no_std`-safe (no `alloc`).
#[derive(Debug)]
enum ConsortiumConnectError {
    /// The descriptor header/channel table was invalid or never reached
    /// the ready state.
    Descriptor(::consortium_ipc_transport_memory::DescriptorError),
    /// The doorbell signal failed during the handshake.
    Doorbell,
    /// Any other transport error during the handshake.
    Other,
}
/// IPC endpoints that have been *constructed* but not yet connected.
/// Call [`connect`](IpcMemoryEndpoints::connect) to run the descriptor
/// handshake and obtain usable transceivers.
struct IpcMemoryEndpoints {
    sensor: ::consortium_ipc_transport_memory::SharedMemoryTransport<
        ::consortium_ipc::Secondary,
        ::consortium_ipc_doorbell_mu::MuDoorbell<::consortium_ipc::Secondary, 7usize>,
        ::consortium_shared_memory::regions::SramRegion,
    >,
}
/// Connected IPC endpoints: each field is a ready-to-use transceiver.
struct IpcMemoryEndpointsConnected {
    sensor: ::consortium_ipc::Transceiver<
        crate::shared::MotionControl,
        crate::shared::SensorData,
        ::consortium_ipc_transport_memory::SharedMemoryTransportTx<
            ::consortium_ipc_transport_memory::Secondary,
            ::consortium_ipc_doorbell_mu::MuDoorbell<
                ::consortium_ipc::Secondary,
                7usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_ipc_transport_memory::SharedMemoryTransportRx<
            ::consortium_ipc_transport_memory::Secondary,
            ::consortium_ipc_doorbell_mu::MuDoorbell<
                ::consortium_ipc::Secondary,
                7usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_codec::RkyvCodec<128usize>,
    >,
}
impl IpcMemoryEndpoints {
    fn new() -> Self {
        Self {
            sensor: {
                let region = unsafe {
                    ::consortium_shared_memory::regions::SramRegion::new(
                        (541622272usize) as *mut u8,
                        16384usize,
                    )
                };
                let doorbell = unsafe {
                    ::consortium_ipc_doorbell_mu::MuDoorbell::<
                        ::consortium_ipc::Secondary,
                        7usize,
                    >::new(1111687168usize)
                };
                let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(7u8) };
                let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(6u8) };
                unsafe {
                    ::consortium_ipc_transport_memory::SharedMemoryTransport::<
                        ::consortium_ipc::Secondary,
                        _,
                        _,
                    >::new(
                        chan_tx,
                        chan_rx,
                        doorbell,
                        region,
                        (541622272usize) as *mut u8,
                    )
                }
            },
        }
    }
    /// Runs the descriptor handshake for every transceiver and returns
    /// the connected endpoints. Each transceiver's primary side writes
    /// and the secondary side acks the descriptor before any traffic.
    async fn connect(
        self,
    ) -> ::core::result::Result<IpcMemoryEndpointsConnected, ConsortiumConnectError> {
        ::core::result::Result::Ok(IpcMemoryEndpointsConnected {
            sensor: {
                let transport = self.sensor;
                let (chan_tx, chan_rx) = transport.channels();
                unsafe {
                    <::consortium_ipc_transport_memory::SharedMemoryTransport<
                        ::consortium_ipc::Secondary,
                        ::consortium_ipc_doorbell_mu::MuDoorbell<
                            ::consortium_ipc::Secondary,
                            7usize,
                        >,
                        ::consortium_shared_memory::regions::SramRegion,
                    >>::descriptor_ack(
                            transport.base(),
                            2usize,
                            128usize,
                            transport.doorbell(),
                            chan_rx,
                            chan_tx,
                        )
                        .await
                }
                    .map_err(|__e| match __e {
                        ::consortium_ipc_transport_memory::Error::Descriptor(__d) => {
                            ConsortiumConnectError::Descriptor(__d)
                        }
                        ::consortium_ipc_transport_memory::Error::Doorbell(_) => {
                            ConsortiumConnectError::Doorbell
                        }
                        _ => ConsortiumConnectError::Other,
                    })?;
                let (t_tx, t_rx) = transport.split();
                let channel_tx = ::consortium_ipc::Channel::<
                    ::consortium_ipc::Tx,
                    crate::shared::MotionControl,
                    _,
                    ::consortium_codec::RkyvCodec<128usize>,
                >::new(
                    chan_tx,
                    t_tx,
                    {
                        static mut SENSOR_RX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(SENSOR_RX_BUF) }
                    },
                );
                let channel_rx = ::consortium_ipc::Channel::<
                    ::consortium_ipc::Rx,
                    crate::shared::SensorData,
                    _,
                    ::consortium_codec::RkyvCodec<128usize>,
                >::new(
                    chan_rx,
                    t_rx,
                    {
                        static mut SENSOR_TX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(SENSOR_TX_BUF) }
                    },
                );
                ::consortium_ipc::Transceiver::new(channel_tx, channel_rx)
            },
        })
    }
}

fn main() {}

#![allow(dead_code, unused, unused_unsafe, non_snake_case)]

mod shared {
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub struct MotionControl {
        pub _v: u32,
    }
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct PowerMode {
        pub _v: u32,
    }
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct PowerStatus {
        pub _v: u32,
    }
    #[derive(::serde::Serialize, ::serde::Deserialize)]
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
        ::consortium_ipc::Primary,
        ::consortium_ipc_doorbell_hsem::HsemDoorbell<::consortium_ipc::Primary, 1usize>,
        ::consortium_shared_memory::regions::SramRegion,
    >,
    lp: ::consortium_ipc_transport_memory::SharedMemoryTransport<
        ::consortium_ipc::Primary,
        ::consortium_ipc_doorbell_ipcc::IpccDoorbell<::consortium_ipc::Primary, 2usize>,
        ::consortium_shared_memory::regions::SramRegion,
    >,
}
/// Connected IPC endpoints: each field is a ready-to-use transceiver.
struct IpcMemoryEndpointsConnected {
    sensor: ::consortium_ipc::Transceiver<
        crate::shared::SensorData,
        crate::shared::MotionControl,
        ::consortium_ipc_transport_memory::SharedMemoryTransportTx<
            ::consortium_ipc_transport_memory::Primary,
            ::consortium_ipc_doorbell_hsem::HsemDoorbell<
                ::consortium_ipc::Primary,
                1usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_ipc_transport_memory::SharedMemoryTransportRx<
            ::consortium_ipc_transport_memory::Primary,
            ::consortium_ipc_doorbell_hsem::HsemDoorbell<
                ::consortium_ipc::Primary,
                1usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_codec::PostcardCodec,
    >,
    lp: ::consortium_ipc::Transceiver<
        crate::shared::PowerStatus,
        crate::shared::PowerMode,
        ::consortium_ipc_transport_memory::SharedMemoryTransportTx<
            ::consortium_ipc_transport_memory::Primary,
            ::consortium_ipc_doorbell_ipcc::IpccDoorbell<
                ::consortium_ipc::Primary,
                2usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_ipc_transport_memory::SharedMemoryTransportRx<
            ::consortium_ipc_transport_memory::Primary,
            ::consortium_ipc_doorbell_ipcc::IpccDoorbell<
                ::consortium_ipc::Primary,
                2usize,
            >,
            ::consortium_shared_memory::regions::SramRegion,
        >,
        ::consortium_codec::RkyvCodec<128usize>,
    >,
}
impl IpcMemoryEndpoints {
    fn new(
        uio: &[::consortium_runtime_app::uio::UioDevice],
    ) -> ::core::result::Result<Self, ::consortium_runtime_app::uio::UioDeviceError> {
        ::core::result::Result::Ok(Self {
            sensor: {
                let region = unsafe {
                    ::consortium_shared_memory::regions::SramRegion::new(
                        (uio[0usize].mmap(0usize)?) as *mut u8,
                        16384usize,
                    )
                };
                let doorbell = unsafe {
                    ::consortium_ipc_doorbell_hsem::HsemDoorbell::<
                        ::consortium_ipc::Primary,
                        1usize,
                    >::new(uio[1usize].mmap(0usize)?)
                };
                uio[0usize].bind_notifier(doorbell.notifier());
                let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(0u8) };
                let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(1u8) };
                unsafe {
                    ::consortium_ipc_transport_memory::SharedMemoryTransport::<
                        ::consortium_ipc::Primary,
                        _,
                        _,
                    >::new(
                        chan_tx,
                        chan_rx,
                        doorbell,
                        region,
                        (uio[0usize].mmap(0usize)?) as *mut u8,
                    )
                }
            },
            lp: {
                let region = unsafe {
                    ::consortium_shared_memory::regions::SramRegion::new(
                        (uio[0usize].mmap(1usize)?) as *mut u8,
                        16384usize,
                    )
                };
                let doorbell = unsafe {
                    ::consortium_ipc_doorbell_ipcc::IpccDoorbell::<
                        ::consortium_ipc::Primary,
                        2usize,
                    >::new(uio[2usize].mmap(0usize)?)
                };
                uio[0usize].bind_notifier(doorbell.notifier());
                let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(0u8) };
                let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(1u8) };
                unsafe {
                    ::consortium_ipc_transport_memory::SharedMemoryTransport::<
                        ::consortium_ipc::Primary,
                        _,
                        _,
                    >::new(
                        chan_tx,
                        chan_rx,
                        doorbell,
                        region,
                        (uio[0usize].mmap(1usize)?) as *mut u8,
                    )
                }
            },
        })
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
                        ::consortium_ipc::Primary,
                        ::consortium_ipc_doorbell_hsem::HsemDoorbell<
                            ::consortium_ipc::Primary,
                            1usize,
                        >,
                        ::consortium_shared_memory::regions::SramRegion,
                    >>::descriptor_init(
                            transport.base(),
                            2u8,
                            128usize,
                            transport.doorbell(),
                            chan_tx,
                            chan_rx,
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
                    crate::shared::SensorData,
                    _,
                    ::consortium_codec::PostcardCodec,
                >::new(
                    chan_tx,
                    t_tx,
                    {
                        static mut SENSOR_TX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(SENSOR_TX_BUF) }
                    },
                );
                let channel_rx = ::consortium_ipc::Channel::<
                    ::consortium_ipc::Rx,
                    crate::shared::MotionControl,
                    _,
                    ::consortium_codec::PostcardCodec,
                >::new(
                    chan_rx,
                    t_rx,
                    {
                        static mut SENSOR_RX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(SENSOR_RX_BUF) }
                    },
                );
                ::consortium_ipc::Transceiver::new(channel_tx, channel_rx)
            },
            lp: {
                let transport = self.lp;
                let (chan_tx, chan_rx) = transport.channels();
                unsafe {
                    <::consortium_ipc_transport_memory::SharedMemoryTransport<
                        ::consortium_ipc::Primary,
                        ::consortium_ipc_doorbell_ipcc::IpccDoorbell<
                            ::consortium_ipc::Primary,
                            2usize,
                        >,
                        ::consortium_shared_memory::regions::SramRegion,
                    >>::descriptor_init(
                            transport.base(),
                            2u8,
                            128usize,
                            transport.doorbell(),
                            chan_tx,
                            chan_rx,
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
                    crate::shared::PowerStatus,
                    _,
                    ::consortium_codec::RkyvCodec<128usize>,
                >::new(
                    chan_tx,
                    t_tx,
                    {
                        static mut LP_TX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(LP_TX_BUF) }
                    },
                );
                let channel_rx = ::consortium_ipc::Channel::<
                    ::consortium_ipc::Rx,
                    crate::shared::PowerMode,
                    _,
                    ::consortium_codec::RkyvCodec<128usize>,
                >::new(
                    chan_rx,
                    t_rx,
                    {
                        static mut LP_RX_BUF: [u8; 128usize] = [0u8; 128usize];
                        unsafe { &mut *::core::ptr::addr_of_mut!(LP_RX_BUF) }
                    },
                );
                ::consortium_ipc::Transceiver::new(channel_tx, channel_rx)
            },
        })
    }
}
/// Aggregate bring-up error for [`init`].
#[derive(Debug)]
enum ConsortiumInitError {
    /// Opening or mapping a UIO device failed.
    Uio(::consortium_runtime_app::uio::UioDeviceError),
    /// An IPC transceiver's descriptor handshake failed.
    Ipc(ConsortiumConnectError),
    Dbg(::consortium_runtime_app::dbg_console::DbgConsoleError),
}
impl ::core::convert::From<::consortium_runtime_app::uio::UioDeviceError>
for ConsortiumInitError {
    fn from(e: ::consortium_runtime_app::uio::UioDeviceError) -> Self {
        ConsortiumInitError::Uio(e)
    }
}
impl ::core::convert::From<ConsortiumConnectError> for ConsortiumInitError {
    fn from(e: ConsortiumConnectError) -> Self {
        ConsortiumInitError::Ipc(e)
    }
}
impl ::core::convert::From<::consortium_runtime_app::dbg_console::DbgConsoleError>
for ConsortiumInitError {
    fn from(e: ::consortium_runtime_app::dbg_console::DbgConsoleError) -> Self {
        ConsortiumInitError::Dbg(e)
    }
}
struct ConsortiumEndpoints {
    _consortium_dbg: ::consortium_runtime_app::dbg_console::DbgConsole,
    uio: ::std::vec::Vec<::consortium_runtime_app::uio::UioDevice>,
    ipc_shm: IpcMemoryEndpointsConnected,
}
async fn init() -> ::core::result::Result<ConsortiumEndpoints, ConsortiumInitError> {
    let uio = ::std::vec![
        ::consortium_runtime_app::uio::UioDevice::open(0usize) ?,
        ::consortium_runtime_app::uio::UioDevice::open(1usize) ?,
        ::consortium_runtime_app::uio::UioDevice::open(2usize) ?,
        ::consortium_runtime_app::uio::UioDevice::open(3usize) ?
    ];
    let ipc_shm = IpcMemoryEndpoints::new(&uio)?.connect().await?;
    let _consortium_dbg = ::consortium_runtime_app::dbg_console::spawn(
        &uio,
        &[
            ::consortium_runtime_app::dbg_console::DbgRingSpec {
                uio_id: 3,
                mmap_id: 0,
                len: 8192,
                elf_path: "./firmware",
                label: "cm33",
            },
        ],
    )?;
    ::core::result::Result::Ok(ConsortiumEndpoints {
        _consortium_dbg,
        uio,
        ipc_shm,
    })
}

fn main() {}

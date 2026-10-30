#![allow(dead_code, unused, unused_unsafe, non_snake_case)]

mod shared {
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct LowPowerData {
        pub _v: u32,
    }
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub struct MotionControl {
        pub _v: u32,
    }
    #[derive(::rkyv::Archive, ::rkyv::Serialize)]
    pub struct PowerControl {
        pub _v: u32,
    }
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub struct SensorData {
        pub _v: u32,
    }
}

::consortium_ipc_transport_memory::channels!(SENSOR, LP);
::consortium_ipc_transport_memory::impl_channel!(
    SENSOR, ::consortium_ipc::Primary,
    ::consortium_ipc_transport_memory::bell::mu::MuDoorbell < ::consortium_ipc::Primary,
    1usize >, ::consortium_runtime_app::uio::UioRegion, crate ::shared::SensorData, crate
    ::shared::MotionControl, ::consortium_codec::PostcardCodec, 128usize,
);
::consortium_ipc_transport_memory::impl_channel!(
    LP, ::consortium_ipc::Primary,
    ::consortium_ipc_transport_memory::bell::mu::MuDoorbell < ::consortium_ipc::Primary,
    2usize >, ::consortium_runtime_app::uio::UioRegion, crate ::shared::LowPowerData,
    crate ::shared::PowerControl, ::consortium_codec::RkyvCodec < 128usize >, 128usize,
);
fn __consortium_install_channels(
    uio: &[::std::sync::Arc<::consortium_runtime_app::uio::UioDevice>],
) -> ::core::result::Result<(), ::consortium_runtime_app::uio::UioDeviceError> {
    ::consortium_ipc_transport_memory::install::<
        channels::SENSOR,
    >({
        let region = ::consortium_runtime_app::uio::UioRegion::zeroed(
                &uio[0usize],
                0usize,
            )?
            .keep(uio[3usize].clone());
        // SAFETY: the base is the UIO-mapped doorbell register window,
        // resolved from the validated Consortium hardware database.
        let doorbell = unsafe {
            ::consortium_ipc_transport_memory::bell::mu::MuDoorbell::<
                ::consortium_ipc::Primary,
                1usize,
            >::new_mapped(uio[3usize].mmap(0usize)?)
        };
        let __service_db = doorbell.clone();
        uio[3usize]
            .bind_service(
                ::std::sync::Arc::new(move || __service_db.service_interrupt()),
            );
        // SAFETY: doorbell channel is verified to be valid in the config.
        let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(0u8) };
        // SAFETY: doorbell channel is verified to be valid in the config.
        let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(1u8) };
        // SAFETY: the region is this channel's validated shared-memory
        // window, whose offset 0x00 holds the descriptor, and the
        // channels/doorbell are this side's.
        unsafe {
            ::consortium_ipc_transport_memory::SharedMemoryTransport::<
                ::consortium_ipc::Primary,
                _,
                _,
            >::new(chan_tx, chan_rx, doorbell, region)
        }
    });
    ::consortium_ipc_transport_memory::install::<
        channels::LP,
    >({
        let region = ::consortium_runtime_app::uio::UioRegion::zeroed(
                &uio[0usize],
                1usize,
            )?
            .keep(uio[2usize].clone());
        // SAFETY: the base is the UIO-mapped doorbell register window,
        // resolved from the validated Consortium hardware database.
        let doorbell = unsafe {
            ::consortium_ipc_transport_memory::bell::mu::MuDoorbell::<
                ::consortium_ipc::Primary,
                2usize,
            >::new_mapped(uio[2usize].mmap(0usize)?)
        };
        let __service_db = doorbell.clone();
        uio[2usize]
            .bind_service(
                ::std::sync::Arc::new(move || __service_db.service_interrupt()),
            );
        // SAFETY: doorbell channel is verified to be valid in the config.
        let chan_tx = unsafe { ::consortium_ipc::Chan::new_unchecked(4u8) };
        // SAFETY: doorbell channel is verified to be valid in the config.
        let chan_rx = unsafe { ::consortium_ipc::Chan::new_unchecked(5u8) };
        // SAFETY: the region is this channel's validated shared-memory
        // window, whose offset 0x00 holds the descriptor, and the
        // channels/doorbell are this side's.
        unsafe {
            ::consortium_ipc_transport_memory::SharedMemoryTransport::<
                ::consortium_ipc::Primary,
                _,
                _,
            >::new(chan_tx, chan_rx, doorbell, region)
        }
    });
    ::core::result::Result::Ok(())
}
/// Aggregate bring-up error for [`init`].
///
/// The variant payloads are surfaced through `Debug` in the
/// consuming binary (a downstream crate this module is `include!`d
/// into), which dead-code analysis cannot see, so allow the
/// otherwise-"never read" fields.
#[derive(Debug)]
#[allow(dead_code)]
enum ConsortiumInitError {
    /// Opening or mapping a UIO device failed.
    Uio(::consortium_runtime_app::uio::UioDeviceError),
    Dbg(::consortium_runtime_app::dbg_console::DbgConsoleError),
}
impl ::core::convert::From<::consortium_runtime_app::uio::UioDeviceError>
for ConsortiumInitError {
    fn from(e: ::consortium_runtime_app::uio::UioDeviceError) -> Self {
        ConsortiumInitError::Uio(e)
    }
}
impl ::core::convert::From<::consortium_runtime_app::dbg_console::DbgConsoleError>
for ConsortiumInitError {
    fn from(e: ::consortium_runtime_app::dbg_console::DbgConsoleError) -> Self {
        ConsortiumInitError::Dbg(e)
    }
}
/// Runtime bring-up policy. Every enabled manifest step defaults on.
#[non_exhaustive]
pub struct Config {
    pub tracing: bool,
    pub uio_irqs: bool,
    pub dbg_console: bool,
}
impl ::core::default::Default for Config {
    fn default() -> Self {
        Self {
            tracing: true,
            uio_irqs: true,
            dbg_console: true,
        }
    }
}
/// Platform resources owned by the application endpoint.
///
/// This type deliberately has no `Drop` implementation so fields can
/// be moved independently. The debug field is declared first so it
/// stops touching mappings before the UIO devices unmap.
pub struct Peripherals {
    pub dbg_console: ::core::option::Option<
        ::consortium_runtime_app::dbg_console::DbgConsole,
    >,
    pub uio: ::std::vec::Vec<::std::sync::Arc<::consortium_runtime_app::uio::UioDevice>>,
}
fn init(
    config: Config,
) -> ::core::result::Result<(Peripherals, Channels), ConsortiumInitError> {
    if config.tracing {
        ::consortium_runtime_app::init_tracing();
    }
    ::consortium_runtime_app::log::trace!(
        "consortium init: opening {} uio device(s)", 4usize
    );
    let uio = ::std::vec![
        ::std::sync::Arc::new(::consortium_runtime_app::uio::UioDevice::open_named("ipc-shm",
        541589504u64, false && config.uio_irqs,) ?),
        ::std::sync::Arc::new(::consortium_runtime_app::uio::UioDevice::open_named("dbg-shm",
        541605888u64, false && config.uio_irqs,) ?),
        ::std::sync::Arc::new(::consortium_runtime_app::uio::UioDevice::open_named("mu2",
        1111687168u64, true && config.uio_irqs,) ?),
        ::std::sync::Arc::new(::consortium_runtime_app::uio::UioDevice::open_named("mu1",
        1143078912u64, true && config.uio_irqs,) ?)
    ];
    ::consortium_runtime_app::log::trace!("consortium init: installing ipc endpoints");
    __consortium_install_channels(&uio)?;
    let channels = Channels::take();
    let dbg_console = if config.dbg_console {
        ::core::option::Option::Some(
            ::consortium_runtime_app::dbg_console::spawn(
                &uio,
                &[
                    ::consortium_runtime_app::dbg_console::DbgRingSpec {
                        uio_id: 1,
                        mmap_id: 0,
                        len: 8192,
                        elf_bytes: {
                            #[repr(align(16))]
                            struct Aligned<const N: usize>([u8; N]);
                            static ELF: Aligned<
                                { include_bytes!("../../fixtures/placeholder.elf").len() },
                            > = Aligned(
                                *include_bytes!("../../fixtures/placeholder.elf"),
                            );
                            &ELF.0
                        },
                        label: "cm33",
                    },
                ],
            )?,
        )
    } else {
        ::core::option::Option::None
    };
    ::consortium_runtime_app::log::trace!("consortium init: resources ready");
    ::core::result::Result::Ok((Peripherals { dbg_console, uio }, channels))
}
/// Error union suitable for `#[consortium_runtime_app::main]`.
///
/// `MainOutcome` reports the payload through derived `Debug`, which
/// dead-code analysis intentionally does not count as a field read.
#[derive(Debug)]
#[allow(dead_code)]
enum ConsortiumError {
    Init(ConsortiumInitError),
    Connect(::consortium_ipc_transport_memory::ConnectError),
}
impl ::core::convert::From<ConsortiumInitError> for ConsortiumError {
    fn from(e: ConsortiumInitError) -> Self {
        Self::Init(e)
    }
}
impl ::core::convert::From<::consortium_ipc_transport_memory::ConnectError>
for ConsortiumError {
    fn from(e: ::consortium_ipc_transport_memory::ConnectError) -> Self {
        Self::Connect(e)
    }
}

fn main() {}

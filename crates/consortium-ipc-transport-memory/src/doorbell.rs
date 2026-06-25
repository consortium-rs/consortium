#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_ipcc as ipcc;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_doorbell_mu as mu;

pub trait SharedMemoryTransportDoorbell {
    fn is_hardware_peripheral() -> bool;

    fn address() -> usize;
}

#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
impl SharedMemoryTransportDoorbell for hsem::HsemDoorbell {
    const fn is_hardware_peripheral() -> bool { true }
    const fn address() -> usize { hsem::HSEM_BASE as usize }
}

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
impl SharedMemoryTransportDoorbell for ipcc::IpccDoorbell {
    const fn is_hardware_peripheral() -> bool { true }
    const fn address() -> usize { ipcc::base::IPCC_BASE as usize }
}

#[cfg(any(feature = "imx93"))]
// HSEM is deprecated in favour of IPCC, but this transport still has to name it
// for the manifests that select it; the warning belongs at the manifest, not
// here.
#[allow(deprecated)]
#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_ipcc as ipcc;

#[cfg(feature = "mimx9x")]
pub use consortium_ipc_doorbell_mu as mu;

pub trait SharedMemoryTransportDoorbell {
    fn is_hardware_peripheral() -> bool;

    fn address() -> usize;
}

#[allow(deprecated)]
#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
impl<S: consortium_ipc::Side, const N: usize> SharedMemoryTransportDoorbell
    for hsem::HsemDoorbell<S, N>
{
    fn is_hardware_peripheral() -> bool {
        true
    }
    fn address() -> usize {
        hsem::registers::base::BASE[N - 1]
    }
}

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
impl<S: consortium_ipc::Side, const N: usize> SharedMemoryTransportDoorbell
    for ipcc::IpccDoorbell<S, N>
{
    fn is_hardware_peripheral() -> bool {
        true
    }
    fn address() -> usize {
        ipcc::registers::base::BASE[N - 1]
    }
}

#[cfg(feature = "mimx9x")]
impl<S: consortium_ipc::Side, const N: usize> SharedMemoryTransportDoorbell
    for mu::MuDoorbell<S, N>
{
    fn is_hardware_peripheral() -> bool {
        true
    }
    fn address() -> usize {
        mu::registers::base::BASE[N - 1]
    }
}

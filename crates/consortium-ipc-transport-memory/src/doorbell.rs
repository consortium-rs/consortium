// HSEM is deprecated in favour of IPCC, but this transport still has to name it
// for the manifests that select it; the warning belongs at the manifest, not
// here.
#[allow(deprecated)]
#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub use crate::bell::ipcc;

#[cfg(feature = "mimx9x")]
pub use crate::bell::mu;

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

impl<H: crate::bell::BellHw> SharedMemoryTransportDoorbell for crate::bell::BellA<H> {
    fn is_hardware_peripheral() -> bool {
        true
    }
    fn address() -> usize {
        H::PHYS_BASE
    }
}

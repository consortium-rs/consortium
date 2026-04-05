/// LOCKID this core writes when locking. Must match the AHB bus master ID.
/// Processor 1 (M-core) = MASTERID 1 per Table 188, STM32MP2.
#[cfg(not(feature = "std"))]
pub(crate) const M_CORE_LOCKID: u8 = 1;

/// LOCKID for the A-core (Linux side). MASTERID 2 per Table 188, STM32MP2.
#[cfg(feature = "std")]
pub(crate) const A_CORE_LOCKID: u8 = 2;

/// LOCKID this core will write when locking. Must match the AHB bus master ID.
/// Processor 1 (M-core) = MASTERID 1 per Table 188.
pub(super) const M_CORE_LOCKID: u8 = 1;

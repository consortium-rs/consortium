#[derive(Debug, PartialEq)]
pub enum MuError {
    /// Unit already locked by another master when ring() was called.
    AlreadyLocked,
    /// Chan id is out of the valid 0..=7 range. (2 MUs, each with 4 tx/rx pairs)
    ChanOutOfRange,
    /// Chan id is in the reserved range (0..=1) and cannot be used for user channels.
    ChanReserved,
}

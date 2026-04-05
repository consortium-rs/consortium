#[derive(Debug, PartialEq)]
pub enum HsemError {
    /// Semaphore already locked by another master when ring() was called.
    AlreadyLocked,
    /// Chan id is out of the valid 0..=15 range.
    ChanOutOfRange,
    /// Chan id is in the reserved range (0..=1) and cannot be used for user channels.
    ChanReserved,
}

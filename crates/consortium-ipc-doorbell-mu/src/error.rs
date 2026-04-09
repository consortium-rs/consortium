#[derive(Debug, PartialEq)]
pub enum MuError {
    /// Chan id is out of the valid range for the selected chip.
    ChanOutOfRange,
    /// Chan id is in the reserved range (0..=1) and cannot be used for user channels.
    ChanReserved,
}

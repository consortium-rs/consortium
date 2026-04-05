use consortium_ipc::chan::ChanValidate;

use crate::error::MuError;

pub struct MuChanValidate;

pub const MU_CHAN_COUNT: u8 = 8;
pub const MU_CHAN_RESERVED: u8 = 2; // chan 0 and 1 reserved for system use

impl ChanValidate for MuChanValidate {
    type Error = MuError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        match id {
            i if i < MU_CHAN_RESERVED => Err(MuError::ChanReserved),
            i if i >= MU_CHAN_COUNT => Err(MuError::ChanOutOfRange),
            _ => Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) }),
        }
    }
}

pub trait MuChanInfo {
    fn mu_id(&self) -> u8;
    fn tx_rx_id(&self) -> u8;
}

impl MuChanInfo for consortium_ipc::Chan {
    fn mu_id(&self) -> u8 {
        self.id() / 4
    }

    fn tx_rx_id(&self) -> u8 {
        self.id() % 4
    }
}

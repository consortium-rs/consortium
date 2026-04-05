use consortium_ipc::chan::ChanValidate;

use crate::hsem::error::HsemError;

pub struct HsemChanValidate;

pub const HSEM_SEM_COUNT: u8 = 16;
pub const HSEM_SEM_RESERVED: u8 = 2; // sem 0 and 1 reserved for system use

impl ChanValidate for HsemChanValidate {
    type Error = HsemError;

    fn validate(id: u8) -> Result<consortium_ipc::Chan, Self::Error> {
        match id {
            i if i < HSEM_SEM_RESERVED => Err(HsemError::ChanReserved),
            i if i >= HSEM_SEM_COUNT => Err(HsemError::ChanOutOfRange),
            _ => Ok(unsafe { consortium_ipc::Chan::new_unchecked(id) }),
        }
    }
}

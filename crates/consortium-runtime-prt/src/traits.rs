pub trait Peripheral {
    const INDEX: u16;
}

pub trait Domain {
    const INDEX: u8;
}

pub trait Role {
    const SECURE: bool;
    const PRIVILEGED: bool;
}
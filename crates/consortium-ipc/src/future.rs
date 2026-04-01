//! The MaybeSend helper trait
#[cfg(feature = "std")]
pub trait MaybeSend: Send {}
#[cfg(feature = "std")]
impl<T: Send> MaybeSend for T {}

#[cfg(not(feature = "std"))]
pub trait MaybeSend {}
#[cfg(not(feature = "std"))]
impl<T> MaybeSend for T {}

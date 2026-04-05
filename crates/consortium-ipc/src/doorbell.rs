//! Doorbell API for signaling events between cores. This is a low-level primitive used by higher-level channel and transport implementations.
use crate::{chan::Chan, future::MaybeSend};

/// Notifies the remote core that something has happened.
/// Carries no data — pure signaling primitive.
///
/// The meaning of Chan:
///   HSEM:   semaphore gate ID (0..=31)
///   MU:     transmit register index (0..=3)
///   Serial: logical channel byte in frame header
///   Host:   Unix socket / TCP port logical channel
pub trait Doorbell {
    type Error;
    type WaitFut<'a>: Future<Output = Result<Chan, Self::Error>> + MaybeSend
    where
        Self: 'a;

    /// Signal the remote core on the given channel.
    /// Sync — notification dispatch must never need to await.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error>;

    /// Await a signal from the remote core.
    /// Returns which channel fired — relevant when one
    /// Doorbell covers multiple channels (e.g. MU has 4).
    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a;

    /// Non-blocking check — has a signal arrived?
    fn pending(&self, ch: Chan) -> bool;
}

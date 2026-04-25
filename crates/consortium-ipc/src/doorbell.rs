// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Doorbell API for signaling events between cores. This is a low-level primitive used by higher-level channel and transport implementations.
use crate::{chan::Chan, future::MaybeSend};

/// Notifies the remote core that something has happened.
/// Carries no data - pure signaling primitive.
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
    /// Sync - notification dispatch must never need to await.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error>;

    /// Await a signal from the remote core.
    /// Returns which channel fired - relevant when one
    /// Doorbell covers multiple channels (e.g. MU has 4).
    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a;

    /// Non-blocking check - has a signal arrived?
    fn pending(&self, ch: Chan) -> bool;
}

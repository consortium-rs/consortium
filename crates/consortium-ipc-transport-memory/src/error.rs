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

use super::descriptor::{DescState, MAGIC};
use consortium_ipc::Doorbell;
#[cfg(feature = "std")]
use uio::UioError;

#[derive(Debug, thiserror::Error)]
pub enum DescriptorError {
    #[error("bad magic: expected 0x{:08X}, got 0x{got:08X}", MAGIC)]
    BadMagic { got: u32 },

    #[error("unsupported descriptor version: {0}")]
    BadVersion(u8),

    #[error("descriptor not ready: state is {0:?}")]
    NotReady(DescState),
}

#[derive(Debug, thiserror::Error)]
pub enum Error<D: Doorbell> {
    #[error("io: {0}")]
    #[cfg(feature = "std")]
    Io(#[from] std::io::Error),

    #[error("uio: {0:?}")]
    #[cfg(feature = "std")]
    Uio(UioError),

    #[error("descriptor: {0}")]
    Descriptor(#[from] DescriptorError),

    #[error("channel {0} out of range (device has {1})")]
    ChannelOutOfRange(u8, u8),

    #[error("doorbell error")]
    Doorbell(D::Error),

    #[error("no message to read")]
    NoMessage,

    #[error("message too large for buffer: {0} bytes, but buffer is {1} bytes")]
    MessageTooLarge(usize, usize),
}

#[cfg(feature = "std")]
impl<D: Doorbell> From<UioError> for Error<D> {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

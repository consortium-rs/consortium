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

use super::descriptor::DescState;
use consortium_ipc::Doorbell;
use consortium_shared_memory::descriptor::DescriptorHeaderError;
#[cfg(all(feature = "std", target_os = "linux"))]
use uio::UioError;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DescriptorError {
    #[error(transparent)]
    Header(#[from] DescriptorHeaderError),
    #[error("descriptor not ready: state is {0:?}")]
    NotReady(DescState),
    #[error("invalid channel count: expected {expected}, got {actual}")]
    InvalidChannelCount { expected: usize, actual: usize },
    #[error("invalid channel {index}: {reason}")]
    InvalidChannel { index: usize, reason: &'static str },
}

#[derive(Debug, thiserror::Error, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<D: Doorbell> {
    #[error("io: {0}")]
    #[cfg(feature = "std")]
    Io(#[from] std::io::Error),

    #[error("uio: {0:?}")]
    #[cfg(all(feature = "std", target_os = "linux"))]
    Uio(UioError),

    #[error("descriptor: {0}")]
    Descriptor(#[from] DescriptorError),

    #[error("channel {0} out of range (device has {1})")]
    ChannelOutOfRange(u8, u8),

    #[error("doorbell error")]
    Doorbell(D::Error),

    #[error("no available channel")]
    NoChannel,

    #[error("payload too large: {payload_len} bytes exceeds channel capacity {capacity}")]
    PayloadTooLarge { payload_len: usize, capacity: usize },

    #[error("unknown error")]
    #[default]
    Unknown,
}

#[cfg(all(feature = "std", target_os = "linux"))]
impl<D: Doorbell> From<UioError> for Error<D> {
    fn from(e: UioError) -> Self {
        Self::Uio(e)
    }
}

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

use embedded_hal::digital::{Error as HalError, ErrorKind};

/// Error raised by the GPIO doorbell.
///
/// Wraps the pin's [`embedded_hal::digital::ErrorKind`] into a stable,
/// address-space-independent type that is `Send + Sync + 'static` and
/// implements [`core::error::Error`], as required by
/// [`consortium_ipc::Doorbell::Error`]. The underlying pin error may not
/// satisfy those bounds directly, so we collapse it to its portable `kind()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GpioError(pub ErrorKind);

impl core::fmt::Display for GpioError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GPIO doorbell pin error: {}", self.0)
    }
}

impl core::error::Error for GpioError {}

// `ErrorKind` has a `defmt::Format` impl only behind embedded-hal's `defmt-03`
// feature, which targets defmt 0.3; format the kind here instead.
#[cfg(feature = "defmt")]
impl defmt::Format for GpioError {
    fn format(&self, f: defmt::Formatter<'_>) {
        match self.0 {
            ErrorKind::Other => defmt::write!(f, "GPIO doorbell pin error: other"),
            _ => defmt::write!(f, "GPIO doorbell pin error: unknown kind"),
        }
    }
}

impl<E: HalError> From<E> for GpioError {
    fn from(err: E) -> Self {
        GpioError(err.kind())
    }
}

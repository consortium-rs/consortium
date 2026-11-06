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

//! Pin multiplexing and electrical configuration.

use crate::MaybeSend;

crate::resource_id! {
    /// A pin (pad), numbered by the backend (an SCMI pin identifier on an SCMI backend).
    PinId
}

crate::resource_id! {
    /// A mux function, numbered by the backend.
    FunctionId
}

/// One electrical setting for a pin.
///
/// The common subset every backend is expected to understand, plus
/// [`Vendor`](Self::Vendor) for anything platform-specific.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum PinConfig {
    /// No pull resistor.
    BiasDisable,
    /// Enable the pull-up.
    PullUp,
    /// Enable the pull-down.
    PullDown,
    /// Open-drain output.
    OpenDrain,
    /// Push-pull output.
    PushPull,
    /// Drive strength, in the backend's unit.
    DriveStrength(u32),
    /// Slew rate, in the backend's unit.
    SlewRate(u32),
    /// Schmitt-trigger input on or off.
    InputSchmitt(bool),
    /// A platform-specific setting: `kind` and `value` are passed to the
    /// backend unchanged (an SCMI backend sends them as one type/value pair).
    Vendor {
        /// Backend-defined configuration type.
        kind: u8,
        /// Backend-defined value.
        value: u32,
    },
}

/// Muxes pins and configures their electrical properties.
pub trait PinctrlProvider {
    /// Backend error.
    type Error: core::fmt::Debug;

    /// Route `function` to `pin`.
    fn set_function(
        &mut self,
        pin: PinId,
        function: FunctionId,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;

    /// Apply every setting in `configs` to `pin`, in order.
    fn set_config(
        &mut self,
        pin: PinId,
        configs: &[PinConfig],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend;
}

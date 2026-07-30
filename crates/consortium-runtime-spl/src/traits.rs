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

//! The three partitioning markers.
//!
//! Each is const-only, so an implementor is a zero-sized marker type and its value is
//! available in `const` contexts — which is what allows a compile-time assertion about
//! an assignment rather than a runtime check that fails on hardware.

/// A peripheral, identified as the resource-isolation controller numbers it.
pub trait Peripheral {
    /// The peripheral's index in the isolation controller's numbering (RIFSC on
    /// STM32MP2, TRDC on i.MX 9).
    ///
    /// This is the controller's index, which is not the same thing as a position in
    /// the chip HAL's peripheral singleton list or an NVIC line.
    const INDEX: u16;
}

/// A security or isolation domain a peripheral can be assigned to.
pub trait Domain {
    /// The domain's index as the isolation controller numbers it.
    const INDEX: u8;
}

/// The privilege and security attributes of an assignment.
pub trait Role {
    /// Whether the assignment is to the secure world.
    ///
    /// Note that `consortium-cfg` currently rejects `[peripheral.*] secure = true`,
    /// because the chip HALs' peripheral singletons carry only the non-secure alias of
    /// each window.
    const SECURE: bool;

    /// Whether the assignment is privileged.
    const PRIVILEGED: bool;
}

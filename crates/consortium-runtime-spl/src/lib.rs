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

//! Type-level markers for partitioning: which peripheral, which security domain,
//! which privilege role.
//!
//! > **Early stage.** This crate is a placeholder for partitioning-oriented code and
//! > currently exposes only three marker traits. Nothing in the workspace consumes it
//! > yet; the shape is expected to change as the partitioning work lands.
//!
//! # What it is for
//!
//! On an STM32MP2 or i.MX 9, a peripheral is not simply "present". It is assigned to a
//! security domain and a privilege level by a resource-isolation controller — RIFSC and
//! TRDC respectively — and getting that assignment wrong produces a bus error rather
//! than a compile error.
//!
//! These traits exist so an assignment can be carried in a *type* instead of a runtime
//! number, which is the same reasoning behind
//! [`Side`](https://docs.rs/consortium-ipc/latest/consortium_ipc/trait.Side.html) in
//! `consortium-ipc` and the peripheral singletons in `consortium-hal`: a mismatch
//! becomes a build failure rather than something discovered on hardware.
//!
//! | Trait          | Carries                                                   |
//! | -------------- | --------------------------------------------------------- |
//! | [`Peripheral`] | The peripheral's index, as the isolation controller numbers it. |
//! | [`Domain`]     | The security/isolation domain index it is assigned to.      |
//! | [`Role`]       | Whether that assignment is secure, and whether privileged.   |
//!
//! All three are const-only, so an implementor is a zero-sized marker and the
//! information is available in `const` contexts — usable for compile-time assertions
//! about an assignment.
//!
//! # Relationship to the rest of the workspace
//!
//! Today the authoritative statement of peripheral ownership is `[peripheral.<name>]`
//! in `Consortium.toml`, validated by `consortium-cfg` against the per-core chip map
//! and lowered into generated bring-up code. `consortium-cfg` currently rejects
//! `secure = true` outright, because the chip HALs' singletons carry only the
//! non-secure alias.
//!
//! This crate is where a type-level encoding of the same facts would live if that
//! restriction is lifted.

pub(crate) mod traits;

pub use traits::{Domain, Peripheral, Role};

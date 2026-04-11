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

/// LOCKID this core writes when locking. Must match the AHB bus master ID.
/// Processor 1 (M-core) = MASTERID 1 per Table 188, STM32MP2.
#[cfg(not(feature = "std"))]
pub(crate) const M_CORE_LOCKID: u8 = 1;

/// LOCKID for the A-core (Linux side). MASTERID 2 per Table 188, STM32MP2.
#[cfg(feature = "std")]
pub(crate) const A_CORE_LOCKID: u8 = 2;

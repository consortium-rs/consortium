#![no_std]
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

//! Shared HAL traits for Consortium chip support crates.

/// Marker for a concrete peripheral instance.
pub trait Peripheral {
    /// Human-readable peripheral name used by generated chip metadata.
    const NAME: &'static str;
}

/// Minimal clock/reset hooks for chip-specific peripheral drivers.
pub trait ClockControl {
    /// Enable the peripheral clock.
    fn enable(&self);

    /// Reset the peripheral when the chip supports a local reset.
    fn reset(&self);
}

/// Interrupt source that can be bound by board startup code.
pub trait InterruptSource {
    /// Human-readable interrupt name.
    const INTERRUPT: &'static str;
}

/// Minimal monotonic timer abstraction shared by chip HAL time drivers.
pub trait MonotonicTimer {
    /// Return monotonically increasing ticks.
    fn now(&self) -> u64;

    /// Program the next hardware alarm.
    fn set_alarm(&self, at: u64) -> bool;
}

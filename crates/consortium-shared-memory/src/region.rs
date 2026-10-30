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

//! Memory region abstractions for transport buffers and frame buffers.

/// Describes a contiguous memory area.
/// Does not imply ownership - just describes what exists.
///
/// Implementations:
///   UncachedRegion - non-cacheable mapping, cache ops are noops
///   CachedRegion   - cacheable mapping, architecture-specific cache ops
pub trait Region {
    /// Raw pointer to the start of the region.
    ///
    /// # Safety
    /// Caller must ensure no aliasing and correct lifetime.
    unsafe fn as_ptr(&self) -> *mut u8;

    /// Length in bytes.
    fn len(&self) -> usize;

    /// Write back dirty cache lines to memory.
    /// Noop on coherent regions (SRAM).
    fn flush(&self);

    /// Invalidate stale cache lines.
    /// Noop on coherent regions (SRAM).
    fn invalidate(&self);

    /// Returns true if this region is always cache-coherent.
    /// Allows Transport impls to skip cache ops at compile time
    /// via monomorphization rather than runtime branching.
    fn is_coherent(&self) -> bool;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

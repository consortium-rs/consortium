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

//! The pull-based [`InputSource`] contract shared by every event source.

use crate::InputEvent;

/// A non-blocking, drain-style source of [`InputEvent`]s.
///
/// The contract is deliberately poll-based rather than async or
/// callback-driven so one trait serves a bare-metal frame loop and a Linux
/// host alike. A display/event-loop thread drains the source once per frame
/// and folds the events into its backend, honoring the single-threaded UI
/// rule: the UI guest is never called from a device callback or worker
/// thread, only at a frame boundary.
///
/// Expected implementors:
/// - **Linux**: a `libinput`/`evdev` reader thread pushing into a bounded
///   channel; `poll_event` pops from the channel.
/// - **Bare metal, local**: an `embedded-hal` touch/GPIO/encoder driver
///   decoded into events on the same core.
/// - **Bare metal, over IPC**: a Consortium IPC receiver whose payloads are
///   codec-decoded [`InputEvent`]s produced by a real-time core.
pub trait InputSource {
    /// Returns the next pending event, or `None` when the source is drained
    /// for this poll. Must not block.
    fn poll_event(&mut self) -> Option<InputEvent>;
}

/// An [`InputSource`] that replays a fixed slice of events once.
///
/// Useful for deterministic tests and for scripted headless playback without
/// requiring `alloc`.
pub struct SliceSource<'a> {
    remaining: core::slice::Iter<'a, InputEvent>,
}

impl<'a> SliceSource<'a> {
    /// Creates a source that yields `events` in order, then `None`.
    pub fn new(events: &'a [InputEvent]) -> Self {
        Self {
            remaining: events.iter(),
        }
    }
}

impl InputSource for SliceSource<'_> {
    fn poll_event(&mut self) -> Option<InputEvent> {
        self.remaining.next().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_source_drains_in_order_then_ends() {
        let events = [
            InputEvent::Pointer { x: 1, y: 2 },
            InputEvent::Key {
                scancode: 5,
                pressed: true,
            },
        ];
        let mut source = SliceSource::new(&events);
        assert_eq!(source.poll_event(), Some(events[0]));
        assert_eq!(source.poll_event(), Some(events[1]));
        assert_eq!(source.poll_event(), None);
        assert_eq!(source.poll_event(), None);
    }
}

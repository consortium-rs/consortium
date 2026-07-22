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

//! The portable [`InputEvent`] type and its component enums.

use consortium_ipc::IpcSafe;
use serde::{Deserialize, Serialize};

/// A semantic input event, independent of its hardware or OS source.
///
/// Positions are in **logical UI coordinates** — the same units an HMI app
/// renders at — so the *source* owns device-to-logical calibration and every
/// backend consumes one coordinate space. `i16` is deliberate: it spans any
/// realistic panel while staying a fixed, `IpcSafe` width across cores
/// (unlike `usize`).
///
/// The enum is `#[non_exhaustive]`; match with a wildcard arm so new event
/// kinds can be added without a breaking change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, IpcSafe, Deserialize, Serialize)]
#[non_exhaustive]
pub enum InputEvent {
    /// A digital button, D-pad direction, or key changed state.
    Button {
        /// Which logical control changed.
        code: ButtonCode,
        /// `true` on press, `false` on release.
        pressed: bool,
    },
    /// Absolute pointer/cursor motion, in logical UI coordinates.
    Pointer {
        /// Logical X, clamped by the source to the UI width.
        x: i16,
        /// Logical Y, clamped by the source to the UI height.
        y: i16,
    },
    /// A pointer (mouse) button changed state at the last known position.
    PointerButton {
        /// Which pointer button changed.
        button: PointerButton,
        /// `true` on press, `false` on release.
        pressed: bool,
    },
    /// A single touch contact, in logical UI coordinates.
    Touch {
        /// Stable contact identifier for the lifetime of one touch.
        id: u8,
        /// Lifecycle phase of this contact.
        phase: TouchPhase,
        /// Logical X of the contact.
        x: i16,
        /// Logical Y of the contact.
        y: i16,
    },
    /// A scroll/wheel tick along one axis, in logical detents.
    Scroll {
        /// Axis the scroll applies to.
        axis: ScrollAxis,
        /// Signed magnitude; sign follows the natural axis direction.
        delta: i16,
    },
    /// A raw keyboard scancode, for text and navigation backends.
    ///
    /// The scancode space is backend-defined (e.g. USB HID usage IDs); this
    /// crate does not enumerate keys, keeping the model small and stable.
    Key {
        /// Backend-defined scancode.
        scancode: u16,
        /// `true` on press, `false` on release.
        pressed: bool,
    },
}

/// A logical control on a gamepad-style input surface.
///
/// Face buttons use direction-neutral `South`/`East`/`West`/`North` names
/// (the standard-gamepad convention) rather than any one vendor's glyphs; a
/// backend maps them to its own button identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IpcSafe, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ButtonCode {
    /// D-pad up.
    DpadUp,
    /// D-pad down.
    DpadDown,
    /// D-pad left.
    DpadLeft,
    /// D-pad right.
    DpadRight,
    /// Bottom face button.
    South,
    /// Right face button.
    East,
    /// Left face button.
    West,
    /// Top face button.
    North,
    /// Left shoulder / trigger.
    L1,
    /// Right shoulder / trigger.
    R1,
    /// Start / menu.
    Start,
    /// Select / back.
    Select,
}

/// A pointer (mouse) button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IpcSafe, Deserialize, Serialize)]
#[non_exhaustive]
pub enum PointerButton {
    /// Primary (usually left) button.
    Left,
    /// Secondary (usually right) button.
    Right,
    /// Middle / wheel button.
    Middle,
}

/// Lifecycle phase of a touch contact.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IpcSafe, Deserialize, Serialize)]
#[non_exhaustive]
pub enum TouchPhase {
    /// Contact began.
    Down,
    /// Contact moved while held.
    Move,
    /// Contact lifted normally.
    Up,
    /// Contact aborted by the system (e.g. palm rejection).
    Cancel,
}

/// Axis a [`InputEvent::Scroll`] applies to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IpcSafe, Deserialize, Serialize)]
#[non_exhaustive]
pub enum ScrollAxis {
    /// Vertical scroll.
    Vertical,
    /// Horizontal scroll.
    Horizontal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_round_trip_through_a_codec() {
        // Proves the IPC path: an event serialized on one core decodes to an
        // identical value on another. postcard is Consortium's default codec.
        let events = [
            InputEvent::Button {
                code: ButtonCode::South,
                pressed: true,
            },
            InputEvent::Pointer { x: -12, y: 300 },
            InputEvent::PointerButton {
                button: PointerButton::Right,
                pressed: false,
            },
            InputEvent::Touch {
                id: 2,
                phase: TouchPhase::Move,
                x: 120,
                y: 64,
            },
            InputEvent::Scroll {
                axis: ScrollAxis::Vertical,
                delta: -3,
            },
            InputEvent::Key {
                scancode: 0x2C,
                pressed: true,
            },
        ];

        let mut buf = [0u8; 32];
        for event in events {
            let encoded = postcard::to_slice(&event, &mut buf).expect("encode");
            let decoded: InputEvent = postcard::from_bytes(encoded).expect("decode");
            assert_eq!(event, decoded);
        }
    }
}

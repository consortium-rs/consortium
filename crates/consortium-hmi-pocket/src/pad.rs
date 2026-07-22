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

//! Folds backend-neutral [`InputEvent`]s into the PocketJS PSP frame contract.
//!
//! PocketJS's `globalThis.frame(buttons, analog)` entry is the only input the
//! guest currently accepts, so this is the PocketJS-specific projection of the
//! shared input model. Pointer, touch, and scroll events are represented by
//! [`InputEvent`] and carried by every source, but they have no PSP frame
//! equivalent yet; [`PadState::apply`] folds button state and ignores the
//! rest until the guest's packed-touch entry is wired.

use consortium_hmi_input::{ButtonCode, InputEvent};

/// PSP button bitmask (spec `BTN`), the first argument to `globalThis.frame`.
pub(crate) mod btn {
    pub const SELECT: u32 = 0x0001;
    pub const START: u32 = 0x0008;
    pub const UP: u32 = 0x0010;
    pub const RIGHT: u32 = 0x0020;
    pub const DOWN: u32 = 0x0040;
    pub const LEFT: u32 = 0x0080;
    pub const LTRIGGER: u32 = 0x0100;
    pub const RTRIGGER: u32 = 0x0200;
    pub const TRIANGLE: u32 = 0x1000;
    pub const CIRCLE: u32 = 0x2000;
    pub const CROSS: u32 = 0x4000;
    pub const SQUARE: u32 = 0x8000;
}

/// Analog stick packed as `(x << 8) | y`, each axis `0..=255` with 128 center.
pub(crate) const ANALOG_CENTER: u32 = 0x8080;

/// Maps a backend-neutral [`ButtonCode`] to its PocketJS PSP button bit.
///
/// Face buttons follow the PSP glyph layout: `South` = Cross, `East` = Circle,
/// `West` = Square, `North` = Triangle. Codes without a PSP equivalent map to
/// `0`, so folding them is a no-op.
pub(crate) fn button_bit(code: ButtonCode) -> u32 {
    match code {
        ButtonCode::DpadUp => btn::UP,
        ButtonCode::DpadDown => btn::DOWN,
        ButtonCode::DpadLeft => btn::LEFT,
        ButtonCode::DpadRight => btn::RIGHT,
        ButtonCode::South => btn::CROSS,
        ButtonCode::East => btn::CIRCLE,
        ButtonCode::West => btn::SQUARE,
        ButtonCode::North => btn::TRIANGLE,
        ButtonCode::L1 => btn::LTRIGGER,
        ButtonCode::R1 => btn::RTRIGGER,
        ButtonCode::Start => btn::START,
        ButtonCode::Select => btn::SELECT,
        // Non-exhaustive: future codes have no PSP frame bit yet.
        _ => 0,
    }
}

/// Accumulated PSP frame input, folded from a stream of [`InputEvent`]s.
///
/// A backend or shell applies events as they arrive and reads the held state
/// at each frame boundary. Analog is reported centered until analog-axis
/// events exist in the shared model.
#[derive(Clone, Copy, Debug, Default)]
pub struct PadState {
    buttons: u32,
}

impl PadState {
    /// Creates an empty pad with no buttons held.
    pub fn new() -> Self {
        Self::default()
    }

    /// Folds one event into the held state.
    ///
    /// Button events set or clear their PSP bit; other event kinds are
    /// carried by the model but have no PSP frame equivalent yet and are
    /// ignored here.
    pub fn apply(&mut self, event: &InputEvent) {
        if let InputEvent::Button { code, pressed } = event {
            let bit = button_bit(*code);
            if *pressed {
                self.buttons |= bit;
            } else {
                self.buttons &= !bit;
            }
        }
    }

    /// The held PSP button bitmask, for `globalThis.frame`.
    pub fn buttons(&self) -> u32 {
        self.buttons
    }

    /// The packed analog value, for `globalThis.frame`.
    pub fn analog(&self) -> u32 {
        ANALOG_CENTER
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_button_press_and_release() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Button {
            code: ButtonCode::South,
            pressed: true,
        });
        assert_eq!(pad.buttons(), btn::CROSS);

        pad.apply(&InputEvent::Button {
            code: ButtonCode::DpadLeft,
            pressed: true,
        });
        assert_eq!(pad.buttons(), btn::CROSS | btn::LEFT);

        pad.apply(&InputEvent::Button {
            code: ButtonCode::South,
            pressed: false,
        });
        assert_eq!(pad.buttons(), btn::LEFT);
        assert_eq!(pad.analog(), ANALOG_CENTER);
    }

    #[test]
    fn ignores_events_without_a_psp_equivalent() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 10, y: 20 });
        pad.apply(&InputEvent::Touch {
            id: 0,
            phase: consortium_hmi_input::TouchPhase::Down,
            x: 1,
            y: 2,
        });
        assert_eq!(pad.buttons(), 0);
    }

    #[test]
    fn face_buttons_match_psp_glyph_layout() {
        assert_eq!(button_bit(ButtonCode::South), btn::CROSS);
        assert_eq!(button_bit(ButtonCode::East), btn::CIRCLE);
        assert_eq!(button_bit(ButtonCode::West), btn::SQUARE);
        assert_eq!(button_bit(ButtonCode::North), btn::TRIANGLE);
    }
}

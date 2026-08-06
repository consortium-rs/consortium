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
//! This is the PocketJS-specific projection of the shared input model onto
//! `globalThis.frame(buttons, analog, touches)`. Buttons fold into the PSP
//! bitmask; touch contacts and the pointer fold into the packed-touch third
//! argument. Scroll and raw key events are carried by [`InputEvent`] but still
//! have no frame equivalent, so [`PadState::apply`] ignores them.
//!
//! # Mice
//!
//! PocketJS has no separate pointer entry: a mouse is presented to the guest
//! as a **touch contact** at [`POINTER_CONTACT_ID`], present only while a
//! pointer button is held, plus [`btn::CIRCLE`] in the button mask for the
//! duration of the hold. That pairing is what makes a click land on the node
//! under the cursor rather than on whatever the d-pad last focused, and it
//! depends on the guest's frame ordering: `render()` runs `runFrameHooks`
//! (the app's `onFrame`) *before* `handleFrame` (focus/press edge detection),
//! so an app that focuses the hit node from `touches()` has already moved
//! focus by the time the CIRCLE edge fires `onPress` in the same frame:
//!
//! ```js
//! import { touches, hitFocusable, focusNode } from "@pocketjs/framework/input";
//! onFrame(() => {
//!   const [c] = touches();
//!   if (c) focusNode(hitFocusable(c.x, c.y));
//! });
//! ```
//!
//! The `null` hit is passed straight through on purpose. Guarding the call —
//! `if (node) focusNode(node)` — leaves focus wherever it last landed, and
//! `handleFrame` fires `onPress` on whatever is focused, so a press on empty
//! space would activate the control the pointer merely passed over earlier.
//!
//! Hover (motion with no button held) updates the stored position but emits no
//! contact, matching the touch contract, where a contact means *contact*. A
//! guest-side virtual cursor with real hover is an upstream PocketJS feature;
//! see this crate's README.
//!
//! # Frame boundaries
//!
//! The guest samples input once per frame, so a contact that opens and closes
//! between two frames would vanish under plain level sampling — at 60 Hz that
//! is any tap or click under ~16 ms, and it is the normal shape when an
//! [`InputSource`] queues events and [`frame_from`] drains the whole queue at
//! once. A contact no frame has reported yet is therefore held for exactly one
//! frame before retiring, so every tap reaches the guest exactly once.
//! [`PadState::end_frame`] performs that retirement and
//! [`PocketApp::frame_pad`] calls it.
//!
//! # Viewport limit
//!
//! The wire format carries 10 bits per axis, so coordinates above
//! [`MAX_COORD`] are not representable. [`PocketApp::new`] rejects a larger
//! logical viewport rather than clamping input onto the edge, and
//! `consortium-cfg-common` rejects it at manifest-validation time.
//!
//! [`InputSource`]: consortium_hmi_input::InputSource
//! [`frame_from`]: crate::PocketApp::frame_from
//! [`PocketApp::frame_pad`]: crate::PocketApp::frame_pad
//! [`PocketApp::new`]: crate::PocketApp::new

use consortium_hmi_input::ButtonCode;
use consortium_hmi_input::{InputEvent, PointerButton, TouchPhase};

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

/// Contacts the packed-touch frame argument carries, per `framework/src/touch.ts`.
pub const MAX_CONTACTS: usize = 8;

/// Contact id used for the synthesized mouse contact.
///
/// Reserved at the top of the 8-bit id space so it cannot collide with a real
/// panel contact, which sources number from 0.
pub const POINTER_CONTACT_ID: u8 = 0xFF;

/// Legacy packing: `(id << 18) | (y << 9) | x`, 9 bits per axis, bit 31 clear.
const LEGACY_COORD_BITS: u32 = 9;
/// Wide packing: bit 31 set, `(id << 20) | (y << 10) | x`, 10 bits per axis.
const WIDE_MARKER: u32 = 0x8000_0000;
const WIDE_COORD_BITS: u32 = 10;
/// Largest coordinate the legacy 9-bit form represents.
const LEGACY_COORD_MAX: i16 = (1 << LEGACY_COORD_BITS) - 1;
/// Largest coordinate the wide 10-bit form represents.
const WIDE_COORD_MAX: i16 = (1 << WIDE_COORD_BITS) - 1;

/// Largest logical coordinate the guest's packed-contact format addresses.
///
/// A viewport wider or taller than this cannot report pointer or touch across
/// its whole surface, so [`PocketApp::new`] rejects one rather than clamping
/// silently. Raising it requires a wider format in `framework/src/touch.ts`.
///
/// [`PocketApp::new`]: crate::PocketApp::new
pub const MAX_COORD: u32 = WIDE_COORD_MAX as u32;

/// Packs one contact into the guest's wire format.
///
/// Picks the narrowest form that holds the coordinates: the legacy 9-bit
/// packing when both fit, else the wide 10-bit form. PocketJS detects the form
/// per contact from bit 31, so a viewport under 512px stays byte-compatible
/// with PSP/Vita tapes and goldens. Coordinates are clamped, not wrapped, so an
/// out-of-range point lands on the edge instead of aliasing to the far side.
fn pack_contact(id: u8, x: i16, y: i16) -> u32 {
    let wide = x > LEGACY_COORD_MAX || y > LEGACY_COORD_MAX;
    let (bits, max, marker) = if wide {
        (WIDE_COORD_BITS, WIDE_COORD_MAX, WIDE_MARKER)
    } else {
        (LEGACY_COORD_BITS, LEGACY_COORD_MAX, 0)
    };
    let cx = x.clamp(0, max) as u32;
    let cy = y.clamp(0, max) as u32;
    marker | ((id as u32) << (bits * 2)) | (cy << bits) | cx
}

/// One live contact: a real panel touch, or the synthesized mouse contact.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Contact {
    id: u8,
    x: i16,
    y: i16,
    /// Opened since the last frame went out, so no frame has reported it yet.
    fresh: bool,
    /// Closed before the frame that opened it went out. The contact is held
    /// for that one frame and retired by [`PadState::end_frame`], so a tap
    /// shorter than a frame is not lost to level sampling.
    closing: bool,
}

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
    /// Live panel contacts, compacted toward index 0.
    touches: [Option<Contact>; MAX_CONTACTS],
    /// Last pointer position, retained across button state so a click with no
    /// intervening motion reuses it (and so a release reports where it landed).
    pointer: Option<(i16, i16)>,
    /// Whether the primary pointer button is held.
    pointer_down: bool,
}

impl PadState {
    /// Creates an empty pad with no buttons held and no contacts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Folds one event into the held state.
    ///
    /// Button events set or clear their PSP bit. Touch events open, move, and
    /// close contacts. Pointer motion updates the stored position, and a
    /// pointer button opens the synthesized [`POINTER_CONTACT_ID`] contact.
    /// Scroll and raw key events have no frame equivalent and are ignored.
    pub fn apply(&mut self, event: &InputEvent) {
        match event {
            InputEvent::Button { code, pressed } => {
                let bit = button_bit(*code);
                if *pressed {
                    self.buttons |= bit;
                } else {
                    self.buttons &= !bit;
                }
            }
            InputEvent::Pointer { x, y } => {
                self.pointer = Some((*x, *y));
                if self.pointer_down {
                    // Dragging: keep the synthesized contact under the cursor.
                    self.set_contact(POINTER_CONTACT_ID, *x, *y);
                }
            }
            InputEvent::PointerButton { button, pressed } => {
                // Only the primary button acts as a contact; secondary buttons
                // have no PocketJS equivalent and would misfire as clicks.
                if *button != PointerButton::Left {
                    return;
                }
                self.pointer_down = *pressed;
                // A press before any motion has no position to report; the
                // next Pointer event opens the contact. A release defers to
                // `clear_contact`, which holds a not-yet-reported press for one
                // frame so a sub-frame click still reaches the guest.
                match (*pressed, self.pointer) {
                    (true, Some((x, y))) => self.set_contact(POINTER_CONTACT_ID, x, y),
                    (true, None) => {}
                    (false, _) => self.clear_contact(POINTER_CONTACT_ID),
                }
            }
            InputEvent::Touch { id, phase, x, y } => match phase {
                TouchPhase::Down | TouchPhase::Move => self.set_contact(*id, *x, *y),
                TouchPhase::Up | TouchPhase::Cancel => self.clear_contact(*id),
                // Non-exhaustive: a future phase that is neither an open nor a
                // close leaves the contact set alone rather than guessing.
                _ => {}
            },
            // Non-exhaustive: carried by the model, no frame equivalent yet.
            _ => {}
        }
    }

    /// Opens or moves a contact. Silently drops the event when all
    /// [`MAX_CONTACTS`] slots are taken, matching the guest, which reads at
    /// most that many per frame.
    fn set_contact(&mut self, id: u8, x: i16, y: i16) {
        if let Some(existing) = self
            .touches
            .iter_mut()
            .flatten()
            .find(|contact| contact.id == id)
        {
            existing.x = x;
            existing.y = y;
            // Re-opening a contact that closed before its frame went out
            // cancels the pending retirement, or `end_frame` would drop a
            // contact that is live again.
            existing.closing = false;
            return;
        }
        if let Some(free) = self.touches.iter_mut().find(|slot| slot.is_none()) {
            *free = Some(Contact {
                id,
                x,
                y,
                fresh: true,
                closing: false,
            });
        }
    }

    /// Closes a contact.
    ///
    /// A contact no frame has reported yet is marked `closing` and held for one
    /// frame instead of being removed, so a tap that opens and closes between
    /// two frames still reaches the guest. Anything else is removed now.
    fn clear_contact(&mut self, id: u8) {
        let Some(at) = self
            .touches
            .iter()
            .position(|slot| matches!(slot, Some(c) if c.id == id))
        else {
            return;
        };
        if let Some(contact) = self.touches[at].as_mut()
            && contact.fresh
        {
            contact.closing = true;
            return;
        }
        self.remove_contact_at(at);
    }

    /// Removes a contact and compacts the remaining ones toward index 0, so the
    /// guest sees a dense array in a stable order.
    fn remove_contact_at(&mut self, at: usize) {
        for i in at..MAX_CONTACTS - 1 {
            self.touches[i] = self.touches[i + 1];
        }
        self.touches[MAX_CONTACTS - 1] = None;
    }

    /// The held PSP button bitmask, for `globalThis.frame`.
    ///
    /// A held primary pointer button reports as [`btn::CIRCLE`], the button
    /// PocketJS's focus manager treats as activation, so a click fires the
    /// focused node's `onPress`. See the module docs for why this pairs with
    /// the contact rather than replacing it.
    pub fn buttons(&self) -> u32 {
        // The mouse contact's presence IS the press: it is opened on the
        // primary button and retired with it, including the one-frame hold a
        // sub-frame click gets, so the two can never disagree.
        if self.contact(POINTER_CONTACT_ID).is_some() {
            self.buttons | btn::CIRCLE
        } else {
            self.buttons
        }
    }

    /// Retires the contacts the frame just delivered.
    ///
    /// A host that drives frames through [`PocketApp::frame_pad`] gets this for
    /// free. One that reads [`buttons`] and [`touches`] itself must call this
    /// after each frame, or a tap shorter than one frame stays held and repeats.
    ///
    /// [`PocketApp::frame_pad`]: crate::PocketApp::frame_pad
    /// [`buttons`]: PadState::buttons
    /// [`touches`]: PadState::touches
    pub fn end_frame(&mut self) {
        let mut i = 0;
        while i < MAX_CONTACTS {
            match self.touches[i].as_mut() {
                // Compacted, so the first empty slot ends the live set.
                None => break,
                Some(contact) if contact.closing => self.remove_contact_at(i),
                Some(contact) => {
                    contact.fresh = false;
                    i += 1;
                }
            }
        }
    }

    /// The live contact with `id`, if any.
    fn contact(&self, id: u8) -> Option<&Contact> {
        self.touches
            .iter()
            .flatten()
            .find(|contact| contact.id == id)
    }

    /// The packed analog value, for `globalThis.frame`.
    pub fn analog(&self) -> u32 {
        ANALOG_CENTER
    }

    /// Live contacts packed for the guest's third `frame` argument.
    ///
    /// Returns the dense prefix of the buffer; the tail is untouched. Empty
    /// when nothing is down, which is the guest's "no contacts" signal.
    pub fn touches(&self, out: &mut [u32; MAX_CONTACTS]) -> usize {
        let mut n = 0;
        for contact in self.touches.iter().flatten() {
            out[n] = pack_contact(contact.id, contact.x, contact.y);
            n += 1;
        }
        n
    }

    /// Last known pointer position in logical UI coordinates, if any.
    pub fn pointer(&self) -> Option<(i16, i16)> {
        self.pointer
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
    fn face_buttons_match_psp_glyph_layout() {
        assert_eq!(button_bit(ButtonCode::South), btn::CROSS);
        assert_eq!(button_bit(ButtonCode::East), btn::CIRCLE);
        assert_eq!(button_bit(ButtonCode::West), btn::SQUARE);
        assert_eq!(button_bit(ButtonCode::North), btn::TRIANGLE);
    }

    /// Reads the packed contacts as `(id, x, y)`, decoding the same way
    /// `framework/src/touch.ts` does.
    fn contacts(pad: &PadState) -> Vec<(u8, u16, u16)> {
        let mut buf = [0u32; MAX_CONTACTS];
        let n = pad.touches(&mut buf);
        buf[..n]
            .iter()
            .map(|v| {
                let wide = v & WIDE_MARKER != 0;
                let bits = if wide {
                    WIDE_COORD_BITS
                } else {
                    LEGACY_COORD_BITS
                };
                let mask = (1u32 << bits) - 1;
                (
                    ((v >> (bits * 2)) & 0xFF) as u8,
                    (v & mask) as u16,
                    ((v >> bits) & mask) as u16,
                )
            })
            .collect()
    }

    #[test]
    fn ignores_events_without_a_frame_equivalent() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Scroll {
            axis: consortium_hmi_input::ScrollAxis::Vertical,
            delta: 3,
        });
        pad.apply(&InputEvent::Key {
            scancode: 42,
            pressed: true,
        });
        assert_eq!(pad.buttons(), 0);
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn hover_reports_no_contact_until_a_button_is_held() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 10, y: 20 });
        // Motion alone is hover: position is retained, but a contact means
        // contact, so the guest sees none and no click fires.
        assert_eq!(pad.pointer(), Some((10, 20)));
        assert!(contacts(&pad).is_empty());
        assert_eq!(pad.buttons(), 0);

        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        assert_eq!(contacts(&pad), vec![(POINTER_CONTACT_ID, 10, 20)]);
        // The click must also press, or the focused node never activates.
        assert_eq!(pad.buttons(), btn::CIRCLE);
        pad.end_frame(); // the press went out on this frame

        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: false,
        });
        assert!(contacts(&pad).is_empty());
        assert_eq!(pad.buttons(), 0);
        // The release position survives for a host that reads it after the fact.
        assert_eq!(pad.pointer(), Some((10, 20)));
    }

    #[test]
    fn a_click_shorter_than_one_frame_survives_to_its_frame() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 10, y: 20 });
        // Press and release both land between two frames, which a 60 Hz shell
        // hits for any click under ~16 ms. Level sampling alone would leave
        // nothing for the frame to carry.
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: false,
        });
        assert_eq!(pad.buttons(), btn::CIRCLE);
        assert_eq!(contacts(&pad), vec![(POINTER_CONTACT_ID, 10, 20)]);

        // ...and it lasts exactly one frame, so the guest sees one click.
        pad.end_frame();
        assert_eq!(pad.buttons(), 0);
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn a_held_press_outlives_the_frame_that_reported_it() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 3, y: 4 });
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        // Retiring the press edge must not drop a button that is still down.
        pad.end_frame();
        assert_eq!(pad.buttons(), btn::CIRCLE);
        assert_eq!(contacts(&pad), vec![(POINTER_CONTACT_ID, 3, 4)]);

        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: false,
        });
        assert_eq!(pad.buttons(), 0);
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn dragging_moves_the_synthesized_contact() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 5, y: 6 });
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        pad.apply(&InputEvent::Pointer { x: 40, y: 41 });
        assert_eq!(contacts(&pad), vec![(POINTER_CONTACT_ID, 40, 41)]);
        assert_eq!(pad.buttons(), btn::CIRCLE);
    }

    #[test]
    fn secondary_pointer_buttons_do_not_click() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Pointer { x: 1, y: 2 });
        for button in [PointerButton::Right, PointerButton::Middle] {
            pad.apply(&InputEvent::PointerButton {
                button,
                pressed: true,
            });
        }
        assert_eq!(pad.buttons(), 0);
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn a_press_before_any_motion_waits_for_a_position() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        // Nowhere to put the contact yet. Reporting CIRCLE here would activate
        // whatever the d-pad last focused, at a position the host cannot name —
        // a phantom click on the wrong node — so the press waits.
        assert!(contacts(&pad).is_empty());
        assert_eq!(pad.buttons(), 0);

        // The first position opens it, and the press lands where it belongs.
        pad.apply(&InputEvent::Pointer { x: 7, y: 8 });
        assert_eq!(contacts(&pad), vec![(POINTER_CONTACT_ID, 7, 8)]);
        assert_eq!(pad.buttons(), btn::CIRCLE);
    }

    #[test]
    fn touch_contacts_open_move_and_close() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Touch {
            id: 0,
            phase: TouchPhase::Down,
            x: 1,
            y: 2,
        });
        pad.apply(&InputEvent::Touch {
            id: 1,
            phase: TouchPhase::Down,
            x: 3,
            y: 4,
        });
        assert_eq!(contacts(&pad), vec![(0, 1, 2), (1, 3, 4)]);
        pad.end_frame(); // both contacts have now been reported

        pad.apply(&InputEvent::Touch {
            id: 0,
            phase: TouchPhase::Move,
            x: 9,
            y: 9,
        });
        assert_eq!(contacts(&pad), vec![(0, 9, 9), (1, 3, 4)]);
        pad.end_frame();

        // Closing the first contact compacts the rest toward index 0.
        pad.apply(&InputEvent::Touch {
            id: 0,
            phase: TouchPhase::Up,
            x: 9,
            y: 9,
        });
        assert_eq!(contacts(&pad), vec![(1, 3, 4)]);
        pad.end_frame();

        pad.apply(&InputEvent::Touch {
            id: 1,
            phase: TouchPhase::Cancel,
            x: 3,
            y: 4,
        });
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn a_tap_shorter_than_one_frame_survives_to_its_frame() {
        let mut pad = PadState::new();
        // A panel tap whose Down and Up both land between two frames — the
        // touch counterpart of a sub-frame mouse click. This is the shape an
        // `InputSource` produces when it queues events and `frame_from` drains
        // the whole queue at once, or when rendering stalls.
        pad.apply(&InputEvent::Touch {
            id: 4,
            phase: TouchPhase::Down,
            x: 30,
            y: 40,
        });
        pad.apply(&InputEvent::Touch {
            id: 4,
            phase: TouchPhase::Up,
            x: 30,
            y: 40,
        });
        assert_eq!(contacts(&pad), vec![(4, 30, 40)]);

        // ...and it lasts exactly one frame.
        pad.end_frame();
        assert!(contacts(&pad).is_empty());
    }

    #[test]
    fn a_contact_reopened_before_its_frame_stays_live() {
        let mut pad = PadState::new();
        for phase in [TouchPhase::Down, TouchPhase::Up, TouchPhase::Down] {
            pad.apply(&InputEvent::Touch {
                id: 2,
                phase,
                x: 5,
                y: 6,
            });
        }
        // Re-opening cancels the pending retirement, or the frame boundary
        // would drop a contact the panel still reports as down.
        assert_eq!(contacts(&pad), vec![(2, 5, 6)]);
        pad.end_frame();
        assert_eq!(contacts(&pad), vec![(2, 5, 6)]);
    }

    #[test]
    fn a_mouse_and_a_panel_contact_coexist() {
        let mut pad = PadState::new();
        pad.apply(&InputEvent::Touch {
            id: 0,
            phase: TouchPhase::Down,
            x: 1,
            y: 2,
        });
        pad.apply(&InputEvent::Pointer { x: 50, y: 60 });
        pad.apply(&InputEvent::PointerButton {
            button: PointerButton::Left,
            pressed: true,
        });
        assert_eq!(
            contacts(&pad),
            vec![(0, 1, 2), (POINTER_CONTACT_ID, 50, 60)]
        );
    }

    #[test]
    fn extra_contacts_beyond_the_frame_limit_are_dropped() {
        let mut pad = PadState::new();
        for id in 0..(MAX_CONTACTS as u8 + 3) {
            pad.apply(&InputEvent::Touch {
                id,
                phase: TouchPhase::Down,
                x: id as i16,
                y: 0,
            });
        }
        assert_eq!(contacts(&pad).len(), MAX_CONTACTS);
    }

    #[test]
    fn packing_stays_legacy_until_a_coordinate_needs_the_wide_form() {
        // A 480x272 viewport is entirely inside the 9-bit legacy form, which
        // keeps PSP/Vita tapes byte-compatible.
        assert_eq!(pack_contact(3, 479, 271) & WIDE_MARKER, 0);
        assert_eq!(pack_contact(3, 479, 271), (3 << 18) | (271 << 9) | 479);

        // Past 511 on either axis the contact switches to the wide form.
        let wide = pack_contact(3, 512, 10);
        assert_ne!(wide & WIDE_MARKER, 0);
        assert_eq!(wide, WIDE_MARKER | (3 << 20) | (10 << 10) | 512);

        // Out-of-range coordinates clamp to the edge instead of aliasing.
        assert_eq!(pack_contact(0, 5000, -7), WIDE_MARKER | 1023);
    }
}

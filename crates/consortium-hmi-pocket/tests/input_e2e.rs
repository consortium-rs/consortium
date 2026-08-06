// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! End-to-end host input coverage for the PocketJS frame boundary.
//!
//! This stays headless so it can run on macOS without opening a Cocoa window or
//! requiring Accessibility permission to synthesize native keyboard events.

use consortium_hmi::Bridge;
use consortium_hmi_pocket::{
    ButtonCode, InputEvent, PocketApp, PocketConfig, PointerButton, SliceSource,
};

#[test]
fn input_source_reaches_the_pocketjs_frame_contract() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .expect("build Tokio runtime");

    let mut app = PocketApp::new(
        r#"
          globalThis.inputFrames = [];
          globalThis.frame = (buttons, analog) => {
            globalThis.inputFrames.push({ buttons, analog });
          };
        "#,
        &[],
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    )
    .expect("mount PocketJS input probe");

    // A backend-neutral source presses Cross and Left in one host frame.
    let pressed = [
        InputEvent::Button {
            code: ButtonCode::South,
            pressed: true,
        },
        InputEvent::Button {
            code: ButtonCode::DpadLeft,
            pressed: true,
        },
    ];
    app.frame_from(&mut SliceSource::new(&pressed))
        .expect("deliver pressed input frame");

    // An empty source on the next frame must retain the held button state.
    app.frame_from(&mut SliceSource::new(&[]))
        .expect("deliver held input frame");

    let released = [
        InputEvent::Button {
            code: ButtonCode::South,
            pressed: false,
        },
        InputEvent::Button {
            code: ButtonCode::DpadLeft,
            pressed: false,
        },
    ];
    app.frame_from(&mut SliceSource::new(&released))
        .expect("deliver released input frame");

    app.eval(
        "assert-input-frames",
        r#"
          if (inputFrames.length !== 3) {
            throw new Error(`expected 3 input frames, got ${inputFrames.length}`);
          }
          const held = 0x4000 | 0x0080; // PSP Cross | Left
          if (inputFrames[0].buttons !== held || inputFrames[1].buttons !== held) {
            throw new Error("button press did not remain held across frame boundaries");
          }
          if (inputFrames[2].buttons !== 0) {
            throw new Error("button release did not reach the PocketJS guest");
          }
          if (!inputFrames.every(frame => frame.analog === 0x8080)) {
            throw new Error("host without an analog source must report centered analog");
          }
        "#,
    )
    .expect("PocketJS observed the expected input frames");
}

/// The guest's own unpacking, transcribed from `framework/src/touch.ts`, so
/// this asserts against PocketJS's decoder rather than a restatement of the
/// host's encoder.
const DECODE_TOUCHES: &str = r#"
  globalThis.decode = (packed) => (packed ?? []).slice(0, 8).map((value) => {
    const wide = (value & 0x80000000) !== 0;
    const coordBits = wide ? 10 : 9;
    const coordMask = (1 << coordBits) - 1;
    const idShift = coordBits * 2;
    return {
      id: (value >>> idShift) & 0xff,
      x: value & coordMask,
      y: (value >>> coordBits) & coordMask,
    };
  });
  globalThis.frames = [];
  globalThis.frame = (buttons, analog, touches) => {
    globalThis.frames.push({ buttons, touches: decode(touches) });
  };
"#;

#[test]
fn a_mouse_reaches_the_guest_as_a_contact_and_a_press() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .expect("build Tokio runtime");

    let mut app = PocketApp::new(
        DECODE_TOUCHES,
        &[],
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    )
    .expect("mount PocketJS touch probe");

    // Hover: the guest must see the frame, but no contact and no press.
    app.apply_input(&InputEvent::Pointer { x: 120, y: 64 });
    app.frame_pad().expect("hover frame");

    // Press, drag, release — the classic click-and-drag a mouse produces.
    app.apply_input(&InputEvent::PointerButton {
        button: PointerButton::Left,
        pressed: true,
    });
    app.frame_pad().expect("press frame");

    app.apply_input(&InputEvent::Pointer { x: 300, y: 200 });
    app.frame_pad().expect("drag frame");

    app.apply_input(&InputEvent::PointerButton {
        button: PointerButton::Left,
        pressed: false,
    });
    app.frame_pad().expect("release frame");

    app.eval(
        "assert-pointer-frames",
        r#"
          const [hover, press, drag, release] = frames;
          if (frames.length !== 4) {
            throw new Error(`expected 4 frames, got ${frames.length}`);
          }
          if (hover.touches.length !== 0 || hover.buttons !== 0) {
            throw new Error("hover must not report a contact or a press");
          }
          const circle = 0x2000;
          if (press.buttons !== circle) {
            throw new Error(`click must press CIRCLE, got ${press.buttons}`);
          }
          if (press.touches.length !== 1) {
            throw new Error("click must open exactly one contact");
          }
          if (press.touches[0].id !== 0xff) {
            throw new Error(`mouse contact must use the reserved id, got ${press.touches[0].id}`);
          }
          if (press.touches[0].x !== 120 || press.touches[0].y !== 64) {
            throw new Error("click must report the position the cursor was at");
          }
          if (drag.touches[0].x !== 300 || drag.touches[0].y !== 200) {
            throw new Error("drag must move the contact with the cursor");
          }
          if (drag.buttons !== circle) {
            throw new Error("the press must stay held across a drag");
          }
          if (release.touches.length !== 0 || release.buttons !== 0) {
            throw new Error("release must close the contact and clear the press");
          }
        "#,
    )
    .expect("PocketJS observed the pointer as a contact plus a CIRCLE press");
}

#[test]
fn a_wide_viewport_packs_coordinates_the_guest_still_decodes() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .expect("build Tokio runtime");

    // A logical viewport past the legacy 9-bit range but inside the wide form.
    let mut app = PocketApp::new(
        DECODE_TOUCHES,
        &[],
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig {
            logical_size: (1000, 900),
            ..PocketConfig::default()
        },
    )
    .expect("mount PocketJS touch probe");

    app.apply_input(&InputEvent::Pointer { x: 900, y: 890 });
    app.apply_input(&InputEvent::PointerButton {
        button: PointerButton::Left,
        pressed: true,
    });
    app.frame_pad().expect("wide press frame");

    app.eval(
        "assert-wide",
        r#"
          const c = frames[0].touches[0];
          if (!c || c.x !== 900 || c.y !== 890) {
            throw new Error(`wide coordinates did not survive packing: ${JSON.stringify(c)}`);
          }
        "#,
    )
    .expect("PocketJS decoded the wide contact form");
}

#[test]
fn a_click_that_opens_and_closes_inside_one_frame_still_reaches_the_guest() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .expect("build Tokio runtime");

    let mut app = PocketApp::new(
        DECODE_TOUCHES,
        &[],
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    )
    .expect("mount PocketJS touch probe");

    app.apply_input(&InputEvent::Pointer { x: 40, y: 50 });
    // A fast click: press and release both land between two host frames, which
    // a 60 Hz shell hits whenever the click is shorter than a frame.
    app.apply_input(&InputEvent::PointerButton {
        button: PointerButton::Left,
        pressed: true,
    });
    app.apply_input(&InputEvent::PointerButton {
        button: PointerButton::Left,
        pressed: false,
    });
    app.frame_pad().expect("click frame");
    app.frame_pad().expect("settle frame");

    app.eval(
        "assert-fast-click",
        r#"
          const pressed = frames.find(f => f.buttons === 0x2000);
          if (!pressed) throw new Error("a click inside one frame never reached the guest");
          if (pressed.touches.length !== 1 || pressed.touches[0].x !== 40) {
            throw new Error("the click frame carried no contact at the cursor");
          }
          const after = frames[frames.length - 1];
          if (after.buttons !== 0 || after.touches.length !== 0) {
            throw new Error("the click never released");
          }
        "#,
    )
    .expect("PocketJS saw the fast click");
}

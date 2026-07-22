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
use consortium_hmi_pocket::{ButtonCode, InputEvent, PocketApp, PocketConfig, SliceSource};

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

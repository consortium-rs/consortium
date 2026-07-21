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

//! Exercises the PocketJS host -> retained UI -> headless wgpu render path.
//! Requires a working GPU adapter (Metal on macOS, Vulkan/GL elsewhere).

use std::path::PathBuf;

use consortium_hmi::Bridge;
use consortium_hmi_pocket::{HeadlessRenderer, PocketApp, PocketConfig};

#[test]
fn renders_a_headless_frame() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .unwrap();

    let mut app = PocketApp::new(
        "globalThis.frame = () => {};",
        &[],
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    )
    .expect("mount PocketJS app");

    app.frame_buttons(0).expect("advance one guest frame");

    let (w, h) = (64u32, 48u32);
    let mut renderer = HeadlessRenderer::new(w, h).expect("create headless GPU renderer");
    assert_eq!(renderer.size(), (w, h));

    let rgba = renderer.render_rgba(&app).expect("render retained UI");
    assert_eq!(rgba.len(), (w * h * 4) as usize);
}

/// Full pipeline: a `bun scripts/build.ts <app>` bundle+pak loaded from disk,
/// driven for a few frames, then rendered to real pixels. Skipped unless the
/// PocketJS `dist/` artifacts exist (they are produced by the JS toolchain, not
/// committed), so host CI without bun stays green.
#[test]
fn renders_a_built_bundle_to_pixels() {
    let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../submodules/pocketjs/dist")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("/nonexistent"));
    // The `-main` entry is the mounted one: its `render()`/`mount()` call
    // installs `globalThis.frame`. The bare `hero` build is only the component.
    let bundle = dist.join("hero-main.js");
    let pak = dist.join("hero-main.pak");
    if !bundle.exists() || !pak.exists() {
        eprintln!(
            "skipping: {} not built (run `bun scripts/build.ts hero-main`)",
            bundle.display()
        );
        return;
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .build()
        .unwrap();

    let mut app = PocketApp::from_files(
        &bundle,
        &pak,
        Bridge::new(),
        runtime.handle().clone(),
        PocketConfig::default(),
    )
    .expect("load built PocketJS bundle and pak");

    // Let the retained UI settle (fonts/images from the pak, layout, animation).
    for _ in 0..8 {
        app.frame_buttons(0).expect("advance guest frame");
    }

    let (w, h) = PocketConfig::default().logical_size;
    let mut renderer = HeadlessRenderer::new(w, h).expect("create headless GPU renderer");
    let rgba = renderer.render_rgba(&app).expect("render retained UI");

    assert_eq!(rgba.len(), (w * h * 4) as usize);
    // The pak carried real glyph/image content; the frame must not be uniformly
    // the clear color, or nothing from the bundle actually reached the pixels.
    let first = &rgba[0..4];
    assert!(
        rgba.chunks_exact(4).any(|px| px != first),
        "rendered frame is a single flat color; bundle/pak content did not render"
    );
}

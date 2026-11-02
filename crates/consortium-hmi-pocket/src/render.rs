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

//! CPU rasterization of a [`PocketApp`]'s retained UI.
//!
//! [`FrameRenderer`] draws the UI core's DrawList and executes it with
//! `pocket-raster` into a caller-owned framebuffer at an integer multiple of
//! the logical viewport. There is no GPU, compositor, or `DISPLAY` involved, so
//! the whole path — QuickJS evaluation, bridge dispatch, UI layout,
//! rasterization — runs in CI and produces the same pixels on every host.
//! Output can be compared against image goldens, which makes a rendering
//! regression a diff rather than a bug report.
//!
//! The `shell` feature's window host presents through the same renderer, and a
//! DRM/KMS presenter writes [`render_xrgb_into`](FrameRenderer::render_xrgb_into)
//! output into a dumb buffer, so a frame verified headlessly is the frame the
//! display shows.

use crate::PocketApp;
use crate::error::{PocketError, Result};

/// Rasterizes the retained UI into RGBA8 or XRGB8888 framebuffers.
pub struct FrameRenderer {
    scale: u32,
    /// DrawList words copied out of the core, reused across frames.
    words: Vec<u32>,
}

/// Byte order of a rasterized pixel.
#[derive(Clone, Copy)]
enum Layout {
    Rgba,
    Xrgb,
}

impl FrameRenderer {
    /// Creates a renderer that draws `scale` physical pixels per logical pixel
    /// along each axis, from 1 through [`pocket_raster::MAX_RENDER_SCALE`].
    pub fn new(scale: u32) -> Result<Self> {
        if !(1..=pocket_raster::MAX_RENDER_SCALE).contains(&scale) {
            return Err(PocketError::InvalidConfiguration(format!(
                "render scale must be from 1 through {}, got {scale}",
                pocket_raster::MAX_RENDER_SCALE
            )));
        }
        Ok(Self {
            scale,
            words: Vec::new(),
        })
    }

    /// Physical pixels per logical pixel, per axis.
    pub fn scale(&self) -> u32 {
        self.scale
    }

    /// Physical framebuffer size for `app`: its logical size times the scale.
    pub fn size(&self, app: &PocketApp) -> (u32, u32) {
        let (width, height) = app.logical_size();
        (width * self.scale, height * self.scale)
    }

    /// Renders the current retained UI into a new tightly packed RGBA8 buffer.
    pub fn render_rgba(&mut self, app: &PocketApp) -> Result<Vec<u8>> {
        let (width, height) = self.size(app);
        let mut frame = vec![0; width as usize * height as usize * 4];
        self.render_rgba_into(app, &mut frame)?;
        Ok(frame)
    }

    /// Renders RGBA8 into `frame`, which must hold exactly
    /// [`size`](Self::size) pixels.
    pub fn render_rgba_into(&mut self, app: &PocketApp, frame: &mut [u8]) -> Result<()> {
        self.render_into(app, frame, Layout::Rgba)
    }

    /// Renders into `frame` as little-endian XRGB8888 words (B, G, R, X bytes),
    /// the layout of a KMS dumb buffer and of a `softbuffer` surface. `frame`
    /// must hold exactly [`size`](Self::size) pixels.
    pub fn render_xrgb_into(&mut self, app: &PocketApp, frame: &mut [u8]) -> Result<()> {
        self.render_into(app, frame, Layout::Xrgb)
    }

    fn render_into(&mut self, app: &PocketApp, frame: &mut [u8], layout: Layout) -> Result<()> {
        let (width, height) = self.size(app);
        let expected = width as usize * height as usize * 4;
        if frame.len() != expected {
            return Err(PocketError::InvalidConfiguration(format!(
                "framebuffer holds {} bytes, {width}x{height} needs {expected}",
                frame.len()
            )));
        }
        let (scale, words) = (self.scale, &mut self.words);
        app.with_ui(|ui| {
            words.clear();
            words.extend_from_slice(&ui.draw().words);
            match layout {
                Layout::Rgba => pocket_raster::render_scaled(&*ui, words, frame, scale),
                Layout::Xrgb => pocket_raster::render_scaled_argb(&*ui, words, frame, scale),
            }
        })
    }
}

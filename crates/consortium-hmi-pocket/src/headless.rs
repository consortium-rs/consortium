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

use std::path::Path;

use anyhow::Result;
use pocket_ui_wgpu::UiRenderer;
use pocket3d::gpu::{Gpu, OFFSCREEN_FORMAT, OffscreenTarget};
use pocket3d::wgpu;

use crate::PocketApp;

/// Offscreen wgpu target for PocketJS smoke tests and image goldens.
pub struct HeadlessRenderer {
    gpu: Gpu,
    target: OffscreenTarget,
    renderer: UiRenderer,
}

impl HeadlessRenderer {
    /// Selects a headless adapter and creates an RGBA render target.
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let gpu = Gpu::new_headless()?;
        let target = OffscreenTarget::new(&gpu, width, height);
        let renderer = UiRenderer::new(&gpu, OFFSCREEN_FORMAT);
        Ok(Self {
            gpu,
            target,
            renderer,
        })
    }

    /// Renders the current retained UI and reads back tightly packed RGBA8.
    pub fn render_rgba(&mut self, app: &PocketApp) -> Result<Vec<u8>> {
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("consortium-pocket-headless"),
            });
        app.ui().with_ui(|ui| {
            self.renderer.render(
                &self.gpu,
                ui,
                &mut encoder,
                &self.target.view,
                self.target.size,
                wgpu::LoadOp::Clear(wgpu::Color::BLACK),
            )
        })?;
        self.gpu.queue.submit([encoder.finish()]);
        self.target.read_rgba(&self.gpu)
    }

    /// Renders and writes a PNG for a golden or diagnostic artifact.
    pub fn save_png(&mut self, app: &PocketApp, path: impl AsRef<Path>) -> Result<()> {
        let rgba = self.render_rgba(app)?;
        pocket3d::gpu::save_png(path.as_ref(), self.target.size.0, self.target.size.1, &rgba)
    }

    /// Physical size of the offscreen target.
    pub fn size(&self) -> (u32, u32) {
        self.target.size
    }
}

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

//! Windowed [`winit`] shell for [`PocketApp`].
//!
//! This is the desktop-oriented counterpart to [`HeadlessRenderer`]: it opens a
//! native window (Wayland or X11 on Linux, Cocoa on macOS — winit selects the
//! backend at runtime), drives the guest once per redraw, renders the retained
//! UI into an offscreen target at the app's logical size, and blits that to the
//! swapchain with letterboxing so the result is DPI- and resolution-independent.
//!
//! It is deliberately a thin bring-up shell, not the production kiosk: there is
//! no DRM/KMS leasing, seat/`libinput` routing, or config-driven surface setup
//! yet. Those build on the same [`PocketApp::frame`] contract this drives.
//!
//! [`HeadlessRenderer`]: crate::HeadlessRenderer

use std::sync::Arc;

use pocket_ui_wgpu::{Blit, UiRenderer};
use pocket3d::gpu::{Gpu, OFFSCREEN_FORMAT, OffscreenTarget};
use pocket3d::wgpu;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::PocketApp;
use crate::error::{PocketError, Result};

/// PSP button bitmask (spec `BTN`), the first argument to `globalThis.frame`.
mod btn {
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
const ANALOG_CENTER: u32 = 0x8080;

/// Window and presentation options for [`run`].
#[derive(Clone, Debug)]
pub struct ShellConfig {
    /// Window title.
    pub title: String,
    /// Initial logical window size. The surface may differ (HiDPI, resize);
    /// the UI is scaled to fit regardless.
    pub size: (u32, u32),
    /// Allow the window to be resized.
    pub resizable: bool,
    /// Draw window chrome (title bar, borders).
    pub decorations: bool,
    /// Cap presentation to the display refresh rate.
    pub vsync: bool,
    /// Smooth the UI upscale (`true`) instead of nearest-neighbor (`false`).
    pub smooth_scaling: bool,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            title: "Consortium PocketJS".into(),
            size: (960, 544),
            resizable: true,
            decorations: true,
            vsync: true,
            smooth_scaling: true,
        }
    }
}

/// Opens a window and runs `app` until the window closes or a frame fails.
///
/// This takes over the calling thread with the winit event loop, which most
/// platforms require to own the main thread. Build the `app` (and its Tokio
/// runtime) first, then hand it here.
pub fn run(app: PocketApp, config: ShellConfig) -> Result<()> {
    let event_loop =
        EventLoop::new().map_err(|e| PocketError::host("creating winit event loop", e))?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut shell = Shell {
        app,
        config,
        render: None,
        buttons: 0,
        error: None,
    };
    event_loop
        .run_app(&mut shell)
        .map_err(|e| PocketError::host("running winit event loop", e))?;
    match shell.error.take() {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

/// GPU resources bound to the live window surface.
struct RenderState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    gpu: Gpu,
    ui: UiRenderer,
    offscreen: OffscreenTarget,
    blit: Blit,
    logical: (u32, u32),
}

struct Shell {
    app: PocketApp,
    config: ShellConfig,
    render: Option<RenderState>,
    buttons: u32,
    error: Option<PocketError>,
}

impl Shell {
    /// Records a fatal error and asks the event loop to unwind.
    fn fail(&mut self, event_loop: &ActiveEventLoop, error: PocketError) {
        self.error = Some(error);
        event_loop.exit();
    }

    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<RenderState> {
        let attrs = Window::default_attributes()
            .with_title(self.config.title.clone())
            .with_inner_size(LogicalSize::new(self.config.size.0, self.config.size.1))
            .with_resizable(self.config.resizable)
            .with_decorations(self.config.decorations);
        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .map_err(|e| PocketError::host("creating window", e))?,
        );

        let instance = Gpu::new_instance();
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| PocketError::host("creating window surface", e))?;
        let gpu = Gpu::from_instance_for_surface(instance, &surface)
            .map_err(|e| PocketError::host("acquiring GPU for surface", e))?;

        let px = window.inner_size();
        let mut config = surface
            .get_default_config(&gpu.adapter, px.width.max(1), px.height.max(1))
            .ok_or_else(|| {
                PocketError::host(
                    "configuring window surface",
                    "adapter does not support this surface",
                )
            })?;
        config.present_mode = if self.config.vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        surface.configure(&gpu.device, &config);

        let logical = self.app.logical_size();
        let ui = UiRenderer::new(&gpu, OFFSCREEN_FORMAT);
        let offscreen = OffscreenTarget::new(&gpu, logical.0, logical.1);
        let filter = if self.config.smooth_scaling {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        };
        // Opaque copy: the offscreen frame already carries the UI's own
        // background, so no alpha blending against the swapchain is needed.
        let blit = Blit::new(&gpu, &offscreen.view, config.format, filter, false);

        window.request_redraw();
        Ok(RenderState {
            window,
            surface,
            config,
            gpu,
            ui,
            offscreen,
            blit,
            logical,
        })
    }

    fn draw(&mut self) -> Result<()> {
        let Some(state) = self.render.as_mut() else {
            return Ok(());
        };

        // Advance the guest: pumps completed bridge commands, runs one JS frame
        // with the held buttons, and ticks the retained UI core.
        self.app.frame(self.buttons, ANALOG_CENTER)?;

        let mut encoder =
            state
                .gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("consortium-pocket-shell"),
                });

        // Pass 1: render the retained UI into the logical-size offscreen target.
        self.app
            .ui()
            .with_ui(|ui| {
                state.ui.render(
                    &state.gpu,
                    ui,
                    &mut encoder,
                    &state.offscreen.view,
                    state.offscreen.size,
                    wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                )
            })
            .map_err(|e| PocketError::host("rendering retained PocketJS UI", e))?;

        // Pass 2: blit the offscreen frame onto the swapchain, letterboxed.
        let frame = match state.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                state.surface.configure(&state.gpu.device, &state.config);
                state.window.request_redraw();
                return Ok(());
            }
            Err(other) => {
                return Err(PocketError::host(
                    "acquiring swapchain frame",
                    other.to_string(),
                ));
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("consortium-pocket-blit"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            let (x, y, w, h) = letterbox((state.config.width, state.config.height), state.logical);
            pass.set_viewport(x, y, w, h, 0.0, 1.0);
            state.blit.draw(&mut pass);
        }

        state.gpu.queue.submit([encoder.finish()]);
        state.window.pre_present_notify();
        frame.present();
        Ok(())
    }
}

impl ApplicationHandler for Shell {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.render.is_some() {
            return;
        }
        match self.init(event_loop) {
            Ok(state) => self.render = Some(state),
            Err(error) => self.fail(event_loop, error),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(state) = self.render.as_mut() {
                    state.config.width = size.width.max(1);
                    state.config.height = size.height.max(1);
                    state.surface.configure(&state.gpu.device, &state.config);
                    state.window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                if code == KeyCode::Escape && state == ElementState::Pressed {
                    event_loop.exit();
                    return;
                }
                if let Some(bit) = map_button(code) {
                    match state {
                        ElementState::Pressed => self.buttons |= bit,
                        ElementState::Released => self.buttons &= !bit,
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.draw() {
                    self.fail(event_loop, error);
                    return;
                }
                // Drive continuous animation; present pacing bounds the rate.
                if let Some(state) = self.render.as_ref() {
                    state.window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

/// Centered, aspect-preserving viewport for a `logical` frame inside `surface`.
fn letterbox(surface: (u32, u32), logical: (u32, u32)) -> (f32, f32, f32, f32) {
    let (sw, sh) = (surface.0 as f32, surface.1 as f32);
    let (lw, lh) = (logical.0.max(1) as f32, logical.1.max(1) as f32);
    let scale = (sw / lw).min(sh / lh);
    let (w, h) = (lw * scale, lh * scale);
    ((sw - w) * 0.5, (sh - h) * 0.5, w, h)
}

/// Maps a physical key to a PSP button bit, mirroring the web host's remap.
fn map_button(code: KeyCode) -> Option<u32> {
    Some(match code {
        KeyCode::ArrowUp | KeyCode::KeyW => btn::UP,
        KeyCode::ArrowDown | KeyCode::KeyS => btn::DOWN,
        KeyCode::ArrowLeft | KeyCode::KeyA => btn::LEFT,
        KeyCode::ArrowRight | KeyCode::KeyD => btn::RIGHT,
        KeyCode::KeyX => btn::CIRCLE,
        KeyCode::KeyZ => btn::CROSS,
        KeyCode::KeyC => btn::SQUARE,
        KeyCode::KeyV => btn::TRIANGLE,
        KeyCode::KeyQ => btn::LTRIGGER,
        KeyCode::KeyE => btn::RTRIGGER,
        KeyCode::Enter => btn::START,
        KeyCode::ShiftRight => btn::SELECT,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letterbox_preserves_aspect_and_centers() {
        // 2:1 logical into a 4:1 surface -> pillarboxed, full height, centered.
        let (x, y, w, h) = letterbox((800, 200), (480, 240));
        assert_eq!((w, h), (400.0, 200.0));
        assert_eq!((x, y), (200.0, 0.0));
    }

    #[test]
    fn button_map_covers_face_and_dpad() {
        assert_eq!(map_button(KeyCode::KeyX), Some(btn::CIRCLE));
        assert_eq!(map_button(KeyCode::ArrowLeft), Some(btn::LEFT));
        assert_eq!(map_button(KeyCode::KeyA), Some(btn::LEFT));
        assert_eq!(map_button(KeyCode::Space), None);
    }
}

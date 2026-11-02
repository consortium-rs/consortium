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
//! This is the desktop-oriented counterpart to a DRM/KMS presenter: it opens a
//! native window (Wayland or X11 on Linux, Cocoa on macOS — winit selects the
//! backend at runtime), drives the guest once per 60 Hz tick, rasterizes the
//! retained UI with [`FrameRenderer`] at the largest integer scale that fits,
//! and copies it into a [`softbuffer`] surface with letterboxing, so the result
//! is DPI- and resolution-independent and pixel-identical to the headless path.
//!
//! It is deliberately a thin bring-up shell, not the production kiosk: there is
//! no DRM/KMS leasing, seat/`libinput` routing, or config-driven surface setup
//! yet. Those build on the same [`PocketApp::frame`] contract this drives.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};

use consortium_hmi_input::{ButtonCode, InputEvent, PointerButton, TouchPhase};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{
    ElementState, KeyEvent, MouseButton, Touch, TouchPhase as WinitTouchPhase, WindowEvent,
};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::error::{PocketError, Result};
use crate::{FrameRenderer, PocketApp, TICK_HZ};

/// Time between guest frames. softbuffer exposes no vsync, so the shell paces
/// itself at the core's tick rate.
const FRAME_INTERVAL: Duration = Duration::from_nanos(1_000_000_000 / TICK_HZ as u64);

/// Window options for [`run`].
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
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            title: "Consortium PocketJS".into(),
            size: (960, 544),
            resizable: true,
            decorations: true,
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
        next_frame: Instant::now(),
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

/// Presentation resources bound to the live window.
struct RenderState {
    window: Arc<Window>,
    surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    /// Physical surface size.
    size: (u32, u32),
    logical: (u32, u32),
    renderer: FrameRenderer,
    /// The rasterized UI at `renderer`'s scale, XRGB8888.
    frame: Vec<u8>,
}

struct Shell {
    app: PocketApp,
    config: ShellConfig,
    render: Option<RenderState>,
    next_frame: Instant,
    error: Option<PocketError>,
}

impl Shell {
    /// Records a fatal error and asks the event loop to unwind.
    fn fail(&mut self, event_loop: &ActiveEventLoop, error: PocketError) {
        self.error = Some(error);
        event_loop.exit();
    }

    /// Converts a physical window position into logical UI coordinates,
    /// inverting the letterbox. Returns `None` before the surface exists.
    fn to_logical(&self, position: PhysicalPosition<f64>) -> Option<(i16, i16)> {
        let state = self.render.as_ref()?;
        Some(physical_to_logical(
            state.size,
            state.logical,
            position.x as f32,
            position.y as f32,
        ))
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
        let context = softbuffer::Context::new(window.clone()).map_err(|e| {
            PocketError::host("connecting softbuffer to the display", e.to_string())
        })?;
        let surface = softbuffer::Surface::new(&context, window.clone())
            .map_err(|e| PocketError::host("creating window surface", e.to_string()))?;

        let logical = self.app.logical_size();
        let mut state = RenderState {
            window,
            surface,
            size: (0, 0),
            logical,
            renderer: FrameRenderer::new(1)?,
            frame: Vec::new(),
        };
        let px = state.window.inner_size();
        state.resize((px.width, px.height))?;
        state.window.request_redraw();
        Ok(state)
    }

    fn draw(&mut self) -> Result<()> {
        let Some(state) = self.render.as_mut() else {
            return Ok(());
        };

        // Advance the guest: pumps completed bridge commands, runs one JS frame
        // with the held input state, and ticks the retained UI core.
        self.app.frame_pad()?;

        state
            .renderer
            .render_xrgb_into(&self.app, &mut state.frame)?;
        let frame_size = state.renderer.size(&self.app);
        let mut buffer = state
            .surface
            .buffer_mut()
            .map_err(|e| PocketError::host("acquiring window buffer", e.to_string()))?;
        blit(
            &state.frame,
            frame_size,
            &mut buffer,
            state.size,
            letterbox(state.size, state.logical),
        );
        state.window.pre_present_notify();
        buffer
            .present()
            .map_err(|e| PocketError::host("presenting window buffer", e.to_string()))
    }
}

impl RenderState {
    /// Resizes the surface and picks the largest integer raster scale whose
    /// frame fits it, so text stays sharp and only the final copy scales.
    fn resize(&mut self, size: (u32, u32)) -> Result<()> {
        let width = NonZeroU32::new(size.0).unwrap_or(NonZeroU32::MIN);
        let height = NonZeroU32::new(size.1).unwrap_or(NonZeroU32::MIN);
        let size = (width.get(), height.get());
        self.surface
            .resize(width, height)
            .map_err(|e| PocketError::host("resizing window surface", e.to_string()))?;
        self.size = size;

        let fit = (size.0 / self.logical.0).min(size.1 / self.logical.1);
        let scale = fit.clamp(1, pocket_raster::MAX_RENDER_SCALE);
        if scale != self.renderer.scale() || self.frame.is_empty() {
            self.renderer = FrameRenderer::new(scale)?;
            let pixels = (self.logical.0 * scale) as usize * (self.logical.1 * scale) as usize;
            self.frame = vec![0; pixels * 4];
        }
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
                    if let Err(error) = state.resize((size.width, size.height)) {
                        self.fail(event_loop, error);
                        return;
                    }
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
                if let Some(button) = map_button(code) {
                    self.app.apply_input(&InputEvent::Button {
                        code: button,
                        pressed: state == ElementState::Pressed,
                    });
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                // Pointer/touch events are carried by the shared input model
                // but have no PSP frame equivalent yet, so the pad ignores
                // them yet; translating them here exercises the source-side
                // logical-coordinate calibration the model expects and is
                // ready for the guest's packed-touch entry.
                if let Some((x, y)) = self.to_logical(position) {
                    self.app.apply_input(&InputEvent::Pointer { x, y });
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(button) = map_pointer_button(button) {
                    self.app.apply_input(&InputEvent::PointerButton {
                        button,
                        pressed: state == ElementState::Pressed,
                    });
                }
            }
            WindowEvent::Touch(Touch {
                phase,
                location,
                id,
                ..
            }) => {
                if let Some((x, y)) = self.to_logical(location) {
                    self.app.apply_input(&InputEvent::Touch {
                        id: id as u8,
                        phase: map_touch_phase(phase),
                        x,
                        y,
                    });
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.draw() {
                    self.fail(event_loop, error);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let Some(state) = self.render.as_ref() else {
            return;
        };
        // Drive continuous animation at the tick rate.
        let now = Instant::now();
        if now >= self.next_frame {
            state.window.request_redraw();
            self.next_frame = (self.next_frame + FRAME_INTERVAL).max(now);
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
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

/// Copies `src` (XRGB8888 bytes, `src_size` pixels) into the `dst` surface,
/// nearest-neighbor scaled into `viewport` and black outside it. softbuffer
/// wants the top byte of each word zero, so the alpha byte is masked off.
fn blit(
    src: &[u8],
    src_size: (u32, u32),
    dst: &mut [u32],
    dst_size: (u32, u32),
    viewport: (f32, f32, f32, f32),
) {
    dst.fill(0);
    let (x0, y0, w, h) = viewport;
    let (dw, dh) = (dst_size.0 as usize, dst_size.1 as usize);
    let (sw, sh) = (src_size.0 as usize, src_size.1 as usize);
    let columns = (x0.round().max(0.0) as usize)..((x0 + w).round() as usize).min(dw);
    let rows = (y0.round().max(0.0) as usize)..((y0 + h).round() as usize).min(dh);
    let source_column =
        |dx: usize| ((((dx as f32 + 0.5 - x0) / w) * sw as f32) as usize).min(sw - 1);
    let map: Vec<usize> = columns.clone().map(source_column).collect();
    for dy in rows {
        let sy = ((((dy as f32 + 0.5 - y0) / h) * sh as f32) as usize).min(sh - 1);
        let src_row = &src[sy * sw * 4..(sy + 1) * sw * 4];
        let dst_row = &mut dst[dy * dw + columns.start..dy * dw + columns.end];
        for (pixel, &sx) in dst_row.iter_mut().zip(&map) {
            let bytes = [src_row[sx * 4], src_row[sx * 4 + 1], src_row[sx * 4 + 2], 0];
            *pixel = u32::from_le_bytes(bytes);
        }
    }
}

/// Inverts [`letterbox`]: maps a physical surface point to logical UI
/// coordinates, clamped to the logical bounds. Points outside the letterboxed
/// viewport clamp to the nearest edge.
fn physical_to_logical(surface: (u32, u32), logical: (u32, u32), px: f32, py: f32) -> (i16, i16) {
    let (x_off, y_off, w, h) = letterbox(surface, logical);
    let (lw, lh) = (logical.0.max(1) as f32, logical.1.max(1) as f32);
    let lx = ((px - x_off) / w * lw).clamp(0.0, lw - 1.0);
    let ly = ((py - y_off) / h * lh).clamp(0.0, lh - 1.0);
    (lx as i16, ly as i16)
}

/// Maps a physical key to a neutral [`ButtonCode`], mirroring the web host's
/// remap. Face keys follow the PSP glyph layout (X = Circle/`East`,
/// Z = Cross/`South`, C = Square/`West`, V = Triangle/`North`).
fn map_button(code: KeyCode) -> Option<ButtonCode> {
    Some(match code {
        KeyCode::ArrowUp | KeyCode::KeyW => ButtonCode::DpadUp,
        KeyCode::ArrowDown | KeyCode::KeyS => ButtonCode::DpadDown,
        KeyCode::ArrowLeft | KeyCode::KeyA => ButtonCode::DpadLeft,
        KeyCode::ArrowRight | KeyCode::KeyD => ButtonCode::DpadRight,
        KeyCode::KeyX => ButtonCode::East,
        KeyCode::KeyZ => ButtonCode::South,
        KeyCode::KeyC => ButtonCode::West,
        KeyCode::KeyV => ButtonCode::North,
        KeyCode::KeyQ => ButtonCode::L1,
        KeyCode::KeyE => ButtonCode::R1,
        KeyCode::Enter => ButtonCode::Start,
        KeyCode::ShiftRight => ButtonCode::Select,
        _ => return None,
    })
}

/// Maps a winit mouse button to the neutral [`PointerButton`] set.
fn map_pointer_button(button: MouseButton) -> Option<PointerButton> {
    Some(match button {
        MouseButton::Left => PointerButton::Left,
        MouseButton::Right => PointerButton::Right,
        MouseButton::Middle => PointerButton::Middle,
        _ => return None,
    })
}

/// Maps a winit touch phase to the neutral [`TouchPhase`].
fn map_touch_phase(phase: WinitTouchPhase) -> TouchPhase {
    match phase {
        WinitTouchPhase::Started => TouchPhase::Down,
        WinitTouchPhase::Moved => TouchPhase::Move,
        WinitTouchPhase::Ended => TouchPhase::Up,
        WinitTouchPhase::Cancelled => TouchPhase::Cancel,
    }
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
    fn blit_scales_into_the_letterbox_and_clears_the_bars() {
        // One red and one blue logical pixel (B, G, R, A bytes) into a 4x1
        // surface whose viewport covers columns 1..3.
        let src = [0, 0, 255, 255, 255, 0, 0, 255];
        let mut dst = [0xdead_beef; 4];
        blit(&src, (2, 1), &mut dst, (4, 1), (1.0, 0.0, 2.0, 1.0));
        assert_eq!(dst, [0, 0x00ff_0000, 0x0000_00ff, 0]);
    }

    #[test]
    fn button_map_covers_face_and_dpad() {
        assert_eq!(map_button(KeyCode::KeyX), Some(ButtonCode::East));
        assert_eq!(map_button(KeyCode::ArrowLeft), Some(ButtonCode::DpadLeft));
        assert_eq!(map_button(KeyCode::KeyA), Some(ButtonCode::DpadLeft));
        assert_eq!(map_button(KeyCode::Space), None);
    }

    #[test]
    fn physical_to_logical_inverts_the_letterbox() {
        // 800x200 surface, 480x240 logical -> 400x200 viewport at x=200,y=0.
        // Viewport center maps to logical center.
        let (x, y) = physical_to_logical((800, 200), (480, 240), 400.0, 100.0);
        assert_eq!((x, y), (240, 120));
        // Top-left of the viewport maps to the logical origin.
        assert_eq!(
            physical_to_logical((800, 200), (480, 240), 200.0, 0.0),
            (0, 0)
        );
        // Points left of the pillarbox clamp to the left edge.
        assert_eq!(physical_to_logical((800, 200), (480, 240), 0.0, 100.0).0, 0);
    }
}

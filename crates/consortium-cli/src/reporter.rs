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

//! The sticky step prompt shared by every long-running subcommand.
//!
//! A cross-compile, a BitBake run, and a registry download all produce a lot of
//! output and take a long time, which is exactly when "which stage am I in?"
//! stops being obvious. [`StickyReporter`] keeps one status line pinned beneath
//! that output on a terminal, redrawing it after each chunk, and degrades to
//! plain sequential lines when stderr is not a TTY (a CI log).

use std::io::{self, IsTerminal, Write};
use std::sync::Mutex;

use consortium_builder::{BuildReporter, BuildStep};

#[derive(Clone, Copy)]
enum StepStatus {
    Running,
    Done,
    Failed,
}

struct CurrentStep {
    index: usize,
    total: usize,
    name: String,
    detail: Option<String>,
    status: StepStatus,
}

struct ReporterState {
    current: Option<CurrentStep>,
    pending_output: String,
}

pub struct StickyReporter {
    state: Mutex<ReporterState>,
    interactive: bool,
    color: bool,
}

impl StickyReporter {
    pub fn new() -> Self {
        let interactive = io::stderr().is_terminal();
        let color = interactive
            && std::env::var_os("NO_COLOR").is_none()
            && std::env::var("TERM")
                .map(|term| term != "dumb")
                .unwrap_or(true);
        Self {
            state: Mutex::new(ReporterState {
                current: None,
                pending_output: String::new(),
            }),
            interactive,
            color,
        }
    }

    /// Clears the prompt and reports the run succeeded, e.g. "Build complete".
    pub fn finish_success(&self, message: &str) {
        self.finish("ok", message, "32;1");
    }

    /// Clears the prompt and reports the run failed, e.g. "Build failed". The
    /// caller still prints the error itself; this only closes out the prompt.
    pub fn finish_failure(&self, message: &str) {
        self.finish("x", message, "31;1");
    }

    fn finish(&self, mark: &str, message: &str, color: &str) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        self.flush_pending(&mut state, &mut stderr);
        state.current = None;
        let _ = writeln!(stderr, "{} {}", self.paint(color, mark), message);
        let _ = stderr.flush();
    }

    fn update_step(&self, step: BuildStep<'_>, status: StepStatus) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        self.flush_pending(&mut state, &mut stderr);

        state.current = Some(CurrentStep {
            index: step.index,
            total: step.total,
            name: step.name.to_string(),
            detail: step.detail.map(str::to_string),
            status,
        });

        if let Some(current) = state.current.as_ref() {
            let prompt = self.render_prompt(current);
            if self.interactive {
                let _ = write!(stderr, "{prompt}");
            } else {
                let _ = writeln!(stderr, "{prompt}");
            }
        }
        let _ = stderr.flush();
    }

    fn flush_pending(&self, state: &mut ReporterState, stderr: &mut impl Write) {
        if state.pending_output.is_empty() {
            return;
        }
        let pending = std::mem::take(&mut state.pending_output);
        let _ = write!(stderr, "{pending}");
        if !pending.ends_with('\n') && !pending.ends_with('\r') {
            let _ = writeln!(stderr);
        }
    }

    fn drain_complete_output(&self, state: &mut ReporterState) -> String {
        let Some(end) = state
            .pending_output
            .char_indices()
            .filter_map(|(idx, ch)| (ch == '\n' || ch == '\r').then_some(idx + ch.len_utf8()))
            .next_back()
        else {
            return String::new();
        };

        let complete = state.pending_output[..end].to_string();
        state.pending_output.replace_range(..end, "");
        normalize_line_endings(&complete)
    }

    fn render_prompt(&self, current: &CurrentStep) -> String {
        let (status, color) = match current.status {
            StepStatus::Running => ("running", "36;1"),
            StepStatus::Done => ("done", "32;1"),
            StepStatus::Failed => ("failed", "31;1"),
        };
        let mut prompt = format!(
            "{} {} {} {}",
            self.paint("36;1", "> consortium"),
            self.paint("33", format!("[{}/{}]", current.index, current.total)),
            self.paint(color, status),
            self.paint("1", &current.name),
        );
        if let Some(detail) = &current.detail {
            prompt.push(' ');
            prompt.push_str(&self.paint("2", detail));
        }
        prompt
    }

    fn paint(&self, color: &str, text: impl AsRef<str>) -> String {
        let text = text.as_ref();
        if self.color {
            format!("\x1b[{color}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }
}

impl BuildReporter for StickyReporter {
    fn step_started(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Running);
    }

    fn step_succeeded(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Done);
    }

    fn step_failed(&self, step: BuildStep<'_>) {
        self.update_step(step, StepStatus::Failed);
    }

    fn command_output(&self, output: &str) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        state.pending_output.push_str(output);
        let complete = self.drain_complete_output(&mut state);
        if complete.is_empty() {
            return;
        }

        let mut stderr = io::stderr().lock();
        if self.interactive {
            let _ = write!(stderr, "\r\x1b[2K");
        }
        let _ = write!(stderr, "{complete}");
        if self.interactive
            && let Some(current) = state.current.as_ref()
        {
            let _ = write!(stderr, "\r\x1b[2K{}", self.render_prompt(current));
        }
        let _ = stderr.flush();
    }
}

fn normalize_line_endings(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut previous_was_cr = false;
    for ch in text.chars() {
        match ch {
            '\r' => {
                normalized.push('\n');
                previous_was_cr = true;
            }
            '\n' if previous_was_cr => {
                previous_was_cr = false;
            }
            _ => {
                normalized.push(ch);
                previous_was_cr = false;
            }
        }
    }
    normalized
}

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

//! Build progress reporting hooks used by the CLI.
//!
//! The builder crate keeps these hooks deliberately small: library callers can
//! ignore them and retain inherited subprocess output, while the CLI can pass a
//! reporter that keeps a persistent step prompt under cargo/vp/bitbake output.

/// A single executable build step.
#[derive(Clone, Copy, Debug)]
pub struct BuildStep<'a> {
    /// One-based position of this step in the executable plan.
    pub index: usize,
    /// Total executable steps in this run.
    pub total: usize,
    /// Short stage name.
    pub name: &'a str,
    /// Optional target or artifact detail.
    pub detail: Option<&'a str>,
}

/// Receives progress events and streamed child-process output.
pub trait BuildReporter: Send + Sync {
    /// A step is about to run.
    fn step_started(&self, step: BuildStep<'_>);

    /// A step completed successfully.
    fn step_succeeded(&self, step: BuildStep<'_>);

    /// A step failed.
    fn step_failed(&self, step: BuildStep<'_>);

    /// Output emitted by a subprocess belonging to the current step.
    fn command_output(&self, output: &str);
}

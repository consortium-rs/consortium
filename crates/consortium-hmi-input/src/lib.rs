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

#![cfg_attr(not(test), no_std)]

//! Backend-neutral input event model for Consortium HMI runtimes.
//!
//! HMI backends (`consortium-hmi-pocket`, and later a WebKit adapter) and the
//! event *sources* that drive them speak different concrete APIs — winit on a
//! desktop, `libinput`/`evdev` on a Linux kiosk, `embedded-hal` drivers on a
//! bare-metal core, or a Consortium IPC channel carrying events from a
//! real-time core. This crate is the neutral middle: a single [`InputEvent`]
//! type expressed in **logical UI coordinates**, plus a pull-based
//! [`InputSource`] contract that every source implements.
//!
//! [`InputEvent`] derives [`consortium_ipc::IpcSafe`] and `serde`, so a
//! real-time core can serialize events with a Consortium codec and forward
//! them across IPC to a Linux HMI host exactly like any other cross-core
//! message. The type is `no_std` and allocation-free, so the same definition
//! compiles for firmware and for the Linux host.

mod event;
mod source;

pub use event::{ButtonCode, InputEvent, PointerButton, ScrollAxis, TouchPhase};
pub use source::{InputSource, SliceSource};

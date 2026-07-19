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
//! Backend-neutral HMI service bridge.
//!
//! Rendering backends live in adapter crates such as
//! `consortium-hmi-webkit` and `consortium-hmi-pocket`. This crate owns only
//! command registration, invocation parsing, and response futures, so product
//! services do not depend on a browser or native UI runtime.

pub mod bridge;

pub use bridge::{Bridge, Dispatch, DispatchError, ResponseFuture};

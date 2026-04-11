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
#![no_std]
#![cfg(target_os = "linux")]
#![no_main]

extern crate optee_utee;

use consortium_tee_test::invoke_command;
use optee_utee::{Parameters, Result, ta_invoke_command};

#[ta_invoke_command]
fn invoker(command_id: u32, params: &mut Parameters) -> Result<()> {
    invoke_command(command_id, params)
}

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));

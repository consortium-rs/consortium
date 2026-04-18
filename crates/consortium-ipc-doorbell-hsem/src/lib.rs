#![cfg_attr(not(feature = "std"), no_std)]
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
#![allow(unused)] // Because this is a library crate, and some items are only used in certain contexts (e.g. std vs no_std).

pub mod chan;
pub mod consts;
pub mod doorbell;
pub mod error;
pub mod registers;

pub use chan::HsemChanValidate;
pub use doorbell::HsemDoorbell;
pub use error::HsemError;

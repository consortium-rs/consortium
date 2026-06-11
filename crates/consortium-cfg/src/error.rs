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

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum RpiError {
    #[error("The peripheral '{0}' not exists")]
    PeripheralNotFound(String),

    #[error("The pin '{0}' is invalid")]
    InvalidPin(String),

    #[error(
        "The pin '{0}' is not available for peripheral '{1}' with function '{2}'. Allowed pins: {3:?}"
    )]
    PinNotAvailable(String, String, String, Vec<String>),
}

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

#[cfg(feature = "alloc")]
extern crate alloc;

pub(crate) mod chan;
pub(crate) mod channel;
pub(crate) mod doorbell;
pub(crate) mod future;
pub(crate) mod sealed;
pub(crate) mod side;
pub(crate) mod transceiver;
pub(crate) mod transport;
pub(crate) mod wire;

pub use {
    chan::{Chan, ChanValidate},
    channel::{Channel, ChannelError, Direction, ReceivedMessage, Rx, Tx},
    doorbell::Doorbell,
    future::MaybeSend,
    side::{Primary, Secondary, Side, Tertiary},
    transceiver::Transceiver,
    transport::{RecvTransport, SendTransport, Transport, TransportError},
    wire::IpcSafe,
};

pub use consortium_ipc_macros::IpcSafe;

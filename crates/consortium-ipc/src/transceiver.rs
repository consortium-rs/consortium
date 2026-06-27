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

//! Typed, multidirectional transceiver built on top of two [`Channel`]s.

use consortium_codec::CodecFor;

use crate::{Channel, ReceivedMessage, Rx, Transport, Tx};

/// A typed, directed communication endpoint over a transport half.
///
/// # Type parameters
///
/// - `T`  - message type
/// - `Tr` - send half ([`SendTransport`]) for `Tx`, receive half ([`RecvTransport`]) for `Rx`
/// - `C`  - codec, defaults to [`PostcardCodec`]
pub struct Transceiver<TyTx, TyRx, Tr, C: CodecFor<TyTx> + CodecFor<TyRx>> {
    tx: Channel<Tx, TyTx, Tr, C>,
    rx: Channel<Rx, TyRx, Tr, C>,
}

impl<TyTx, TyRx, Tr: Transport, C: CodecFor<TyTx> + CodecFor<TyRx>> Transceiver<TyTx, TyRx, Tr, C> {
    /// Create a new [`Transceiver`] from a send half and a receive half.
    pub fn new(tx: Channel<Tx, TyTx, Tr, C>, rx: Channel<Rx, TyRx, Tr, C>) -> Self {
        Self { tx, rx }
    }

    /// Get the send half of the transceiver.
    pub fn tx(&self) -> &Channel<Tx, TyTx, Tr, C> {
        &self.tx
    }

    /// Get the receive half of the transceiver.
    pub fn rx(&self) -> &Channel<Rx, TyRx, Tr, C> {
        &self.rx
    }

    /// send message
    pub async fn send(&mut self, msg: &TyTx) -> Result<(), crate::ChannelError<C, Tr>> {
        self.tx.send(msg).await
    }

    /// receive message
    pub async fn recv(
        &mut self,
    ) -> Result<ReceivedMessage<'_, TyRx, C>, crate::ChannelError<C, Tr>> {
        self.rx.recv().await
    }

    /// split into two channels
    pub fn split(self) -> (Channel<Tx, TyTx, Tr, C>, Channel<Rx, TyRx, Tr, C>) {
        (self.tx, self.rx)
    }
}

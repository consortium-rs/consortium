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

use crate::transport::{RecvTransport, SendTransport};
use crate::{Channel, ChannelError, ReceivedMessage, Rx, Tx};

/// A typed, multidirectional endpoint pairing a send half and a receive half.
///
/// The two halves carry independent transport types: the full-duplex
/// [`SharedMemoryTransport`](consortium_ipc_transport_memory::SharedMemoryTransport)
/// is `split` into a `…Tx` send half and a `…Rx` receive half, and each is the
/// transport of its respective [`Channel`].
///
/// # Type parameters
///
/// - `TyTx` / `TyRx` - the sent and received message types
/// - `TrTx` - send-half transport ([`SendTransport`])
/// - `TrRx` - receive-half transport ([`RecvTransport`])
/// - `C`  - codec
pub struct Transceiver<TyTx, TyRx, TrTx, TrRx, C: CodecFor<TyTx> + CodecFor<TyRx>> {
    tx: Channel<Tx, TyTx, TrTx, C>,
    rx: Channel<Rx, TyRx, TrRx, C>,
}

impl<TyTx, TyRx, TrTx, TrRx, C: CodecFor<TyTx> + CodecFor<TyRx>>
    Transceiver<TyTx, TyRx, TrTx, TrRx, C>
{
    /// Create a new [`Transceiver`] from a send half and a receive half.
    pub fn new(tx: Channel<Tx, TyTx, TrTx, C>, rx: Channel<Rx, TyRx, TrRx, C>) -> Self {
        Self { tx, rx }
    }

    /// Get the send half of the transceiver.
    pub fn tx(&self) -> &Channel<Tx, TyTx, TrTx, C> {
        &self.tx
    }

    /// Get the receive half of the transceiver.
    pub fn rx(&self) -> &Channel<Rx, TyRx, TrRx, C> {
        &self.rx
    }

    /// split into two channels
    #[allow(clippy::type_complexity)]
    pub fn split(self) -> (Channel<Tx, TyTx, TrTx, C>, Channel<Rx, TyRx, TrRx, C>) {
        (self.tx, self.rx)
    }
}

impl<TyTx, TyRx, TrTx, TrRx, C> Transceiver<TyTx, TyRx, TrTx, TrRx, C>
where
    TrTx: SendTransport,
    C: CodecFor<TyTx> + CodecFor<TyRx>,
{
    /// send message
    pub async fn send(&mut self, msg: &TyTx) -> Result<(), ChannelError<C, TrTx>> {
        self.tx.send(msg).await
    }
}

impl<TyTx, TyRx, TrTx, TrRx, C> Transceiver<TyTx, TyRx, TrTx, TrRx, C>
where
    TrRx: RecvTransport,
    C: CodecFor<TyTx> + CodecFor<TyRx>,
{
    /// receive message
    pub async fn recv(&mut self) -> Result<ReceivedMessage<'_, TyRx, C>, ChannelError<C, TrRx>> {
        self.rx.recv().await
    }
}

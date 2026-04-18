#![cfg(target_os = "linux")]
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

pub mod mmap;
pub mod remoteproc;

use consortium_common::codec::CodecFor;
pub use consortium_ipc as ipc;
pub use consortium_ipc_transport_uio as ipc_uio;
pub use consortium_tee as tee;

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(feature = "imx9x")]
pub use consortium_ipc_doorbell_mu as mu;

/// Open a UIO device and create a typed channel for receiving messages.
///
/// # Arguments
///
/// * `uio_num` — the UIO device number (N in `/dev/uioN`)
/// * `chan_id` — the channel ID
/// * `buf` — static scratch buffer for message payload
///
/// # Returns
///
/// A `Channel<Rx, T, UioChannel>` ready for receiving messages.
///
/// # Errors
///
/// Returns an error if the UIO device cannot be opened or the channel ID is invalid.
///
/// # Example
///
/// ```ignore
/// let buf = static_buf!(MY_BUF, 512);
/// let rx: Channel<Rx, MyMessage, _> = open_rx_channel(0, Chan(1), buf.init([0u8; 512]))?;
/// ```
pub async fn open_rx_channel<T, const N: usize, C>(
    uio_num: usize,
    chan: ipc::Chan,
    buf: &'static mut [u8; N],
) -> Result<ipc::Channel<ipc::channel::Rx, T, ipc_uio::UioChannel, C>, Box<dyn std::error::Error>>
where
    C: CodecFor<T>,
{
    // Call wait_for_ready before opening if you need to wait for M-core initialization.
    // Example: mmap::wait_for_ready(uio_num, Duration::from_secs(10)).await?;

    let dev = ipc_uio::UioDevice::open(uio_num)?;
    let transport = dev.channel(chan)?;
    let ch = ipc::Channel::new(chan, transport, buf);
    Ok(ch)
}

/// Open a UIO device and create a typed channel for sending messages.
///
/// # Arguments
///
/// * `uio_num` — the UIO device number (N in `/dev/uioN`)
/// * `chan_id` — the channel ID
/// * `buf` — static scratch buffer for message payload
///
/// # Returns
///
/// A `Channel<Tx, T, UioChannel>` ready for sending messages.
///
/// # Errors
///
/// Returns an error if the UIO device cannot be opened or the channel ID is invalid.
pub async fn open_tx_channel<T, const N: usize, C>(
    uio_num: usize,
    chan: ipc::Chan,
    buf: &'static mut [u8; N],
) -> Result<ipc::Channel<ipc::channel::Tx, T, ipc_uio::UioChannel, C>, Box<dyn std::error::Error>>
where
    C: CodecFor<T>,
{
    let dev = ipc_uio::UioDevice::open(uio_num)?;
    let transport = dev.channel(chan)?;
    let ch = ipc::Channel::new(chan, transport, buf);
    Ok(ch)
}

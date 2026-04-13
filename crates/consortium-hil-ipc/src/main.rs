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

use consortium_codec_postcard::PostcardCodec;
use consortium_ipc::Chan;
use consortium_runtime_app::{
    descriptor::wait_for_ready,
    open_rx_channel, open_tx_channel,
    remoteproc::{RemoteProc, RemoteProcState},
};
use std::time::Duration;

const UIO_NUM: usize = 0;
const FIRMWARE: &str = "consortium-firmware-hil";

#[tokio::main]
async fn main() {
    println!("Starting HIL IPC test...");

    // 1. Load and start M-core firmware
    println!("Loading firmware...");
    let rp = RemoteProc::new(0);
    if let Err(e) = rp.set_firmware(FIRMWARE) {
        eprintln!("Failed to set firmware: {}", e);
        return;
    }

    if let Err(e) = rp.start() {
        eprintln!("Failed to start remoteproc: {}", e);
        return;
    }

    // Wait for M-core to boot
    match rp.wait_state(RemoteProcState::Running, Duration::from_secs(5)).await {
        Ok(_) => println!("M-core firmware started"),
        Err(e) => {
            eprintln!("Remoteproc did not start: {}", e);
            return;
        }
    }

    // 2. Wait for descriptor Ready (M-core writes MReady, kernel sets Ready)
    println!("Waiting for IPC descriptor to be ready...");
    match wait_for_ready(UIO_NUM, Duration::from_secs(3)).await {
        Ok(_) => println!("IPC descriptor is ready"),
        Err(e) => {
            eprintln!("Descriptor never became ready: {}", e);
            let _ = rp.stop();
            return;
        }
    }

    // 3. Open channels
    println!("Opening IPC channels...");
    let mut tx_buf = [0u8; 264];
    let mut rx_buf = [0u8; 264];

    let chan = match Chan::new_unchecked(0) {
        // Chan is a newtype; just construct it directly
        c => c,
    };

    let mut tx_ch = match open_tx_channel::<u32, 264, PostcardCodec>(UIO_NUM, chan, &mut tx_buf).await {
        Ok(ch) => ch,
        Err(e) => {
            eprintln!("Failed to open TX channel: {}", e);
            let _ = rp.stop();
            return;
        }
    };

    let mut rx_ch = match open_rx_channel::<u32, 264, PostcardCodec>(UIO_NUM, chan, &mut rx_buf).await {
        Ok(ch) => ch,
        Err(e) => {
            eprintln!("Failed to open RX channel: {}", e);
            let _ = rp.stop();
            return;
        }
    };

    // 4. Send 5 pings, assert echoes are val+1
    println!("Running echo test...");
    let mut success = true;
    for i in 0u32..5 {
        // Send ping
        if let Err(e) = tx_ch.send(&i).await {
            eprintln!("Failed to send message {}: {}", i, e);
            success = false;
            break;
        }

        // Receive echo
        match rx_ch.recv().await {
            Ok(reply) => {
                let expected = i.wrapping_add(1);
                let reply_val = reply.into_inner();
                if reply_val == expected {
                    println!("HIL ping {} → echo {} ✓", i, reply_val);
                } else {
                    eprintln!("Echo mismatch at i={}: expected {}, got {}", i, expected, reply_val);
                    success = false;
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to receive echo for message {}: {}", i, e);
                success = false;
                break;
            }
        }
    }

    // 5. Stop firmware
    println!("Stopping firmware...");
    let _ = rp.stop();

    if success {
        println!("HIL IPC test PASSED");
    } else {
        println!("HIL IPC test FAILED");
        std::process::exit(1);
    }
}

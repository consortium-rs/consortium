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

use consortium_hmi::Bridge;
use pocket_mod::{
    Guest,
    qjs::{Coerced, Function},
};
use tokio::sync::mpsc::{UnboundedReceiver, unbounded_channel};

use crate::error::{PocketError, Result};

/// Installed before the product bundle. The native surface remains a minimal
/// queueing primitive; this JS adapter owns correlation and integrates with
/// PocketJS's host-injected effect driver.
const SERVICE_PRELUDE: &str = include_str!("service_prelude.js");

struct Completion {
    ticket: i32,
    result: String,
}

pub(super) struct PocketService {
    completions: UnboundedReceiver<Completion>,
}

impl PocketService {
    pub(super) fn mount(
        guest: &Guest,
        bridge: Bridge,
        runtime: tokio::runtime::Handle,
    ) -> Result<Self> {
        let (completion_tx, completions) = unbounded_channel();
        guest
            .mount("__consortiumNative", move |ctx, namespace| {
                namespace.set(
                    "invoke",
                    Function::new(
                        ctx.clone(),
                        move |ticket: i32,
                              command: Coerced<String>,
                              payload: Coerced<String>|
                              -> bool {
                            if ticket <= 0 {
                                return false;
                            }
                            let Some(response) = bridge.invoke(&command.0, payload.0) else {
                                return false;
                            };
                            let completion_tx = completion_tx.clone();
                            let task = runtime.spawn(async move {
                                let result = response.await;
                                let _ = completion_tx.send(Completion { ticket, result });
                            });
                            drop(task);
                            true
                        },
                    )?,
                )?;
                Ok(())
            })
            .map_err(|e| PocketError::host("mounting Consortium PocketJS service surface", e))?;
        guest
            .eval("consortium-service", SERVICE_PRELUDE)
            .map_err(|e| PocketError::host("installing Consortium PocketJS service adapter", e))?;
        Ok(Self { completions })
    }

    pub(super) fn pump(&mut self, guest: &Guest) -> Result<usize> {
        let mut delivered = 0;
        while let Ok(completion) = self.completions.try_recv() {
            guest
                .with(|ctx| -> pocket_mod::qjs::Result<()> {
                    let callback: Function = ctx.globals().get("__consortiumComplete")?;
                    callback.call((completion.ticket, completion.result.as_str()))
                })
                .map_err(|error| {
                    PocketError::host("delivering Consortium response", error.to_string())
                })?;
            delivered += 1;
        }
        if delivered > 0 {
            guest.drain_jobs();
        }
        Ok(delivered)
    }
}

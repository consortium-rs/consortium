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

;(() => {
  let nextTicket = 1
  const pending = new Map()

  const send = (cmd, payload, deliver) => {
    const ticket = nextTicket++
    let encoded
    try {
      encoded = JSON.stringify(payload ?? null)
    } catch (error) {
      deliver({ error: `Consortium payload is not JSON: ${String(error)}` })
      return
    }

    pending.set(ticket, deliver)
    try {
      if (!globalThis.__consortiumNative.invoke(ticket, String(cmd), encoded)) {
        pending.delete(ticket)
        deliver({ error: `Unknown Consortium command: ${String(cmd)}` })
      }
    } catch (error) {
      pending.delete(ticket)
      deliver({ error: `Consortium bridge failed: ${String(error)}` })
    }
  }

  globalThis.__consortiumComplete = (ticket, encoded) => {
    const deliver = pending.get(ticket)
    if (!deliver) return
    pending.delete(ticket)
    try {
      deliver(JSON.parse(encoded))
    } catch (error) {
      deliver({ error: `Consortium response is not JSON: ${String(error)}` })
    }
  }

  globalThis.__pocketEffectDriver = (command, deliver) => {
    send(command.kind, command.payload, deliver)
  }

  globalThis.__BRIDGE__ = {
    invoke(cmd, payload) {
      return new Promise((resolve) => send(cmd, payload, resolve))
    },
  }
  globalThis.consortium = globalThis.__BRIDGE__
})()

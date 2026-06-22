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

pub(super) const JS_PRELUDE: &str = r#"window.__BRIDGE__ = (() => {
  let _seq = 0;
  const _pending = {};
  return {
    invoke(cmd, payload) {
      return new Promise(resolve => {
        const id = _seq++;
        _pending[id] = resolve;
        window.webkit.messageHandlers.bridge.postMessage(
          JSON.stringify({ id, cmd, payload: payload ?? null })
        );
      });
    },
    _resolve(id, result) {
      const cb = _pending[id];
      if (cb) { delete _pending[id]; cb(result); }
    },
  };
})();"#;

pub(super) fn resolve_script(id: u64, result: &str) -> String {
    format!("window.__BRIDGE__._resolve({id},{result})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_script_inlines_json_result() {
        assert_eq!(
            resolve_script(5, r#"{"ok":true}"#),
            r#"window.__BRIDGE__._resolve(5,{"ok":true})"#,
        );
    }
}

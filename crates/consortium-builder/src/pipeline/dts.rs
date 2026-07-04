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

//! Renders the Linux `generic-uio` device-tree overlay from the lowered IR.
//!
//! Rendering lives here, mirroring how the systemd unit is rendered before the
//! `dist/` assembly consumes it. Writing the overlay into `dist/dts/linux/` and
//! verifying it with `dtc` is [`super::dist`]'s job, so this module stays free of
//! filesystem and toolchain concerns and is a pure IR -> text transform.

use consortium_cfg::bridge::LoweredIr;
use consortium_cfg::components::uio::{RegLayout, reserved_memory_container};
use consortium_dts::{File, Node, ToDts};

/// File name of the generated Linux overlay under `dist/dts/linux/`.
pub const LINUX_OVERLAY_NAME: &str = "consortium-uio.dtso";

/// Renders the Linux `generic-uio` overlay for the lowered manifest, or `None`
/// when the manifest declares no UIO devices.
///
/// The overlay is a `/plugin/;` fragment rooted at `/`, holding a single
/// `reserved-memory` container (aggregating every RAM-backed carveout, when any)
/// followed by one `generic-uio` node per device. `reg`/cell widths use the
/// AArch64 wide layout ([`RegLayout::Wide`]).
///
/// The nodes carry `dt-bindings` interrupt macros (`GIC_SPI`, `IRQ_TYPE_*`)
/// verbatim, so the emitted text is not standalone-compilable: a consumer (and
/// [`super::dist`]'s verification) must run it through the C preprocessor with
/// those macros defined before `dtc`.
pub fn render_linux_overlay(ir: &LoweredIr) -> Option<String> {
    if ir.uio.is_empty() {
        return None;
    }

    let mut root = Node::root();

    // Reserved-memory carveouts for the RAM-backed devices are aggregated into
    // one `/reserved-memory` container at the root, rendered first.
    let carveouts: Vec<Node> = ir
        .uio
        .iter()
        .flat_map(|(name, device)| device.reserved_memory_nodes(name.as_str()))
        .collect();
    if let Some(container) = reserved_memory_container(carveouts, RegLayout::Wide) {
        root = root.child(container);
    }

    for (name, device) in &ir.uio {
        root = root.child(device.to_node(name.as_str()));
    }

    Some(File::dtso().item(root).to_dts())
}

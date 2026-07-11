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

//! Userspace I/O (`uio_pdrv_genirq`) device-tree node generation.
//!
//! Consortium exposes shared-memory regions and doorbell registers to the Linux
//! application through UIO devices. A [`UioDevice`] renders to a `generic-uio`
//! DTS node and an optional interrupt becomes the UIO IRQ.
//!
//! How the device's windows are described depends on its [`UioBacking`]:
//!
//! - [`UioBacking::Registers`] (doorbells): MMIO register blocks listed inline
//!   with `reg`.
//! - [`UioBacking::ReservedMemory`] (shared RAM): each window is carved out of
//!   system RAM by a `reserved-memory` child node marked `no-map`, and the UIO
//!   node references those carveouts via `memory-region` rather than `reg`. RAM
//!   exposed to a coprocessor must be a `no-map` reservation so Linux never maps
//!   it cacheably or hands it to the page allocator.

use consortium_dts::utils::irq_method::IrqMethod;
use consortium_dts::{Cell, Node, Property, Reference};

use crate::error::CfgError;

/// `compatible` string matched by the Linux `uio_pdrv_genirq` driver.
pub const GENERIC_UIO_COMPATIBLE: &str = "generic-uio";

/// Conversion from a configuration or database resource into a [`UioDevice`].
pub trait ToUio {
    /// Builds the UIO device that exposes this resource to the Linux side.
    ///
    /// Returns `Ok(None)` when the resource exposes nothing, and an error when
    /// the resource cannot be represented as a UIO device.
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError>;
}

/// How a [`UioDevice`]'s windows are described in the device tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UioBacking {
    /// MMIO peripheral registers, described inline with `reg` (e.g. a doorbell
    /// register block). These are device registers, not RAM, so they are never
    /// a `reserved-memory` carveout.
    #[default]
    Registers,
    /// A RAM carveout: each window becomes a `reserved-memory` child node marked
    /// `no-map`, and the UIO node references them through `memory-region`.
    ReservedMemory,
}

/// Width of the `reg` address/size cells, set by the parent bus's
/// `#address-cells` / `#size-cells`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RegLayout {
    /// 64-bit addressing (`#address-cells = <2>`, `#size-cells = <2>`): each
    /// base and length is a high/low cell pair. This is the AArch64 default.
    #[default]
    Wide,
    /// 32-bit addressing (`#address-cells = <1>`, `#size-cells = <1>`): one
    /// cell per base and length. Used by ARMv7 SoCs (rare).
    Narrow,
}

/// Rendering options for [`UioDevice::to_node_with`].
#[derive(Debug, Clone, Default)]
pub struct UioOptions {
    /// `reg` cell width. Defaults to [`RegLayout::Wide`] (AArch64).
    pub layout: RegLayout,
}

/// A device-tree clock reference (`<&provider id>`) attached to a UIO node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UioClock {
    /// Device-tree label of the clock provider (e.g. `scmi_clk`).
    pub provider: String,
    /// Clock cell, emitted verbatim (a binding macro such as `CK_SCMI_IPCC1`).
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UioDevice {
    /// address, length
    pub mmap: Vec<(u32, u32)>,
    /// irq spi number, starting from 0, not +32
    pub irq: Option<(u32, IrqMethod)>,
    /// How the windows are described: inline `reg` registers, or a
    /// `reserved-memory`/`memory-region` RAM carveout.
    pub backing: UioBacking,
    /// Device-tree label of the interrupt controller for [`irq`](Self::irq),
    /// emitted as `interrupt-parent = <&…>`. Required whenever the node sits
    /// somewhere the interrupt parent is not inherited (e.g. the STM32MP2 root);
    /// `None` leaves the parent to be inherited from the surrounding tree.
    pub interrupt_parent: Option<String>,
    /// Optional clock kept enabled for this window's register block, emitted as
    /// `clocks = <&provider id>`.
    pub clock: Option<UioClock>,
}

impl UioDevice {
    /// Renders this device as a `generic-uio` DTS node named `name`, using the
    /// default [`UioOptions`] (AArch64 `reg` layout, inherited interrupt parent).
    pub fn to_node(&self, name: impl Into<String>) -> Node {
        self.to_node_with_options(name, &UioOptions::default())
    }

    /// Renders this device as a `generic-uio` DTS node named `name`.
    ///
    /// The node is `<name>@<base>` using the first mapping's base address
    /// (no unit address when there are no mappings) and carries the
    /// `generic-uio` `compatible` string, an optional GIC SPI `interrupts` entry
    /// (with an optional `interrupt-parent`), and `status = "okay";`.
    ///
    /// The windows are described per [`backing`](UioDevice::backing):
    /// [`UioBacking::Registers`] lists each window inline with `reg`;
    /// [`UioBacking::ReservedMemory`] instead references the `reserved-memory`
    /// carveouts (see [`reserved_memory_nodes`](Self::reserved_memory_nodes))
    /// through `memory-region`.
    pub fn to_node_with_options(&self, name: impl Into<String>, options: &UioOptions) -> Node {
        let name = name.into();

        let mut node = match self.mmap.first() {
            Some((base, _)) => Node::with_unit_address(name.clone(), hex(*base)),
            None => Node::new(name.clone()),
        };

        node = node.property(Property::string("compatible", GENERIC_UIO_COMPATIBLE));

        if !self.mmap.is_empty() {
            match self.backing {
                UioBacking::Registers => {
                    let cells = self
                        .mmap
                        .iter()
                        .flat_map(|(base, length)| reg_cells(*base, *length, options.layout));
                    node = node.property(Property::cells("reg", cells));
                }
                UioBacking::ReservedMemory => {
                    // Reference one carveout phandle per window, in mapping order.
                    let refs = (0..self.mmap.len()).map(|index| {
                        Cell::reference(Reference::label(reserved_label(
                            &name,
                            index,
                            self.mmap.len(),
                        )))
                    });
                    node = node.property(Property::cells("memory-region", refs));
                }
            }
        }

        if let Some((spi, method)) = self.irq {
            if let Some(parent) = &self.interrupt_parent {
                // `interrupt-parent` is a phandle *cell* (`<&intc>`), not a bare
                // path reference (`&intc`): the latter encodes as a path string
                // and makes the kernel's interrupt parsing (and dtc) choke.
                node = node.property(Property::cells(
                    "interrupt-parent",
                    [Cell::reference(Reference::label(parent.as_str()))],
                ));
            }
            // GIC SPI form: <GIC_SPI <spi> <trigger>>. The `GIC_SPI` macro
            // applies the +32 SPI base, so `spi` is the raw SPI number.
            node = node.property(Property::cells(
                "interrupts",
                [
                    Cell::raw("GIC_SPI"),
                    Cell::number(u64::from(spi)),
                    Cell::raw(irq_trigger_macro(method)),
                ],
            ));
        }

        // Keep the register block clocked from the Linux side. The id is a
        // binding macro name emitted verbatim; the vendor base tree's `#include`d
        // clock header resolves it.
        if let Some(clock) = &self.clock {
            node = node.property(Property::cells(
                "clocks",
                [
                    Cell::reference(Reference::label(clock.provider.as_str())),
                    Cell::raw(clock.id.as_str()),
                ],
            ));
        }

        node.status_okay()
    }

    /// The `reserved-memory` carveout child nodes backing this device, using the
    /// default [`UioOptions`].
    ///
    /// Empty for a [`UioBacking::Registers`] device. Place the returned nodes
    /// under a `/reserved-memory` parent (see [`reserved_memory_container`]); the
    /// node from [`to_node`](Self::to_node) references them by label.
    pub fn reserved_memory_nodes(&self, name: impl Into<String>) -> Vec<Node> {
        self.reserved_memory_nodes_with_options(name, &UioOptions::default())
    }

    /// The `reserved-memory` carveout child nodes backing this device.
    ///
    /// One labeled, `no-map` node per window, named `<name>@<base>`, with a `reg`
    /// in the requested [`layout`](UioOptions::layout). The label matches the
    /// phandle the UIO node's `memory-region` references. Empty for a
    /// [`UioBacking::Registers`] device.
    pub fn reserved_memory_nodes_with_options(
        &self,
        name: impl Into<String>,
        options: &UioOptions,
    ) -> Vec<Node> {
        if self.backing != UioBacking::ReservedMemory {
            return Vec::new();
        }
        let name = name.into();
        let count = self.mmap.len();
        self.mmap
            .iter()
            .enumerate()
            .map(|(index, (base, length))| {
                Node::with_unit_address(name.clone(), hex(*base))
                    .label(reserved_label(&name, index, count))
                    .property(Property::cells(
                        "reg",
                        reg_cells(*base, *length, options.layout),
                    ))
                    // `no-map`: keep Linux from mapping the carveout cacheably or
                    // handing it to the allocator; the coprocessor owns it.
                    .property(Property::boolean("no-map"))
            })
            .collect()
    }
}

/// Builds the `/reserved-memory` container holding `children`, with cell widths
/// matching `layout` and an empty `ranges`, or `None` when there are no
/// children.
///
/// Aggregate the [`reserved_memory_nodes`](UioDevice::reserved_memory_nodes) of
/// every [`UioBacking::ReservedMemory`] device into one container at the device
/// tree root.
pub fn reserved_memory_container(
    children: impl IntoIterator<Item = Node>,
    layout: RegLayout,
) -> Option<Node> {
    let children: Vec<Node> = children.into_iter().collect();
    if children.is_empty() {
        return None;
    }
    let cells = match layout {
        RegLayout::Wide => 2u64,
        RegLayout::Narrow => 1u64,
    };
    let mut node = Node::new("reserved-memory")
        .property(Property::cells("#address-cells", [Cell::number(cells)]))
        .property(Property::cells("#size-cells", [Cell::number(cells)]))
        .property(Property::boolean("ranges"));
    for child in children {
        node = node.child(child);
    }
    Some(node)
}

/// The device-tree label for window `index` of a `reserved-memory` device named
/// `name` (`count` windows total).
///
/// Labels cannot contain `-`, so hyphens become underscores; with more than one
/// window the index is appended (`ipc_shm_0`, `ipc_shm_1`).
fn reserved_label(name: &str, index: usize, count: usize) -> String {
    let base = name.replace('-', "_");
    if count > 1 {
        format!("{base}_{index}")
    } else {
        base
    }
}

/// The `reg` cells for one window under the given address/size cell width.
///
/// Bases and lengths are `u32`, so the [`RegLayout::Wide`] high cells are
/// always `0x0`.
fn reg_cells(base: u32, length: u32, layout: RegLayout) -> Vec<Cell> {
    match layout {
        RegLayout::Wide => vec![
            Cell::raw("0x0"),
            Cell::raw(hex(base)),
            Cell::raw("0x0"),
            Cell::raw(hex(length)),
        ],
        RegLayout::Narrow => vec![Cell::raw(hex(base)), Cell::raw(hex(length))],
    }
}

/// The `dt-bindings/interrupt-controller/irq.h` macro name for a trigger type.
fn irq_trigger_macro(method: IrqMethod) -> &'static str {
    match method {
        IrqMethod::None => "IRQ_TYPE_NONE",
        IrqMethod::EdgeRising => "IRQ_TYPE_EDGE_RISING",
        IrqMethod::EdgeFalling => "IRQ_TYPE_EDGE_FALLING",
        IrqMethod::EdgeBoth => "IRQ_TYPE_EDGE_BOTH",
        IrqMethod::LevelHigh => "IRQ_TYPE_LEVEL_HIGH",
        IrqMethod::LevelLow => "IRQ_TYPE_LEVEL_LOW",
    }
}

/// Formats an address/size as a `0x`-prefixed lowercase hex literal, matching
/// device-tree `reg` conventions.
fn hex(value: u32) -> String {
    format!("0x{value:x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_dts::ToDts;

    /// A register-backed device (the doorbell case).
    fn registers(mmap: Vec<(u32, u32)>, irq: Option<(u32, IrqMethod)>) -> UioDevice {
        UioDevice {
            mmap,
            irq,
            backing: UioBacking::Registers,
            interrupt_parent: None,
            clock: None,
        }
    }

    /// A reserved-memory-backed device (the shared-memory case).
    fn reserved(mmap: Vec<(u32, u32)>) -> UioDevice {
        UioDevice {
            mmap,
            irq: None,
            backing: UioBacking::ReservedMemory,
            interrupt_parent: None,
            clock: None,
        }
    }

    #[test]
    fn renders_single_window_generic_uio() {
        let device = registers(vec![(0x0624_0000, 0x400)], None);

        // Default AArch64 layout: 2 address cells + 2 size cells per window.
        let dts = device.to_node("hsem").to_dts();
        assert!(dts.contains("hsem@6240000 {"));
        assert!(dts.contains("compatible = \"generic-uio\";"));
        assert!(dts.contains("reg = <0x0 0x6240000 0x0 0x400>;"));
        assert!(dts.contains("status = \"okay\";"));
        assert!(!dts.contains("interrupts"));
    }

    #[test]
    fn aggregates_multiple_register_mappings() {
        let device = registers(vec![(0x0040_0000, 0x8000), (0x0040_8000, 0x2000)], None);

        let dts = device.to_node("regs").to_dts();
        assert!(dts.contains("reg = <0x0 0x400000 0x0 0x8000 0x0 0x408000 0x0 0x2000>;"));
    }

    #[test]
    fn narrow_layout_uses_single_cells() {
        let device = registers(vec![(0x0040_0000, 0x8000)], None);

        let options = UioOptions {
            layout: RegLayout::Narrow,
            ..UioOptions::default()
        };
        let dts = device.to_node_with_options("regs", &options).to_dts();
        assert!(dts.contains("reg = <0x400000 0x8000>;"));
    }

    #[test]
    fn renders_gic_spi_interrupt() {
        let mut device = registers(
            vec![(0x0624_0000, 0x400)],
            Some((199, IrqMethod::LevelHigh)),
        );
        device.interrupt_parent = Some("intc".to_string());

        let dts = device.to_node("hsem").to_dts();
        assert!(dts.contains("interrupt-parent = <&intc>;"));
        assert!(dts.contains("interrupts = <GIC_SPI 199 IRQ_TYPE_LEVEL_HIGH>;"));
    }

    #[test]
    fn renders_doorbell_clock() {
        let mut device = registers(
            vec![(0x0049_0000, 0x400)],
            Some((171, IrqMethod::LevelHigh)),
        );
        device.interrupt_parent = Some("intc".to_string());
        device.clock = Some(UioClock {
            provider: "scmi_clk".to_string(),
            id: "CK_SCMI_IPCC1".to_string(),
        });

        let dts = device.to_node("ipcc1").to_dts();
        assert!(dts.contains("clocks = <&scmi_clk CK_SCMI_IPCC1>;"));
    }

    #[test]
    fn omits_unit_address_without_mappings() {
        let device = registers(vec![], None);
        let node = device.to_node("uio");
        assert_eq!(node.unit_address, None);
    }

    #[test]
    fn reserved_memory_device_references_carveout_not_reg() {
        let device = reserved(vec![(0x2040_0000, 0x8000)]);

        // The UIO node references the carveout phandle; no inline `reg`.
        let dts = device.to_node("ipc-shm").to_dts();
        assert!(dts.contains("ipc-shm@20400000 {"));
        assert!(dts.contains("memory-region = <&ipc_shm>;"));
        assert!(!dts.contains("reg ="));

        // The carveout node is labeled, sized via `reg`, and `no-map`.
        let nodes = device.reserved_memory_nodes("ipc-shm");
        assert_eq!(nodes.len(), 1);
        let carveout = nodes[0].to_dts();
        assert!(carveout.contains("ipc_shm: ipc-shm@20400000 {"));
        assert!(carveout.contains("reg = <0x0 0x20400000 0x0 0x8000>;"));
        assert!(carveout.contains("no-map;"));
    }

    #[test]
    fn reserved_memory_multiple_windows_index_labels() {
        let device = reserved(vec![(0x2040_0000, 0x8000), (0x2040_8000, 0x2000)]);

        let dts = device.to_node("ipc-shm").to_dts();
        assert!(dts.contains("memory-region = <&ipc_shm_0 &ipc_shm_1>;"));

        let nodes = device.reserved_memory_nodes("ipc-shm");
        assert_eq!(nodes.len(), 2);
        assert!(nodes[0].to_dts().contains("ipc_shm_0: ipc-shm@20400000 {"));
        assert!(nodes[1].to_dts().contains("ipc_shm_1: ipc-shm@20408000 {"));
    }

    #[test]
    fn register_device_has_no_reserved_memory_nodes() {
        let device = registers(vec![(0x0624_0000, 0x400)], None);
        assert!(device.reserved_memory_nodes("hsem").is_empty());
    }

    #[test]
    fn reserved_memory_container_wraps_children() {
        let device = reserved(vec![(0x2040_0000, 0x8000)]);
        let children = device.reserved_memory_nodes("ipc-shm");

        let container = reserved_memory_container(children, RegLayout::Wide)
            .expect("children present yields a container");
        let dts = container.to_dts();
        assert!(dts.contains("reserved-memory {"));
        assert!(dts.contains("#address-cells = <2>;"));
        assert!(dts.contains("#size-cells = <2>;"));
        assert!(dts.contains("ranges;"));
        assert!(dts.contains("ipc_shm: ipc-shm@20400000 {"));

        // No children -> no container.
        assert!(reserved_memory_container(Vec::new(), RegLayout::Wide).is_none());
    }
}

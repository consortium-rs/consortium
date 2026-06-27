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
//! DTS node whose `reg` windows become the device's `mapN` mappings and whose
//! optional interrupt becomes the UIO IRQ.

use consortium_dts::utils::irq_method::IrqMethod;
use consortium_dts::{Cell, Node, Property, Reference};

/// `compatible` string matched by the Linux `uio_pdrv_genirq` driver.
pub const GENERIC_UIO_COMPATIBLE: &str = "generic-uio";

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
    /// Explicit `interrupt-parent` (e.g. `&intc`). When `None`, the parent is
    /// inherited from the surrounding device tree. Only emitted when the device
    /// has an interrupt.
    pub interrupt_parent: Option<Reference>,
}

pub struct UioDevice {
    /// address, length
    pub mmap: Vec<(u32, u32)>,
    /// irq spi number, starting from 0, not +32
    pub irq: Option<(u32, IrqMethod)>,
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
    /// (no unit address when there are no mappings). It carries the
    /// `generic-uio` `compatible` string, a `reg` property listing every
    /// mapping as a `<base length>` entry in the requested
    /// [`layout`](UioOptions::layout), an optional GIC SPI `interrupts` entry
    /// (with an optional `interrupt-parent`), and `status = "okay";`.
    pub fn to_node_with_options(&self, name: impl Into<String>, options: &UioOptions) -> Node {
        let name = name.into();

        let mut node = match self.mmap.first() {
            Some((base, _)) => Node::with_unit_address(name, hex(*base)),
            None => Node::new(name),
        };

        node = node.property(Property::string("compatible", GENERIC_UIO_COMPATIBLE));

        if !self.mmap.is_empty() {
            let cells = self
                .mmap
                .iter()
                .flat_map(|(base, length)| reg_cells(*base, *length, options.layout));
            node = node.property(Property::cells("reg", cells));
        }

        if let Some((spi, method)) = self.irq {
            if let Some(parent) = &options.interrupt_parent {
                node = node.property(Property::reference("interrupt-parent", parent.clone()));
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

        node.status_okay()
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

    #[test]
    fn renders_single_window_generic_uio() {
        let device = UioDevice {
            mmap: vec![(0x0040_0000, 0x8000)],
            irq: None,
        };

        // Default AArch64 layout: 2 address cells + 2 size cells per window.
        let dts = device.to_node("ipc-shm").to_dts();
        assert!(dts.contains("ipc-shm@400000 {"));
        assert!(dts.contains("compatible = \"generic-uio\";"));
        assert!(dts.contains("reg = <0x0 0x400000 0x0 0x8000>;"));
        assert!(dts.contains("status = \"okay\";"));
        assert!(!dts.contains("interrupts"));
    }

    #[test]
    fn aggregates_multiple_mappings() {
        let device = UioDevice {
            mmap: vec![(0x0040_0000, 0x8000), (0x0040_8000, 0x2000)],
            irq: None,
        };

        let dts = device.to_node("shm").to_dts();
        assert!(dts.contains("reg = <0x0 0x400000 0x0 0x8000 0x0 0x408000 0x0 0x2000>;"));
    }

    #[test]
    fn narrow_layout_uses_single_cells() {
        let device = UioDevice {
            mmap: vec![(0x0040_0000, 0x8000)],
            irq: None,
        };

        let options = UioOptions {
            layout: RegLayout::Narrow,
            ..UioOptions::default()
        };
        let dts = device.to_node_with_options("ipc-shm", &options).to_dts();
        assert!(dts.contains("reg = <0x400000 0x8000>;"));
    }

    #[test]
    fn renders_gic_spi_interrupt() {
        let device = UioDevice {
            mmap: vec![(0x0624_0000, 0x400)],
            irq: Some((199, IrqMethod::LevelHigh)),
        };

        let options = UioOptions {
            interrupt_parent: Some(Reference::label("intc")),
            ..UioOptions::default()
        };
        let dts = device.to_node_with_options("hsem", &options).to_dts();
        assert!(dts.contains("interrupt-parent = &intc;"));
        assert!(dts.contains("interrupts = <GIC_SPI 199 IRQ_TYPE_LEVEL_HIGH>;"));
    }

    #[test]
    fn omits_unit_address_without_mappings() {
        let device = UioDevice {
            mmap: vec![],
            irq: None,
        };
        let node = device.to_node("uio");
        assert_eq!(node.unit_address, None);
    }
}

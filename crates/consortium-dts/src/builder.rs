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

use crate::ast::{
    Cell, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference,
    TopLevelItem,
};

impl File {
    /// Create an empty DTS file.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a DTS/DTSI file that emits `/dts-v1/;`.
    pub fn dtsi() -> Self {
        Self {
            version: true,
            ..Self::default()
        }
    }

    /// Create a DTSO file that emits `/dts-v1/;` and `/plugin/;`.
    pub fn dtso() -> Self {
        Self {
            version: true,
            plugin: true,
            ..Self::default()
        }
    }

    /// Set whether this file emits `/dts-v1/;`.
    pub fn version(mut self, version: bool) -> Self {
        self.version = version;
        self
    }

    /// Set whether this file emits `/plugin/;`.
    pub fn plugin(mut self, plugin: bool) -> Self {
        self.plugin = plugin;
        self
    }

    /// Add a memory reservation.
    pub fn mem_reserve(mut self, mem_reserve: MemReserve) -> Self {
        self.mem_reserves.push(mem_reserve);
        self
    }

    /// Add a top-level item.
    pub fn item(mut self, item: impl Into<TopLevelItem>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Add an include.
    pub fn include(mut self, include: Include) -> Self {
        self.includes.push(include);
        self
    }

    /// Add a top-level delete-node directive.
    pub fn delete_node(mut self, reference: Reference) -> Self {
        self.items.push(TopLevelItem::DeleteNode(reference));
        self
    }
}

impl Node {
    /// Create a node by name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            label: None,
            name: name.into(),
            unit_address: None,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Create a node with a unit address.
    pub fn with_unit_address(name: impl Into<String>, unit_address: impl Into<String>) -> Self {
        Self {
            unit_address: Some(normalize_unit_address(unit_address.into())),
            ..Self::new(name)
        }
    }

    /// Create a root node.
    pub fn root() -> Self {
        Self::new("/")
    }

    /// Set the node label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Add a property.
    pub fn property(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }

    /// Add a child node.
    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// Add `status = "okay";`.
    pub fn status_okay(self) -> Self {
        self.property(Property::string("status", "okay"))
    }
}

impl NodeOverlay {
    /// Create an overlay that targets a label.
    pub fn label(label: impl Into<String>) -> Self {
        Self::new(Reference::Label(label.into()))
    }

    /// Create an overlay that targets an absolute path.
    pub fn path(path: impl Into<String>) -> Self {
        Self::new(Reference::Path(path.into()))
    }

    /// Create an overlay from an explicit reference.
    pub fn new(target: Reference) -> Self {
        Self {
            target,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Add a property.
    pub fn property(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }

    /// Add a child node.
    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// Add `status = "okay";`.
    pub fn status_okay(self) -> Self {
        self.property(Property::string("status", "okay"))
    }
}

impl Property {
    /// Create a boolean property.
    pub fn boolean(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    /// Create a string property.
    pub fn string(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(PropValue::Strings(vec![value.into()])),
        }
    }

    /// Create a string-list property.
    pub fn strings<I, S>(name: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            name: name.into(),
            value: Some(PropValue::Strings(
                values.into_iter().map(Into::into).collect(),
            )),
        }
    }

    /// Create a cell-list property.
    pub fn cells<I>(name: impl Into<String>, cells: I) -> Self
    where
        I: IntoIterator<Item = Cell>,
    {
        Self {
            name: name.into(),
            value: Some(PropValue::Cells(cells.into_iter().collect())),
        }
    }

    /// Create a byte-array property.
    pub fn bytes<I>(name: impl Into<String>, bytes: I) -> Self
    where
        I: IntoIterator<Item = u8>,
    {
        Self {
            name: name.into(),
            value: Some(PropValue::Bytes(bytes.into_iter().collect())),
        }
    }

    /// Create a reference property.
    pub fn reference(name: impl Into<String>, reference: Reference) -> Self {
        Self {
            name: name.into(),
            value: Some(PropValue::Reference(reference)),
        }
    }

    /// Create a compound property.
    pub fn compound<I>(name: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = PropValue>,
    {
        Self {
            name: name.into(),
            value: Some(PropValue::Compound(values.into_iter().collect())),
        }
    }

    /// Create a raw property value.
    pub fn raw(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: Some(PropValue::Raw(value.into())),
        }
    }
}

impl Cell {
    /// Create a numeric cell.
    pub fn number(value: u64) -> Self {
        Self::Number(value)
    }

    /// Create a reference cell.
    pub fn reference(reference: Reference) -> Self {
        Self::Reference(reference)
    }

    /// Create a macro-call cell.
    pub fn macro_call<I, S>(name: impl Into<String>, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::MacroCall {
            name: name.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    /// Create a raw cell.
    pub fn raw(value: impl Into<String>) -> Self {
        Self::Raw(value.into())
    }
}

impl Reference {
    /// Create a label reference.
    pub fn label(label: impl Into<String>) -> Self {
        Self::Label(label.into())
    }

    /// Create a path reference.
    pub fn path(path: impl Into<String>) -> Self {
        Self::Path(path.into())
    }
}

impl MemReserve {
    /// Create a memory reservation.
    pub fn new(address: impl Into<String>, size: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            size: size.into(),
        }
    }
}

impl Include {
    /// Create a quoted include, such as `#include "board.dtsi"`.
    pub fn quoted(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            angled: false,
        }
    }

    /// Create an angled include, such as `#include <dt-bindings/gpio/gpio.h>`.
    pub fn angled(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            angled: true,
        }
    }
}

fn normalize_unit_address(unit_address: String) -> String {
    unit_address
        .strip_prefix("0x")
        .or_else(|| unit_address.strip_prefix("0X"))
        .unwrap_or(&unit_address)
        .to_ascii_lowercase()
}

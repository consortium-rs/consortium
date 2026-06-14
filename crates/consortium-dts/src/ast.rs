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

/// A DTS, DTSI, or DTSO file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    /// Emit `/dts-v1/;` when true.
    pub version: bool,
    /// Emit `/plugin/;` when true.
    pub plugin: bool,
    /// Reserved memory declarations.
    pub mem_reserves: Vec<MemReserve>,
    /// Top-level file items.
    pub items: Vec<TopLevelItem>,
}

/// A top-level DTS item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    /// A root or standalone node.
    Root(Node),
    /// An overlay block such as `&uart4 { ... };`.
    Overlay(NodeOverlay),
    /// A preprocessor include.
    Include(Include),
    /// A top-level delete-node directive.
    DeleteNode(Reference),
}

/// A DTS node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// Optional node label.
    pub label: Option<String>,
    /// Node name. The root node uses `/`.
    pub name: String,
    /// Optional unit address, without `@`.
    pub unit_address: Option<String>,
    /// Node properties.
    pub properties: Vec<Property>,
    /// Child nodes.
    pub children: Vec<Node>,
}

/// An overlay node that targets an existing node by label or path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeOverlay {
    /// Overlay target.
    pub target: Reference,
    /// Properties to merge into the target.
    pub properties: Vec<Property>,
    /// Child nodes to merge into the target.
    pub children: Vec<Node>,
}

/// A DTS property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    /// Property name.
    pub name: String,
    /// Property value. `None` means boolean property.
    pub value: Option<PropValue>,
}

/// A DTS property value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropValue {
    /// Cell list, printed inside `<...>`.
    Cells(Vec<Cell>),
    /// One or more string literals.
    Strings(Vec<String>),
    /// Byte array, printed inside `[...]`.
    Bytes(Vec<u8>),
    /// A node reference such as `&uart4`.
    Reference(Reference),
    /// A comma-separated compound value.
    Compound(Vec<PropValue>),
    /// Raw value text for DTS syntax not yet modeled.
    Raw(String),
}

/// A single cell expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    /// Numeric cell value.
    Number(u64),
    /// Reference cell value.
    Reference(Reference),
    /// Macro invocation, preserving raw argument expressions.
    MacroCall { name: String, args: Vec<String> },
    /// Raw cell text.
    Raw(String),
}

/// A DTS node reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Reference {
    /// A label reference, printed as `&label`.
    Label(String),
    /// A path reference, printed as `&{/path}`.
    Path(String),
}

/// A `/memreserve/` declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemReserve {
    /// Start address text.
    pub address: String,
    /// Size text.
    pub size: String,
}

/// A preprocessor include.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Include {
    /// Included path.
    pub path: String,
    /// Whether to use angle brackets instead of quoted include syntax.
    pub angled: bool,
}

impl From<Node> for TopLevelItem {
    fn from(node: Node) -> Self {
        Self::Root(node)
    }
}

impl From<NodeOverlay> for TopLevelItem {
    fn from(overlay: NodeOverlay) -> Self {
        Self::Overlay(overlay)
    }
}

impl From<Include> for TopLevelItem {
    fn from(include: Include) -> Self {
        Self::Include(include)
    }
}

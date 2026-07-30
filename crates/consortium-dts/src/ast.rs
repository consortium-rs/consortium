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

//! Owned, syntax-shaped device-tree values.
//!
//! The AST mirrors DTS *syntax*, not a resolved device tree. A [`Reference`] is stored
//! as the label it names, not as a pointer to the node it will eventually reach; an
//! [`Include`] is a path, not its contents. That keeps every value owned, `Clone`, and
//! printable in isolation — which is what lets `consortium-cfg` build a fragment
//! without a resolution pass and lets [`crate::codegen`] guarantee deterministic
//! output.
//!
//! | Type            | DTS construct                                          |
//! | --------------- | ------------------------------------------------------ |
//! | [`File`]        | A whole `.dts` / `.dtsi` / `.dtso`, with its version tag |
//! | [`TopLevelItem`]| One item in a file: a node, overlay, include, or reserve  |
//! | [`Node`]        | A named node and its children (`uart4 { … }`)             |
//! | [`NodeOverlay`] | A label override (`&uart4 { … }`)                         |
//! | [`Property`]    | `name = value;` or a valueless boolean property            |
//! | [`PropValue`]   | A property's value: string, cells, bytes, reference, raw    |
//! | [`Cell`]        | One entry inside a `<…>` cell list                          |
//! | [`Reference`]   | `&label` or `&{/full/path}`                                 |
//! | [`Deletion`]    | `/delete-node/` or `/delete-property/`                       |
//! | [`MemReserve`]  | `/memreserve/ base len;`                                     |
//!
//! [`PropValue::Raw`] is the deliberate escape hatch: an expression this crate does not
//! model yet can still be emitted verbatim, so `consortium-cfg` is never blocked on
//! parser coverage.
//!
//! Construct these through [`crate::builder`] rather than struct literals — the
//! builders are what make codegen read like its output.

/// A DTS, DTSI, or DTSO file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    /// Emit `/dts-v1/;` when true.
    pub version: bool,
    /// Emit `/plugin/;` when true.
    pub plugin: bool,
    /// Reserved memory declarations.
    pub mem_reserves: Vec<MemReserve>,
    /// Source-level include directives.
    pub includes: Vec<Include>,
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
    /// A top-level delete-node directive.
    DeleteNode(Reference),
    /// A C preprocessor directive line captured verbatim, such as
    /// `#define FOO 0x1` or `#ifndef GUARD`. `#include` is modeled by
    /// [`Include`] instead.
    CppDirective(String),
}

/// A DTS node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// Emit a leading `/omit-if-no-ref/` marker when true.
    pub omit_if_no_ref: bool,
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
    /// `/delete-property/` and `/delete-node/` operations in this node body.
    pub deletions: Vec<Deletion>,
}

/// An overlay node that targets an existing node by label or path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeOverlay {
    /// Optional label applied to the override, as in `foo: &bar { ... };`.
    pub label: Option<String>,
    /// Overlay target.
    pub target: Reference,
    /// Properties to merge into the target.
    pub properties: Vec<Property>,
    /// Child nodes to merge into the target.
    pub children: Vec<Node>,
    /// `/delete-property/` and `/delete-node/` operations in this overlay body.
    pub deletions: Vec<Deletion>,
}

/// A `/delete-property/` or `/delete-node/` operation inside a node body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deletion {
    /// `/delete-property/ name;`
    Property(String),
    /// `/delete-node/ &target;`
    Node(Reference),
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
    /// A sized cell list, printed as `/bits/ N <...>`.
    Bits {
        /// Cell width in bits, such as `8`, `16`, or `64`.
        width: u32,
        /// Cell values.
        cells: Vec<Cell>,
    },
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

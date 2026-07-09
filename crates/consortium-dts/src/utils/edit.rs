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

//! Read-only analysis helpers over a parsed [`File`].
//!
//! These support building a merged device tree on top of a vendor base: locating
//! a node by label, its `compatible` strings, and whether it carries a given
//! property. Mutation is expressed by appending [`crate::NodeOverlay`] items
//! (label overrides with `/delete-property/`, `status = "disabled"`, …) to the
//! parsed file rather than editing nodes in place.

use crate::ast::{File, Node, NodeOverlay, PropValue, Property, Reference, TopLevelItem};

impl Node {
    /// The property named `name`, if present.
    ///
    /// Named `get_property` to avoid clashing with the `property` builder, which
    /// consumes `self`.
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Whether a property named `name` is present (including boolean
    /// properties).
    pub fn has_property(&self, name: &str) -> bool {
        self.get_property(name).is_some()
    }

    /// The `compatible` string list of this node, or empty when absent or not a
    /// plain string list.
    pub fn compatible(&self) -> Vec<&str> {
        match self
            .get_property("compatible")
            .and_then(|p| p.value.as_ref())
        {
            Some(PropValue::Strings(values)) => values.iter().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }

    /// Whether any `compatible` string contains `needle`.
    pub fn compatible_contains(&self, needle: &str) -> bool {
        self.compatible().iter().any(|c| c.contains(needle))
    }
}

impl NodeOverlay {
    /// The label this overlay targets, if it targets a label (`&foo`) rather
    /// than a path (`&{/soc/…}`).
    pub fn target_label(&self) -> Option<&str> {
        match &self.target {
            Reference::Label(label) => Some(label.as_str()),
            Reference::Path(_) => None,
        }
    }

    /// The property named `name`, if this overlay sets it.
    pub fn get_property(&self, name: &str) -> Option<&Property> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Whether this overlay sets a property named `name`.
    pub fn has_property(&self, name: &str) -> bool {
        self.get_property(name).is_some()
    }

    /// The `compatible` string list this overlay sets, or empty.
    pub fn compatible(&self) -> Vec<&str> {
        match self
            .get_property("compatible")
            .and_then(|p| p.value.as_ref())
        {
            Some(PropValue::Strings(values)) => values.iter().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }
}

/// Every top-level overlay item of a file (the `&label { … }` / `&{/path} { … }`
/// blocks).
pub fn overlays(file: &File) -> impl Iterator<Item = &NodeOverlay> {
    file.items.iter().filter_map(|item| match item {
        TopLevelItem::Overlay(overlay) => Some(overlay),
        _ => None,
    })
}

/// Every root-level node of a file (the `TopLevelItem::Root` items).
pub fn root_nodes(file: &File) -> impl Iterator<Item = &Node> {
    file.items.iter().filter_map(|item| match item {
        TopLevelItem::Root(node) => Some(node),
        _ => None,
    })
}

/// Visits every node in the file's tree (root nodes and their descendants),
/// depth-first.
pub fn for_each_node<'a>(file: &'a File, mut visitor: impl FnMut(&'a Node)) {
    fn walk<'a>(node: &'a Node, visitor: &mut dyn FnMut(&'a Node)) {
        visitor(node);
        for child in &node.children {
            walk(child, visitor);
        }
    }
    for node in root_nodes(file) {
        walk(node, &mut visitor);
    }
}

/// The first node in the file's tree carrying the given label.
pub fn find_by_label<'a>(file: &'a File, label: &str) -> Option<&'a Node> {
    let mut found = None;
    for_each_node(file, |node| {
        if found.is_none() && node.label.as_deref() == Some(label) {
            found = Some(node);
        }
    });
    found
}

/// Whether any node in the file's tree carries the given label.
pub fn label_exists(file: &File, label: &str) -> bool {
    find_by_label(file, label).is_some()
}

/// Collects `(label, node)` for every labeled node whose `compatible` contains
/// `needle`, across the file's tree.
pub fn labeled_by_compatible<'a>(file: &'a File, needle: &str) -> Vec<(&'a str, &'a Node)> {
    let mut out = Vec::new();
    for_each_node(file, |node| {
        if let Some(label) = node.label.as_deref()
            && node.compatible_contains(needle)
        {
            out.push((label, node));
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_dts;

    const SAMPLE: &str = r#"/dts-v1/;
/ {
    cm33: remoteproc-cm33 {
        compatible = "fsl,imx93-cm33";
        mboxes = <&mailbox 0>;
        status = "disabled";
    };
    soc {
        mu7: mailbox@42430000 {
            compatible = "fsl,imx95-mu-v3.0", "fsl,imx8ulp-mu";
            status = "okay";
        };
    };
};
"#;

    #[test]
    fn finds_node_by_label_across_tree() {
        let file = parse_dts(SAMPLE).unwrap();
        let mu7 = find_by_label(&file, "mu7").expect("mu7 present");
        assert_eq!(mu7.name, "mailbox");
        assert!(mu7.compatible_contains("mu"));
        assert!(label_exists(&file, "cm33"));
        assert!(!label_exists(&file, "nope"));
    }

    #[test]
    fn detects_remoteproc_mboxes() {
        let file = parse_dts(SAMPLE).unwrap();
        let cm33 = find_by_label(&file, "cm33").unwrap();
        assert!(cm33.has_property("mboxes"));
        assert!(cm33.compatible_contains("-cm33"));
    }

    #[test]
    fn labeled_by_compatible_collects_matches() {
        let file = parse_dts(SAMPLE).unwrap();
        let mus = labeled_by_compatible(&file, "mu-v3.0");
        assert_eq!(mus.len(), 1);
        assert_eq!(mus[0].0, "mu7");
    }
}

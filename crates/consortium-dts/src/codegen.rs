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

//! Deterministic DTS text output.
//!
//! [`ToDts`] renders any AST value to source; [`CodegenOptions`] controls indentation
//! and similar formatting choices, and [`Codegen`] carries them through a nested render.
//!
//! **Ordering is fixed, not incidental.** Generated device trees are committed to the
//! repository and reviewed as diffs, so a codegen change must produce a reviewable diff
//! rather than a reshuffle. Properties and children are emitted in declaration order and
//! nothing is sorted or deduplicated behind the caller's back.
//!
//! That determinism is also what makes the round trip hold: for the fragment shapes this
//! crate emits, `parse_dts(x.to_dts())?.to_dts() == x.to_dts()`. The snapshot tests in
//! `consortium-cfg` depend on both properties.

use core::fmt::Write;

use crate::ast::{
    Cell, Deletion, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference,
    TopLevelItem,
};

/// Code generation options for DTS output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenOptions {
    /// One indentation unit.
    pub indent: String,
    /// Ensure the generated output ends with a newline.
    pub trailing_newline: bool,
}

impl Default for CodegenOptions {
    fn default() -> Self {
        Self {
            indent: "    ".to_string(),
            trailing_newline: true,
        }
    }
}

/// Convert an AST value to DTS text.
pub trait ToDts {
    /// Generate DTS using default options.
    fn to_dts(&self) -> String {
        self.to_dts_with_options(&CodegenOptions::default())
    }

    /// Generate DTS using explicit options.
    fn to_dts_with_options(&self, options: &CodegenOptions) -> String;
}

/// Deterministic DTS text printer.
#[derive(Debug, Clone)]
pub struct Codegen {
    options: CodegenOptions,
    output: String,
    indent: usize,
}

impl Codegen {
    /// Create a printer with explicit options.
    pub fn new(options: CodegenOptions) -> Self {
        Self {
            options,
            output: String::new(),
            indent: 0,
        }
    }

    /// Generate a file.
    pub fn generate_file(mut self, file: &File) -> String {
        self.print_file(file);
        self.finish()
    }

    /// Generate a node.
    pub fn generate_node(mut self, node: &Node) -> String {
        self.print_node(node);
        self.finish()
    }

    /// Generate an overlay node.
    pub fn generate_overlay(mut self, overlay: &NodeOverlay) -> String {
        self.print_overlay(overlay);
        self.finish()
    }

    fn finish(mut self) -> String {
        if self.options.trailing_newline {
            if !self.output.is_empty() && !self.output.ends_with('\n') {
                self.output.push('\n');
            }
        } else {
            while self.output.ends_with('\n') {
                self.output.pop();
            }
        }

        self.output
    }

    fn print_file(&mut self, file: &File) {
        let mut wrote_directive = false;

        if file.version {
            self.output.push_str("/dts-v1/;\n");
            wrote_directive = true;
        }

        if file.plugin {
            self.output.push_str("/plugin/;\n");
            wrote_directive = true;
        }

        let has_preamble = !file.mem_reserves.is_empty() || !file.includes.is_empty();
        if wrote_directive && has_preamble {
            self.output.push('\n');
        }

        for mem_reserve in &file.mem_reserves {
            self.print_mem_reserve(mem_reserve);
        }

        for include in &file.includes {
            self.print_include(include);
        }

        if !file.items.is_empty() && !self.output.is_empty() && !self.output.ends_with("\n\n") {
            self.output.push('\n');
        }

        for (idx, item) in file.items.iter().enumerate() {
            if idx > 0 && !self.output.ends_with("\n\n") {
                self.output.push('\n');
            }
            self.print_top_level_item(item);
        }
    }

    fn print_top_level_item(&mut self, item: &TopLevelItem) {
        match item {
            TopLevelItem::Root(node) => self.print_node(node),
            TopLevelItem::Overlay(overlay) => self.print_overlay(overlay),
            TopLevelItem::DeleteNode(reference) => {
                self.write_indent();
                let _ = writeln!(
                    self.output,
                    "/delete-node/ {};",
                    format_reference(reference)
                );
            }
            TopLevelItem::CppDirective(directive) => {
                self.write_indent();
                self.output.push_str(directive);
                self.output.push('\n');
            }
        }
    }

    fn print_node(&mut self, node: &Node) {
        if node.omit_if_no_ref {
            self.write_indent();
            self.output.push_str("/omit-if-no-ref/\n");
        }

        self.write_indent();
        if let Some(label) = &node.label {
            let _ = write!(self.output, "{label}: ");
        }

        if node.name == "/" {
            self.output.push_str("/ {\n");
        } else {
            self.output.push_str(&node.name);
            if let Some(unit_address) = &node.unit_address {
                let _ = write!(self.output, "@{unit_address}");
            }
            self.output.push_str(" {\n");
        }

        self.indent += 1;
        self.print_node_body(&node.properties, &node.children, &node.deletions);
        self.indent -= 1;

        self.write_indent();
        self.output.push_str("};\n");
    }

    fn print_overlay(&mut self, overlay: &NodeOverlay) {
        self.write_indent();
        if let Some(label) = &overlay.label {
            let _ = write!(self.output, "{label}: ");
        }
        let _ = writeln!(self.output, "{} {{", format_reference(&overlay.target));

        self.indent += 1;
        self.print_node_body(&overlay.properties, &overlay.children, &overlay.deletions);
        self.indent -= 1;

        self.write_indent();
        self.output.push_str("};\n");
    }

    fn print_node_body(
        &mut self,
        properties: &[Property],
        children: &[Node],
        deletions: &[Deletion],
    ) {
        for property in properties {
            self.print_property(property);
        }

        // `/delete-property/` and `/delete-node/` are property-level statements
        // that `dtc` requires *before* any subnode, so they are emitted with the
        // properties rather than after the children (the parser collects them
        // separately, losing their original interleaving).
        for deletion in deletions {
            self.print_deletion(deletion);
        }

        let leading = !properties.is_empty() || !deletions.is_empty();
        if leading && !children.is_empty() {
            self.output.push('\n');
        }

        for (idx, child) in children.iter().enumerate() {
            if idx > 0 {
                self.output.push('\n');
            }
            self.print_node(child);
        }
    }

    fn print_deletion(&mut self, deletion: &Deletion) {
        self.write_indent();
        match deletion {
            Deletion::Property(name) => {
                let _ = writeln!(self.output, "/delete-property/ {name};");
            }
            Deletion::Node(reference) => {
                let _ = writeln!(
                    self.output,
                    "/delete-node/ {};",
                    format_reference(reference)
                );
            }
        }
    }

    fn print_property(&mut self, property: &Property) {
        self.write_indent();
        self.output.push_str(&property.name);
        if let Some(value) = &property.value {
            self.output.push_str(" = ");
            self.print_prop_value(value);
        }
        self.output.push_str(";\n");
    }

    fn print_prop_value(&mut self, value: &PropValue) {
        match value {
            PropValue::Cells(cells) => self.print_cells(cells),
            PropValue::Bits { width, cells } => {
                let _ = write!(self.output, "/bits/ {width} ");
                self.print_cells(cells);
            }
            PropValue::Strings(strings) => {
                for (idx, string) in strings.iter().enumerate() {
                    if idx > 0 {
                        self.output.push_str(", ");
                    }
                    self.print_string_literal(string);
                }
            }
            PropValue::Bytes(bytes) => self.print_bytes(bytes),
            PropValue::Reference(reference) => self.output.push_str(&format_reference(reference)),
            PropValue::Compound(values) => {
                for (idx, value) in values.iter().enumerate() {
                    if idx > 0 {
                        self.output.push_str(", ");
                    }
                    self.print_prop_value(value);
                }
            }
            PropValue::Raw(value) => self.output.push_str(value),
        }
    }

    fn print_cells(&mut self, cells: &[Cell]) {
        self.output.push('<');
        for (idx, cell) in cells.iter().enumerate() {
            if idx > 0 {
                self.output.push(' ');
            }
            self.print_cell(cell);
        }
        self.output.push('>');
    }

    fn print_cell(&mut self, cell: &Cell) {
        match cell {
            Cell::Number(value) => {
                let _ = write!(self.output, "{value}");
            }
            Cell::Reference(reference) => self.output.push_str(&format_reference(reference)),
            Cell::MacroCall { name, args } => {
                self.output.push_str(name);
                self.output.push('(');
                for (idx, arg) in args.iter().enumerate() {
                    if idx > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(arg);
                }
                self.output.push(')');
            }
            Cell::Raw(value) => self.output.push_str(value),
        }
    }

    fn print_bytes(&mut self, bytes: &[u8]) {
        self.output.push('[');
        for (idx, byte) in bytes.iter().enumerate() {
            if idx > 0 {
                self.output.push(' ');
            }
            let _ = write!(self.output, "{byte:02x}");
        }
        self.output.push(']');
    }

    fn print_string_literal(&mut self, value: &str) {
        self.output.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => self.output.push_str("\\\\"),
                '"' => self.output.push_str("\\\""),
                '\n' => self.output.push_str("\\n"),
                '\r' => self.output.push_str("\\r"),
                '\t' => self.output.push_str("\\t"),
                _ => self.output.push(ch),
            }
        }
        self.output.push('"');
    }

    fn print_mem_reserve(&mut self, mem_reserve: &MemReserve) {
        let _ = writeln!(
            self.output,
            "/memreserve/ {} {};",
            mem_reserve.address, mem_reserve.size
        );
    }

    fn print_include(&mut self, include: &Include) {
        if include.angled {
            let _ = writeln!(self.output, "#include <{}>", include.path);
        } else {
            let _ = writeln!(self.output, "#include \"{}\"", include.path);
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str(&self.options.indent);
        }
    }
}

impl ToDts for File {
    fn to_dts_with_options(&self, options: &CodegenOptions) -> String {
        Codegen::new(options.clone()).generate_file(self)
    }
}

impl ToDts for Node {
    fn to_dts_with_options(&self, options: &CodegenOptions) -> String {
        Codegen::new(options.clone()).generate_node(self)
    }
}

impl ToDts for NodeOverlay {
    fn to_dts_with_options(&self, options: &CodegenOptions) -> String {
        Codegen::new(options.clone()).generate_overlay(self)
    }
}

fn format_reference(reference: &Reference) -> String {
    match reference {
        Reference::Label(label) => format!("&{label}"),
        Reference::Path(path) => format!("&{{{path}}}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Cell, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference,
    };

    #[test]
    fn deletions_precede_subnodes() {
        // `dtc` requires `/delete-property/` before any subnode; the node body
        // must emit deletions with the properties, ahead of the children, even
        // though the AST stores them in a separate list.
        use crate::Deletion;
        let dts = NodeOverlay::label("i2c")
            .property(Property::string("status", "okay"))
            .child(Node::with_unit_address("touch", "0x41").status_okay())
            .deletion(Deletion::Property("dmas".to_string()))
            .to_dts();

        let del = dts
            .find("/delete-property/ dmas;")
            .expect("deletion emitted");
        let child = dts.find("touch@41").expect("child emitted");
        assert!(del < child, "deletion must come before the subnode:\n{dts}");
    }

    #[test]
    fn formats_root_and_regular_nodes() {
        let dts = Node::root()
            .property(Property::strings(
                "compatible",
                ["test,board", "simple-bus"],
            ))
            .child(Node::with_unit_address("serial", "0x40010000").status_okay())
            .to_dts();

        assert_eq!(
            dts,
            "/ {\n    compatible = \"test,board\", \"simple-bus\";\n\n    serial@40010000 {\n        status = \"okay\";\n    };\n};\n"
        );
    }

    #[test]
    fn formats_overlay() {
        let dts = NodeOverlay::label("uart4")
            .property(Property::string("status", "okay"))
            .to_dts();

        assert_eq!(dts, "&uart4 {\n    status = \"okay\";\n};\n");
    }

    #[test]
    fn formats_property_value_variants() {
        let dts = Node::new("pins")
            .property(Property::boolean("bias-disable"))
            .property(Property::strings("compatible", ["a", "b"]))
            .property(Property::cells(
                "pinmux",
                [
                    Cell::macro_call("STM32_PINMUX", ["'A'", "5", "AF1"]),
                    Cell::reference(Reference::label("gpioa")),
                    Cell::number(7),
                    Cell::raw("0x20"),
                ],
            ))
            .property(Property::bytes(
                "local-mac-address",
                [0xde, 0xad, 0xbe, 0xef],
            ))
            .property(Property::reference(
                "target",
                Reference::path("/soc/uart@4000"),
            ))
            .property(Property::compound(
                "compound",
                [
                    PropValue::Strings(vec!["one".to_string()]),
                    PropValue::Cells(vec![Cell::number(2)]),
                    PropValue::Raw("&raw_ref".to_string()),
                ],
            ))
            .property(Property::raw("raw-prop", "<1 2>, [aa bb]"))
            .to_dts();

        assert_eq!(
            dts,
            "pins {\n    bias-disable;\n    compatible = \"a\", \"b\";\n    pinmux = <STM32_PINMUX('A', 5, AF1) &gpioa 7 0x20>;\n    local-mac-address = [de ad be ef];\n    target = &{/soc/uart@4000};\n    compound = \"one\", <2>, &raw_ref;\n    raw-prop = <1 2>, [aa bb];\n};\n"
        );
    }

    #[test]
    fn formats_label_and_unit_address() {
        let dts = Node::with_unit_address("i2c", "0X40013000")
            .label("i2c1")
            .to_dts();

        assert_eq!(dts, "i2c1: i2c@40013000 {\n};\n");
    }

    #[test]
    fn formats_includes_before_nodes_and_memreserve_after_directive() {
        let dts = File::dtsi()
            .item(NodeOverlay::label("uart4").status_okay())
            .mem_reserve(MemReserve::new("0x80000000", "0x1000"))
            .include(Include::quoted("board.dtsi"))
            .include(Include::angled("dt-bindings/gpio/gpio.h"))
            .to_dts();

        assert_eq!(
            dts,
            "/dts-v1/;\n\n/memreserve/ 0x80000000 0x1000;\n#include \"board.dtsi\"\n#include <dt-bindings/gpio/gpio.h>\n\n&uart4 {\n    status = \"okay\";\n};\n"
        );
    }

    #[test]
    fn escapes_strings() {
        let dts = Node::new("test")
            .property(Property::string("message", "quote \" slash \\ newline\n"))
            .to_dts();

        assert_eq!(
            dts,
            "test {\n    message = \"quote \\\" slash \\\\ newline\\n\";\n};\n"
        );
    }

    #[test]
    fn formats_dtso() {
        let dts = File::dtso()
            .item(NodeOverlay::path("/soc/serial@40010000").status_okay())
            .to_dts();

        assert_eq!(
            dts,
            "/dts-v1/;\n/plugin/;\n\n&{/soc/serial@40010000} {\n    status = \"okay\";\n};\n"
        );
    }

    #[test]
    fn can_omit_trailing_newline() {
        let options = CodegenOptions {
            trailing_newline: false,
            ..CodegenOptions::default()
        };
        let dts = Node::new("empty").to_dts_with_options(&options);

        assert_eq!(dts, "empty {\n};");
    }
}

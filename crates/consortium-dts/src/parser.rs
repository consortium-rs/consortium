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
use crate::error::{DtsError, LexError, ParseError, Span};
use crate::lexer::{Token, TokenKind, lex};

/// Parse DTS source into a `File`.
pub fn parse_dts(source: &str) -> Result<File, DtsError> {
    Ok(Parser::new(source)?.parse_file()?)
}

/// Recursive-descent parser for generated DTS fragments.
#[derive(Debug)]
pub struct Parser<'src> {
    source: &'src str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'src> Parser<'src> {
    /// Lex source and create a parser.
    pub fn new(source: &'src str) -> Result<Self, LexError> {
        Ok(Self {
            source,
            tokens: lex(source)?,
            pos: 0,
        })
    }

    /// Parse a complete DTS file.
    pub fn parse_file(&mut self) -> Result<File, ParseError> {
        let mut file = File::new();

        while !self.at(TokenKind::Eof) {
            match self.peek().kind {
                TokenKind::DtsV1 => {
                    self.advance();
                    self.expect(TokenKind::Semi, "`;` after /dts-v1/")?;
                    file.version = true;
                }
                TokenKind::Plugin => {
                    self.advance();
                    self.expect(TokenKind::Semi, "`;` after /plugin/")?;
                    file.plugin = true;
                }
                TokenKind::MemReserve => file.mem_reserves.push(self.parse_mem_reserve()?),
                TokenKind::Include => file.includes.push(self.parse_include()?),
                _ => file.items.push(self.parse_top_level_item()?),
            }
        }

        Ok(file)
    }

    fn parse_top_level_item(&mut self) -> Result<TopLevelItem, ParseError> {
        match self.peek().kind {
            TokenKind::Slash => {
                self.advance();
                Ok(TopLevelItem::Root(self.parse_node_body_for(Node::root())?))
            }
            TokenKind::Amp => {
                let target = self.parse_reference()?;
                Ok(TopLevelItem::Overlay(
                    self.parse_overlay_after_target(target)?,
                ))
            }
            TokenKind::DeleteNode => {
                self.advance();
                let reference = self.parse_reference()?;
                self.expect(TokenKind::Semi, "`;` after /delete-node/")?;
                Ok(TopLevelItem::DeleteNode(reference))
            }
            TokenKind::Ident => Ok(TopLevelItem::Root(self.parse_node_or_labeled_node()?)),
            _ => Err(self.expected("top-level DTS item")),
        }
    }

    fn parse_mem_reserve(&mut self) -> Result<MemReserve, ParseError> {
        self.expect(TokenKind::MemReserve, "`/memreserve/`")?;
        let address = self.expect_atom_text("memreserve address")?;
        let size = self.expect_atom_text("memreserve size")?;
        self.expect(TokenKind::Semi, "`;` after /memreserve/")?;
        Ok(MemReserve { address, size })
    }

    fn parse_include(&mut self) -> Result<Include, ParseError> {
        self.expect(TokenKind::Include, "`#include`")?;
        match self.peek().kind {
            TokenKind::String => {
                let token = self.advance();
                Ok(Include::quoted(self.parse_string_literal(token.span)?))
            }
            TokenKind::Lt => {
                self.advance();
                let start = self.peek().span.start;
                while !self.at(TokenKind::Gt) {
                    if self.at(TokenKind::Eof) {
                        return Err(ParseError::UnexpectedEof {
                            span: self.peek().span,
                            context: "angled include path",
                        });
                    }
                    self.advance();
                }
                let end = self.peek().span.start;
                self.expect(TokenKind::Gt, "`>` after include path")?;
                Ok(Include::angled(self.source[start..end].trim()))
            }
            _ => Err(self.expected("include path")),
        }
    }

    fn parse_overlay_after_target(&mut self, target: Reference) -> Result<NodeOverlay, ParseError> {
        self.expect(TokenKind::LBrace, "`{` after overlay target")?;
        let (properties, children) = self.parse_node_body_items()?;
        self.expect(TokenKind::RBrace, "`}` after overlay body")?;
        self.expect(TokenKind::Semi, "`;` after overlay")?;
        Ok(NodeOverlay {
            target,
            properties,
            children,
        })
    }

    fn parse_node_or_labeled_node(&mut self) -> Result<Node, ParseError> {
        let first = self.expect(TokenKind::Ident, "node name or label")?;
        let first_text = self.text(first.span).to_string();

        let (label, name) = if self.eat(TokenKind::Colon).is_some() {
            let name = self.expect(TokenKind::Ident, "node name after label")?;
            (Some(first_text), self.text(name.span).to_string())
        } else {
            (None, first_text)
        };

        let unit_address = if self.eat(TokenKind::At).is_some() {
            Some(self.expect_atom_text("unit address")?)
        } else {
            None
        };

        let node = Node {
            label,
            name,
            unit_address,
            properties: Vec::new(),
            children: Vec::new(),
        };

        self.parse_node_body_for(node)
    }

    fn parse_node_body_for(&mut self, mut node: Node) -> Result<Node, ParseError> {
        self.expect(TokenKind::LBrace, "`{` after node name")?;
        let (properties, children) = self.parse_node_body_items()?;
        node.properties = properties;
        node.children = children;
        self.expect(TokenKind::RBrace, "`}` after node body")?;
        self.expect(TokenKind::Semi, "`;` after node")?;
        Ok(node)
    }

    fn parse_node_body_items(&mut self) -> Result<(Vec<Property>, Vec<Node>), ParseError> {
        let mut properties = Vec::new();
        let mut children = Vec::new();

        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                return Err(ParseError::UnexpectedEof {
                    span: self.peek().span,
                    context: "node body",
                });
            }

            if self.item_is_node() {
                children.push(self.parse_node_or_labeled_node()?);
            } else {
                properties.push(self.parse_property()?);
            }
        }

        Ok((properties, children))
    }

    fn item_is_node(&self) -> bool {
        let mut idx = self.pos;
        if self.kind_at(idx) != Some(TokenKind::Ident) {
            return false;
        }
        idx += 1;
        if self.kind_at(idx) == Some(TokenKind::Colon) {
            idx += 1;
            if self.kind_at(idx) != Some(TokenKind::Ident) {
                return false;
            }
            idx += 1;
        }
        if self.kind_at(idx) == Some(TokenKind::At) {
            idx += 2;
        }
        self.kind_at(idx) == Some(TokenKind::LBrace)
    }

    fn parse_property(&mut self) -> Result<Property, ParseError> {
        let name = self.expect(TokenKind::Ident, "property name")?;
        let name = self.text(name.span).to_string();

        if self.eat(TokenKind::Semi).is_some() {
            return Ok(Property { name, value: None });
        }

        self.expect(TokenKind::Eq, "`=` after property name")?;
        let value = self.parse_prop_value()?;
        self.expect(TokenKind::Semi, "`;` after property")?;
        Ok(Property {
            name,
            value: Some(value),
        })
    }

    fn parse_prop_value(&mut self) -> Result<PropValue, ParseError> {
        let mut values = vec![self.parse_prop_atom()?];
        while self.eat(TokenKind::Comma).is_some() {
            values.push(self.parse_prop_atom()?);
        }

        if values.len() == 1 {
            return Ok(values.remove(0));
        }

        if values
            .iter()
            .all(|value| matches!(value, PropValue::Strings(_)))
        {
            let strings = values
                .into_iter()
                .flat_map(|value| match value {
                    PropValue::Strings(strings) => strings,
                    _ => unreachable!(),
                })
                .collect();
            Ok(PropValue::Strings(strings))
        } else {
            Ok(PropValue::Compound(values))
        }
    }

    fn parse_prop_atom(&mut self) -> Result<PropValue, ParseError> {
        match self.peek().kind {
            TokenKind::String => {
                let token = self.advance();
                Ok(PropValue::Strings(vec![
                    self.parse_string_literal(token.span)?,
                ]))
            }
            TokenKind::Lt => Ok(PropValue::Cells(self.parse_cells()?)),
            TokenKind::LBracket => Ok(PropValue::Bytes(self.parse_bytes()?)),
            TokenKind::Amp => Ok(PropValue::Reference(self.parse_reference()?)),
            _ => self.parse_raw_value(),
        }
    }

    fn parse_cells(&mut self) -> Result<Vec<Cell>, ParseError> {
        self.expect(TokenKind::Lt, "`<` before cell list")?;
        let mut cells = Vec::new();
        while !self.at(TokenKind::Gt) {
            if self.at(TokenKind::Eof) {
                return Err(ParseError::UnexpectedEof {
                    span: self.peek().span,
                    context: "cell list",
                });
            }
            cells.push(self.parse_cell()?);
        }
        self.expect(TokenKind::Gt, "`>` after cell list")?;
        Ok(cells)
    }

    fn parse_cell(&mut self) -> Result<Cell, ParseError> {
        match self.peek().kind {
            TokenKind::Number => {
                let token = self.advance();
                Ok(Cell::Number(self.parse_number(token.span)?))
            }
            TokenKind::Amp => Ok(Cell::Reference(self.parse_reference()?)),
            TokenKind::Ident if self.kind_at(self.pos + 1) == Some(TokenKind::LParen) => {
                self.parse_macro_call()
            }
            TokenKind::Ident | TokenKind::Char => {
                let token = self.advance();
                Ok(Cell::Raw(self.text(token.span).to_string()))
            }
            _ => {
                let token = self.advance();
                Ok(Cell::Raw(self.text(token.span).to_string()))
            }
        }
    }

    fn parse_macro_call(&mut self) -> Result<Cell, ParseError> {
        let name = self.expect(TokenKind::Ident, "macro name")?;
        let name = self.text(name.span).to_string();
        self.expect(TokenKind::LParen, "`(` after macro name")?;

        let mut args = Vec::new();
        let mut depth = 0usize;
        let mut arg_start = self.peek().span.start;
        loop {
            match self.peek().kind {
                TokenKind::Eof => {
                    return Err(ParseError::UnexpectedEof {
                        span: self.peek().span,
                        context: "macro call",
                    });
                }
                TokenKind::LParen | TokenKind::LBracket | TokenKind::Lt => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RParen if depth == 0 => {
                    let end = self.peek().span.start;
                    if arg_start != end {
                        args.push(self.source[arg_start..end].trim().to_string());
                    }
                    self.advance();
                    break;
                }
                TokenKind::RParen | TokenKind::RBracket | TokenKind::Gt => {
                    depth = depth.saturating_sub(1);
                    self.advance();
                }
                TokenKind::Comma if depth == 0 => {
                    let end = self.peek().span.start;
                    args.push(self.source[arg_start..end].trim().to_string());
                    let comma = self.advance();
                    arg_start = comma.span.end;
                }
                _ => {
                    self.advance();
                }
            }
        }

        Ok(Cell::MacroCall { name, args })
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>, ParseError> {
        self.expect(TokenKind::LBracket, "`[` before byte array")?;
        let mut bytes = Vec::new();

        while !self.at(TokenKind::RBracket) {
            if self.at(TokenKind::Eof) {
                return Err(ParseError::UnexpectedEof {
                    span: self.peek().span,
                    context: "byte array",
                });
            }
            let token = self.advance();
            if token.kind == TokenKind::Comma {
                continue;
            }
            let literal = self.text(token.span);
            let hex = literal
                .strip_prefix("0x")
                .or_else(|| literal.strip_prefix("0X"))
                .unwrap_or(literal);
            let byte = u8::from_str_radix(hex, 16).map_err(|_| ParseError::InvalidByte {
                span: token.span,
                literal: literal.to_string(),
            })?;
            bytes.push(byte);
        }

        self.expect(TokenKind::RBracket, "`]` after byte array")?;
        Ok(bytes)
    }

    fn parse_reference(&mut self) -> Result<Reference, ParseError> {
        self.expect(TokenKind::Amp, "`&` before reference")?;
        match self.peek().kind {
            TokenKind::Ident => {
                let token = self.advance();
                Ok(Reference::Label(self.text(token.span).to_string()))
            }
            TokenKind::LBrace => {
                self.advance();
                let start = self.peek().span.start;
                while !self.at(TokenKind::RBrace) {
                    if self.at(TokenKind::Eof) {
                        return Err(ParseError::UnexpectedEof {
                            span: self.peek().span,
                            context: "path reference",
                        });
                    }
                    self.advance();
                }
                let end = self.peek().span.start;
                self.expect(TokenKind::RBrace, "`}` after path reference")?;
                Ok(Reference::Path(self.source[start..end].trim().to_string()))
            }
            _ => Err(self.expected("reference label or path")),
        }
    }

    fn parse_raw_value(&mut self) -> Result<PropValue, ParseError> {
        let start = self.peek().span.start;
        let mut depth = 0usize;

        while !(depth == 0 && (self.at(TokenKind::Comma) || self.at(TokenKind::Semi))) {
            if self.at(TokenKind::Eof) {
                return Err(ParseError::UnexpectedEof {
                    span: self.peek().span,
                    context: "raw property value",
                });
            }
            match self.peek().kind {
                TokenKind::LBrace | TokenKind::LParen | TokenKind::LBracket | TokenKind::Lt => {
                    depth += 1
                }
                TokenKind::RBrace | TokenKind::RParen | TokenKind::RBracket | TokenKind::Gt => {
                    depth = depth.saturating_sub(1)
                }
                _ => {}
            }
            self.advance();
        }

        let end = self.peek().span.start;
        if start == end {
            return Err(self.expected("property value"));
        }
        Ok(PropValue::Raw(self.source[start..end].trim().to_string()))
    }

    fn parse_number(&self, span: Span) -> Result<u64, ParseError> {
        let literal = self.text(span);
        if let Some(hex) = literal
            .strip_prefix("0x")
            .or_else(|| literal.strip_prefix("0X"))
        {
            u64::from_str_radix(hex, 16).map_err(|_| ParseError::InvalidNumber {
                span,
                literal: literal.to_string(),
            })
        } else {
            literal
                .parse::<u64>()
                .map_err(|_| ParseError::InvalidNumber {
                    span,
                    literal: literal.to_string(),
                })
        }
    }

    fn parse_string_literal(&self, span: Span) -> Result<String, ParseError> {
        let literal = self.text(span);
        let Some(inner) = literal
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
        else {
            return Err(ParseError::InvalidString {
                span,
                message: "missing surrounding quotes".to_string(),
            });
        };

        let mut output = String::new();
        let mut chars = inner.chars();
        while let Some(ch) = chars.next() {
            if ch != '\\' {
                output.push(ch);
                continue;
            }

            let escaped = chars.next().ok_or_else(|| ParseError::InvalidString {
                span,
                message: "trailing escape".to_string(),
            })?;
            match escaped {
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '\\' => output.push('\\'),
                '"' => output.push('"'),
                other => output.push(other),
            }
        }

        Ok(output)
    }

    fn expect_atom_text(&mut self, expected: &'static str) -> Result<String, ParseError> {
        match self.peek().kind {
            TokenKind::Ident | TokenKind::Number | TokenKind::String | TokenKind::Char => {
                let token = self.advance();
                Ok(self.text(token.span).to_string())
            }
            _ => Err(self.expected(expected)),
        }
    }

    fn expect(&mut self, kind: TokenKind, expected: &'static str) -> Result<Token, ParseError> {
        if self.at(kind) {
            Ok(self.advance())
        } else {
            Err(self.expected(expected))
        }
    }

    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        if self.at(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn kind_at(&self, pos: usize) -> Option<TokenKind> {
        self.tokens.get(pos).map(|token| token.kind)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if token.kind != TokenKind::Eof {
            self.pos += 1;
        }
        token
    }

    fn expected(&self, expected: &'static str) -> ParseError {
        let token = self.peek();
        ParseError::Expected {
            span: token.span,
            expected,
            found: format!("{:?}", token.kind),
        }
    }

    fn text(&self, span: Span) -> &str {
        &self.source[span.start..span.end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ToDts, TopLevelItem};

    #[test]
    fn parses_directives_memreserve_and_includes() {
        let file = parse_dts(
            "/dts-v1/;\n\n/memreserve/ 0x80000000 0x1000;\n#include \"board.dtsi\"\n#include <dt-bindings/gpio/gpio.h>\n",
        )
        .expect("parse should succeed");

        assert!(file.version);
        assert_eq!(
            file.mem_reserves,
            vec![MemReserve::new("0x80000000", "0x1000")]
        );
        assert_eq!(file.includes.len(), 2);
        assert_eq!(file.includes[0], Include::quoted("board.dtsi"));
        assert_eq!(file.includes[1], Include::angled("dt-bindings/gpio/gpio.h"));
    }

    #[test]
    fn parses_root_labeled_nodes_and_unit_addresses() {
        let file = parse_dts(
            "/dts-v1/;\n\n/ {\n    compatible = \"test,board\", \"simple-bus\";\n\n    uart4: serial@40010000 {\n        status = \"okay\";\n    };\n};\n",
        )
        .expect("parse should succeed");

        let TopLevelItem::Root(root) = &file.items[0] else {
            panic!("expected root node");
        };
        assert_eq!(root.name, "/");
        assert_eq!(root.children[0].label.as_deref(), Some("uart4"));
        assert_eq!(root.children[0].name, "serial");
        assert_eq!(root.children[0].unit_address.as_deref(), Some("40010000"));
    }

    #[test]
    fn parses_overlays_by_label_and_path() {
        let file = parse_dts(
            "/dts-v1/;\n/plugin/;\n\n&uart4 {\n    status = \"okay\";\n};\n\n&{/soc/serial@40010000} {\n    status = \"disabled\";\n};\n",
        )
        .expect("parse should succeed");

        assert!(file.plugin);
        let TopLevelItem::Overlay(label_overlay) = &file.items[0] else {
            panic!("expected overlay");
        };
        assert_eq!(label_overlay.target, Reference::label("uart4"));

        let TopLevelItem::Overlay(path_overlay) = &file.items[1] else {
            panic!("expected overlay");
        };
        assert_eq!(path_overlay.target, Reference::path("/soc/serial@40010000"));
    }

    #[test]
    fn parses_property_value_variants() {
        let node = parse_dts(
            "pins {\n    bias-disable;\n    compatible = \"a\", \"b\";\n    pinmux = <STM32_PINMUX('A', 5, AF1) &gpioa 7 0x20>;\n    local-mac-address = [de ad be ef];\n    target = &{/soc/uart@4000};\n    compound = \"one\", <2>, &raw_ref;\n    raw-prop = foo + bar;\n};\n",
        )
        .expect("parse should succeed");

        let TopLevelItem::Root(node) = &node.items[0] else {
            panic!("expected node");
        };
        assert_eq!(node.properties[0], Property::boolean("bias-disable"));
        assert_eq!(
            node.properties[1],
            Property::strings("compatible", ["a", "b"])
        );
        assert!(matches!(
            node.properties[2].value,
            Some(PropValue::Cells(_))
        ));
        assert_eq!(
            node.properties[3],
            Property::bytes("local-mac-address", [0xde, 0xad, 0xbe, 0xef])
        );
        assert_eq!(
            node.properties[4],
            Property::reference("target", Reference::path("/soc/uart@4000"))
        );
        assert_eq!(node.properties[6], Property::raw("raw-prop", "foo + bar"));
    }

    #[test]
    fn parses_top_level_delete_node() {
        let file = parse_dts("/delete-node/ &uart4;\n/delete-node/ &{/soc/uart@4000};\n")
            .expect("parse should succeed");

        assert_eq!(
            file.items,
            vec![
                TopLevelItem::DeleteNode(Reference::label("uart4")),
                TopLevelItem::DeleteNode(Reference::path("/soc/uart@4000")),
            ]
        );
    }

    #[test]
    fn round_trips_generated_dts() {
        let dts = File::dtsi()
            .include(Include::quoted("board.dtsi"))
            .item(
                NodeOverlay::label("uart4")
                    .property(Property::string("status", "okay"))
                    .child(
                        Node::new("pins")
                            .property(Property::cells(
                                "pinmux",
                                [Cell::macro_call("STM32_PINMUX", ["'A'", "5", "AF1"])],
                            ))
                            .property(Property::boolean("bias-disable")),
                    ),
            )
            .to_dts();

        assert_eq!(parse_dts(&dts).unwrap().to_dts(), dts);
    }

    #[test]
    fn rejects_missing_semicolon() {
        let err = parse_dts("node { status = \"okay\" }").expect_err("parse should fail");
        assert!(matches!(err, DtsError::Parse(ParseError::Expected { .. })));
    }

    #[test]
    fn rejects_bad_byte_array() {
        let err = parse_dts("node { bytes = [zz]; };").expect_err("parse should fail");
        assert!(matches!(
            err,
            DtsError::Parse(ParseError::InvalidByte { .. })
        ));
    }

    #[test]
    fn rejects_unterminated_string() {
        let err = parse_dts("node { message = \"oops; };").expect_err("parse should fail");
        assert!(matches!(err, DtsError::Lex(_)));
    }
}

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

use logos::Logos;

use crate::error::{LexError, Span};

/// DTS token kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Logos)]
#[logos(skip r"[ \t\n\r\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum TokenKind {
    /// `/dts-v1/`
    #[token("/dts-v1/")]
    DtsV1,
    /// `/plugin/`
    #[token("/plugin/")]
    Plugin,
    /// `/memreserve/`
    #[token("/memreserve/")]
    MemReserve,
    /// `/delete-node/`
    #[token("/delete-node/")]
    DeleteNode,
    /// `#include`
    #[token("#include")]
    Include,
    /// `{`
    #[token("{")]
    LBrace,
    /// `}`
    #[token("}")]
    RBrace,
    /// `(`
    #[token("(")]
    LParen,
    /// `)`
    #[token(")")]
    RParen,
    /// `[`
    #[token("[")]
    LBracket,
    /// `]`
    #[token("]")]
    RBracket,
    /// `<`
    #[token("<")]
    Lt,
    /// `>`
    #[token(">")]
    Gt,
    /// `;`
    #[token(";")]
    Semi,
    /// `=`
    #[token("=")]
    Eq,
    /// `,`
    #[token(",")]
    Comma,
    /// `:`
    #[token(":")]
    Colon,
    /// `@`
    #[token("@")]
    At,
    /// `&`
    #[token("&")]
    Amp,
    /// `+`
    #[token("+")]
    Plus,
    /// `-`
    #[token("-")]
    Minus,
    /// `*`
    #[token("*")]
    Star,
    /// `|`
    #[token("|")]
    Pipe,
    /// `?`
    #[token("?")]
    Question,
    /// `/`
    #[token("/")]
    Slash,
    /// Number literal.
    #[regex(r"0[xX][0-9a-fA-F]+|[0-9]+")]
    Number,
    /// String literal.
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    /// Character literal, primarily for macro-call arguments.
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    Char,
    /// Identifier-like DTS atom.
    #[regex(r"[A-Za-z_#][A-Za-z0-9,_+.#-]*")]
    Ident,
    /// Synthetic end-of-file token.
    Eof,
}

/// DTS token with source span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// Token kind.
    pub kind: TokenKind,
    /// Source span.
    pub span: Span,
}

/// Lex DTS source into tokens.
pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = TokenKind::lexer(source);
    let mut tokens = Vec::new();

    while let Some(kind) = lexer.next() {
        let range = lexer.span();
        let span = Span::new(range.start, range.end);
        let kind = kind.map_err(|()| LexError::Unrecognized { span })?;
        tokens.push(Token { kind, span });
    }

    tokens.push(Token {
        kind: TokenKind::Eof,
        span: Span::new(source.len(), source.len()),
    });
    Ok(tokens)
}

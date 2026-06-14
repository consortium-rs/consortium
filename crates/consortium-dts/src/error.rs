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

use thiserror::Error;

/// Byte span into the parsed source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Inclusive byte start.
    pub start: usize,
    /// Exclusive byte end.
    pub end: usize,
}

impl Span {
    /// Create a span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Top-level DTS parsing error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DtsError {
    /// Lexing failed.
    #[error(transparent)]
    Lex(#[from] LexError),
    /// Parsing failed.
    #[error(transparent)]
    Parse(#[from] ParseError),
}

/// DTS lexer error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexError {
    /// The lexer encountered an unrecognized token.
    #[error("unrecognized token at byte range {span:?}")]
    Unrecognized { span: Span },
}

/// DTS parser error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    /// The parser expected a different token.
    #[error("expected {expected} at byte range {span:?}, found {found}")]
    Expected {
        /// Error span.
        span: Span,
        /// Expected token description.
        expected: &'static str,
        /// Found token description.
        found: String,
    },
    /// The parser reached the end of input unexpectedly.
    #[error("unexpected end of input while parsing {context}")]
    UnexpectedEof {
        /// End-of-input span.
        span: Span,
        /// Parser context.
        context: &'static str,
    },
    /// A numeric literal could not be parsed.
    #[error("invalid numeric literal `{literal}` at byte range {span:?}")]
    InvalidNumber {
        /// Error span.
        span: Span,
        /// Original literal.
        literal: String,
    },
    /// A string literal could not be parsed.
    #[error("invalid string literal at byte range {span:?}: {message}")]
    InvalidString {
        /// Error span.
        span: Span,
        /// Error detail.
        message: String,
    },
    /// A byte-array entry could not be parsed.
    #[error("invalid byte literal `{literal}` at byte range {span:?}")]
    InvalidByte {
        /// Error span.
        span: Span,
        /// Original literal.
        literal: String,
    },
    /// Generic parser failure.
    #[error("{message} at byte range {span:?}")]
    Message {
        /// Error span.
        span: Span,
        /// Error detail.
        message: String,
    },
}

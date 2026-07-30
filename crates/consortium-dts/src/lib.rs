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

//! Device Tree Source AST construction, parsing, and deterministic code generation.
//!
//! `consortium-dts` is the DTS *language-tooling* layer. It owns syntax-shaped AST
//! values, ergonomic builders, a lexer and parser for generated fragments, and stable
//! text output.
//!
//! It deliberately owns no *policy*. Which peripheral belongs to which core, what is
//! secure, how RIFSC and TRDC map, whether a pin assignment is legal — all of that
//! stays in `consortium-cfg`, which uses this crate to construct, parse, and print the
//! resulting fragments. Keeping the split means a device-tree question and a resource
//! question never get answered in the same place.
//!
//! # Modules
//!
//! ```text
//!   builder ──► ast ──► codegen ──► DTS text
//!                ▲                     │
//!                └──── parser ◄── lexer ┘
//! ```
//!
//! | Module      | Role                                                            |
//! | ----------- | --------------------------------------------------------------- |
//! | [`ast`]     | Owned, syntax-shaped values: [`File`], [`Node`], [`Property`], … |
//! | [`builder`] | Chainable constructors so codegen reads like the output.          |
//! | [`codegen`] | [`ToDts`] and [`CodegenOptions`] — deterministic text output.      |
//! | [`lexer`]   | Tokenizer, with [`Span`]s for diagnostics.                        |
//! | [`parser`]  | [`parse_dts`] — token stream to [`File`].                         |
//! | [`error`]   | [`DtsError`], [`LexError`], [`ParseError`].                        |
//! | [`utils`]   | Read-only analysis over a parsed file, plus [`IrqMethod`].          |
//!
//! [`IrqMethod`]: utils::irq_method::IrqMethod
//!
//! # Determinism is the requirement
//!
//! Output ordering is fixed rather than incidental, because generated device trees are
//! committed and diffed: a codegen change should produce a reviewable diff, not a
//! reshuffle. That is also what makes the round-trip property below hold.
//!
//! # Example
//!
//! ```rust
//! use consortium_dts::{File, NodeOverlay, Property, ToDts, parse_dts};
//!
//! let file = File::dtsi().item(
//!     NodeOverlay::label("uart4").property(Property::string("status", "okay")),
//! );
//!
//! assert_eq!(file.to_dts(), "/dts-v1/;\n\n&uart4 {\n    status = \"okay\";\n};\n");
//!
//! // Parsing is the inverse of printing for fragments this crate emits.
//! let parsed = parse_dts(&file.to_dts()).unwrap();
//! assert_eq!(parsed.to_dts(), file.to_dts());
//! ```
//!
//! # Mutation is expressed by appending, not editing
//!
//! [`utils::edit`] is read-only: it locates a node by label, reads its `compatible`
//! strings, checks for a property. To *change* a vendor base tree you append
//! [`NodeOverlay`] items — a label override carrying `/delete-property/`,
//! `status = "disabled"`, and so on — rather than rewriting nodes in place. That keeps
//! parsed AST values immutable and makes the delta explicit in the output, which is
//! what a reviewer of a merged tree needs to see.
//!
//! # Limitations
//!
//! - **No include, label, or phandle resolution.** The parser handles the fragment
//!   shapes this workspace generates and the vendor trees it merges onto; it is not a
//!   general `dtc` replacement, and hand-written board files may not parse.
//! - **Unknown expressions can be emitted raw** while the parser matures, so
//!   `consortium-cfg` is never blocked on a construct this crate cannot yet model.
//! - **No overlay diffing and no `dtc` validation** yet; the latter belongs in
//!   `consortium-builder` or `csti` when it lands.
//!
//! See the crate README for the planned resolver and bundler work.

pub mod ast;
pub mod builder;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;

pub use ast::{
    Cell, Deletion, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference,
    TopLevelItem,
};
pub use codegen::{Codegen, CodegenOptions, ToDts};
pub use error::{DtsError, LexError, ParseError, Span};
pub use parser::{Parser, parse_dts};

pub mod utils;

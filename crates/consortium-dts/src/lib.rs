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

//! Device Tree Source AST construction and deterministic code generation.
//!
//! `consortium-dts` is the language-tooling layer for generated DTS, DTSI, and
//! DTSO fragments. This first milestone intentionally focuses on owned AST
//! values, ergonomic construction helpers, and stable text output. Parsing,
//! include resolution, label resolution, overlay diffing, and `dtc` integration
//! are planned follow-up work.

pub mod ast;
pub mod builder;
pub mod codegen;
pub mod error;
pub mod lexer;
pub mod parser;

pub use ast::{
    Cell, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference,
    TopLevelItem,
};
pub use codegen::{Codegen, CodegenOptions, ToDts};
pub use error::{DtsError, LexError, ParseError, Span};
pub use parser::{Parser, parse_dts};

pub mod utils;
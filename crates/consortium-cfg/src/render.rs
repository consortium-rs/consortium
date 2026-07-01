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

//! Renders generated codegen tokens to formatted Rust source.
//!
//! `proc_macro2::TokenStream` (and therefore `quote!`) cannot carry `//` line
//! comments: they are not tokens, so the `quote!` lexer discards them before
//! they ever reach the stream. A `///` doc comment *does* survive, because it
//! is sugar for a `#[doc = "..."]` attribute, which is a real token.
//!
//! So the `components` codegen writes each `unsafe`-block justification as a
//! `///`-style safety doc comment, and [`render`] rewrites those doc lines back
//! into ordinary `//`-style line comments after formatting. That keeps the
//! safety rationale attached to the emitted `unsafe` blocks (a plain comment on
//! an expression, not a stray doc attribute) and satisfies
//! `clippy::undocumented_unsafe_blocks` in the generated crate.

/// Formats generated `tokens` as Rust source with the codegen's safety doc
/// comments restored to ordinary line comments (see [`restore_safety_comments`]).
///
/// # Errors
///
/// Returns the [`syn::Error`] from [`syn::parse_file`] if `tokens` is not a
/// parseable Rust file.
pub fn render(tokens: proc_macro2::TokenStream) -> Result<String, syn::Error> {
    let file = syn::parse_file(&tokens.to_string())?;
    Ok(restore_safety_comments(&prettyplease::unparse(&file)))
}

/// Rewrites the safety doc comments the codegen emits (see the module docs)
/// into plain line comments, including their continuation lines.
///
/// A run starts at a `///` line whose content begins with the safety keyword
/// and extends across the immediately following `///` lines (a multi-line
/// comment), ending at the first non-doc line — in practice the `unsafe` block
/// being annotated. Doc comments that do not open with the keyword (ordinary
/// item documentation) are left untouched.
pub fn restore_safety_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut in_safety = false;

    for line in src.split_inclusive('\n') {
        // Preserve the exact line terminator (`split_inclusive` keeps it).
        let (content, newline) = match line.strip_suffix('\n') {
            Some(rest) => (rest, "\n"),
            None => (line, ""),
        };

        let indent_len = content.len() - content.trim_start().len();
        let (indent, body) = content.split_at(indent_len);

        if let Some(doc) = body.strip_prefix("///") {
            // A `SAFETY`-opening doc line starts a run; continuation `///`
            // lines stay in the run until a non-doc line resets it.
            if doc.trim_start().starts_with("SAFETY") {
                in_safety = true;
            }
            if in_safety {
                out.push_str(indent);
                out.push_str("//");
                out.push_str(doc);
                out.push_str(newline);
                continue;
            }
        } else {
            in_safety = false;
        }

        out.push_str(line);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_safety_doc_into_line_comment() {
        let src = "fn f() {\n    /// SAFETY: valid handle\n    unsafe { g() }\n}\n";
        let out = restore_safety_comments(src);
        assert!(out.contains("    // SAFETY: valid handle\n"));
        assert!(!out.contains("///"));
    }

    #[test]
    fn rewrites_multi_line_safety_run() {
        let src = "fn f() {\n    /// SAFETY: base covers the descriptor and\n    /// the channels are this side's.\n    unsafe { g() }\n}\n";
        let out = restore_safety_comments(src);
        assert!(out.contains("    // SAFETY: base covers the descriptor and\n"));
        assert!(out.contains("    // the channels are this side's.\n"));
    }

    #[test]
    fn leaves_ordinary_doc_comments_untouched() {
        let src = "/// Ordinary item docs.\nstruct S;\n";
        assert_eq!(restore_safety_comments(src), src);
    }

    #[test]
    fn run_ends_at_non_doc_line() {
        // A `SAFETY` run must not swallow a later, unrelated doc comment.
        let src = "/// SAFETY: one\nunsafe { a() }\n/// Item docs\nstruct S;\n";
        let out = restore_safety_comments(src);
        assert!(out.contains("// SAFETY: one\n"));
        assert!(out.contains("/// Item docs\n"));
    }

    #[test]
    fn render_restores_safety_from_tokens() {
        let tokens = quote::quote! {
            fn f() {
                /// SAFETY: verified in config
                unsafe { g() }
            }
        };
        let out = render(tokens).expect("valid Rust");
        assert!(out.contains("// SAFETY: verified in config"));
        assert!(!out.contains("///"));
    }
}

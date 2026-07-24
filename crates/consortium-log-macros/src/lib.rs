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

//! Internal format-string normalization for `consortium-log`.

use std::collections::BTreeMap;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{
    Expr, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

struct PortableArgs {
    format: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for PortableArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let format = input.parse()?;
        let args = if input.is_empty() {
            Punctuated::new()
        } else {
            input.parse::<Token![,]>()?;
            Punctuated::parse_terminated(input)?
        };
        Ok(Self { format, args })
    }
}

/// Rewrites Rust implicit captures (`"{name}"`) into indexed arguments before
/// invoking the supplied `macro_rules!` callback.
///
/// Input is three parenthesized groups: callback path, log level, and the
/// original log arguments. Keeping the callback path in the input preserves
/// the facade's `$crate` hygiene.
#[proc_macro]
pub fn expand_captures(input: TokenStream) -> TokenStream {
    expand_captures_impl(input.into()).into()
}

fn expand_captures_impl(input: TokenStream2) -> TokenStream2 {
    let Some((callback, level, original_args)) = unpack(input) else {
        return quote!(compile_error!("invalid consortium-log macro expansion"));
    };

    let Ok(args) = syn::parse2::<PortableArgs>(original_args.clone()) else {
        return quote!(#callback!(#level, #original_args));
    };
    // Named format arguments (`name = expression`) already have meaning to
    // Rust/tracing and are not part of the cross-backend portable subset.
    // Preserve those tokens verbatim instead of mistaking `{name}` for an
    // implicit capture.
    if args.args.iter().any(|arg| matches!(arg, Expr::Assign(_))) {
        return quote!(#callback!(#level, #original_args));
    }

    let mut captures = Vec::<Ident>::new();
    let mut capture_indices = BTreeMap::<String, usize>::new();
    let first_capture_index = args.args.len();
    let rewritten = rewrite_format(
        &args.format.value(),
        args.format.span(),
        first_capture_index,
        &mut captures,
        &mut capture_indices,
    );
    let format = LitStr::new(&rewritten, args.format.span());
    let explicit_args: Vec<_> = args.args.into_iter().collect();

    quote!(
        #callback!(
            #level,
            #format
            #(, #explicit_args)*
            #(, #captures)*
        )
    )
}

fn unpack(input: TokenStream2) -> Option<(TokenStream2, TokenStream2, TokenStream2)> {
    let mut tokens = input.into_iter();
    let callback = group_contents(tokens.next()?)?;
    let level = group_contents(tokens.next()?)?;
    let args = group_contents(tokens.next()?)?;
    tokens.next().is_none().then_some((callback, level, args))
}

fn group_contents(token: TokenTree) -> Option<TokenStream2> {
    match token {
        TokenTree::Group(group) => Some(group.stream()),
        _ => None,
    }
}

fn rewrite_format(
    input: &str,
    capture_span: Span,
    first_capture_index: usize,
    captures: &mut Vec<Ident>,
    capture_indices: &mut BTreeMap<String, usize>,
) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        if ch != '{' {
            output.push(ch);
            continue;
        }

        if chars.peek().is_some_and(|(_, next)| *next == '{') {
            output.push_str("{{");
            chars.next();
            continue;
        }

        let Some((end, _)) = chars.clone().find(|(_, candidate)| *candidate == '}') else {
            output.push_str(&input[start..]);
            break;
        };
        let content_start = start + ch.len_utf8();
        let content = &input[content_start..end];
        let name = content.split_once(':').map_or(content, |(name, _)| name);

        let Ok(mut ident) = syn::parse_str::<Ident>(name) else {
            output.push(ch);
            continue;
        };
        ident.set_span(capture_span);

        let index = *capture_indices.entry(name.to_owned()).or_insert_with(|| {
            let index = first_capture_index + captures.len();
            captures.push(ident);
            index
        });
        output.push('{');
        output.push_str(&index.to_string());
        output.push_str(&content[name.len()..]);
        output.push('}');

        while chars.peek().is_some_and(|(position, _)| *position <= end) {
            chars.next();
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::rewrite_format;
    use proc_macro2::Span;
    use std::collections::BTreeMap;

    fn rewrite(input: &str, explicit_args: usize) -> (String, Vec<String>) {
        let mut captures = Vec::new();
        let mut indices = BTreeMap::new();
        let output = rewrite_format(
            input,
            Span::call_site(),
            explicit_args,
            &mut captures,
            &mut indices,
        );
        (
            output,
            captures
                .into_iter()
                .map(|ident| ident.to_string())
                .collect(),
        )
    }

    #[test]
    fn rewrites_implicit_captures_to_appended_indices() {
        assert_eq!(
            rewrite("{error}: value={value:?}; again={error}", 1),
            (
                "{1}: value={2:?}; again={1}".to_owned(),
                vec!["error".to_owned(), "value".to_owned()]
            )
        );
    }

    #[test]
    fn preserves_positional_parameters_and_escaped_braces() {
        assert_eq!(
            rewrite("{{name}} {} {0} {=u8}", 1),
            ("{{name}} {} {0} {=u8}".to_owned(), Vec::new())
        );
    }
}

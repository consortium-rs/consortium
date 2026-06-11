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

use proc_macro::TokenStream;

#[proc_macro]
pub fn tee_service(input: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::tee_service(input.into()).into()
}

#[proc_macro_attribute]
pub fn tee_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::tee_command(attr.into(), item.into()).into()
}

#[proc_macro_derive(TeeParam, attributes(tee))]
pub fn derive_tee_param(item: TokenStream) -> TokenStream {
    consortium_tee_macros_impl::derive_tee_param(item.into()).into()
}

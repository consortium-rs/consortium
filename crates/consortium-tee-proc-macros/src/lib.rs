use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro]
pub fn tee_service(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as syn::Expr);
    quote! { #tokens }.into()
}

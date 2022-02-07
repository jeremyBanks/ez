//! internal

#![allow(non_snake_case)]
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn throws(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    ez_internal::throws(attribute_tokens.into(), function_tokens.into()).into()
}

#[proc_macro_attribute]
pub fn panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    ez_internal::panics(attribute_tokens.into(), function_tokens.into()).into()
}

#[proc_macro_attribute]
pub fn main(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    ez_internal::main(attribute_tokens.into(), function_tokens.into()).into()
}

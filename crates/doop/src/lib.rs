#![warn(unused_crate_dependencies)]

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

pub(crate) mod evaluate;
pub(crate) mod parse;
pub(crate) mod tokens;

#[proc_macro]
pub fn doop(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: parse::DoopBlock = match syn::parse2(input) {
        Ok(input) => input,
        Err(report) => return report.to_compile_error().into(),
    };

    let output: proc_macro2::TokenStream = match evaluate::evaluate(input) {
        Ok(output) => output,
        Err(report) => return report.to_compile_error().into(),
    };

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn dooped(mut tokens: TokenStream, item: TokenStream) -> TokenStream {
    tokens.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, item))));
    doop(tokens)
}

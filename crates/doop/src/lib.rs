use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

pub(crate) mod parse;
pub(crate) mod token_stream;
pub(crate) mod token_stream;

#[proc_macro]
pub fn doop(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: parse::DoopBlock = match syn::parse2(input) {
        Ok(input) => input,
        Err(report) => return report.to_compile_error().into(),
    };

    let output: proc_macro2::TokenStream = match token_stream::token_stream(input) {
        Ok(output) => output,
        Err(report) => return report.to_compile_error().into(),
    };

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn from(mut tokens: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::new()
}

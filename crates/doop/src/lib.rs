use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

pub(crate) mod evaluation;
pub(crate) mod input;
pub(crate) mod synthesis;

#[proc_macro]
pub fn doop(tokens: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();

    let input: input::DoopBlock = match syn::parse2(tokens) {
        Ok(input) => input,
        Err(report) => return report.to_compile_error().into(),
    };

    let evaluation: evaluation::Doop = match input.try_into() {
        Ok(evaluation) => evaluation,
        Err(report) => return report.to_compile_error().into(),
    };

    let synthesis: synthesis::Doop = match evaluation.try_into() {
        Ok(synthesis) => synthesis,
        Err(report) => return report.to_compile_error().into(),
    };

    let tokens = synthesis.into_iter();
    let tokens: proc_macro2::TokenStream = tokens.collect();
    let tokens: TokenStream = tokens.into();

    tokens
}

#[proc_macro_attribute]
pub fn dooped(mut tokens: TokenStream, item: TokenStream) -> TokenStream {
    tokens.extend(TokenStream::from(TokenTree::from(Group::new(Delimiter::Brace, item))));
    doop(tokens)
}

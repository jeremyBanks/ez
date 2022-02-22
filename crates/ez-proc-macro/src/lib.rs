use {
    proc_macro2::{Span, TokenStream},
    std::borrow::BorrowMut,
};

mod common;
mod doop;
mod errors;
mod main;

#[proc_macro_attribute]
pub fn throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::errors::throws(attribute_tokens.into(), function_tokens.into()).unwrap_into()
}

#[proc_macro_attribute]
pub fn try_throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::errors::try_throws(attribute_tokens.into(), function_tokens.into()).unwrap_into()
}

#[proc_macro_attribute]
pub fn main(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::main::main(attribute_tokens.into(), function_tokens.into()).unwrap_into()
}

#[proc_macro]
pub fn doop(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::doop::doop(tokens.into()).unwrap_into()
}

trait ResultExt: Sized {
    fn result(self) -> Result<TokenStream, eyre::Report>;

    fn unwrap_into<T: From<TokenStream>>(self) -> T {
        let result = self.result();

        result
            .unwrap_or_else(|report| {
                if let Some(error) = report.downcast_ref::<syn::Error>() {
                    syn::Error::new(error.span(), format!("{error}\n{report}"))
                } else {
                    syn::Error::new(Span::call_site(), format!("{report}"))
                }
                .to_compile_error()
            })
            .into()
    }
}

impl ResultExt for Result<TokenStream, eyre::Report> {
    fn result(self) -> Self {
        self
    }
}

use proc_macro2::{Span, TokenStream};

mod common;
mod doop;
mod errors;
mod main;

#[proc_macro_attribute]
pub fn throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::errors::throws(attribute_tokens.into(), function_tokens.into()).unwrap_token()
}

#[proc_macro_attribute]
pub fn try_throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::errors::try_throws(attribute_tokens.into(), function_tokens.into()).unwrap_token()
}

#[proc_macro_attribute]
pub fn main(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::main::main(attribute_tokens.into(), function_tokens.into()).unwrap_token()
}

/// > _easily,_  \
/// > _let's get carried away_
///
/// >  ― [Red Hot Chili Peppers](https://genius.com/25345758)
///
/// ```
/// #[ez::ly]
/// fn main() {
///     info!("using ez::prelude::* and #[ez::main]");
/// }
/// ```
#[proc_macro_attribute]
pub fn ly(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::main::ly(attribute_tokens.into(), function_tokens.into()).unwrap_token()
}
trait ResultExt: Sized {
    fn result(self) -> Result<TokenStream, eyre::Report>;

    fn unwrap_token<TokenStream: From<proc_macro2::TokenStream>>(self) -> TokenStream {
        self.result()
            .unwrap_or_else(|report| {
                let span = if let Some(error) = report.downcast_ref::<syn::Error>() {
                    error.span()
                } else {
                    Span::call_site()
                };
                syn::Error::new(span, format!("{report:?}")).to_compile_error()
            })
            .into()
    }
}

impl ResultExt for Result<TokenStream, eyre::Report> {
    fn result(self) -> Self {
        self
    }
}

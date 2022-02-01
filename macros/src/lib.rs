use proc_macro::TokenStream;

mod main;
mod throws;

#[proc_macro_attribute]
pub fn try_or_panics(attribute: TokenStream, input: TokenStream) -> TokenStream {
    throws::try_or_panics(attribute, input)
}

#[proc_macro_attribute]
pub fn throws(attribute: TokenStream, input: TokenStream) -> TokenStream {
    throws::throws(attribute, input)
}

#[proc_macro_attribute]
pub fn panics(attribute: TokenStream, input: TokenStream) -> TokenStream {
    throws::panics(attribute, input)
}

#[proc_macro_attribute]
pub fn main(attributes: TokenStream, input: TokenStream) -> TokenStream {
    main::main(attributes, input)
}

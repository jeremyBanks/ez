use proc_macro::TokenStream;

mod main;
mod throws;

/// Adds a panicking alternative to a fallible function.
#[proc_macro_attribute]
pub fn throws(attribute: TokenStream, input: TokenStream) -> TokenStream {
    throws::throws(attribute, input)
}

#[proc_macro_attribute]
pub fn main(attributes: TokenStream, input: TokenStream) -> TokenStream {
    main::main(attributes, input)
}

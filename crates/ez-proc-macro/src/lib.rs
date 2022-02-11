//! internal

#[proc_macro_attribute]
pub fn throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::throws(attribute_tokens.into(), function_tokens.into()).into()
}

#[proc_macro_attribute]
pub fn try_throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::try_throws(attribute_tokens.into(), function_tokens.into()).into()
}

#[proc_macro_attribute]
pub fn panics(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::panics(attribute_tokens.into(), function_tokens.into()).into()
}

#[proc_macro_attribute]
pub fn main(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::main(attribute_tokens.into(), function_tokens.into()).into()
}

use quote::quote;

#[proc_macro_attribute]
pub fn throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::proc_macros::throws(attribute_tokens.into(), function_tokens.into())
        .unwrap_or_else(|err| {
            let err = format!("{err:?}");
            quote! { compile_error!(#err); }
        })
        .into()
}

#[proc_macro_attribute]
pub fn try_throws(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::proc_macros::try_throws(attribute_tokens.into(), function_tokens.into())
        .unwrap_or_else(|err| {
            let err = format!("{err:?}");
            quote! { compile_error!(#err); }
        })
        .into()
}

#[proc_macro_attribute]
pub fn main(
    attribute_tokens: proc_macro::TokenStream,
    function_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ez_internal::proc_macros::main(attribute_tokens.into(), function_tokens.into())
        .unwrap_or_else(|err| {
            let err = format!("{err:?}");
            quote! { compile_error!(#err); }
        })
        .into()
}

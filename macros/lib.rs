use {
    proc_macro::{Ident, TokenStream},
    quote::{quote, quote_spanned},
    syn::{parse_macro_input, spanned::Spanned},
};

#[proc_macro_attribute]
pub fn throws(_attributes: TokenStream, function: TokenStream) -> TokenStream {
    function
}

#[proc_macro_attribute]
pub fn main(attributes: TokenStream, function: TokenStream) -> TokenStream {
    let function: syn::ImplItemMethod = parse_macro_input!(function);

    let args_item = if !attributes.is_empty() {
        let attributes: syn::ItemStruct = parse_macro_input!(attributes);
        quote_spanned! {
            attributes.span() =>
            #[derive(clap::Parser)]
            #attributes
        }
    } else {
        quote! {}
    };

    let fn_item = quote_spanned! {
        function.span() =>
        #function
    };

    quote! {
        #args_item
        #fn_item
    }
    .into()
}

#[proc_macro_derive(Int)]
pub fn derive_int(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(Float)]
pub fn derive_float(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

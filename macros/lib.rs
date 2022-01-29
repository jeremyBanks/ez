use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn throws(_attributes: TokenStream, function: TokenStream) -> TokenStream {
    function
}

#[proc_macro_attribute]
pub fn main(_attributes: TokenStream, function: TokenStream) -> TokenStream {
    function
}

#[proc_macro_derive(Int)]
pub fn derive_int(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(Float)]
pub fn derive_float(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

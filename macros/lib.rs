use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn try_or_unwrap(_attributes: TokenStream, function: TokenStream) -> TokenStream {
    let f = function.to_string();

    eprintln!("{:?}", f);

    let g = format!("#[::fehler::throws(::eyre::Report)]\n{f}");

    eprintln!("{:?}", g);

    g.parse().unwrap()
}

#[proc_macro_derive(Int)]
pub fn derive_int(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

use {proc_macro::TokenStream, quote::ToTokens, syn::parse_macro_input};

pub fn main(_attributes: TokenStream, input: TokenStream) -> TokenStream {
    let function: syn::ItemFn = parse_macro_input!(input);

    function.into_token_stream().into()
}

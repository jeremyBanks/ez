use {
    proc_macro::TokenStream,
    quote::{quote_spanned, ToTokens},
    syn::{parse_macro_input, parse_quote, spanned::Spanned, ReturnType},
};

#[proc_macro_attribute]
pub fn main(_attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let mut inner_function: syn::ItemFn = parse_macro_input!(function_tokens);
    let mut outer_function = inner_function.clone();
    let ident = inner_function.sig.ident.clone();

    match inner_function.sig.inputs.len() {
        0 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::main::Ignored,));
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::main::Ignored,));
        },
        1 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::main::Ignored,));
        },
        2 => {},
        _ => {
            return quote_spanned! {inner_function.sig.inputs.span()=>
                compile_error!("#[ez::main] function must have at most 2 arguments (for example, `fn main(args: Vec<String>, env: Vec<(String, String)>)`).");
            }.into_token_stream().into();
        },
    }

    outer_function.sig.inputs = syn::punctuated::Punctuated::new();
    outer_function.sig.output = parse_quote! { -> Result<(), eyre::Report> };

    let block = inner_function.block.clone();
    if inner_function.sig.output == ReturnType::Default {
        inner_function.sig.output = parse_quote! { -> Result<(), eyre::Report> };
    }
    inner_function.block = parse_quote! { {
        Ok(#block)
    } };

    outer_function.block = parse_quote! { {
        #inner_function;
        ::ez::main::run_main(env!("CARGO_CRATE_NAME"), #ident)
    } };

    outer_function.to_token_stream().into()
}

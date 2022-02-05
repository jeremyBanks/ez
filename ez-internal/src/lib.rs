use {
    proc_macro::TokenStream,
    quote::{quote, quote_spanned, ToTokens},
    syn::{parse_macro_input, parse_quote, spanned::Spanned, ReturnType},
};

#[proc_macro_attribute]
pub fn throws(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = proc_macro2::TokenStream::from(attribute_tokens);
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::deps::eyre::Report }
    } else {
        attribute_tokens
    };

    let function_tokens = proc_macro2::TokenStream::from(function_tokens);
    let mut function_tokens = Vec::from_iter(function_tokens);
    if let Some(last) = function_tokens.last_mut() {
        if let proc_macro2::TokenTree::Group(group) = last {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
                *last = parse_quote! { {
                    use ::ez::errors::throw;
                    #last
                } };
            }
        };
    }
    let function_tokens = proc_macro2::TokenStream::from_iter(function_tokens);

    quote! {
        #[::ez::deps::fehler::throws(#attribute_tokens)]
        #function_tokens
    }
    .into_token_stream()
    .into()
}

#[proc_macro_attribute]
pub fn panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = proc_macro2::TokenStream::from(attribute_tokens);
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::errors::Panic }
    } else {
        quote! { compile_error!("#[ez::panics] macro takes no arguments") }
    };


    let function_tokens = proc_macro2::TokenStream::from(function_tokens);
    let mut function_tokens = Vec::from_iter(function_tokens);
    if let Some(last) = function_tokens.last_mut() {
        if let proc_macro2::TokenTree::Group(group) = last {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
                *last = parse_quote! { {
                    use ::ez::errors::throw;
                    #last
                } };
            }
        };
    }
    let function_tokens = proc_macro2::TokenStream::from_iter(function_tokens);

    quote! {
        #[::ez::deps::fehler::throws(#attribute_tokens)]
        #function_tokens
    }
    .into_token_stream()
    .into()
}

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
                .push(parse_quote!(_: ::ez::main::Ignored));
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::main::Ignored));
        },
        1 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::main::Ignored));
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

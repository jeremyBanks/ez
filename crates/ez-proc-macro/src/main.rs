use {
    crate::errors::*,
    proc_macro2::TokenStream,
    quote::{quote, quote_spanned, ToTokens},
    syn::{parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, ItemFn, Visibility},
};

pub fn main(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    if !attribute_tokens.is_empty() {
        syn::Error::new(attribute_tokens.span(), "#[ez::main] macro takes no arguments");
    };

    let function_tokens = tryify_trailing_block(function_tokens)?;
    let mut inner_function: ItemFn = syn::parse2(function_tokens)?;
    let mut outer_function = inner_function.clone();

    // inner function must always take two arguments
    match inner_function.sig.inputs.len() {
        0 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote_spanned! { inner_function.sig.inputs.span() => _: ::ez::__::IteratorDropper });
            inner_function
                .sig
                .inputs
                .push(parse_quote_spanned! { inner_function.sig.inputs.span() => _: ::ez::__::IteratorDropper });
        },
        1 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote_spanned! { inner_function.sig.inputs.span() => _: ::ez::__::IteratorDropper });
        },
        2 => {},
        _ => {
            return Ok(quote_spanned! {inner_function.sig.inputs.span()=>
                compile_error!("#[ez::main] function must have at most 2 arguments (for example, `fn main(args: Vec<String>, env: Vec<(String, String)>)`).");
            }.into_token_stream())
        },
    }

    inner_function.sig.output = wrap_return_with_result(
        inner_function.sig.output.clone(),
        parse_quote_spanned! { inner_function.sig.output.span() => ::ez::Error },
    );

    outer_function.sig.inputs = Punctuated::new();
    outer_function.sig.output =
        parse_quote_spanned! { outer_function.sig.output.span() => -> Result<(), ::ez::Error> };
    outer_function.sig.asyncness = None;

    if inner_function.sig.asyncness.is_some() {
        let block = inner_function.block.clone();
        inner_function.block = parse_quote_spanned! { inner_function.block.span() => {
            ::ez::__::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?
                .block_on(async #block)
        } };

        inner_function.sig.asyncness = None;
    }

    inner_function.vis = Visibility::Inherited;
    let ident = inner_function.sig.ident.clone();

    outer_function.block = parse_quote_spanned! { outer_function.block.span() => {
        #inner_function
        ::ez::__::entry_point(env!("CARGO_CRATE_NAME"), #ident)
    } };

    Ok(outer_function.to_token_stream())
}

pub fn ly(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let item = main(attribute_tokens, function_tokens)?;
    Ok(quote! {
        use ::ez::prelude::*;

        #item
    })
}

use {
    proc_macro2::TokenStream,
    quote::{quote, quote_spanned, ToTokens},
    syn::{parse_quote, spanned::Spanned, ReturnType},
};

/// Returns from the enclosing function with a [`Result::Err`].
///
/// This will contain either a provided error object, or an error message
/// (if the first argument is a formatting string literal). Error messages
/// are formatted into [`eyre::Report`]s using [`eyre::eyre!`], so you'll
/// get an error if you try to use an error message in a function that
/// expects a more-specific error type in its [`Result`].
#[macro_export]
macro_rules! throw {
    ($msg:literal $(,)?) => {
        $crate::deps::fehler::throw!(eyre::Report::msg($msg));
    };

    ($msg:literal $(, $rest:tt)* $(,)?) => {
        $crate::deps::fehler::throw!(eyre::Report::msg(format!($msg $(, $rest)*)));
    };

    ($error:expr $(,)?) => {
        $crate::deps::fehler::throw!($error);
    };

    ($(,)?) => {
        $crate::deps::fehler::throw!();
    };
}

pub fn throws(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::deps::eyre::Report }
    } else {
        attribute_tokens
    };

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
}

pub fn panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::dysfunctional::ErrorPanicker }
    } else {
        quote! { compile_error!("#[ez::panics] macro takes no arguments") }
    };

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
}

pub fn main(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    if !attribute_tokens.is_empty() {
        return quote! { compile_error!("#[ez::main] macro takes no arguments") }
            .into_token_stream();
    };

    let mut inner_function: syn::ItemFn = syn::parse2(function_tokens).unwrap();
    let mut outer_function = inner_function.clone();

    match inner_function.sig.inputs.len() {
        0 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::dysfunctional::IteratorDropper));
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::dysfunctional::IteratorDropper));
        },
        1 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::dysfunctional::IteratorDropper));
        },
        2 => {},
        _ => {
            return quote_spanned! {inner_function.sig.inputs.span()=>
                compile_error!("#[ez::main] function must have at most 2 arguments (for example, `fn main(args: Vec<String>, env: Vec<(String, String)>)`).");
            }.into_token_stream()
        },
    }

    let extra_inner_attributes = if inner_function.sig.asyncness.is_some() {
        quote! {
            #[::ez::deps::tokio::main(flavor = "current_thread")]
        }
    } else {
        quote! {}
    };

    outer_function.sig.inputs = syn::punctuated::Punctuated::new();
    if outer_function.sig.output == ReturnType::Default {
        outer_function.sig.output = parse_quote! { -> Result<(), ::ez::deps::eyre::Report> };
    } else {
        // emit a compile error telling the user that we don't allow this
        // macro to be used with an explicit return type
        return quote_spanned! {inner_function.sig.output.span()=>
            compile_error!("#[ez::main] macro cannot be used with an explicit return type.");
        }
        .into_token_stream();
    }
    outer_function.sig.asyncness = None;

    let block = inner_function.block.clone();
    if inner_function.sig.output == ReturnType::Default {
        inner_function.sig.output = parse_quote! { -> Result<(), ::ez::deps::eyre::Report> };
    }
    inner_function.block = parse_quote! { {
        Ok(#block)
    } };
    inner_function.vis = syn::Visibility::Inherited;
    inner_function.sig.ident = parse_quote! { ez_inner_main };

    outer_function.block = parse_quote! { {
        #extra_inner_attributes
        #inner_function;
        ::ez::main::run(env!("CARGO_CRATE_NAME"), ez_inner_main)
    } };

    outer_function.to_token_stream()
}

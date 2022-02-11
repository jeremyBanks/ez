//! internal

use {
    proc_macro2::TokenStream,
    quote::{quote, quote_spanned, ToTokens},
    syn::{parse_quote, spanned::Spanned, ReturnType},
};

pub mod dysfunctional;

pub mod main;

pub mod deps {
    pub use {
        crate::{alloc, core, std},
        eyre, ezio, fehler, proc_macro2, quote, syn, tokio, tracing,
    };
}

pub extern crate alloc;
pub extern crate core;
pub extern crate std;

#[macro_export]
macro_rules! throw {
    ($msg:literal $(,)?) => { {
        ::ez::internal::deps::core::result::Result::Err(::ez::internal::deps::eyre::Report::msg($msg))?;
        unreachable!()
    } };

    ($msg:literal $(, $rest:tt)* $(,)?) => { {
        ::ez::internal::deps::core::result::Result::Err(::ez::internal::deps::eyre::Report::msg(format!($msg $(, $rest)*)))?;
        unreachable!()
    } };

    ($error:expr $(,)?) => { {
        ::ez::internal::deps::core::result::Result::Err($error)?;
        unreachable!()
    } };

    ($(,)?) => { {
        ::ez::internal::deps::core::result::Result::Err(::ez::internal::deps::core::default::Default::default())?;
        unreachable!()
    } };
}

pub fn try_throws(_attribute_tokens: TokenStream, _function_tokens: TokenStream) -> TokenStream {
    todo!();
}

pub fn throws(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::internal::deps::eyre::Report }
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
        #[::ez::internal::deps::fehler::throws(#attribute_tokens)]
        #function_tokens
    }
    .into_token_stream()
}

pub fn panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::internal::dysfunctional::ErrorPanicker }
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
        #[::ez::internal::deps::fehler::throws(#attribute_tokens)]
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
                .push(parse_quote!(_: ::ez::internal::dysfunctional::IteratorDropper));
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::internal::dysfunctional::IteratorDropper));
        },
        1 => {
            inner_function
                .sig
                .inputs
                .push(parse_quote!(_: ::ez::internal::dysfunctional::IteratorDropper));
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
            #[::ez::internal::deps::tokio::main(flavor = "current_thread")]
        }
    } else {
        quote! {}
    };

    outer_function.sig.inputs = syn::punctuated::Punctuated::new();
    outer_function.sig.output = parse_quote! { -> Result<(), ::ez::internal::deps::eyre::Report> };
    outer_function.sig.asyncness = None;

    let block = inner_function.block.clone();

    if let ReturnType::Type(_, ref inner) = inner_function.sig.output {
        let output = inner.clone();
        inner_function.sig.output =
            parse_quote! { -> Result<#output, ::ez::internal::deps::eyre::Report> };
    } else {
        inner_function.sig.output =
            parse_quote! { -> Result<(), ::ez::internal::deps::eyre::Report> };
    }
    inner_function.block = parse_quote! { {
        Ok(#block)
    } };
    inner_function.vis = syn::Visibility::Inherited;
    inner_function.sig.ident = parse_quote! { ez_inner_main };

    outer_function.block = parse_quote! { {
        #extra_inner_attributes
        #inner_function;
        ::ez::internal::main::run(env!("CARGO_CRATE_NAME"), ez_inner_main)
    } };

    outer_function.to_token_stream()
}

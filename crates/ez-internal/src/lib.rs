//! internal

use {
    proc_macro2::TokenStream,
    quote::{quote, quote_spanned, ToTokens},
    syn::{
        fold::Fold, parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned,
        Block, ExprAsync, ExprClosure, ExprReturn, ImplItemMethod, ItemFn, ReturnType, Visibility,
    },
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

pub fn try_throws(
    _attribute_tokens: TokenStream,
    _function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    todo!();
}

pub fn return_ok(block: Block) -> Block {
    struct Folder;
    impl Fold for Folder {
        fn fold_expr_return(&mut self, expr: ExprReturn) -> ExprReturn {
            let inner = expr.expr.clone();
            parse_quote_spanned! { expr.span() =>
                return ::ez::internal::deps::core::result::Result::Ok(#inner)
            }
        }

        fn fold_item_fn(&mut self, item_fn: ItemFn) -> ItemFn {
            item_fn
        }

        fn fold_expr_closure(&mut self, expr_closure: ExprClosure) -> ExprClosure {
            expr_closure
        }

        fn fold_expr_async(&mut self, expr_async: ExprAsync) -> ExprAsync {
            expr_async
        }
    }

    Folder.fold_block(block)
}

pub fn throws(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::internal::deps::eyre::Report }
    } else {
        attribute_tokens
    };

    let mut function_tokens = Vec::from_iter(function_tokens);

    if let Some(last) = function_tokens.last_mut() {
        if let proc_macro2::TokenTree::Group(group) = last {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
                let block: syn::Block = syn::parse2(last.clone().into_token_stream())?;
                let block = return_ok(block);
                *last = parse_quote! { {
                    use ::ez::errors::throw;
                    ::ez::internal::deps::core::result::Result::Ok({#block})
                } };
            }
        };
    }

    let mut function: ImplItemMethod = syn::parse2(function_tokens.into_iter().collect())?;

    match &function.sig.output {
        ReturnType::Default => {
            function.sig.output = parse_quote! { -> ::ez::internal::deps::core::result::Result<(), #attribute_tokens> };
        },
        ReturnType::Type(_, t) => {
            function.sig.output = parse_quote! { -> ::ez::internal::deps::core::result::Result<#t, #attribute_tokens> };
        },
    }

    Ok(function.into_token_stream())
}

pub fn panics(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let attribute_tokens = if attribute_tokens.is_empty() {
        quote! { ::ez::internal::dysfunctional::ErrorPanicker }
    } else {
        eyre::bail!("#[ez::panics] macro takes no arguments");
    };

    todo!()
}

pub fn main(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    if !attribute_tokens.is_empty() {
        eyre::bail!("#[ez::main] macro takes no arguments");
    };

    let mut inner_function: ItemFn = syn::parse2(function_tokens)?;
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
            return Ok(quote_spanned! {inner_function.sig.inputs.span()=>
                compile_error!("#[ez::main] function must have at most 2 arguments (for example, `fn main(args: Vec<String>, env: Vec<(String, String)>)`).");
            }.into_token_stream())
        },
    }

    let extra_inner_attributes = if inner_function.sig.asyncness.is_some() {
        quote! {
            #[::ez::internal::deps::tokio::main(flavor = "current_thread")]
        }
    } else {
        quote! {}
    };

    outer_function.sig.inputs = Punctuated::new();
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
    inner_function.vis = Visibility::Inherited;
    inner_function.sig.ident = parse_quote! { ez_inner_main };

    outer_function.block = parse_quote! { {
        #extra_inner_attributes
        #inner_function;
        ::ez::internal::main::run(env!("CARGO_CRATE_NAME"), ez_inner_main)
    } };

    Ok(outer_function.to_token_stream())
}

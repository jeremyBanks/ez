use ::{
    proc_macro2::{Delimiter, Ident, TokenStream, TokenTree},
    quote::ToTokens,
    syn::{
        fold::Fold, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Block,
        ExprAsync, ExprClosure, ExprReturn, ImplItemMethod, ItemFn, Path, ReturnType,
    },
};

// We use this as a general purpose function representation because
// its supported syntax seems to be a superset of other function types.
type Function = ImplItemMethod;

/// Returns the training block of this token stream, if it has one.
fn trailing_block(tokens: &TokenStream) -> Result<Option<Block>, syn::Error> {
    let mut tokens = Vec::from_iter(tokens.clone());

    if let Some(trailing) = tokens.last_mut() {
        if let TokenTree::Group(group) = &trailing {
            if group.delimiter() == Delimiter::Brace {
                return Ok(Some(syn::parse2(trailing.into_token_stream())?));
            }
        }
    }

    Ok(None)
}

/// Wrap every return statement in `Ok`, but don't recur into nested
/// functions/closures/async blocks.
pub fn wrap_returns_in_ok(block: Block) -> Block {
    struct Folder;
    impl Fold for Folder {
        fn fold_expr_return(&mut self, expr: ExprReturn) -> ExprReturn {
            let inner = expr.expr.clone();
            parse_quote_spanned! { expr.span() =>
                return ::ez::__::Ok(#inner)
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

/// If this token stream has a trailing block, import `throw!` and wrap every
/// return value in `Ok`.
pub fn tryify_trailing_block(tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let mut tokens = Vec::from_iter(tokens);

    if let Some(last) = tokens.last_mut() {
        if let TokenTree::Group(group) = last {
            if group.delimiter() == Delimiter::Brace {
                let block: syn::Block = syn::parse2(last.clone().into_token_stream())?;
                let block = wrap_returns_in_ok(block);
                *last = parse_quote_spanned! { block.span() => {
                    #[allow(unused_imports)]
                    use ::ez::throw;
                    let _ez_inner = #block;
                    #[allow(unreachable_code)]
                    ::ez::__::Ok(_ez_inner)
                } };
            }
        };
    }

    Ok(tokens.into_iter().collect())
}

// Wraps a `ReturnType` in a `Result` with the indicated `error_type`.
pub fn wrap_return_with_result(return_type: ReturnType, error_type: Path) -> ReturnType {
    match &return_type {
        ReturnType::Default => {
            parse_quote_spanned! { return_type.span() => -> ::ez::__::Result<(), #error_type> }
        }
        ReturnType::Type(_, t) => {
            parse_quote_spanned! { return_type.span() => -> ::ez::__::Result<#t, #error_type> }
        }
    }
}

pub fn throws(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let error_type: Path = if attribute_tokens.is_empty() {
        parse_quote_spanned! { attribute_tokens.span() => ::ez::Error }
    } else {
        syn::parse2(attribute_tokens)?
    };

    let function_tokens = tryify_trailing_block(function_tokens)?;

    let mut function: Function = syn::parse2(function_tokens.into_iter().collect())?;

    function.sig.output = wrap_return_with_result(function.sig.output, error_type);

    Ok(function.into_token_stream())
}

fn panics(function_tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let function_tokens = tryify_trailing_block(function_tokens)?;

    let mut function: Function = syn::parse2(function_tokens.into_iter().collect())?;

    let block = function.block.clone();
    function.block = parse_quote_spanned! {
        function.block.span() => {
            #[allow(unused_mut, clippy::needless_late_init)]
            let mut _ez_inner;
            _ez_inner = move || -> ::ez::__::Result<_, ::ez::__::ErrorPanicker> #block;
            _ez_inner().unwrap()
        }
    };

    Ok(function.into_token_stream())
}

pub fn try_throws(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let has_block = trailing_block(&function_tokens)?.is_some();
    let source: Function = syn::parse2(function_tokens.clone())?;
    let args = parameters_to_arguments(&source.sig.inputs);

    let panicking_ident = source.sig.ident;
    let throwing_ident = format!("try_{}", panicking_ident);
    let throwing_ident = Ident::new(&throwing_ident, panicking_ident.span());

    let throwing = throws(
        attribute_tokens,
        if !has_block {
            let mut panicking: Function = syn::parse2(function_tokens.clone())?;
            panicking.block = parse_quote_spanned! { function_tokens.span() => {
                Self::#panicking_ident(#args)
            } };
            panicking.into_token_stream()
        } else {
            function_tokens.clone()
        },
    )?;
    let mut throwing: Function = syn::parse2(throwing)?;
    throwing.sig.ident = throwing_ident.clone();

    let panicking = panics(if !has_block {
        let mut panicking: Function = syn::parse2(function_tokens.clone())?;
        panicking.block = parse_quote_spanned! { function_tokens.span() => {
            Self::#throwing_ident(#args)?
        } };
        panicking.into_token_stream()
    } else {
        function_tokens
    })?;

    Ok(parse_quote_spanned! {
        panicking.span() =>
        #throwing
        #panicking
    })
}

fn parameters_to_arguments(
    parameters: &Punctuated<syn::FnArg, syn::Token![,]>,
) -> Punctuated<syn::Ident, syn::Token![,]> {
    parameters
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(receiver) => syn::Ident::new("self", receiver.span()),
            syn::FnArg::Typed(arg) => match &*arg.pat {
                syn::Pat::Ident(pat) => pat.ident.clone(),
                _ => panic!("unsupported pattern in arguments"),
            },
        })
        .collect()
}

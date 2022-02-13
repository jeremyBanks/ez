use {
    proc_macro2::{Ident, TokenStream},
    quote::{quote_spanned, ToTokens},
    syn::{
        fold::Fold, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, visit::Visit,
        Block, Expr, ExprAsync, ExprClosure, ExprReturn, ImplItemMethod, Item, ItemFn, ItemImpl,
        ItemTrait, Path, ReturnType, Visibility,
    },
};

// We use this as a general purpose function representation because
// its supported syntax seems to be a superset of other function types.
type Function = ImplItemMethod;

/// Wrap every return statement in `Ok`, but don't recur into nested
/// functions/closures/async blocks.
fn wrap_returns_in_ok(block: Block) -> Block {
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
fn tryify_trailing_block(tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let mut tokens = Vec::from_iter(tokens);

    if let Some(last) = tokens.last_mut() {
        if let proc_macro2::TokenTree::Group(group) = last {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
                let block: syn::Block = syn::parse2(last.clone().into_token_stream())?;
                let block = wrap_returns_in_ok(block);
                *last = parse_quote_spanned! { block.span() => {
                    #[allow(unused_imports)]
                    use ::ez::throw;
                    let _inner = #block;
                    #[allow(unreachable_code)]
                    ::ez::__::Ok(_inner)
                } };
            }
        };
    }

    Ok(tokens.into_iter().collect())
}

/// Determines whether a function definition contains a reference to `self` or
/// `Self`, either in the signature or in the body (but not recurring into
/// nested trait or impl blocks).
///
/// Returns true if a path containing `Self` or `self` is found, false
/// otherwise.
fn contains_self(function: Function) -> Result<bool, eyre::Report> {
    struct SelfFinder {
        found: bool,
    }
    impl<'ast> Visit<'ast> for SelfFinder {
        fn visit_path(&mut self, path: &'ast Path) {
            if path.leading_colon.is_none() {
                for segment in &path.segments {
                    if segment.ident == "self" || segment.ident == "Self" {
                        self.found = true;
                        return;
                    }
                }
            }
            syn::visit::visit_path(self, path);
        }

        fn visit_item(&mut self, item: &'ast Item) {
            if self.found {
                return;
            }
            syn::visit::visit_item(self, item);
        }

        fn visit_expr(&mut self, expr: &'ast Expr) {
            if self.found {
                return;
            }
            syn::visit::visit_expr(self, expr);
        }

        fn visit_item_trait(&mut self, _: &'ast ItemTrait) {
            // don't recur into nested trait blocks
        }

        fn visit_item_impl(&mut self, _: &'ast ItemImpl) {
            // don't recur into nested impl blocks
        }
    }

    let mut finder = SelfFinder { found: false };

    finder.visit_impl_item_method(&function);

    Ok(finder.found)
}

// Wraps a `ReturnType` in a `Result` with the indicated `error_type`.
fn wrap_return_with_result(return_type: ReturnType, error_type: Path) -> ReturnType {
    match &return_type {
        ReturnType::Default => {
            parse_quote_spanned! { return_type.span() => -> ::ez::__::Result<(), #error_type> }
        },
        ReturnType::Type(_, t) => {
            parse_quote_spanned! { return_type.span() => -> ::ez::__::Result<#t, #error_type> }
        },
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

pub fn panics(function_tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let function_tokens = tryify_trailing_block(function_tokens)?;

    let mut function: Function = syn::parse2(function_tokens.into_iter().collect())?;

    let block = function.block.clone();
    function.block = parse_quote_spanned! {
        function.block.span() => {
            let _inner = move || -> ::ez::__::Result<_, ::ez::__::ErrorPanicker> #block;
            _inner().unwrap()
        }
    };

    Ok(function.into_token_stream())
}

pub fn try_throws(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    let panicking = panics(function_tokens.clone())?;
    let throwing = throws(attribute_tokens, function_tokens.clone())?;

    let mut throwing: Function = syn::parse2(throwing)?;
    let throwing_ident = format!("try_{}", throwing.sig.ident);
    throwing.sig.ident = Ident::new(&throwing_ident, throwing.sig.span());

    Ok(parse_quote_spanned! {
        function_tokens.span() =>
        #throwing
        #panicking
    })
}

pub fn main(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> Result<TokenStream, eyre::Report> {
    if !attribute_tokens.is_empty() {
        eyre::bail!("#[ez::main] macro takes no arguments");
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

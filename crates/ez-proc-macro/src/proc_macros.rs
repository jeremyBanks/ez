use {
    proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree},
    quote::{quote_spanned, ToTokens},
    std::borrow::{Borrow, BorrowMut},
    syn::{
        fold::Fold, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Block,
        ExprAsync, ExprClosure, ExprReturn, ImplItemMethod, ItemFn, Path, ReturnType, Visibility,
    },
};

// We use this as a general purpose function representation because
// its supported syntax seems to be a superset of other function types.
type Function = ImplItemMethod;

/// Returns the training block of this token stream, if it has one.
fn trailing_block(tokens: &TokenStream) -> Result<Option<Block>, eyre::Report> {
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
fn tryify_trailing_block(tokens: TokenStream) -> eyre::Result<TokenStream> {
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
) -> eyre::Result<TokenStream> {
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

fn panics(function_tokens: TokenStream) -> eyre::Result<TokenStream> {
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
) -> eyre::Result<TokenStream> {
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

pub fn main(
    attribute_tokens: TokenStream,
    function_tokens: TokenStream,
) -> eyre::Result<TokenStream> {
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

impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
trait TokenTreeExt: Borrow<TokenTree> + BorrowMut<TokenTree> {
    fn is_group(&self) -> bool {
        matches!(self.borrow(), TokenTree::Group(_))
    }

    fn is_ident(&self) -> bool {
        matches!(self.borrow(), TokenTree::Ident(_))
    }

    fn children(&self) -> eyre::Result<Vec<TokenTree>> {
        if let TokenTree::Group(g) = self.borrow() {
            Ok(g.stream().into_iter().collect())
        } else {
            panic!("expected a group")
        }
    }

    fn only(&self) -> eyre::Result<TokenTree> {
        let children = self.children()?;
        assert_eq!(children.len(), 1);
        Ok(children[0].clone())
    }

    fn map<R>(&self, f: impl Fn(TokenTree) -> R) -> Vec<R> {
        self.children().unwrap().into_iter().map(f).collect()
    }

    fn for_each(&self, f: impl FnMut(TokenTree)) {
        self.children().unwrap().into_iter().for_each(f)
    }

    fn ident(&self) -> eyre::Result<Ident> {
        if let TokenTree::Ident(i) = self.borrow() {
            Ok(i.clone())
        } else {
            eyre::bail!("expected an ident, got: {:?}", self.borrow())
        }
    }
}

/*


*/

// TODO: also export as macro attribute?
//
//
pub fn doop(tokens: TokenStream) -> eyre::Result<TokenStream> {
    #[derive(Debug)]
    struct Repetition {
        ident: Ident,
        replacements: Vec<TokenTree>,
    }

    let input: Vec<TokenTree> = tokens.into_iter().collect();
    assert_eq!(input.len(), 2);

    let repetitions = input[0].map(|t| {
        let children = t.children().unwrap();
        let ident = children[0].only().unwrap().ident().unwrap();
        let replacements = children[1].children().unwrap();
        Repetition {
            ident,
            replacements,
        }
    });

    let block = input[1].children()?;

    let mut output: TokenStream = block.into_iter().collect();

    for Repetition {
        ident,
        replacements,
    } in repetitions
    {
        let base = output.clone();
        output = TokenStream::new();

        for replacement in replacements {
            output.extend(replace_ident_in_token_stream(
                base.clone(),
                &ident,
                [replacement.clone()].into_iter().collect(),
            )?);
        }
    }

    Ok(output.into_iter().collect())
}

fn replace_ident_in_token_stream(
    input: TokenStream,
    ident: &Ident,
    replacement: TokenStream,
) -> eyre::Result<TokenStream> {
    let mut output = TokenStream::new();
    for token in input {
        match token {
            TokenTree::Ident(ref candidate) =>
                if *candidate == *ident {
                    output.extend(replacement.clone().into_token_stream());
                } else {
                    output.extend([token.clone()]);
                },

            TokenTree::Group(group) => output.extend([TokenTree::Group(Group::new(
                group.delimiter(),
                replace_ident_in_token_stream(group.stream(), ident, replacement.clone())?,
            ))]),
            _ => output.extend([token.clone()]),
        }
    }
    Ok(output)
}

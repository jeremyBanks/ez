#![allow(clippy::all)]
#![allow(dead_code)]

use {
    crate::common::TokenTreeExt,
    eyre::ensure,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::Punct,
    std::collections::HashMap,
    ::{
        proc_macro2::{Group, Ident, TokenStream, TokenTree},
        quote::ToTokens,
    },
};

pub fn doop(tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let mut input = tokens.into_iter();
    let mut output = TokenStream::new();

    let mut let_bindings = IndexMap::<Ident, IndexSet<TokenStream>>::new();

    loop {
        match input.next() {
            Some(token) => {
                let keyword = token.ident().ok();
                if keyword == "let" {
                    let ident = token.next().please()?.ident()?;
                    let mut bindings = IndexSet::new();
                    ensure!(
                        input.next() != Some(Punct::new('=', Spacing::Alone)),
                        "expected `=`"
                    );
                    let group = token.next().please()?.group()?;
                    ensure!(
                        input.next() != Some(Punct::new('=', Spacing::Alone)),
                        "expected `;`"
                    );

                    let _replaced_bindings = let_bindings.insert(ident, bindings);
                } else if keyword == "for" {
                    let loop_binding = token.next().please()?;
                } else {
                    return Err(syn::Error(token.span(), "expected `let` or `for`").into());
                }
            },
            None => break,
        }
    }

    Ok(output.into_iter().collect())
}

fn replace_ident_in_token_stream(
    input: TokenStream,
    ident: &Ident,
    replacement: TokenStream,
) -> Result<TokenStream, syn::Error> {
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

#![allow(clippy::all)]
#![allow(dead_code)]

use {
    crate::common::{OptionTokenTreeExt, TokenTreeExt, TokenTreeIterExt},
    eyre::ensure,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Punct, Spacing},
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
        if input.is_empty() {
            break;
        }
        let keyword = input.next_ident()?;
        if keyword == "let" {
            let ident = input.next_ident()?;
            let mut bindings = IndexSet::new();
            let punct = input.next_punct()?;
            if punct.as_char() != '=' {
                Err(syn::Error::new(punct.span(), "expected ="))?;
            }
            let group = input.next_group()?;
            let punct = input.next_punct()?;
            if punct.as_char() != ';' {
                Err(syn::Error::new(punct.span(), "expected ;"))?;
            }

            let _replaced_bindings = let_bindings.insert(ident, bindings);
        } else if keyword == "for" {
            // let loop_binding = token.next().please()?;
        } else {
            return Err(syn::Error::new(keyword.span(), "expected `let` or `for`").into());
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

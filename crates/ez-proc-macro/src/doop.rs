#![allow(clippy::all)]
#![allow(dead_code)]

use {
    crate::common::TokenTreeIterExt,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Group, Ident, Span, TokenStream, TokenTree},
    quote::ToTokens,
};

mod input {
    use proc_macro2::{Group, Ident};

    #[derive(Debug, Clone)]
    pub struct Root {
        pub items: Vec<Item>,
    }

    #[derive(Debug, Clone)]
    pub enum Item {
        Let { binding: Binding },
        For { bindings: Vec<Binding>, body: Group },
    }

    #[derive(Debug, Clone)]
    pub struct Binding {
        pub name: Ident,
        pub terms: Vec<Binding>,
    }

    #[derive(Debug, Clone)]
    pub enum Term {
        Add { items: Vec<Binding> },
        Remove { items: Vec<Binding> },
    }
}

enum BindingItem {
    TokenList { token_list: Vec<TokenTree> },
    BoundName { name: Ident },
}

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
            input.next_puncts_eq("=")?;

            if let Ok(group) = input.next_group() {
            } else if let Ok(ident) = input.next_ident() {
            } else {
                return Err(syn::Error::new(
                    input
                        .next()
                        .map(|tt| tt.span())
                        .unwrap_or_else(Span::call_site),
                    "expected group or identifier",
                )
                .into());
            }

            // next_doop_binding
            // which is made up of next_doop_bindings delimited by + or -

            let group = input.next_group()?;

            input.next_puncts_eq(";")?;

            // let mut bindings = IndexSet::new();
            // let _replaced_bindings = let_bindings.insert(ident, bindings);
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

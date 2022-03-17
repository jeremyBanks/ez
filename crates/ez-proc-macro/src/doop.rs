#![allow(clippy::all)]
#![allow(dead_code)]

use {
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Group, Ident, Span, TokenStream, TokenTree},
    quote::ToTokens,
};

pub mod input {
    use {
        crate::common::{TokenTreeExt, TokenTreeIterExt},
        proc_macro2::{Group, Ident, Span, TokenStream},
    };

    #[derive(Debug, Clone, Default)]
    pub struct Root {
        pub items: Vec<Item>,
    }

    #[derive(Debug, Clone)]
    pub enum Item {
        Let {
            target: Ident,
            binding: Binding,
        },
        For {
            target: ForBindingTarget,
            bindings: Vec<Binding>,
            body: Group,
        },
    }

    #[derive(Debug, Clone)]
    pub enum ForBindingTarget {
        Ident(Ident),
        Tuple(Vec<Ident>),
    }

    #[derive(Debug, Clone)]
    pub struct Binding {
        pub ident: Ident,
        pub terms: Vec<Binding>,
    }

    #[derive(Debug, Clone)]
    pub enum Term {
        Add { items: Vec<Binding> },
        Remove { items: Vec<Binding> },
    }

    pub fn next_terms(
        iter: &mut proc_macro2::token_stream::IntoIter,
    ) -> Result<Vec<Term>, syn::Error> {
        let mut terms = Vec::new();
        let mut negated = false;

        loop {
            let term = iter.clone().next_tt()?;
            let mut items = Vec::new();

            if let Ok(group) = term.group() {
            } else if let Ok(ident) = term.ident() {
            } else {
                iter.err_on_next("expected an identifier or group")?;
            }
            let _ = iter.next();

            terms.push(if negated {
                Term::Remove { items }
            } else {
                Term::Add { items }
            });

            if iter.next_puncts_eq("+").is_ok() {
                negated = false;
                continue;
            } else if iter.next_puncts_eq("-").is_ok() {
                negated = true;
                continue;
            } else {
                break;
            }
        }

        Ok(terms)
    }

    pub fn from_tokens(tokens: TokenStream) -> Result<Root, eyre::Report> {
        let mut root = Root::default();

        let mut iter = tokens.into_iter();

        while !iter.is_empty() {
            match iter.next_ident()?.to_string().as_ref() {
                "let" => {
                    // let terms = vec![];

                    let ident = iter.next_ident()?;
                    iter.next_puncts_eq("=")?;

                    if let Ok(group) = iter.next_group() {
                    } else if let Ok(ident) = iter.next_ident() {
                    } else {
                        iter.err_on_next("expected group or identifier")?;
                    }

                    // next_doop_binding
                    // which is made up of next_doop_bindings delimited by + or -

                    let group = iter.next_group()?;

                    iter.next_puncts_eq(";")?;

                    // let mut bindings = IndexSet::new();
                    // let _replaced_bindings = let_bindings.insert(ident,
                    // bindings);

                    // root.items.push(Item::Let {
                    //     binding: Binding { ident, terms },
                    // });
                }

                "for" => {

                    // let body;
                    // let mut bindings = vec![];

                    // // let loop_binding = token.next().please()?;
                    // root.items.push(Item::For { bindings, body });
                }

                _ => {
                    iter.err_on_next("expected `let` or `for`")?;
                }
            }
        }

        Ok(root)
    }
}

pub fn doop(tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let input = input::from_tokens(tokens)?;

    let mut output = TokenStream::new();

    let mut let_bindings = IndexMap::<Ident, IndexSet<TokenStream>>::new();

    Ok(output.into_iter().collect())
}

// We should also support punctuation subsitution?
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

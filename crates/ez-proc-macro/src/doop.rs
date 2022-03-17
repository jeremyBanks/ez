#![allow(clippy::all)]
#![allow(dead_code)]

use {
    crate::common::{TokenTreeExt, TokenTreeIterExt},
    derive_syn_parse::Parse,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Group, Ident, Span, TokenStream, TokenTree},
    quote::ToTokens,
    syn::{parse::Parse, punctuated::Punctuated, Token},
};

#[derive(Parse, Debug, Clone)]
pub struct DoopBlock {
    pub items: Vec<DoopItem>,
}

#[derive(Parse, Debug, Clone)]
pub enum DoopItem {
    #[peek(Token![let], name = "let")]
    Let(DoopLetItem),
    #[peek(Token![for], name = "for")]
    For(DoopForItem),
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForItem {
    pub bindings: Vec<DoopForBinding>,
    #[brace]
    pub braces: syn::token::Brace,
    #[inside(braces)]
    pub body: proc_macro2::TokenTree,
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForBinding {
    #[prefix(Token![for])]
    pub target: ForBindingTarget,
    #[prefix(Token![in])]
    pub first_term: BindingTerm,
    pub rest_terms: Vec<RestTerm>,
}

#[derive(Parse, Debug, Clone)]
pub struct ForBindingTarget {}

#[derive(Parse, Debug, Clone)]
pub struct DoopLetItem {
    pub _let: Token![let],
    pub ident: Ident,
    pub _eq: Token![=],
    pub first_term: BindingTerm,
    pub rest_terms: Vec<RestTerm>,
    pub semi: Token![;],
}

#[derive(Parse, Debug, Clone)]
pub struct RestTerm {
    pub operation: PlusOrMinus,
    pub term: BindingTerm,
}

#[derive(Parse, Debug, Clone)]
pub enum PlusOrMinus {
    #[peek(Token![+], name = "plus")]
    Plus(Token![+]),
    #[peek(Token![-], name = "minus")]
    Minus(Token![-]),
}

#[derive(Parse, Debug, Clone)]
pub enum BindingTerm {
    #[peek(syn::Ident, name = "ident")]
    Ident(Ident),
    #[peek(syn::token::Bracket, name = "bracket list")]
    BracketedList(BracketList),
    #[peek(syn::token::Paren, name = "paren list")]
    ParenList(ParenList),
    #[peek(syn::token::Bracket, name = "brace list")]
    BraceList(BraceList),
}

#[derive(Parse, Debug, Clone)]
pub struct BracketList {
    #[bracket]
    bracket: syn::token::Bracket,
    #[inside(bracket)]
    _todo: TokenStream,
}

#[derive(Parse, Debug, Clone)]
pub struct ParenList {
    #[paren]
    paren: syn::token::Paren,
    #[inside(paren)]
    _todo: TokenStream,
}

#[derive(Parse, Debug, Clone)]
pub struct BraceList {
    #[brace]
    brace: syn::token::Brace,
    #[inside(brace)]
    _todo: TokenStream,
}

pub fn next_terms(iter: &mut proc_macro2::token_stream::IntoIter) -> Result<Vec<Term>, syn::Error> {
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

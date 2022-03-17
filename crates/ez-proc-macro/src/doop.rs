#![allow(clippy::all)]
#![allow(dead_code)]

use {
    crate::common::{TokenTreeExt, TokenTreeIterExt},
    derive_syn_parse::Parse,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Group, Ident, Span, TokenStream, TokenTree},
    quote::ToTokens,
    syn::{
        ext::IdentExt,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token, Token,
    },
};

#[derive(Debug, Clone)]
pub struct DoopBlock {
    pub items: Vec<DoopItem>,
}

impl Parse for DoopBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(DoopBlock { items })
    }
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
    #[call(DoopForBinding::parse_vec)]
    pub bindings: Vec<DoopForBinding>,
    #[brace]
    pub braces: token::Brace,
    #[inside(braces)]
    pub body: proc_macro2::TokenTree,
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForBinding {
    #[prefix(Token![for])]
    pub target: ForBindingTarget,
    #[prefix(Token![in])]
    pub first_term: BindingTerm,
    #[call(RestTerm::parse_vec)]
    pub rest_terms: Vec<RestTerm>,
}

impl DoopForBinding {
    pub fn parse_vec(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut for_binding = Vec::new();
        while input.peek(Token![for]) {
            for_binding.push(input.parse()?);
        }
        Ok(for_binding)
    }
}

#[derive(Parse, Debug, Clone)]
pub enum ForBindingTarget {
    #[peek(syn::Ident::peek_any, name = "ident")]
    Ident(Ident),
    #[peek(token::Paren, name = "tuple")]
    Tuple(TupleBinding),
}

#[derive(Parse, Debug, Clone)]
pub struct TupleBinding {
    #[paren]
    pub paren: token::Paren,
    #[inside(paren)]
    #[call(Punctuated::parse_separated_nonempty)]
    pub items: Punctuated<Ident, Token![,]>,
}

#[derive(Parse, Debug, Clone)]
pub struct DoopLetItem {
    #[prefix(Token![let])]
    pub ident: Ident,
    #[prefix(Token![=])]
    pub first_term: BindingTerm,
    #[call(RestTerm::parse_vec)]
    pub rest_terms: Vec<RestTerm>,
    pub semi: Token![;],
}

#[derive(Parse, Debug, Clone)]
pub struct RestTerm {
    pub operation: PlusOrMinus,
    pub term: BindingTerm,
}

impl RestTerm {
    pub fn parse_vec(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut rest_terms = Vec::new();
        while input.peek(Token![+]) || input.peek(Token![-]) {
            rest_terms.push(input.parse()?);
        }
        Ok(rest_terms)
    }
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
    #[peek(token::Bracket, name = "bracket list")]
    BracketedList(BracketList),
    #[peek(token::Paren, name = "paren list")]
    ParenList(ParenList),
    #[peek(token::Bracket, name = "brace list")]
    BraceList(BraceList),
}

#[derive(Parse, Debug, Clone)]
pub struct BracketList {
    #[bracket]
    bracket: token::Bracket,
    #[inside(bracket)]
    _todo: TokenStream,
}

#[derive(Parse, Debug, Clone)]
pub struct ParenList {
    #[paren]
    paren: token::Paren,
    #[inside(paren)]
    _todo: TokenStream,
}

#[derive(Parse, Debug, Clone)]
pub struct BraceList {
    #[brace]
    brace: token::Brace,
    #[inside(brace)]
    _todo: TokenStream,
}

pub fn doop(tokens: TokenStream) -> Result<TokenStream, eyre::Report> {
    let input: DoopBlock = syn::parse2(tokens)?;

    println!("{input:#?}");

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

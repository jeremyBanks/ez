use {
    derive_syn_parse::Parse,
    proc_macro2::{Delimiter, Ident, TokenStream, TokenTree},
    syn::{
        ext::IdentExt,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token, Token,
    },
};

/// The top-level `doop!{ ... }` macro invocation block.
#[derive(Debug, Clone)]
pub struct DoopBlock {
    pub items: Vec<DoopBlockItem>,
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
pub enum DoopBlockItem {
    #[peek(Token![let], name = "let SOME_NAME = [1, 2, 3...] + [4] - [1];")]
    Let(DoopLetItem),
    #[peek(Token![type], name = "type")]
    Type(DoopTypeItem),
    #[peek(Token![for], name = "for")]
    For(DoopForItem),
    #[peek(Token![static], name = "static")]
    Static(DoopStaticItem),
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForItem {
    #[call(DoopForBinding::parse_vec)]
    pub bindings: Vec<DoopForBinding>,
    #[brace]
    pub _brace: token::Brace,
    #[inside(_brace)]
    pub body: TokenStream,
}

#[derive(Parse, Debug, Clone)]
pub struct DoopStaticItem {
    #[prefix(Token![static])]
    #[brace]
    pub _brace: token::Brace,
    #[inside(_brace)]
    pub body: TokenStream,
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
pub enum IdentOrUnderscore {
    #[peek(Token![_], name = "Unidentified")]
    Unidentified(Token![_]),
    #[peek(syn::Ident::peek_any, name = "Ident")]
    Ident(Ident),
}

impl IdentOrUnderscore {
    pub fn ident(&self) -> Option<Ident> {
        match self {
            IdentOrUnderscore::Ident(ident) => Some(ident.clone()),
            _ => None,
        }
    }
}

#[derive(Parse, Debug, Clone)]
pub enum ForBindingTarget {
    #[peek(token::Paren, name = "tuple")]
    Tuple(TupleBinding),
    #[peek(syn::Ident::peek_any, name = "ident")]
    Ident(IdentOrUnderscore),
}

#[derive(Parse, Debug, Clone)]
pub struct TupleBinding {
    #[paren]
    pub paren: token::Paren,
    #[inside(paren)]
    #[call(Punctuated::parse_separated_nonempty)]
    pub items: Punctuated<IdentOrUnderscore, Token![,]>,
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
pub struct DoopTypeItem {
    #[prefix(Token![type])]
    pub ident: Ident,
    #[prefix(Token![=])]
    #[call(tokens_to_semicolon)]
    pub tokens: Vec<TokenTree>,
    pub semi: Token![;],
}

pub fn tokens_to_semicolon(input: ParseStream) -> syn::Result<Vec<TokenTree>> {
    let mut tokens = Vec::new();
    while !input.peek(Token![;]) {
        tokens.push(input.parse()?);
    }
    Ok(tokens)
}

#[derive(Parse, Debug, Clone)]
pub struct RestTerm {
    pub operation: AddOrSub,
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
pub enum AddOrSub {
    #[peek(Token![+], name = "add")]
    Add(Token![+]),
    #[peek(Token![-], name = "sub")]
    Sub(Token![-]),
}

#[derive(Parse, Debug, Clone)]
pub enum BindingTerm {
    #[peek(syn::Ident, name = "ident")]
    Ident(Ident),
    #[peek(token::Bracket, name = "bracket list")]
    BracketList(BracketList),
    #[peek(token::Brace, name = "brace list")]
    BraceList(BraceList),
}

#[derive(Parse, Debug, Clone)]
pub struct BracketList {
    #[bracket]
    _bracket: token::Bracket,
    #[inside(_bracket)]
    #[call(Self::parse_entries)]
    pub entries: Vec<TokenStream>,
}

#[derive(Parse, Debug, Clone)]
pub struct BraceList {
    #[brace]
    _brace: token::Brace,
    #[inside(_brace)]
    #[call(Self::parse_entries)]
    pub entries: Vec<TokenStream>,
}

trait GroupList {
    const DELIMITER: Delimiter;

    fn parse_entries(input: ParseStream) -> syn::Result<Vec<TokenStream>> {
        let mut entries: Vec<Vec<TokenTree>> = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![,]) {
                if let Some(last) = entries.last() {
                    if last.is_empty() {
                        return Err(input.error(match Self::DELIMITER {
                            Delimiter::Bracket =>
                                "Missing entry in replacement list.\nIf you want an empty \
                                 replacement, please use empty delimiters `[]` instead.",
                            Delimiter::Brace =>
                                "Missing entry in replacement list.\n\nIf you want an empty \
                                 replacement, please use empty delimiters `{}` instead.",
                            _ => unreachable!(),
                        }));
                    }
                }
                entries.push(Vec::new());
                input.parse::<Token![,]>()?;
            } else {
                let token = input.parse::<TokenTree>()?;
                if entries.is_empty() {
                    entries.push(Vec::new());
                }
                entries.last_mut().unwrap().push(token);
            }
        }

        // Handle a single trailing comma
        if let Some(last) = entries.last() {
            if last.is_empty() {
                entries.pop();
            }
        }

        let entries = entries
            .into_iter()
            .map(|mut entry| {
                if entry.len() == 1 {
                    if let TokenTree::Group(ref mut group) = entry[0] {
                        if group.delimiter() == Self::DELIMITER {
                            entry = group.stream().into_iter().collect();
                        }
                    }
                }
                TokenStream::from_iter(entry)
            })
            .collect();

        Ok(entries)
    }
}

impl GroupList for BracketList {
    const DELIMITER: Delimiter = Delimiter::Bracket;
}

impl GroupList for BraceList {
    const DELIMITER: Delimiter = Delimiter::Brace;
}

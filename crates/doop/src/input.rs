use {
    derive_syn_parse::Parse,
    proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree},
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

#[derive(Debug, Clone)]
pub struct DoopItem {
    pub bindings: DoopForBindings,
    pub item: TokenStream,
}

impl DoopItem {
    pub fn try_from_tokens(attr: TokenStream, item: TokenStream) -> syn::Result<DoopItem> {
        Ok(DoopItem {
            bindings: syn::parse2(attr)?,
            item,
        })
    }
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForBindings {
    #[call(DoopForBinding::parse_vec)]
    bindings: Vec<DoopForBinding>,
}

#[derive(Parse, Debug, Clone)]
pub enum DoopBlockItem {
    #[peek(Token![let], name = "let")]
    Let(DoopLetItem),
    #[peek(Token![for], name = "for")]
    For(DoopForItem),
}

#[derive(Parse, Debug, Clone)]
pub struct DoopForItem {
    pub bindings: DoopForBindings,
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
    BracketedList(BracketList),
    #[peek(token::Paren, name = "paren list")]
    ParenList(ParenList),
    #[peek(token::Bracket, name = "brace list")]
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
pub struct ParenList {
    #[paren]
    _paren: token::Paren,
    #[inside(_paren)]
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
        let mut entries = Vec::new();

        while !input.is_empty() {
            if input.peek(Token![,]) {
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

impl GroupList for ParenList {
    const DELIMITER: Delimiter = Delimiter::Parenthesis;
}

impl GroupList for BraceList {
    const DELIMITER: Delimiter = Delimiter::Brace;
}

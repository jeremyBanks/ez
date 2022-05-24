use {
    proc_macro2::TokenTree,
    quote::ToTokens,
    std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        hash::{Hash, Hasher},
        ops::Deref,
    },
    syn::parse::Parse,
};

/// An immutable token list that supports equality and hashing (by its string
/// representation).
#[derive(Clone, Debug)]
pub struct TokenStream {
    tokens: Vec<TokenTree>,
    as_strings: Vec<String>,
}

impl TokenStream {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        let as_strings = tokens.iter().map(ToString::to_string).collect();
        Self { tokens, as_strings }
    }

    pub fn into_inner(self) -> Vec<TokenTree> {
        self.tokens
    }
}

impl Parse for TokenStream {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tokens: Vec<TokenTree> = Vec::new();
        let mut as_strings: Vec<String> = Vec::new();
        while !input.is_empty() {
            let token: TokenTree = input.parse()?;
            as_strings.push(token.to_string());
            tokens.push(token);
        }
        Ok(Tokens { tokens, as_strings })
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for token in &self.tokens {
            token.to_tokens(tokens);
        }
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;
    fn into_iter(self) -> std::vec::IntoIter<TokenTree> {
        self.tokens.into_iter()
    }
}

impl Deref for TokenStream {
    type Target = Vec<TokenTree>;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl Hash for TokenStream {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_strings.hash(state);
    }
}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        self.as_strings == other.as_strings
    }
}

impl Ord for TokenStream {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_strings.cmp(&other.as_strings)
    }
}

impl PartialOrd for TokenStream {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TokenStream {}

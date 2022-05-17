use {
    inherent::inherent,
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
pub struct Tokens {
    tokens: Vec<TokenTree>,
    as_strings: Vec<String>,
}

impl Tokens {
    pub fn new(tokens: Vec<TokenTree>) -> Self {
        let as_strings = tokens.iter().map(ToString::to_string).collect();
        Self { tokens, as_strings }
    }

    pub fn into_inner(self) -> Vec<TokenTree> {
        self.tokens
    }

    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.tokens.iter()
    }
}

impl Parse for Tokens {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tokens: Vec<TokenTree> = Vec::new();
        let mut as_strings: Vec<String> = Vec::new();
        while !input.is_empty() {
            let token = input.parse()?;
            tokens.push(token);
            as_strings.push(token.to_string());
        }
        Ok(Tokens { tokens, as_strings })
    }
}

impl ToTokens for Tokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for token in &self.tokens {
            token.to_tokens(tokens);
        }
    }
}

#[inherent]
impl FromIterator<TokenTree> for Tokens {
    fn from_iter<I: IntoIterator<Item = TokenTree>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

#[inherent]
impl IntoIterator for Tokens {
    type Item = TokenTree;
    type IntoIter = std::vec::IntoIter<TokenTree>;
    fn into_iter(self) -> std::vec::IntoIter<TokenTree> {
        self.tokens.into_iter()
    }
}

impl Deref for Tokens {
    type Target = Vec<TokenTree>;
    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl Hash for Tokens {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_strings.hash(state);
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        self.as_strings == other.as_strings
    }
}

impl Ord for Tokens {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_strings.cmp(&other.as_strings)
    }
}

impl PartialOrd for Tokens {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Tokens {}

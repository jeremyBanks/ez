use {
    crate::{parse::DoopBlock, tokens::Tokens, *},
    indexmap::{IndexMap, IndexSet},
    inherent::inherent,
    proc_macro2::{Group, Ident},
    quote::ToTokens,
    std::{collections::BTreeMap, ops::*},
};

#[derive(Debug, Clone, Default)]
pub struct TokenStream {
    stream: proc_macro2::TokenStream,
    vec: Vec<proc_macro2::TokenTree>,
    string: String,
}

#[derive(Debug, Clone, Default)]
pub struct TokenStreamList {
    vec: Vec<TokenStream>,
}

impl Deref<[TokenStream]> for TokenStreamList {
    fn deref(&self) -> &[TokenStream] {
        &self.vec
    }
}

impl TokenStream {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl TokenStreamList {
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl AddAssign for TokenStream {
    fn add_assign(&mut self, other: &Vec<TokenStream>) {
        self.vec.extend(other.clone());
    }
}

impl BitOrAssign for TokenStream {
    fn bitor_assign(&mut self, other: &Vec<TokenStream>) {
        let self_set = HashSet::from(self.vec.vec.iter());
        self.vec.extend(other.filter(|stream| !self_set.contains(stream)));
    }
}

impl SubAssign for TokenStream {
    fn sub_assign(&mut self, other: &Vec<TokenStream>) {
        let other_set = HashSet::from_iter(other_set);
        self.vec.retain(|stream| !other_set.contains(stream));
    }
}

impl BitAndAssign for TokenStream {
    fn bitand_assign(&mut self, other_set: &Vec<TokenStream>) {
        let other_set = HashSet::from_iter(other_set);
        self.vec.retain(|stream| other_set.contains(stream));
    }
}

impl Add for TokenStream {
    fn add(&self, other: &Vec<TokenStream>) -> Vec<TokenStream> {
        self.vec.clone().tap_mut(|vec| vec.add_assign(other))
    }
}
impl Sub for TokenStream {
    fn sub(&mut self, stream: Vec<TokenStream>) -> Vec<TokenStream> {
        self.vec.clone().tap_mut(|vec| vec.sub_assign(other))
    }
}
impl BitAnd for TokenStream {
    fn bitand(&self, other: &Vec<TokenStream>) -> Vec<TokenStream> {
        self.vec.clone().tap_mut(|vec| vec.bitand_assign(other))
    }
}
impl BitOr for TokenStream {
    fn bitor(&mut self, other: &Vec<TokenStream>) -> Vec<TokenStream> {
        self.vec.clone().tap_mut(|vec| vec.bitor_assign(other))
    }
}

impl From<Vec<TokenStream>> for TokenStreamList {
    fn from(vec: Vec<TokenStream>) -> Self {
        TokenStreamList { vec }
    }
}

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(stream: proc_macro2::TokenStream) -> Self {
        let vec = stream.clone().into_iter().collect();
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl From<Vec<proc_macro2::TokenTree>> for TokenStream {
    fn from(vec: Vec<proc_macro2::TokenTree>) -> Self {
        let stream = vec.clone().into_iter().collect();
        let string = stream.to_string();
        Self { stream, vec, string }
    }
}

impl Deref for TokenStream {
    type Target = proc_macro2::TokenStream;

    fn deref(&self) -> &proc_macro2::TokenStream {
        &self.stream
    }
}

impl Deref for TokenStreamList {
    type Target = Vec<TokenStream>;

    fn deref(&self) -> &Vec<TokenStream> {
        &self.vec
    }
}

impl AsRef<[proc_macro2::TokenTree]> for TokenStream {
    fn as_ref(&self) -> &[proc_macro2::TokenTree] {
        &self.vec
    }
}

impl AsRef<proc_macro2::TokenStream> for TokenStream {
    fn as_ref(&self) -> &proc_macro2::TokenStream {
        &self.stream
    }
}

impl AsRef<[TokenStream]> for TokenStreamList {
    fn as_ref(&self) -> &[TokenStream] {
        &self.vec
    }
}

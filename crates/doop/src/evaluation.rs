use {
    indexmap::IndexSet,
    proc_macro2::{TokenStream, TokenTree},
    std::hash::{Hash, Hasher},
};

pub struct BindingEntry {
    tokens: Vec<TokenTree>,
    as_strings: Vec<String>,
}

impl BindingEntry {
    pub fn iter(&self) -> impl Iterator<Item = &TokenTree> {
        self.tokens.iter()
    }
}

impl FromIterator<TokenTree> for BindingEntry {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(tokens: T) -> Self {
        let tokens: Vec<TokenTree> = tokens.into_iter().collect();
        let as_strings = tokens.iter().map(|token| token.to_string()).collect();
        BindingEntry { tokens, as_strings }
    }
}

impl IntoIterator for BindingEntry {
    type Item = TokenTree;
    type IntoIter = ::std::vec::IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl BindingEntry {
    pub fn tokens(&self) -> &[TokenTree] {
        &self.tokens
    }
}

impl Hash for BindingEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_strings.hash(state);
    }
}

impl PartialEq for BindingEntry {
    fn eq(&self, other: &Self) -> bool {
        self.as_strings == other.as_strings
    }
}

impl Ord for BindingEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_strings.cmp(&other.as_strings)
    }
}

impl PartialOrd for BindingEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for BindingEntry {}

pub struct Doop {
    pub items: Vec<DoopItem>,
}

impl Doop {
    pub fn new() -> Self {
        Doop { items: Vec::new() }
    }
}

pub struct DoopItem {
    pub for_bindings: Vec<ForBinding>,
    pub body: TokenStream,
}

pub struct ForBinding {
    pub target: ForBindingTarget,
    pub entries: Vec<TokenStream>,
}

impl From<crate::input::DoopForBinding> for ForBinding {
    fn from(binding: crate::input::DoopForBinding) -> Self {
        todo!()
        // Self {
        //     target: binding.target.into(),
        //     entries: vec![vec![binding.first_term],
        // binding.rest_terms].flatten(), }
    }
}

pub enum ForBindingTarget {
    Ident(syn::Ident),
    Tuple(Vec<syn::Ident>),
}

impl TryFrom<crate::input::DoopBlock> for Doop {
    type Error = syn::Error;
    fn try_from(input: crate::input::DoopBlock) -> Result<Self, Self::Error> {
        Ok(Doop { items: todo!("{}", &format!("{input:#?}")[..=256]) })
    }
}

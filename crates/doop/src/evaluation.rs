use {
    indexmap::IndexSet,
    proc_macro2::{TokenStream, TokenTree},
    std::hash::{Hash, Hasher},
};

pub struct BindingEntries {
    bindings: IndexSet<BindingEntry>,
}

impl BindingEntries {
    pub fn add(&mut self, binding: BindingEntry) {
        self.bindings.insert(binding);
    }

    pub fn remove(&mut self, binding: BindingEntry) {
        self.bindings.remove(&binding);
    }
}

pub struct BindingEntry {
    tokens: Vec<TokenTree>,
    as_string: String,
}

impl Hash for BindingEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_string.hash(state);
    }
}

impl PartialEq for BindingEntry {
    fn eq(&self, other: &Self) -> bool {
        self.as_string == other.as_string
    }
}

impl Ord for BindingEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_string.cmp(&other.as_string)
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
        Ok(Doop {
            items: panic!("{input:#?}"),
        })
    }
}

impl TryFrom<crate::input::DoopItem> for Doop {
    type Error = syn::Error;
    fn try_from(input: crate::input::DoopItem) -> Result<Self, Self::Error> {
        todo!()
        // Ok(Doop {
        // items: vec![DoopItem {
        //     for_bindings: input.bindings.into(),
        //     body: input.item,
        // }],
        // })
    }
}

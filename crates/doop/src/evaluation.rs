use {
    crate::*,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{TokenStream, TokenTree},
    std::hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
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

impl Default for Doop {
    fn default() -> Self {
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

impl From<input::DoopForBinding> for ForBinding {
    fn from(binding: input::DoopForBinding) -> Self {
        let mut entries = Default::default();

        Self { target: ForBindingTarget::from(binding.target), entries }
    }
}

pub enum ForBindingTarget {
    Ident(syn::Ident),
    Tuple(Vec<syn::Ident>),
}

impl From<input::ForBindingTarget> for ForBindingTarget {
    fn from(target: input::ForBindingTarget) -> Self {
        match target {
            input::ForBindingTarget::Ident(ident) => Self::Ident(ident),
            input::ForBindingTarget::Tuple(tuple) => todo!("tuple bindings not implemented"),
        }
    }
}

impl TryFrom<input::DoopBlock> for Doop {
    type Error = syn::Error;
    fn try_from(input: input::DoopBlock) -> Result<Doop, Self::Error> {
        let mut let_bindings = IndexMap::<syn::Ident, IndexSet<BindingEntry>>::new();
        let items = vec![];

        eprintln!("{input:#?}");

        let evaluate_binding_entry =
            |let_bindings: &mut IndexMap<syn::Ident, IndexSet<BindingEntry>>,
             entry: &input::BindingTerm|
             -> Result<IndexSet<BindingEntry>, syn::Error> {
                Ok(match entry {
                    input::BindingTerm::Ident(ident) =>
                        let_bindings.get(ident).expect("undefined variable?").clone(),
                    input::BindingTerm::BraceList(list) => list
                        .entries
                        .iter()
                        .map(|entry| BindingEntry::from_iter(entry.clone().into_iter()))
                        .collect(),
                    input::BindingTerm::BracketList(list) => list
                        .entries
                        .iter()
                        .map(|entry| BindingEntry::from_iter(entry.clone().into_iter()))
                        .collect(),
                })
            };

        for item in input.items {
            match item {
                input::DoopBlockItem::Let(binding) => {
                    let mut terms = evaluate_binding_entry(&mut let_bindings, &binding.first_term)?;

                    for rest in binding.rest_terms {
                        let rest_term = evaluate_binding_entry(&mut let_bindings, &rest.term)?;
                        match rest.operation {
                            input::AddOrSub::Add(_) => terms.extend(rest_term),
                            input::AddOrSub::Sub(_) =>
                                terms = terms.difference(&rest_term).cloned().collect(),
                        }
                    }

                    let_bindings.insert(binding.ident.clone(), terms);
                }
                input::DoopBlockItem::For(binding) => {
                    eprintln!("{let_bindings:#?}");
                    // let loop_bindings: IndexMap<syn::Ident, Vec<TokenTree>> =
                    // Default::default();

                    // this adds an item
                }
            }
        }

        Ok(Doop { items })
    }
}

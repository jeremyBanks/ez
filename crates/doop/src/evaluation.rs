use {
    crate::*,
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{TokenStream, TokenTree},
    quote::ToTokens,
    std::{
        cell::UnsafeCell,
        hash::{Hash, Hasher},
    },
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

pub enum ForBindingTarget {
    Ident(syn::Ident),
    Tuple(Vec<syn::Ident>),
}

impl From<input::ForBindingTarget> for ForBindingTarget {
    fn from(target: input::ForBindingTarget) -> Self {
        match target {
            input::ForBindingTarget::Ident(ident) => Self::Ident(ident),
            input::ForBindingTarget::Tuple(tuple) => Self::Tuple(tuple.items.into_iter().collect()),
        }
    }
}

impl TryFrom<input::DoopBlock> for Doop {
    type Error = syn::Error;
    fn try_from(input: input::DoopBlock) -> Result<Doop, Self::Error> {
        let mut let_bindings = IndexMap::<syn::Ident, IndexSet<BindingEntry>>::new();
        let mut items = vec![];

        fn evaluate_binding_term(
            let_bindings: &mut IndexMap<syn::Ident, IndexSet<BindingEntry>>,
            term: &input::BindingTerm,
        ) -> Result<IndexSet<BindingEntry>, syn::Error> {
            Ok(match term {
                input::BindingTerm::Ident(ident) =>
                    let_bindings.get(ident).expect("undefined variable?").clone(),
                input::BindingTerm::BraceList(list) => list
                    .entries
                    .iter()
                    .map(|term| BindingEntry::from_iter(term.clone().into_iter()))
                    .collect(),
                input::BindingTerm::BracketList(list) => list
                    .entries
                    .iter()
                    .map(|term| BindingEntry::from_iter(term.clone().into_iter()))
                    .collect(),
            })
        }

        fn evaluate_first_and_rest(
            mut let_bindings: &mut IndexMap<syn::Ident, IndexSet<BindingEntry>>,
            first: &input::BindingTerm,
            rest: &[input::RestTerm],
        ) -> Result<IndexSet<BindingEntry>, syn::Error> {
            let mut terms = evaluate_binding_term(&mut let_bindings, &first)?;

            for rest in rest {
                let rest_term = evaluate_binding_term(&mut let_bindings, &rest.term)?;
                match rest.operation {
                    input::AddOrSub::Add(_) => terms.extend(rest_term),
                    input::AddOrSub::Sub(_) =>
                        terms = terms.difference(&rest_term).cloned().collect(),
                }
            }

            Ok(terms)
        }

        for item in input.items {
            match item {
                input::DoopBlockItem::Let(binding) => {
                    let terms = evaluate_first_and_rest(
                        &mut let_bindings,
                        &binding.first_term,
                        &binding.rest_terms,
                    )?;
                    let_bindings.insert(binding.ident.clone(), terms);
                }
                input::DoopBlockItem::For(item) => {
                    let body = item.body.into_token_stream();
                    let mut for_bindings: Vec<ForBinding> = Default::default();
                    let input_bindings = item.bindings.bindings;

                    for binding in input_bindings {
                        let terms = evaluate_first_and_rest(
                            &mut let_bindings,
                            &binding.first_term,
                            &binding.rest_terms,
                        )?;
                        for_bindings.push(ForBinding {
                            target: match binding.target {
                                input::ForBindingTarget::Ident(ident) =>
                                    ForBindingTarget::Ident(ident),
                                input::ForBindingTarget::Tuple(tuple) =>
                                    ForBindingTarget::Tuple(tuple.items.into_iter().collect()),
                            },
                            entries: terms
                                .into_iter()
                                .map(|term| term.into_iter().collect())
                                .collect(),
                        });
                    }
                    items.push(DoopItem { for_bindings, body });
                }
            }
        }

        Ok(Doop { items })
    }
}

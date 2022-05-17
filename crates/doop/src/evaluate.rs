use {
    crate::{
        parse::{DoopBlock, DoopForBindings},
        tokens::Tokens,
        *,
    },
    indexmap::{IndexMap, IndexSet},
    proc_macro2::{Group, Ident, TokenStream, TokenTree},
    quote::ToTokens,
    std::{
        hash::{Hash, Hasher},
        iter::empty,
        ops::Deref,
    },
};

pub struct DoopItem {
    pub for_bindings: Vec<ForBinding>,
    pub body: TokenStream,
}

pub struct ForBinding {
    pub target: Option<syn::Ident>,
    pub entries: Vec<TokenStream>,
}

pub fn evaluate(input: DoopBlock) -> Result<TokenStream, syn::Error> {
    let mut output = TokenStream::new();

    // Mappings from identifiers to ordered sets of tokens representing possible
    // replacements. These are the bindings created by use of the top-level
    // `let` statement.
    let mut let_bindings = IndexMap::<syn::Ident, IndexSet<Tokens>>::new();

    // Mappings from identifiers to tokens representing a single replacement.
    // These are the bindings created for each iteration of a `for` loop body.
    let mut for_bindings = IndexMap::<syn::Ident, Tokens>::new();

    for item in input.items {
        use parse::DoopBlockItem::*;
        match item {
            Static(item) => {
                output.extend(item.body);
            }
            Let(item) => {
                let ident = item.ident;
                let first = item.first_term;
                let rest = item.rest_terms;
            }
            For(item) => {
                let input_body = item.body;
                for binding in item.bindings {}
            }
        }
    }

    fn evaluate_binding_term(
        let_bindings: &mut IndexMap<syn::Ident, IndexSet<Tokens>>,
        for_bindings: &mut IndexMap<syn::Ident, IndexSet<Tokens>>,
        term: &parse::BindingTerm,
    ) -> Result<IndexSet<Tokens>, syn::Error> {
        Ok(match term {
            parse::BindingTerm::Ident(ident) =>
                get_either(let_bindings, for_bindings, ident)?.clone(),
            parse::BindingTerm::BraceList(list) => list
                .entries
                .iter()
                .map(|term| Tokens::from_iter(term.clone().into_iter()))
                .collect(),
            parse::BindingTerm::BracketList(list) => list
                .entries
                .iter()
                .map(|term| Tokens::from_iter(term.clone().into_iter()))
                .collect(),
        })
    }

    fn evaluate_first_and_rest(
        mut let_bindings: &mut IndexMap<syn::Ident, IndexSet<Tokens>>,
        for_bindings: &mut IndexMap<syn::Ident, IndexSet<Tokens>>,
        first: &parse::BindingTerm,
        rest: &[parse::RestTerm],
    ) -> Result<IndexSet<Tokens>, syn::Error> {
        let mut terms = evaluate_binding_term(&mut let_bindings, &mut for_bindings, &first)?;

        for rest in rest {
            let rest_term =
                evaluate_binding_term(&mut let_bindings, &mut for_bindings, &rest.term)?;
            match rest.operation {
                parse::AddOrSub::Add(_) => terms.extend(rest_term),
                parse::AddOrSub::Sub(_) => terms = terms.difference(&rest_term).cloned().collect(),
            }
        }

        Ok(terms)
    }

    for item in input.items {
        match item {
            parse::DoopBlockItem::Let(binding) => {
                for_bindings.clear();
                let terms = evaluate_first_and_rest(
                    &mut let_bindings,
                    &mut for_bindings,
                    &binding.first_term,
                    &binding.rest_terms,
                )?;
                let_bindings.insert(binding.ident.clone(), terms);
            }
            parse::DoopBlockItem::Static(item) => {
                items.push(DoopItem {
                    for_bindings: vec![ForBinding {
                        target: None,
                        entries: vec![empty().collect()],
                    }],
                    body: item.body.stream(),
                });
            }
            parse::DoopBlockItem::For(item) => {
                let body = item.body.stream();
                let input_bindings = item.bindings.bindings;

                for binding in input_bindings {
                    let terms = evaluate_first_and_rest(
                        &mut let_bindings,
                        &mut for_bindings,
                        &binding.first_term,
                        &binding.rest_terms,
                    )?;
                    // XXX: don't insert tuples as a single item,
                    // break them up at this stage, which also lets us insert them into the
                    // for_bindings map.0
                    for_bindings.push(ForBinding {
                        target: match binding.target {
                            parse::ForBindingTarget::Ident(ident) =>
                                ForBindingTarget::Ident(ident.ident()),
                            parse::ForBindingTarget::Tuple(tuple) => ForBindingTarget::Tuple(
                                tuple.items.into_iter().map(|ident| ident.ident()).collect(),
                            ),
                        },
                        entries: terms.into_iter().map(|term| term.into_iter().collect()).collect(),
                    });
                }
                items.push(DoopItem { for_bindings, body });
            }
        }
    }

    Ok(output)
}

fn replace_in_token_stream(
    input: TokenStream,
    replacements: &IndexMap<Ident, TokenStream>,
) -> Result<TokenStream, syn::Error> {
    let mut output = TokenStream::new();
    for token in input {
        match token {
            TokenTree::Ident(ref candidate) =>
                if let Some(replacement) = replacements.get(candidate) {
                    output.extend(replacement.clone().into_token_stream());
                } else {
                    output.extend(Some(token));
                },

            TokenTree::Group(group) => output.extend([TokenTree::Group(Group::new(
                group.delimiter(),
                replace_in_token_stream(group.stream(), replacements)?,
            ))]),

            _ => output.extend(Some(token)),
        }
    }
    Ok(output)
}

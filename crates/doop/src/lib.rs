mod token_stream;
mod token_stream_list;
mod token_tree;

#[allow(unused)]
pub(crate) use {
    crate::{token_stream::*, token_stream_list::*, token_tree::*},
    indexmap::{IndexMap, IndexSet},
    inherent::inherent,
    proc_macro::TokenStream as TokenStream1,
    proc_macro2::{
        Delimiter, Group, Ident, Literal, Punct, TokenStream as TokenStream2, TokenTree,
    },
    quote::ToTokens,
    std::{
        borrow::{Borrow, BorrowMut},
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        collections::{HashMap, HashSet},
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        ops::*,
    },
    tap::Tap,
};

#[proc_macro]
pub fn doop(input: TokenStream1) -> TokenStream1 {
    let input = TokenStream::from(TokenStream2::from(input));
    let mut output = TokenStream2::new();

    let braced: Vec<_> = input.iter().flat_map(|tree| tree.braced()).collect();
    assert_eq!(braced.len(), 1, "expected exactly one braced block in item statement");

    for line in braced[0].lines() {
        let line = TokenStream::from_iter(line.into_iter().cloned());

        if line.is_empty() || line.punct().map(|punct| punct.as_char()) == Some(';') {
            println!("skipping empty: {line}");
        } else if let Some(braced) = line.braced() {
            println!("EMITTING: {braced}");
            output.extend(braced);
        } else if let Some(ident) = line.first().and_then(TokenTree::ident) {
            println!("KEYWORD {ident}")
        } else {
            println!("UNEXPECTED! {line}")
        }
    }

    output.into()
}

#[proc_macro_attribute]
pub fn from(attribute: TokenStream1, item: TokenStream1) -> TokenStream1 {
    assert!(attribute.is_empty(), "no attribute arguments expected");

    doop(item)
}

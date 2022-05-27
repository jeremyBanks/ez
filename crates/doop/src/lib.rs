#![allow(unused)]

mod span;
mod token_stream;
mod token_tree;
mod tokens;
mod tokens_list;

pub(crate) use {
    crate::{span::*, token_stream::*, token_tree::*, tokens::*, tokens_list::*},
    indexmap::{IndexMap, IndexSet},
    inherent::inherent,
    itertools::Itertools,
    proc_macro::TokenStream as TokenStream1,
    proc_macro2::{
        Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream as TokenStream2,
        TokenTree,
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
/// A macro for local code duplication in Rust.
pub fn doop(input: TokenStream1) -> TokenStream1 {
    let input = Tokens::from(TokenStream2::from(input));
    let mut output = TokenStream2::new();

    for line in input.split_lines() {
        let line = Tokens::from_iter(line.into_iter().cloned());

        if line.is_empty() || line.punct().map(|punct| punct.as_char()) == Some(';') {
            // ignore empty lines
        } else if let Some(braced) = line.braced() {
            output.extend(braced);
        } else if let Some(ident) = line.first().and_then(TokenTree::ident) {
            match ident.to_string().as_str() {
                "let" => println!("TODO: let {line}"),
                "type" => println!("TODO: type {line}"),
                "const" => println!("TODO: const {line}"),
                "static" => println!("TODO: static {line}"),
                "for" => println!("TODO: for {line}"),
                _ => return ident.error("unrecognized keyword"),
            };
        } else {
            return line.error("expected keyword or braced block");
        }
    }

    output.into()
}

#[proc_macro_attribute]
/// Evaluates the contents of the next braced expression as though it was the
/// body of a [`doop!`] macro invocation. Any other tokens on the line are
/// discarded. This is more syntactically limited than using the macro directly
/// because your item needs to parse as a typical valid Rust item, even if we're
/// interpreting it directly. However, in return we get rustfmt support, which
/// is often nonexistent for non-attribute macro invocations.
///
/// ```rust
/// #[doop::block]
/// static DOOP: ! = {
///     for Name in [Foo, Bar] {
///         struct Name;
///     }
/// };
/// ```
///
/// If you need to use some tokens that are not valid Rust syntax, you can
/// "escape" them by using the `Tokens!()` pseudo-macro. (This is only accepted
/// in location where a `Tokens` value is expected.) Here's an example case:
///
/// ```rust
/// #[doop::block]
/// static LIFETIMES: ! = {
///     const LIFETIME = Tokens!(static);
///     {
///        struct LifeBytes(&'LIFETIME Vec<u8>);
///     }
/// };
/// ```
///
/// Equivalent to:
///
/// ```rust
/// # use doop::doop;
/// doop! {
///    const LIFETIME = static;
///    {
///        struct LifeBytes(&'LIFETIME Vec<u8>);
///    }
/// }
/// ```
///
/// Equivalent to:
///
/// ```rust
/// #[doop::item(const LIFETIME = static)]
/// struct LifeBytes(&'LIFETIME Vec<u8>);
/// ```
pub fn block(attribute: TokenStream1, item: TokenStream1) -> TokenStream1 {
    assert!(attribute.is_empty(), "no attribute arguments expected");
    let input = Tokens::from(TokenStream2::from(item));

    let braced = input.iter().flat_map(TokenTree::braced).collect_vec();
    assert_eq!(braced.len(), 1, "expected exactly one braced block in item statement");

    let block = braced[0].clone();

    doop(TokenStream2::from(block).into())
}

#[proc_macro_attribute]
/// Duplicates a single item in the manner of the [`doop!`] macro,
/// using a `for` expression as the attribute argument.
///
/// ```rust
/// #[doop::item(for Name in [Foo, Bar])]
/// struct Name;
///
/// let _: (Foo, Bar);
/// ```
pub fn item(attribute: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let attribute = TokenStream2::from(attribute);
    let item = TokenStream2::from(item);

    let mut input = TokenStream2::new();
    input.extend(attribute);
    let group = Group::new(Delimiter::Brace, item);
    input.extend(Some(TokenTree::Group(group)));

    doop(TokenStream1::from(input))
}

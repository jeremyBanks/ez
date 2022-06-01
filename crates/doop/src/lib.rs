#![warn(missing_docs, clippy::pedantic)]
#![allow(unused)]

//! `doop!`—spelled like "loop" and pronounced like "dupe"—is a macro for local
//! code duplication in Rust, using a loop-style syntax.

mod all_ext;
mod generate;
mod parse;
mod span_range;
mod string_like;
mod token_stream;
mod tokens_list;

pub(crate) use {
    crate::{
        all_ext::*, generate::*, parse::*, span_range::*, string_like::*, token_stream::*,
        tokens_list::*,
    },
    ::once_cell::unsync::OnceCell,
    proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree},
    std::{
        borrow::Borrow,
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        collections::{HashMap, HashSet},
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        ops::*,
        rc::Rc,
        str::FromStr,
    },
};

#[proc_macro]
/// A macro for local code duplication in Rust.
pub fn doop(body: TokenStream) -> TokenStream {
    fn doop(body: TokenStream) -> Result<TokenStream, TokenStream> {
        let mut output = TokenStream::new();

        let only = body.only()?;
        Ok([only].into_iter().collect())
    }
    match doop(body) {
        Ok(body) => body,
        Err(err) => err,
    }
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
/// Again, tokens outside of the braced block are ignored, so they can be
/// adjusted to whatever fits in the current context grammatically. If you're
/// producing an item or statement, a suggested "default" is to use
/// `static DOOP: ! = {...};` to evoke the look of a `doop! {...}` macro
/// invocation.
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
pub fn unwrap(attribute: TokenStream, item: TokenStream) -> TokenStream {
    return item.error("oh boy");
    // let attribute = attribute.into_tokens();
    // let item = item.into_tokens();

    // return item.error("oh noes").into_tokens().into_stream();

    // if !attribute.is_empty() {
    //     return attribute
    //         .error("no arguments expected for #[doop::block] attribute
    // macro")         .into_tokens()
    //         .into_stream();
    // }

    // let input = Tokens::from(TokenStream::from(item));

    // let braced =
    //     input.iter().filter_map(|tt|
    // (Tokens::from(tt.clone()).braced())).collect::<Vec<_>>();
    // if braced.len() != 1 {
    //     return input
    //         .error("expected exactly one braced block in item statement")
    //         .into_tokens()
    //         .into_stream();
    // }

    // braced[0].clone().pipe(Tokens::from).pipe_ref(parse).map(generate).
    // into_tokens().into_stream()
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
pub fn item(attribute: TokenStream, item: TokenStream) -> TokenStream {
    return item.error("oh boy");
    // return item.into_tokens().error("oh noes").into_tokens().into_stream();

    // let attribute = attribute.into_tokens();
    // let item = item.into_tokens();

    // let mut input = TokenStream::new();
    // input.extend(attribute);
    // let group = Group::new(Delimiter::Brace, item.into());
    // input.extend(Some(TokenTree::Group(group)));

    // input.pipe(Tokens::from).pipe_ref(parse).map(generate).into_tokens().
    // into_stream()
}

/// Duplicates a trait impl block as an inherent impl block.
///
/// This is similar to the macro from the `inherent` crate, but it's cruder,
/// as it simply duplicates instead of delegating, supporting fewer cases.
#[proc_macro_attribute]
pub fn inherent(attribute: TokenStream, item: TokenStream) -> TokenStream {
    return item.error("oh boy");
    // return item.into_tokens().error("oh noes").into_tokens().into_stream();

    // let attribute = attribute.into_tokens();
    // let item = item.into_tokens();

    // if !attribute.is_empty() {
    //     return attribute
    //         .error("no arguments expected for #[doop::inherent] attribute
    // macro")         .into_tokens()
    //         .into_stream();
    // }

    // item.error("#[doop::inherent] is not yet
    // implemented").into_tokens().into_stream()
}

#[proc_macro]
/// Removes the contents from the syntax tree, replacing them with nothing.
pub fn ignore(body: TokenStream) -> TokenStream {
    for _ in body {}

    TokenStream::new()
}

#[proc_macro_attribute]
/// Removes an item from the syntax tree, replacing it with nothing.
pub fn ignore_item(attribute: TokenStream, item: TokenStream) -> TokenStream {
    for _ in attribute {}
    for _ in item {}

    TokenStream::new()
}

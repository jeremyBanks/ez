mod tokens;
mod tokens_list;

pub(crate) use {
    crate::{tokens::*, tokens_list::*},
    ::{
        once_cell::unsync::OnceCell,
        proc_macro::{
            Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
        },
        std::{
            borrow::Borrow,
            cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
            collections::{HashMap, HashSet},
            fmt::{Debug, Display},
            hash::{Hash, Hasher},
            ops::*,
        },
    },
};

#[proc_macro]
/// A macro for local code duplication in Rust.
pub fn doop(input: TokenStream) -> TokenStream {
    let input = input.into_tokens();

    let mut output = Tokens::new();
    let mut tokens_bindings: HashMap<String, Tokens> = HashMap::new();
    let mut tokens_list_bindings: HashMap<String, TokensList> = HashMap::new();

    let lines = input.split_lines();

    for line in input.split_lines() {
        let line = Tokens::from_iter(line.into_iter().cloned());

        if line.is_empty() || line.punct().map(|punct| punct.as_char()) == Some(';') {
            continue;
        }

        let item: Item;
        enum Item {
            TokensAssignment { lhs: Ident, rhs: Tokens },
            TokensListAssignment { lhs: Ident, rhs: Tokens },
            EmitForLoop { body: Tokens, replacement_list: Vec<HashMap<String, Tokens>> },
        }

        if let Some(braced) = line.braced() {
            item = Item::EmitForLoop {
                body: braced.into_tokens(),
                replacement_list: vec![HashMap::new()],
            };
        } else if let Some(TokenTree::Ident(ident)) = line.first() {
            let keyword = ident.to_string();

            if keyword == "for" {
                todo!("parse for loop");
            } else if matches!(keyword.as_ref(), "let" | "static" | "type" | "const") {
                todo!("parse assignment");
            } else {
                return line.into_error(&format!("unexpected keyword: {keyword}"));
            }
        } else {
            return line.into_error("expected keyword or braced block");
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
pub fn block(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = attribute.into_tokens();
    let item = item.into_tokens();

    if !attribute.is_empty() {
        return attribute.into_error("no arguments expected for #[doop::block] attribute macro");
    }

    let input = Tokens::from(TokenStream::from(item));

    let braced =
        input.iter().flat_map(|tt| (Tokens::from(tt.clone()).braced())).collect::<Vec<_>>();
    assert_eq!(braced.len(), 1, "expected exactly one braced block in item statement");

    let block = braced[0].clone();

    doop(TokenStream::from(block).into())
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
    let attribute = attribute.into_tokens();
    let item = item.into_tokens();

    let mut input = TokenStream::new();
    input.extend(attribute);
    let group = Group::new(Delimiter::Brace, item.into());
    input.extend(Some(TokenTree::Group(group)));

    doop(TokenStream::from(input))
}

/// Duplicates a trait impl block as an inherent impl block.
///
/// This is similar to the macro from the `inherent` crate, but it's cruder,
/// as it simply duplicates instead of delegating, supporting fewer cases.
#[proc_macro_attribute]
pub fn inherent(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = attribute.into_tokens();
    let item = item.into_tokens();

    let trait_keyword = item.first().ident();

    if !attribute.is_empty() {
        return attribute.into_error("no arguments expected for #[doop::inherent] attribute macro");
    }

    todo!()
}

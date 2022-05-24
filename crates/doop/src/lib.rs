pub(crate) use {
    token_stream::TokenStream,
    proc_macro::TokenStream as TokenStream1,
    proc_macro2::{Delimiter, Group, Ident, TokenStream as TokenStream2, TokenTree},
    indexmap::{IndexMap, IndexSet},
    inherent::inherent,
    quote::ToTokens,
    std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        collections::{BTreeMap, HashMap},
        hash::{Hash, Hasher},
        ops::*,
        fmt::{Debug, Display},
    },
    tap::Tap,
};

mod token_stream;

#[proc_macro]
pub fn doop(input: TokenStream) -> TokenStream {
    Default::default()
}

#[proc_macro_attribute]
pub fn from(mut tokens: TokenStream1, item: TokenStream1) -> TokenStream {
    Default::default()
}

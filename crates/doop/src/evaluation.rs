use proc_macro2::{TokenStream, TokenTree};

pub struct Doop {
    pub items: Vec<DoopItem>,
}

pub struct DoopItem {
    pub for_bindings: Vec<ForBinding>,
    pub body: TokenTree,
}

pub struct ForBinding {
    pub target: ForBindingTarget,
    pub values: Vec<TokenStream>,
}

pub enum ForBindingTarget {
    Ident(syn::Ident),
    Tuple(Vec<syn::Ident>),
}

impl TryFrom<crate::input::DoopBlock> for Doop {
    type Error = syn::Error;
    fn try_from(input: crate::input::DoopBlock) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryFrom<crate::input::DoopItem> for Doop {
    type Error = syn::Error;
    fn try_from(input: crate::input::DoopItem) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

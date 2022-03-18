pub struct Doop {
    pub items: Vec<()>,
}

impl TryFrom<crate::syntax::DoopBlock> for Doop {
    type Error = syn::Error;
    fn try_from(syntax: crate::syntax::DoopBlock) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

impl TryFrom<crate::syntax::DoopItem> for Doop {
    type Error = syn::Error;
    fn try_from(syntax: crate::syntax::DoopItem) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

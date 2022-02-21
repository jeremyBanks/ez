use ::{
    proc_macro2::{Ident, TokenTree},
    std::borrow::{Borrow, BorrowMut},
};

impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
pub trait TokenTreeExt: Borrow<TokenTree> + BorrowMut<TokenTree> {
    fn is_group(&self) -> bool {
        matches!(self.borrow(), TokenTree::Group(_))
    }

    fn is_ident(&self) -> bool {
        matches!(self.borrow(), TokenTree::Ident(_))
    }

    fn children(&self) -> Result<Vec<TokenTree>, syn::Error> {
        if let TokenTree::Group(g) = self.borrow() {
            Ok(g.stream().into_iter().collect())
        } else {
            Err(syn::Error::new(self.borrow().span(), "expected a group"))
        }
    }

    fn only(&self) -> Result<TokenTree, syn::Error> {
        let children = self.children()?;
        assert_eq!(children.len(), 1);
        Ok(children[0].clone())
    }

    fn map<R>(&self, f: impl Fn(TokenTree) -> R) -> Vec<R> {
        self.children().unwrap().into_iter().map(f).collect()
    }

    fn for_each(&self, f: impl FnMut(TokenTree)) {
        self.children().unwrap().into_iter().for_each(f)
    }

    fn ident(&self) -> Result<Ident, syn::Error> {
        if let TokenTree::Ident(i) = self.borrow() {
            Ok(i.clone())
        } else {
            Err(syn::Error::new(self.borrow().span(), "expected an ident"))
        }
    }
}

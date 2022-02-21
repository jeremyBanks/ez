use {
    proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree},
    quote::{quote_spanned, ToTokens},
    std::borrow::{Borrow, BorrowMut},
    syn::{
        fold::Fold, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Block,
        ExprAsync, ExprClosure, ExprReturn, ImplItemMethod, ItemFn, Path, ReturnType, Visibility,
    },
};

impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
pub trait TokenTreeExt: Borrow<TokenTree> + BorrowMut<TokenTree> {
    fn is_group(&self) -> bool {
        matches!(self.borrow(), TokenTree::Group(_))
    }

    fn is_ident(&self) -> bool {
        matches!(self.borrow(), TokenTree::Ident(_))
    }

    fn children(&self) -> eyre::Result<Vec<TokenTree>> {
        if let TokenTree::Group(g) = self.borrow() {
            Ok(g.stream().into_iter().collect())
        } else {
            panic!("expected a group")
        }
    }

    fn only(&self) -> eyre::Result<TokenTree> {
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

    fn ident(&self) -> eyre::Result<Ident> {
        if let TokenTree::Ident(i) = self.borrow() {
            Ok(i.clone())
        } else {
            eyre::bail!("expected an ident, got: {:?}", self.borrow())
        }
    }
}

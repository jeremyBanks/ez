use ::{
    proc_macro2::{Ident, TokenTree},
    std::borrow::{Borrow, BorrowMut},
};

impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
pub trait TokenTreeExt: Borrow<TokenTree> + BorrowMut<TokenTree> {
    fn tagged(&self, tag: &str) -> Result<Vec<TokenTree>, syn::Error> {
        if let TokenTree::Group(g) = self.borrow() {
            let mut tokens = g.stream().into_iter();
            let actual_tag = tokens
                .next()
                .ok_or_else(|| {
                    let message = format!("expected ({} ...)", tag);
                    syn::Error::new(self.borrow().span(), message)
                })?
                .ident()
                .map_err(|_err| {
                    let message = format!("expected ({} ...)", tag);
                    syn::Error::new(self.borrow().span(), message)
                })?;
            if actual_tag != tag {
                let message = format!("expected ({} ...) but found ({} ...)", tag, actual_tag);
                return Err(syn::Error::new(self.borrow().span(), message))?;
            }

            Ok(tokens.collect())
        } else {
            let message = format!("expected ({} ...)", tag);
            Err(syn::Error::new(self.borrow().span(), &message))
        }
    }

    fn ident(&self) -> Result<Ident, syn::Error> {
        if let TokenTree::Ident(i) = self.borrow() {
            Ok(i.clone())
        } else {
            Err(syn::Error::new(
                self.borrow().span(),
                "expected an identifier",
            ))
        }
    }
}

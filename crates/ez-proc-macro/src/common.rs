use ::{
    proc_macro2::{Ident, TokenTree},
    std::borrow::{Borrow, BorrowMut},
};

impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
impl<T: Borrow<Option<TokenTree>> + BorrowMut<Option<TokenTree>>> OptionTokenTreeExt for T {}

pub trait OptionTokenTreeExt: Borrow<Option<TokenTree>> + BorrowMut<Option<TokenTree>> + Sized {
    fn please(self) -> Option<TokenTree> {
        let borrowed = self.borrow();
        if let Some(token) = borrowed.as_ref() {
            token.clone()
        } else {
            let message = format!("unexpected end of input");
            return Err(syn::Error::new(Span::call_site(), message))?;
        }
    }
}

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

    fn group(&self) -> Result<Group, syn::Error> {
        if let TokenTree::Group(g) = self.borrow() {
            Ok(g.clone())
        } else {
            Err(syn::Error::new(
                self.borrow().span(),
                "expected a group",
            ))
        }
    }

    fn punct(&self) -> Result<Punct, syn::Error> {
        if let TokenTree::Punct(p) = self.borrow() {
            Ok(p.clone())
        } else {
            Err(syn::Error::new(
                self.borrow().span(),
                "expected punctuation",
            ))
        }
    }
}

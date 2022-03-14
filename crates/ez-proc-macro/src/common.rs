use {
    itertools::Itertools,
    proc_macro2::{
        token_stream::IntoIter as TokenStreamIter, Group, Ident, Punct, Span, TokenTree,
    },
    std::{
        borrow::{Borrow, BorrowMut},
        iter::Peekable,
    },
};

impl<T: Borrow<TokenStreamIter> + BorrowMut<TokenStreamIter>> TokenTreeIterExt for T {}
impl<T: Borrow<TokenTree> + BorrowMut<TokenTree>> TokenTreeExt for T {}
impl<T: Borrow<Option<TokenTree>> + BorrowMut<Option<TokenTree>>> OptionTokenTreeExt for T {}

#[deprecated]
pub trait OptionTokenTreeExt:
    Borrow<Option<TokenTree>> + BorrowMut<Option<TokenTree>> + Sized
{
    #[deprecated]
    fn please(self) -> Result<TokenTree, syn::Error> {
        let borrowed = self.borrow();
        if let Some(token) = borrowed.as_ref() {
            Ok(token.clone())
        } else {
            let message = format!("unexpected end of input");
            return Err(syn::Error::new(Span::call_site(), message));
        }
    }
}

pub trait TokenTreeIterExt: Borrow<TokenStreamIter> + BorrowMut<TokenStreamIter> + Sized {
    fn next_if<T>(
        &mut self,
        f: impl Fn(TokenTree) -> Option<T>,
        expected: &str,
    ) -> Result<T, syn::Error> {
        let iter = self.borrow_mut();
        let next = iter.clone().next();

        if let Some(next) = next {
            if let Some(mapped) = f(next.clone()) {
                // only now do we advance the Self iterator
                let _ = iter.next();
                Ok(mapped)
            } else {
                let message = format!("expected {expected}");
                Err(syn::Error::new(next.span(), message))
            }
        } else {
            let message = format!("unexpected end of input, expected {expected}.");
            Err(syn::Error::new(Span::call_site(), message))
        }
    }

    fn is_empty(&self) -> bool {
        self.borrow().clone().next().is_none()
    }

    fn next_tt(&mut self) -> Result<TokenTree, syn::Error> {
        self.next_if(|tt| Some(tt), "any token")
    }

    fn next_ident(&mut self) -> Result<Ident, syn::Error> {
        self.next_if(|tt| tt.ident().ok(), "an ident(ifier or keyword)")
    }

    fn next_group(&mut self) -> Result<Group, syn::Error> {
        self.next_if(
            |tt| tt.group().ok(),
            "a group (wrapped in braces, brackets, or parentheses)",
        )
    }

    fn next_ident_eq(&mut self, ident: &str) -> Result<Ident, syn::Error> {
        let expected = format!("`{ident}`");
        self.next_if(
            |tt| {
                let as_ident = tt.ident().ok()?;
                if as_ident == ident {
                    Some(as_ident)
                } else {
                    None
                }
            },
            &expected,
        )
    }

    fn next_punct(&mut self) -> Result<Punct, syn::Error> {
        self.next_if(|tt| tt.punct().ok(), "a punct(uation)")
    }

    fn next_puncts_eq(&mut self, puncts: &str) -> Result<Vec<Punct>, syn::Error> {
        let chars = puncts.chars().collect_vec();
        let _iter = self.borrow_mut();
        todo!()
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
            Err(syn::Error::new(self.borrow().span(), "expected a group"))
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

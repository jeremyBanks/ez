use crate::*;

impl TokenTreeExt for TokenTree {}
pub trait TokenTreeExt: Borrow<TokenTree> + BorrowMut<TokenTree> {
    fn group(&self) -> Option<&Group> {
        if let TokenTree::Group(group) = self.borrow() {
            Some(group)
        } else {
            None
        }
    }

    fn ident(&self) -> Option<&Ident> {
        if let TokenTree::Ident(ident) = self.borrow() {
            Some(&ident)
        } else {
            None
        }
    }

    fn literal(&self) -> Option<&Literal> {
        if let TokenTree::Literal(literal) = self.borrow() {
            Some(&literal)
        } else {
            None
        }
    }

    fn punct(&self) -> Option<&Punct> {
        if let TokenTree::Punct(punct) = self.borrow() {
            Some(&punct)
        } else {
            None
        }
    }

    fn braced(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Brace {
            Some(group.stream().into())
        } else {
            None
        }
    }

    fn bracketed(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Brace {
            Some(group.stream().into())
        } else {
            None
        }
    }

    fn parenthesized(&self) -> Option<Tokens> {
        let group = self.group()?;
        if group.delimiter() == Delimiter::Parenthesis {
            Some(group.stream().into())
        } else {
            None
        }
    }
}

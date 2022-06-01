use crate::*;

pub trait TokenStreamExt: Borrow<TokenStream> {
    fn to_iter(&self) -> proc_macro::token_stream::IntoIter {
        self.borrow().clone().into_iter()
    }

    fn to_vec(&self) -> Vec<TokenTree> {
        self.to_iter().collect()
    }

    fn first(&self) -> Result<TokenTree, TokenStream> {
        self.to_iter()
            .next()
            .ok_or_else(|| self.borrow().error("empty token stream, expecting first() token"))
    }

    fn last(&self) -> Result<TokenTree, TokenStream> {
        self.to_iter()
            .last()
            .ok_or_else(|| self.borrow().error("empty token stream, expecting last() token"))
    }

    fn only(&self) -> Result<TokenTree, TokenStream> {
        let mut iter = self.to_iter();
        match iter.next() {
            Some(first) => match iter.next() {
                Some(second) =>
                    Err(SpanRange::new(second.span_range().start, self.borrow().span_range().end)
                        .error(
                            "expecting exactly one TokenTree, but TokenStream had additional items",
                        )),
                None => Ok(first),
            },
            None => Err(self
                .borrow()
                .error("expecting exactly one TokenTree, but TokenStream was empty")),
        }
    }
}

impl TokenStreamExt for TokenStream {}

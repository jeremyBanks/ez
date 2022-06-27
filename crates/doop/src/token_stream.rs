use crate::*;

fn code(tree: &TokenTree) -> String {
    let mut s = tree.to_string();
    if s.len() > 32 {
        s.truncate(29);
        s.push_str("...");
    }
    format!("`{s}`")
}

impl TokenStreamExt for TokenStream {
    fn take(self) -> TokenStream {
        self
    }
}

pub trait TokenStreamExt: Borrow<TokenStream> + BorrowMut<TokenStream> + Sized {
    fn take(self) -> TokenStream;

    fn to_iter(&self) -> TokenStreamIterator {
        self.borrow().clone().into_iter()
    }

    fn into_vec(self) -> Vec<TokenTree> {
        self.take().into_iter().collect()
    }

    fn to_vec(&self) -> Vec<TokenTree> {
        self.to_iter().collect()
    }

    fn first(&self) -> TokenResult<TokenTree> {
        self.to_iter().next().ok_or_else(|| self.borrow().error("Unexpected empty TokenStream."))
    }

    fn last(&self) -> TokenResult<TokenTree> {
        self.to_iter().last().ok_or_else(|| self.borrow().error("Unexpected empty TokenStream."))
    }

    fn empty(&self) -> Result<(), TokenStream> {
        if self.to_iter().next().is_some() {
            Err(self.borrow().error("Nothing should be here. Expected empty TokenStream."))
        } else {
            Ok(())
        }
    }

    fn only(&self) -> TokenResult<TokenTree> {
        let mut iter = self.to_iter();
        match iter.next() {
            Some(first) => match iter.next() {
                Some(second) => {
                    let first_code = code(&first);
                    Err(SpanRange::new(second.span_range().start, self.borrow().span_range().end)
                        .error(format!("Nothing expected after `{first}.`")))
                }
                None => Ok(first),
            },
            None => Err(self.borrow().error("Unexpectedly empty TokenStream.")),
        }
    }

    fn group(&self) -> TokenResult<Group> {
        match self.only() {
            Ok(TokenTree::Group(group)) => Ok(group),
            Ok(other) => Err(other.span_range().error("Expected one group.")),
            Err(err) =>
                Err(err.tap_mut(|err| err.extend(self.borrow().error("Expected one group.")))),
        }
    }

    fn punct(&self) -> TokenResult<Punct> {
        match self.only() {
            Ok(TokenTree::Punct(punct)) => Ok(punct),
            Ok(other) => Err(other.span_range().error("Expected one punctuation.")),
            Err(err) =>
                Err(err.tap_mut(|err| err.extend(self.borrow().error("Expected one punctuation.")))),
        }
    }

    fn ident(&self) -> TokenResult<Ident> {
        match self.only() {
            Ok(TokenTree::Ident(ident)) => Ok(ident),
            Ok(other) => Err(other.span_range().error("Expected one identifier.")),
            Err(err) =>
                Err(err.tap_mut(|err| err.extend(self.borrow().error("Expected one identifier.")))),
        }
    }

    fn literal(&self) -> TokenResult<Literal> {
        match self.only() {
            Ok(TokenTree::Literal(literal)) => Ok(literal),
            Ok(other) => Err(other.span_range().error("Expected one literal.")),
            Err(err) =>
                Err(err.tap_mut(|err| err.extend(self.borrow().error("Expected one literal.")))),
        }
    }

    fn bracketed(&self) -> Result<TokenStream, TokenStream> {
        match self.group() {
            Ok(group) if group.delimiter() == Delimiter::Bracket => Ok(group.stream()),
            Ok(other) => Err(other.span_range().error("Expected group to be bracketed: [...].")),
            Err(err) => Err(err),
        }
    }

    fn braced(&self) -> Result<TokenStream, TokenStream> {
        match self.group() {
            Ok(group) if group.delimiter() == Delimiter::Brace => Ok(group.stream()),
            Ok(other) => Err(other.span_range().error("Expected group to be braced: {...}.")),
            Err(err) => Err(err),
        }
    }

    fn parenthesized(&self) -> Result<TokenStream, TokenStream> {
        match self.group() {
            Ok(group) if group.delimiter() == Delimiter::Parenthesis => Ok(group.stream()),
            Ok(other) =>
                Err(other.span_range().error("Expected group to be parenthesized: (...).")),
            Err(err) => Err(err),
        }
    }

    fn flat_map_shallow<F, I>(&self, f: F) -> TokenStream
    where
        F: Fn(TokenTree) -> I,
        I: IntoIterator<Item = TokenTree>,
    {
        self.to_iter().flat_map(f).collect()
    }

    fn flat_map_deep<F, I>(&self, f: F) -> TokenStream
    where
        F: Fn(TokenTree) -> I,
        I: IntoIterator<Item = TokenTree>,
    {
        self.to_iter()
            .flat_map(f)
            .map(|tt| {
                if let TokenTree::Group(group) = tt {
                    let mut tt = TokenTree::Group(Group::new(
                        group.delimiter(),
                        group.stream().into_iter().flat_map(f).collect(),
                    ));
                    tt.set_span(group.span());
                    tt
                } else {
                    tt
                }
            })
            .collect()
    }

    fn with_span_shallow(&self, span: Span) -> TokenStream {
        self.flat_map_shallow(|mut tree| {
            tree.set_span(span);
            Some(tree)
        })
    }

    fn with_span_deep(&self, span: Span) -> TokenStream {
        self.flat_map_deep(|mut tree| {
            tree.set_span(span);
            Some(tree)
        })
    }

    fn replace_deep(&self, replacements: &HashMap<String, TokenStream>) -> TokenStream {
        self.flat_map_deep(|tt| match tt {
            TokenTree::Ident(ident) =>
                if let Some(replacement) = replacements.get(&ident.to_string()) {
                    replacement.clone()
                } else {
                    TokenStream::from_iter([tt])
                },
            _ => TokenStream::from_iter([tt]),
        })
    }

    fn replace_shallow(&self, replacements: &HashMap<String, TokenStream>) -> TokenStream {
        self.flat_map_shallow(|tt| match tt {
            TokenTree::Ident(ident) =>
                if let Some(replacement) = replacements.get(&ident.to_string()) {
                    replacement.clone()
                } else {
                    TokenStream::from_iter([tt])
                },
            _ => TokenStream::from_iter([tt]),
        })
    }

    fn split_lines(&self) -> Vec<TokenStream> {
        let vec = self.to_vec();
        let mut lines = Vec::new();
        let mut next_line_start_index = 0;
        for (i, tt) in vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ';' => {
                    lines.push(TokenStream::from_iter(
                        vec[next_line_start_index..=i].iter().cloned(),
                    ));
                    next_line_start_index = i + 1;
                }
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    lines.push(TokenStream::from_iter(
                        vec[next_line_start_index..=i].iter().cloned(),
                    ));
                    next_line_start_index = i + 1;
                }
                _ => {}
            }
        }
        lines.push(TokenStream::from_iter(vec[next_line_start_index..].iter().cloned()));

        lines
    }

    fn split_commas(&self) -> Vec<TokenStream> {
        let vec = self.to_vec();
        let mut slices = Vec::new();
        let mut next_comma_start_index = 0;
        for (i, tt) in vec.iter().enumerate() {
            match tt {
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    slices.push(TokenStream::from_iter(
                        vec[next_comma_start_index..=i].iter().cloned(),
                    ));
                    next_comma_start_index = i + 1;
                }
                _ => {}
            }
        }

        slices.push(TokenStream::from_iter(vec[next_comma_start_index..].iter().cloned()));

        slices
    }
}

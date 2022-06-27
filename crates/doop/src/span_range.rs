use crate::*;

// a hacky way of combining spans
#[derive(Clone, Copy, Debug)]
pub struct SpanRange {
    pub start: Span,
    pub end: Span,
}

impl SpanRange {
    pub fn new(start: Span, end: Span) -> Self {
        SpanRange { start, end }
    }

    pub fn start(&self) -> Span {
        self.start
    }

    pub fn end(&self) -> Span {
        self.end
    }
}

pub trait ExtSpanRange {
    fn span_range(&self) -> SpanRange;

    fn error(&self, message: impl AsRef<str>) -> TokenStream {
        let span_range = self.span_range();
        let message = message.as_ref();

        TokenStream::from_iter([
            TokenTree::Ident(Ident::new("compile_error", span_range.start)),
            TokenTree::Punct(
                Punct::new('!', Spacing::Alone).tap_mut(|g| g.set_span(span_range.end)),
            ),
            TokenTree::Group(
                Group::new(
                    Delimiter::Brace,
                    TokenStream::from_iter([TokenTree::Literal(Literal::string(message))
                        .tap_mut(|g| g.set_span(span_range.end))]),
                )
                .tap_mut(|g| g.set_span(span_range.end)),
            ),
        ])
    }
}

impl ExtSpanRange for SpanRange {
    fn span_range(&self) -> SpanRange {
        *self
    }
}

impl ExtSpanRange for Span {
    fn span_range(&self) -> SpanRange {
        SpanRange { start: *self, end: *self }
    }
}

impl ExtSpanRange for TokenStream {
    fn span_range(&self) -> SpanRange {
        let mut iter = self.to_iter();

        if let Some(start) = iter.next() {
            let end = iter.last().unwrap_or(start.clone());

            SpanRange::new(start.span_range().start, end.span_range().end)
        } else {
            Span::call_site().span_range()
        }
    }
}

impl ExtSpanRange for TokenTree {
    fn span_range(&self) -> SpanRange {
        match self {
            TokenTree::Group(g) => g.span_range(),
            TokenTree::Punct(p) => p.span_range(),
            TokenTree::Ident(i) => i.span_range(),
            TokenTree::Literal(l) => l.span_range(),
        }
    }
}

impl ExtSpanRange for Group {
    fn span_range(&self) -> SpanRange {
        SpanRange { start: self.span_open(), end: self.span_close() }
    }
}

impl ExtSpanRange for Punct {
    fn span_range(&self) -> SpanRange {
        self.span().span_range()
    }
}

impl ExtSpanRange for Ident {
    fn span_range(&self) -> SpanRange {
        self.span().span_range()
    }
}

impl ExtSpanRange for Literal {
    fn span_range(&self) -> SpanRange {
        self.span().span_range()
    }
}

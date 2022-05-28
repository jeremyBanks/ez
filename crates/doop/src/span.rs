use crate::*;

/// Extension trait for producing a compile_error! from any piece of syntax with
/// a .span() method.
pub trait SpannedError: Spanned {
    fn error(&self, message: &str) -> TokenStream {
        let span = self.spanned_span();

        let ident = Ident::new("compile_error", span.clone());

        let mut punct = Punct::new('!', Spacing::Alone);
        punct.set_span(span.clone());

        let mut group = Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(Literal::string(message))),
        );
        group.set_span(span.clone());

        TokenStream::from_iter([
            TokenTree::Ident(ident),
            TokenTree::Punct(punct),
            TokenTree::Group(group),
        ])
        .into()
    }
}

trait Spanned {
    fn spanned_span(&self) -> Span;
}

impl<T: Spanned> SpannedError for T {}

impl Spanned for Span {
    fn spanned_span(&self) -> Span {
        self.clone()
    }
}

impl Spanned for TokenTree {
    fn spanned_span(&self) -> Span {
        self.span()
    }
}

impl Spanned for Group {
    fn spanned_span(&self) -> Span {
        self.span()
    }
}

impl Spanned for Punct {
    fn spanned_span(&self) -> Span {
        self.span()
    }
}

impl Spanned for Ident {
    fn spanned_span(&self) -> Span {
        self.span()
    }
}

impl Spanned for Literal {
    fn spanned_span(&self) -> Span {
        self.span()
    }
}

impl Spanned for Tokens {
    fn spanned_span(&self) -> Span {
        self.first().map(|tt| tt.span()).unwrap_or(Span::call_site())
    }
}

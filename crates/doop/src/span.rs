use crate::*;

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

pub trait Spanned {
    fn spanned_span(&self) -> Span;

    fn error(&self, message: &str) -> TokenStream1 {
        let span = self.spanned_span();

        let ident = Ident::new("compile_error", span.clone());

        let punct = Punct::new('!', Spacing::Alone).tap_mut(|tt| tt.set_span(span.clone()));

        let group = Group::new(
            Delimiter::Parenthesis,
            TokenStream2::from(TokenTree::Literal(Literal::string(message))),
        )
        .tap_mut(|tt| tt.set_span(span.clone()));

        TokenStream2::from_iter([
            TokenTree::Ident(ident),
            TokenTree::Punct(punct),
            TokenTree::Group(group),
        ])
        .into()
    }
}

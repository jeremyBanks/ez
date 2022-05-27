use crate::*;

impl SpanExt for Span {}
pub trait SpanExt: Borrow<Span> + BorrowMut<Span> {
    fn error(&self, message: &str) -> TokenStream1 {
        let span = self.borrow();

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

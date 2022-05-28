use crate::*;

impl TokenStreamExt for TokenStream {}
pub trait TokenStreamExt: Borrow<TokenStream> + BorrowMut<TokenStream> {
    fn replace(&self, replacements: &HashMap<String, TokenStream>) -> TokenStream {
        let mut output = TokenStream::new();
        for tree in self.borrow().clone() {
            match &tree {
                TokenTree::Ident(ident) =>
                    if let Some(replacement) = replacements.get(&ident.to_string()) {
                        output.extend(replacement.clone());
                    } else {
                        output.extend(Some(tree));
                    },
                TokenTree::Group(group) => {
                    output.extend(Some(TokenTree::Group(Group::new(
                        group.delimiter(),
                        group.stream().replace(replacements),
                    ))));
                }
                _ => output.extend(Some(tree)),
            }
        }
        output
    }
}

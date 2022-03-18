use {
    proc_macro2::{Group, Ident, TokenStream, TokenTree},
    quote::ToTokens,
};

pub struct Doop {
    pub evaluation: crate::evaluation::Doop,
}

impl TryFrom<crate::evaluation::Doop> for Doop {
    type Error = syn::Error;
    fn try_from(evaluation: crate::evaluation::Doop) -> Result<Self, Self::Error> {
        Ok(Doop { evaluation })
    }
}

impl IntoIterator for Doop {
    type Item = proc_macro2::TokenTree;
    type IntoIter = proc_macro2::token_stream::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        TokenStream::new().into_iter()
    }
}

fn replace_ident_in_token_stream(
    input: TokenStream,
    ident: &Ident,
    replacement: TokenStream,
) -> Result<TokenStream, syn::Error> {
    let mut output = TokenStream::new();
    for token in input {
        match token {
            TokenTree::Ident(ref candidate) =>
                if *candidate == *ident {
                    output.extend(replacement.clone().into_token_stream());
                } else {
                    output.extend([token.clone()]);
                },

            TokenTree::Group(group) => output.extend([TokenTree::Group(Group::new(
                group.delimiter(),
                replace_ident_in_token_stream(group.stream(), ident, replacement.clone())?,
            ))]),
            _ => output.extend([token.clone()]),
        }
    }
    Ok(output)
}

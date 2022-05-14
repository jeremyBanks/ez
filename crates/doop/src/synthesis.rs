use {
    crate::*,
    proc_macro2::{Group, Ident, TokenStream, TokenTree},
    quote::ToTokens,
};

pub struct Doop {
    pub output: TokenStream,
}

impl TryFrom<crate::evaluation::Doop> for Doop {
    type Error = syn::Error;
    fn try_from(evaluation: crate::evaluation::Doop) -> Result<Self, Self::Error> {
        let mut output = TokenStream::new();

        for item in evaluation.items {
            let mut body = item.body;

            for binding in item.for_bindings {
                let mut binding_body = TokenStream::new();

                let ident = match binding.target {
                    crate::evaluation::ForBindingTarget::Ident(ident) => ident,
                    crate::evaluation::ForBindingTarget::Tuple(_) => todo!(),
                };
                for entry in binding.entries {
                    binding_body.extend(replace_ident_in_token_stream(body.clone(), &ident, entry))
                }
                body = binding_body;
            }
            output.extend(body);
        }

        Ok(Doop { output })
    }
}

impl IntoIterator for Doop {
    type Item = proc_macro2::TokenTree;
    type IntoIter = proc_macro2::token_stream::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.output.into_iter()
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

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

                match binding.target {
                    crate::evaluation::ForBindingTarget::Ident(ident) => {
                        for entry in binding.entries {
                            binding_body.extend(replace_ident_in_token_stream(body.clone(), &ident, entry))
                        }
                    },
                    crate::evaluation::ForBindingTarget::Tuple(idents) => {
                        for entry in binding.entries {
                            let mut tuple_binding_body = body.clone();

                            if let TokenTree::Group(group) = &entry.into_iter().next().unwrap() {
                                if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                                    return Err(syn::Error::new(
                                        group.span(),
                                        "expected tuple binding to be wrapped in parentheses",
                                    ));
                                }

                                // idiot, you need to comma-delimit stuff yourself
                                let tuple_tokens = groupw.stream().into_iter().collect::<Vec<_>>();

                                let expected_len = if idents.is_empty() {
                                    0
                                } else {
                                    idents.len() - 1
                                };
                                assert_eq!(tuple_replacements.len(), expected_len, "wrong number of replacements");
                                let tuple_replacements = tuple_replacements.into_iter();

                                let replacements = vec![];
                                for (i, ident) in idents.iter().enumerate() {
                                    let replacement = tuple_replacements.next().unwrap();
                                    tuple_binding_body = replace_ident_in_token_stream(tuple_binding_body, &ident, replacement)?;
                                    tuple_replacements.next();
                                }

                                // HACK
                                let items = group.stream().into_iter().filter(|tt| tt.to_string() != ",").collect::<Vec<_>>();

                                for (ident, entry) in idents.iter().zip(items) {
                                    tuple_binding_body = replace_ident_in_token_stream(tuple_binding_body, ident, entry)?;
                                }
                            } else {
                                panic!("bad tuple binding")
                            }

                            binding_body.extend(tuple_binding_body)
                        }
                    },
                };
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

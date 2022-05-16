use {
    crate::*,
    proc_macro2::{Group, Ident, TokenStream, TokenTree},
    quote::ToTokens,
};

pub struct Doop {
    pub output: TokenStream,
}

impl TryFrom<evaluation::Doop> for Doop {
    type Error = syn::Error;
    fn try_from(evaluation: evaluation::Doop) -> Result<Self, Self::Error> {
        let mut output = TokenStream::new();

        for item in evaluation.items {
            let mut body = item.body;

            for binding in item.for_bindings.into_iter().rev() {
                let mut binding_body = TokenStream::new();

                match binding.target {
                    evaluation::ForBindingTarget::Ident(ident) =>
                        for entry in binding.entries {
                            if let Some(ident) = &ident {
                                binding_body.extend(replace_ident_in_token_stream(
                                    body.clone(),
                                    ident,
                                    entry,
                                ));
                            } else {
                                binding_body.extend(body.clone());
                            }
                        },
                    evaluation::ForBindingTarget::Tuple(idents) =>
                        for entry in binding.entries {
                            let mut tuple_binding_body = body.clone();

                            if let TokenTree::Group(group) =
                                &entry.into_iter().next().expect("no tuple?")
                            {
                                if group.delimiter() != proc_macro2::Delimiter::Parenthesis {
                                    return Err(syn::Error::new(
                                        group.span(),
                                        "expected tuple binding to be wrapped in parentheses",
                                    ));
                                }

                                let tuple_tokens = group.stream().into_iter().collect::<Vec<_>>();

                                let mut tuple_streams = vec![];
                                let mut next_tuple_stream = vec![];
                                for token in tuple_tokens {
                                    if token.to_string() == "," {
                                        tuple_streams.push(next_tuple_stream);
                                        next_tuple_stream = vec![];
                                    } else {
                                        next_tuple_stream.push(token);
                                    }
                                }
                                if !(tuple_streams.is_empty() && next_tuple_stream.is_empty()) {
                                    tuple_streams.push(next_tuple_stream);
                                }

                                assert_eq!(
                                    idents.len(),
                                    tuple_streams.len(),
                                    "wrong number of replacements"
                                );

                                for (ident, replacement) in
                                    idents.clone().into_iter().zip(tuple_streams)
                                {
                                    if let Some(ident) = ident {
                                        tuple_binding_body = replace_ident_in_token_stream(
                                            tuple_binding_body,
                                            &ident,
                                            replacement.into_iter().collect(),
                                        )?;
                                    }
                                }
                            } else {
                                panic!("bad tuple binding")
                            }

                            binding_body.extend(tuple_binding_body)
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

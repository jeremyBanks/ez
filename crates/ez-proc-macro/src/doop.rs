use {
    crate::common::TokenTreeExt,
    std::collections::HashMap,
    ::{
        proc_macro2::{Group, Ident, TokenStream, TokenTree},
        quote::ToTokens,
    },
};

pub fn doop(tokens: TokenStream) -> Result<TokenStream, syn::Error> {
    let tokens: Vec<TokenTree> = tokens.into_iter().collect();

    let doop_args = tokens[0].tagged("doop_args")?;

    let mut named_replacements = HashMap::<Ident, Vec<TokenTree>>::new();
    doop_args[0]
        .tagged("let_bindings")?
        .into_iter()
        .try_for_each(|input| {
            let let_binding = input.tagged("let_binding")?;

            let name = &let_binding[0].tagged("let_binding_name")?[0];
            let name = name.ident()?;

            let refs = let_binding[1]
                .tagged("let_binding_refs")?
                .into_iter()
                .map(|r| r.ident())
                .collect::<Result<Vec<Ident>, _>>()?;

            let mut replacements = let_binding[2].tagged("let_binding_replacements")?;

            for ref_ in refs {
                match named_replacements.get(&ref_) {
                    Some(named_replacements) => {
                        replacements.extend(named_replacements.clone());
                    },
                    None => {
                        return Err(syn::Error::new(ref_.span(), "not defined"));
                    },
                }
            }

            named_replacements.insert(name, replacements.into_iter().collect());

            Ok(())
        })?;

    let mut output = TokenStream::new();

    {
        // let loops = doop_args[1].tagged("loops")?;
        // let loops = loops
        //     .into_iter()
        //     .map(|input| {
        //         let loop_ = input.tagged("loop")?;

        //         let loop_bindings = loop_[0].tagged("loop_bindings")?;
        //         let loop_bindings = loop_bindings
        //             .into_iter()
        //             .map(|input| {
        //                 let loop_binding = input.tagged("loop_binding")?;

        //                 let name =
        // loop_binding[0].tagged("loop_binding_name")?;

        //                 let refs = loop_binding[1]
        //                     .tagged("loop_binding_refs")?
        //                     .into_iter()
        //                     .map(|r| r.ident())
        //                     .collect::<Result<Vec<Ident>, _>>()?;

        //                 let replacements =
        // loop_binding[2].tagged("loop_binding_replacements")?;

        //                 Ok((name, refs, replacements))
        //             })
        //             .collect::<Result<_, _>>()?;

        //         let body = loop_[1].tagged("body")?;

        //         Ok((loop_bindings, body))
        //     })
        //     .collect::<Result<_, _>>()?;
    }

    // for Repetition {
    //     ident,
    //     replacements,
    // } in repetitions
    // {
    //     let base = output.clone();
    //     output = TokenStream::new();

    //     for replacement in replacements {
    //         output.extend(replace_ident_in_token_stream(
    //             base.clone(),
    //             &ident,
    //             [replacement.clone()].into_iter().collect(),
    //         )?);
    //     }
    // }

    Ok(output.into_iter().collect())
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

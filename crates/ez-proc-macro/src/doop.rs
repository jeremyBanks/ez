use {
    crate::common::TokenTreeExt,
    ::{
        proc_macro2::{Group, Ident, TokenStream, TokenTree},
        quote::ToTokens,
    },
};

pub fn doop(tokens: TokenStream) -> eyre::Result<TokenStream> {
    #[derive(Debug)]
    struct Repetition {
        ident: Ident,
        replacements: Vec<TokenTree>,
    }

    let input: Vec<TokenTree> = tokens.into_iter().collect();
    assert_eq!(input.len(), 2);

    let repetitions = input[0].map(|t| {
        let children = t.children().unwrap();
        let ident = children[0].only().unwrap().ident().unwrap();
        let replacements = children[1].children().unwrap();
        Repetition {
            ident,
            replacements,
        }
    });

    let block = input[1].children()?;

    let mut output: TokenStream = block.into_iter().collect();

    for Repetition {
        ident,
        replacements,
    } in repetitions
    {
        let base = output.clone();
        output = TokenStream::new();

        for replacement in replacements {
            output.extend(replace_ident_in_token_stream(
                base.clone(),
                &ident,
                [replacement.clone()].into_iter().collect(),
            )?);
        }
    }

    Ok(output.into_iter().collect())
}

fn replace_ident_in_token_stream(
    input: TokenStream,
    ident: &Ident,
    replacement: TokenStream,
) -> eyre::Result<TokenStream> {
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

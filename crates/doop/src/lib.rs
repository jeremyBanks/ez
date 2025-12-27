//! `doop` - A macro for local code duplication using loop-style syntax
//!
//! Pronounced like "dupe", spelled like "loop".

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, Token,
};

/// The main doop macro for code duplication
///
/// # Example
/// ```ignore
/// doop::doop! {
///     for Type in [u8, i8, u16, i16] {
///         impl Display for Type {
///             fn fmt(&self, f: &mut Formatter) -> fmt::Result {
///                 write!(f, "{}", *self)
///             }
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn doop(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DoopInput);

    match expand_doop(input) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Parse the doop input syntax
struct DoopInput {
    loops: Vec<DoopLoop>,
}

struct DoopLoop {
    var_name: Ident,
    values: Vec<TokenStream2>,
    body: TokenStream2,
}

impl Parse for DoopInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut loops = Vec::new();

        while !input.is_empty() {
            loops.push(input.parse()?);
        }

        Ok(DoopInput { loops })
    }
}

impl Parse for DoopLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse: for VarName in [value1, value2, ...]
        input.parse::<Token![for]>()?;
        let var_name: Ident = input.parse()?;
        input.parse::<Token![in]>()?;

        // Parse the array of values
        let content;
        syn::bracketed!(content in input);
        let values: Punctuated<syn::Expr, Token![,]> =
            content.parse_terminated(syn::Expr::parse, Token![,])?;

        let values: Vec<TokenStream2> = values
            .into_iter()
            .map(|expr| quote! { #expr })
            .collect();

        // Parse the body block
        let body_content;
        syn::braced!(body_content in input);
        let body: TokenStream2 = body_content.parse()?;

        Ok(DoopLoop {
            var_name,
            values,
            body,
        })
    }
}

fn expand_doop(input: DoopInput) -> syn::Result<TokenStream2> {
    let mut output = TokenStream2::new();

    for doop_loop in input.loops {
        output.extend(expand_loop(doop_loop)?);
    }

    Ok(output)
}

fn expand_loop(doop_loop: DoopLoop) -> syn::Result<TokenStream2> {
    let DoopLoop { var_name, values, body } = doop_loop;

    let mut expanded = TokenStream2::new();

    // For each value, substitute it into the body
    for value in values {
        let substituted = substitute_tokens(&body, &var_name, &value);
        expanded.extend(substituted);
    }

    Ok(expanded)
}

/// Simple token substitution: replace all instances of `target` with `replacement`
fn substitute_tokens(
    tokens: &TokenStream2,
    target: &Ident,
    replacement: &TokenStream2,
) -> TokenStream2 {
    use proc_macro2::TokenTree;

    let mut result = TokenStream2::new();

    for tt in tokens.clone() {
        match tt {
            TokenTree::Ident(ref ident) if ident == target => {
                result.extend(replacement.clone());
            }
            TokenTree::Group(group) => {
                let substituted = substitute_tokens(
                    &group.stream(),
                    target,
                    replacement,
                );
                result.extend(std::iter::once(TokenTree::Group(
                    proc_macro2::Group::new(group.delimiter(), substituted),
                )));
            }
            other => {
                result.extend(std::iter::once(other));
            }
        }
    }

    result
}

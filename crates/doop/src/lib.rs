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
    var_names: Vec<Ident>,  // Can be single or tuple
    values: Vec<Vec<TokenStream2>>,  // Each value is a Vec (for tuples)
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
        //    or: for (Var1, Var2) in [(val1, val2), ...]
        input.parse::<Token![for]>()?;

        // Check if we have a tuple pattern or single identifier
        let var_names = if input.peek(syn::token::Paren) {
            // Parse tuple of identifiers
            let content;
            syn::parenthesized!(content in input);
            let idents: Punctuated<Ident, Token![,]> =
                content.parse_terminated(Ident::parse, Token![,])?;
            idents.into_iter().collect()
        } else {
            // Single identifier
            vec![input.parse::<Ident>()?]
        };

        input.parse::<Token![in]>()?;

        // Parse the array of values
        let array_content;
        syn::bracketed!(array_content in input);
        let raw_values: Punctuated<syn::Expr, Token![,]> =
            array_content.parse_terminated(syn::Expr::parse, Token![,])?;

        // Parse each value - it might be a tuple
        let mut values = Vec::new();
        for expr in raw_values {
            if let syn::Expr::Tuple(ref tuple) = expr {
                // It's a tuple - extract each element
                let elements: Vec<TokenStream2> = tuple
                    .elems
                    .iter()
                    .map(|e| quote! { #e })
                    .collect();

                if elements.len() != var_names.len() {
                    return Err(syn::Error::new_spanned(
                        expr,
                        format!("Tuple has {} elements but pattern has {} variables",
                                elements.len(), var_names.len())
                    ));
                }
                values.push(elements);
            } else {
                // Single value
                if var_names.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        &expr,
                        format!("Expected tuple with {} elements, got single value",
                                var_names.len())
                    ));
                }
                values.push(vec![quote! { #expr }]);
            }
        }

        // Parse the body block
        let body_content;
        syn::braced!(body_content in input);
        let body: TokenStream2 = body_content.parse()?;

        Ok(DoopLoop {
            var_names,
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
    let DoopLoop { var_names, values, body } = doop_loop;

    let mut expanded = TokenStream2::new();

    // For each set of values, substitute all variables into the body
    for value_set in values {
        let mut substituted = body.clone();

        // Apply each substitution in order
        for (var_name, value) in var_names.iter().zip(value_set.iter()) {
            substituted = substitute_tokens(&substituted, var_name, value);
        }

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

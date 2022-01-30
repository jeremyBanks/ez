use {
    proc_macro::{Ident, TokenStream},
    quote::{quote, quote_spanned, ToTokens},
    syn::{parse_macro_input, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned},
};

/// Adds a panicking alternative to a fallible function.
#[proc_macro_attribute]
pub fn throws(attribute: TokenStream, input: TokenStream) -> TokenStream {
    let function: syn::ImplItemMethod = parse_macro_input!(input);

    let attrs = function.attrs.clone();
    let vis = function.vis.clone();

    let unwrapped_signature = function.sig.clone();
    let fallible_block = function.block.clone();

    let unwrapped_name = unwrapped_signature.ident.to_string();
    let fallible_name = format!("try_{unwrapped_name}");
    let mut must_be_method = false;
    let args = Punctuated::<syn::Ident, syn::Token![,]>::from_iter(
        unwrapped_signature.inputs.iter().map(|arg| match arg {
            syn::FnArg::Typed(arg) => match &*arg.pat {
                syn::Pat::Ident(pat) => pat.ident.clone(),
                _ => panic!("Only named arguments are supported"),
            },
            syn::FnArg::Receiver(receiver) => {
                must_be_method = true;
                syn::Ident::new("self", receiver.span())
            },
        }),
    );

    let unwrapped_return_type = match &unwrapped_signature.output {
        syn::ReturnType::Default => quote_spanned!(unwrapped_signature.output.span() => ()),
        syn::ReturnType::Type(_, ref ty) =>
            quote_spanned!(unwrapped_signature.output.span() => #ty),
    };

    let error_type = if attribute.is_empty() {
        quote_spanned! {
            unwrapped_signature.span() =>
            ::eyre::Report
        }
    } else {
        attribute.into()
    };

    let fallible_return_type: syn::ReturnType = parse_quote_spanned! {
        unwrapped_signature.span() =>
        -> std::result::Result<#unwrapped_return_type, #error_type>
    };

    let unwrapped_doc_suffix = format!(
        "This is a variation of [`{}`] that handles errors by [panicking][panic!] (crashing) \
         instead of returning a [`Result::Err`].",
        fallible_name
    );
    let fallible_doc_suffix = format!(
        "This is a variation of [`{}`] that handles errors by returning a [`Result::Err`] instead \
         of [panicking][panic!] (crashing).",
        unwrapped_name
    );

    let mut fallible_signature = unwrapped_signature.clone();
    let fallible_ident = syn::Ident::new(&fallible_name, function.sig.ident.span());
    fallible_signature.ident = fallible_ident.clone();
    fallible_signature.output = fallible_return_type;

    let fallible = if must_be_method {
        quote_spanned! { fallible_ident.span() => Self::#fallible_ident }
    } else {
        quote_spanned! { fallible_ident.span() => #fallible_ident }
    };

    let fallible_block = if fallible_block.clone().into_token_stream().to_string() != "{ ; }" {
        quote_spanned! { fallible_block.span() => {
            let result = { #fallible_block };
            Ok(result)
        } }
    } else {
        quote_spanned! { fallible_block.span() => ; }
    };

    TokenStream::from(quote_spanned! {
        function.span() =>
        #(#attrs)*
        ///
        #[doc = #unwrapped_doc_suffix]
        #vis
        #unwrapped_signature {
            #fallible(#args).unwrap()
        }
        #[allow(unused_functions)]
        #(#attrs)*
        ///
        #[doc = #fallible_doc_suffix]
        #vis
        #fallible_signature #fallible_block
    })
}

#[proc_macro_attribute]
pub fn main(attributes: TokenStream, function: TokenStream) -> TokenStream {
    let function: syn::ImplItemMethod = parse_macro_input!(function);

    let args_item = if !attributes.is_empty() {
        let attributes: syn::ItemStruct = parse_macro_input!(attributes);
        quote_spanned! {
            attributes.span() =>
            #[derive(clap::Parser)]
            #attributes
        }
    } else {
        quote! {}
    };

    let fn_item = quote_spanned! {
        function.span() =>
        #function
    };

    quote! {
        #args_item
        #fn_item
    }
    .into()
}

#[proc_macro_derive(Int)]
pub fn derive_int(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(Float)]
pub fn derive_float(_int_struct: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

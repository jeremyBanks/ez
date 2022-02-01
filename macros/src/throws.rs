use {
    proc_macro::TokenStream,
    quote::{quote, ToTokens},
    syn::{parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned},
};

pub fn panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let error_type = error_type_for_attribute(attribute_tokens);
    let has_body = function_has_body(function_tokens.clone());
    let mut function: syn::ImplItemMethod = must_parse(function_tokens);
    let output_type = match &function.sig.output {
        syn::ReturnType::Default => parse_quote! { () },
        syn::ReturnType::Type(_, output) => output.clone(),
    };

    if has_body {
        let try_block = try_block(&function.block, &output_type, &error_type);
        function.block = parse_quote! { {
            #try_block.expect("error in #[ez::panics] function")
        } };
    } else {
        return quote! { compile_error!("#[ez::panics] function must have a body"); }
            .to_token_stream()
            .into();
    }

    function.to_token_stream().into()
}

pub fn try_or_panics(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let error_type = error_type_for_attribute(attribute_tokens);
    let has_body = function_has_body(function_tokens.clone());
    let mut function: syn::ImplItemMethod = must_parse(function_tokens);
    let has_receiver = function.sig.receiver().is_some();
    let output_type = match &function.sig.output {
        syn::ReturnType::Default => parse_quote! { () },
        syn::ReturnType::Type(_, output) => output.clone(),
    };

    if !has_body && !has_receiver {
        return quote! { compile_error!("#[ez::try_or_panics] function must have a body or a receiver (`self` parameter)"); }.to_token_stream().into();
    }

    let mut try_function = function.clone();
    try_function.sig.output = parse_quote_spanned! {
        try_function.sig.output.span() =>
        -> ::core::result::Result<#output_type, #error_type>
    };
    let try_name = format!("try_{}", function.sig.ident);
    try_function.sig.ident = syn::Ident::new(&try_name, try_function.sig.ident.span());
    let try_ident = try_function.sig.ident.clone();

    if has_receiver {
        let args = Punctuated::<syn::Ident, syn::Token![,]>::from_iter(
            try_function.sig.inputs.iter().map(|arg| match arg {
                syn::FnArg::Receiver(receiver) => syn::Ident::new("self", receiver.span()),
                syn::FnArg::Typed(arg) => match &*arg.pat {
                    syn::Pat::Ident(pat) => pat.ident.clone(),
                    _ => panic!(
                        "#[ez::try_or_panics] doesn't support complicated patterns in arguments"
                    ),
                },
            }),
        );

        let try_block = try_block(&function.block, &output_type, &error_type);

        function.block = parse_quote! { {
            Self::#try_ident(#args).expect("error in #[ez::panics] function")
        } };

        if has_body {
            try_function.block = parse_quote! { {
                #try_block
            } };
        }
    } else {
        let try_block = try_block(&function.block, &output_type, &error_type);

        function.block = parse_quote! { {
            #try_block.expect("error in #[ez::panics] function")
        } };

        try_function.block = parse_quote! { {
            #try_block
        } };
    }

    quote! {
        #function

        #try_function
    }
    .to_token_stream()
    .into()
}

pub fn throws(attribute_tokens: TokenStream, function_tokens: TokenStream) -> TokenStream {
    let error_type = error_type_for_attribute(attribute_tokens);
    let has_body = function_has_body(function_tokens.clone());
    let mut function: syn::ImplItemMethod = must_parse(function_tokens);
    let output_type = match &function.sig.output {
        syn::ReturnType::Default => parse_quote! { () },
        syn::ReturnType::Type(_, output) => output.clone(),
    };

    if has_body {
        let try_block = try_block(&function.block, &output_type, &error_type);

        function.block = parse_quote! { {
            #try_block
        } };
    }

    function.sig.output = parse_quote_spanned! {
        function.sig.output.span() =>
        -> ::core::result::Result<#output_type, #error_type>
    };

    function.to_token_stream().into()
}

fn try_block(block: &syn::Block, output_type: &syn::Type, error_type: &syn::ExprPath) -> syn::Expr {
    parse_quote_spanned! {
        block.span() => {
            let ez_unhygienic_inner = || {
                let ez_unhygienic_value: #output_type = #block;
                ::core::result::Result::<#output_type, #error_type>::Ok(value)
            };
            ez_unhygienic_inner()
        }
    }
}

fn error_type_for_attribute(attribute: TokenStream) -> syn::ExprPath {
    attribute
        .is_empty()
        .then(|| {
            parse_quote! { ::eyre::Report }
        })
        .unwrap_or_else(|| must_parse(attribute))
}

fn must_parse<T: syn::parse::Parse>(input: TokenStream) -> T {
    syn::parse(input)
        .map_err(|err| err.to_compile_error())
        .unwrap()
}

fn function_has_body(function: TokenStream) -> bool {
    let _as_fn: syn::ImplItemMethod = must_parse(function.clone());
    let as_trait_method: Option<syn::TraitItemMethod> = syn::parse(function).ok();
    as_trait_method.map_or(true, |method| method.default.is_some())
}

pub(crate) mod semantics;
pub(crate) mod syntax;
pub(crate) mod synthesis;

#[proc_macro]
pub fn doop(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();

    let syntax: syntax::DoopBlock = match syn::parse2(tokens) {
        Ok(syntax) => syntax,
        Err(report) => return report.to_compile_error().into(),
    };

    let semantics: semantics::Doop = match syntax.try_into() {
        Ok(semantics) => semantics,
        Err(report) => return report.to_compile_error().into(),
    };

    let synthesis: synthesis::Doop = match semantics.try_into() {
        Ok(synthesis) => synthesis,
        Err(report) => return report.to_compile_error().into(),
    };

    let tokens = synthesis.into_iter();
    let tokens: proc_macro2::TokenStream = tokens.collect();
    let tokens: proc_macro::TokenStream = tokens.into();

    tokens
}

#[proc_macro_attribute]
pub fn dooped(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let item: proc_macro2::TokenStream = item.into();

    let syntax = match syntax::DoopItem::try_from_tokens(attr, item) {
        Ok(syntax) => syntax,
        Err(report) => return report.to_compile_error().into(),
    };

    let semantics: semantics::Doop = match syntax.try_into() {
        Ok(semantics) => semantics,
        Err(report) => return report.to_compile_error().into(),
    };

    let synthesis: synthesis::Doop = match semantics.try_into() {
        Ok(synthesis) => synthesis,
        Err(report) => return report.to_compile_error().into(),
    };

    let tokens = synthesis.into_iter();
    let tokens: proc_macro2::TokenStream = tokens.collect();
    let tokens: proc_macro::TokenStream = tokens.into();

    tokens
}

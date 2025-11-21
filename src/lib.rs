use proc_macro::TokenStream;
use quote::quote;

mod core;

#[proc_macro]
pub fn quip(input: TokenStream) -> TokenStream {
    let path = quote!(::quote::quote!);
    core::expand(path, input.into()).into()
}

#[proc_macro]
pub fn quip_spanned(input: TokenStream) -> TokenStream {
    let path = quote!(::quote::quote_spanned!);
    core::expand(path, input.into()).into()
}

#[proc_macro]
pub fn parse_quip(input: TokenStream) -> TokenStream {
    let path = quote!(::syn::parse_quote!);
    core::expand(path, input.into()).into()
}

#[proc_macro]
pub fn parse_quip_spanned(input: TokenStream) -> TokenStream {
    let path = quote!(::syn::parse_quote_spanned!);
    core::expand(path, input.into()).into()
}

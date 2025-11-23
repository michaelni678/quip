use quip::quip;
use quote::quote;

mod utilities;

#[test]
fn lookalike() {
    assert!(utilities::tokens::eq(quip!(r#""#{x}), quote!(r#""#{x})));
}

#[test]
fn empty() {
    assert!(utilities::tokens::eq(quip!(#{}), quote!(#{})));
}

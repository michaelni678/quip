use quip::quip;
use quote::quote;

mod utilities;

#[test]
fn lookalike() {
    assert!(utilities::tokens::eq(quip!(r#""#{x}), quote!(r#""#{x})));
}

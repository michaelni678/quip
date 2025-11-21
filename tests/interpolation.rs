use quip::quip;
use quote::quote;

mod utilities;

#[test]
fn variable() {
    let x = quote!(X);

    assert!(utilities::tokens::eq(quip!(#{x}), quote!(#x)));
    assert!(utilities::tokens::eq(quip!(<#{x}>), quote!(<#x>)));
    assert!(utilities::tokens::eq(quip!((#{x})), quote!((#x))));
    assert!(utilities::tokens::eq(quip!([#{x}]), quote!([#x])));
    assert!(utilities::tokens::eq(quip!({#{x}}), quote!({#x})));
}

#[test]
fn variable_repetition() {
    let xy = [quote!(X), quote!(Y)];

    assert!(utilities::tokens::eq(quip!(#(#{xy})*), quote!(#(#xy)*)));
    assert!(utilities::tokens::eq(quip!(#(#{xy},)*), quote!(#(#xy,)*)));
    assert!(utilities::tokens::eq(quip!(#(#{xy}),*), quote!(#(#xy),*)));
}

#[test]
fn expression() {
    let tx = (quote!(X),);
    let x = &tx.0;

    assert!(utilities::tokens::eq(quip!(#{tx.0}), quote!(#x)));
    assert!(utilities::tokens::eq(quip!(<#{tx.0}>), quote!(<#x>)));
    assert!(utilities::tokens::eq(quip!((#{tx.0})), quote!((#x))));
    assert!(utilities::tokens::eq(quip!([#{tx.0}]), quote!([#x])));
    assert!(utilities::tokens::eq(quip!({#{tx.0}}), quote!({#x})));
}

#[test]
fn expression_repetition() {
    let txy = ([quote!(X), quote!(Y)],);
    let xy = &txy.0;

    assert!(utilities::tokens::eq(quip!(#(#{txy.0})*), quote!(#(#xy)*)));
    assert!(utilities::tokens::eq(
        quip!(#(#{txy.0},)*),
        quote!(#(#xy,)*)
    ));
    assert!(utilities::tokens::eq(
        quip!(#(#{txy.0}),*),
        quote!(#(#xy),*)
    ));
}

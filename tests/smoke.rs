use met::{TokenStreamExt, tokens};
use proc_macro2::Span;
use quip::{parse_quip, parse_quip_spanned, quip, quip_spanned};
use quote::quote_spanned;
use syn::{ExprIf, parse_quote, parse_quote_spanned};

#[test]
fn quip() {
    let capacity = tokens!(256);
    let error = tokens!(Error::InsufficientCapacity);

    let output = quip! {
        if length + additional > #{capacity} {
            return Err(#{error});
        }
    };

    let expected = tokens! {
        if length + additional > 256 {
            return Err(Error::InsufficientCapacity);
        }
    };

    assert!(output.equals(&expected));
}

#[test]
fn quip_spanned() {
    let span = Span::call_site();

    let capacity = tokens!(256);
    let error = tokens!(Error::InsufficientCapacity);

    let output = quip_spanned! { span =>
        if length + additional > #{capacity} {
            return Err(#{error});
        }
    };

    let expected = quote_spanned! { span =>
        if length + additional > 256 {
            return Err(Error::InsufficientCapacity);
        }
    };

    assert!(output.equals(&expected));
}

#[test]
fn parse_quip() {
    let capacity = tokens!(256);
    let error = tokens!(Error::InsufficientCapacity);

    let output: ExprIf = parse_quip! {
        if length + additional > #{capacity} {
            return Err(#{error});
        }
    };

    let expected: ExprIf = parse_quote! {
        if length + additional > 256 {
            return Err(Error::InsufficientCapacity);
        }
    };

    assert_eq!(output, expected);
}

#[test]
fn parse_quip_spanned() {
    let span = Span::call_site();

    let capacity = tokens!(256);
    let error = tokens!(Error::InsufficientCapacity);

    let output: ExprIf = parse_quip_spanned! { span =>
        if length + additional > #{capacity} {
            return Err(#{error});
        }
    };

    let expected: ExprIf = parse_quote_spanned! { span =>
        if length + additional > 256 {
            return Err(Error::InsufficientCapacity);
        }
    };

    assert_eq!(output, expected);
}

use proc_macro2::Span;
use quip::{parse_quip, parse_quip_spanned, quip, quip_spanned};
use syn::Generics;

#[test]
fn quip() {
    let _ = quip!(<'a, B: Into<String>, const C: usize>);
}

#[test]
fn quip_spanned() {
    let span = Span::call_site();
    let _ = quip_spanned!(span=> <'a, B: Into<String>, const C: usize>);
    let _ = quip_spanned! {span=>
        <'a, B: Into<String>, const C: usize>
    };
}

#[test]
fn parse_quip() {
    let _: Generics = parse_quip!(<'a, B: Into<String>, const C: usize>);
}

#[test]
fn parse_quip_spanned() {
    let span = Span::call_site();
    let _: Generics = parse_quip_spanned!(span=> <'a, B: Into<String>, const C: usize>);
    let _: Generics = parse_quip_spanned! {span=>
        <'a, B: Into<String>, const C: usize>
    };
}

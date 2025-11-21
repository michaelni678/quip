use proc_macro2::{TokenStream, TokenTree};

pub fn eq(tokens1: TokenStream, tokens2: TokenStream) -> bool {
    let mut tts2 = tokens2.into_iter();

    for tt1 in tokens1 {
        let Some(tt2) = tts2.next() else {
            return false;
        };

        let matches = match (tt1, tt2) {
            (TokenTree::Group(group1), TokenTree::Group(group2)) => {
                group1.delimiter() == group2.delimiter() && eq(group1.stream(), group2.stream())
            }
            (TokenTree::Ident(ident1), TokenTree::Ident(ident2)) => ident1 == ident2,
            (TokenTree::Punct(punct1), TokenTree::Punct(punct2)) => {
                punct1.as_char() == punct2.as_char() && punct1.spacing() == punct2.spacing()
            }
            (TokenTree::Literal(lit1), TokenTree::Literal(lit2)) => {
                lit1.to_string() == lit2.to_string()
            }
            _ => false,
        };

        if !matches {
            return false;
        }
    }

    tts2.next().is_none()
}

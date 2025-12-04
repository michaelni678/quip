use proc_macro2::{TokenStream, TokenTree};

pub fn token_streams_eq(left_token_stream: TokenStream, right_token_stream: TokenStream) -> bool {
    let mut left_token_trees = left_token_stream.into_iter();
    let mut right_token_trees = right_token_stream.into_iter();

    loop {
        match (left_token_trees.next(), right_token_trees.next()) {
            (Some(left_token_tree), Some(right_token_tree))
                if token_trees_eq(&left_token_tree, &right_token_tree) => {}
            (None, None) => return true,
            _ => return false,
        }
    }
}

pub fn token_trees_eq(left_token_tree: &TokenTree, right_token_tree: &TokenTree) -> bool {
    match (left_token_tree, right_token_tree) {
        (TokenTree::Group(left_group), TokenTree::Group(right_group)) => {
            left_group.delimiter() == right_group.delimiter()
                && token_streams_eq(left_group.stream(), right_group.stream())
        }
        (TokenTree::Ident(left_ident), TokenTree::Ident(right_ident)) => left_ident == right_ident,
        (TokenTree::Punct(left_punct), TokenTree::Punct(right_punct)) => {
            left_punct.as_char() == right_punct.as_char()
                && left_punct.spacing() == right_punct.spacing()
        }
        (TokenTree::Literal(left_literal), TokenTree::Literal(right_literal)) => {
            left_literal.to_string() == right_literal.to_string()
        }
        _ => false,
    }
}

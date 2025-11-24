use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{TokenStreamExt, format_ident, quote};

pub fn expand(path: TokenStream, input: TokenStream) -> TokenStream {
    let mut counter = 0;

    let mut variables = Vec::new();
    let mut expressions = Vec::new();

    let output = for_each_expression_interpolation(input, &mut |expression| {
        let variable = format_ident!("__interpolation{counter}");

        variables.push(variable.clone());
        expressions.push(expression);

        counter += 1;

        quote!(# #variable)
    });

    quote! {
        {
            #(let #variables = &#expressions;)*
            #path { #output }
        }
    }
}

fn for_each_expression_interpolation<F>(input: TokenStream, apply: &mut F) -> TokenStream
where
    F: FnMut(TokenStream) -> TokenStream,
{
    let mut output = TokenStream::new();
    let mut tts = input.into_iter().peekable();

    while let Some(tt) = tts.next() {
        if let TokenTree::Punct(ref punct) = tt
            && punct.as_char() == '#'
            && let Some(TokenTree::Group(group)) = tts.peek()
            && group.delimiter() == Delimiter::Brace
            && let expression = group.stream()
            && !expression.is_empty()
        {
            output.extend(apply(expression));

            tts.next();
        } else if let TokenTree::Group(group) = tt {
            output.append(Group::new(
                group.delimiter(),
                for_each_expression_interpolation(group.stream(), apply),
            ));
        } else {
            output.append(tt);
        }
    }

    output
}

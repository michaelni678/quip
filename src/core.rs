use proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree};
use quote::{TokenStreamExt, format_ident, quote};

pub fn expand(path: TokenStream, input: TokenStream) -> TokenStream {
    let mut variables = Vec::new();
    let mut expressions = Vec::new();

    let output = replace(input, &mut variables, &mut expressions);

    quote! {
        match (#(&#expressions,)*) {
            (#(#variables,)*) => {
                #path { #output }
            }
        }
    }
}

fn replace(
    input: TokenStream,
    variables: &mut Vec<Ident>,
    expressions: &mut Vec<TokenStream>,
) -> TokenStream {
    let mut counter = 0;

    walk(input, &mut |expression| {
        let variable = format_ident!("__interpolation{counter}");

        variables.push(variable.clone());
        expressions.push(expression);

        counter += 1;

        quote!(#variable)
    })
}

fn walk<F>(input: TokenStream, apply: &mut F) -> TokenStream
where
    F: FnMut(TokenStream) -> TokenStream,
{
    let mut output = TokenStream::new();
    let mut tts = input.into_iter().peekable();

    while let Some(tt) = tts.next() {
        match tt {
            TokenTree::Punct(punct) => {
                let character = punct.as_char();
                output.append(punct);

                if character == '#'
                    && let Some(TokenTree::Group(group)) = tts.peek()
                    && group.delimiter() == Delimiter::Brace
                    && let expression = group.stream()
                    && !expression.is_empty()
                {
                    output.extend(apply(expression));
                    tts.next();
                }
            }
            TokenTree::Group(group) => {
                let span = group.span();

                let mut group = Group::new(group.delimiter(), walk(group.stream(), apply));
                group.set_span(span);

                output.append(group);
            }
            _ => output.append(tt),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::replace;

    use proc_macro2::TokenStream;
    use quote::quote;
    use utilities::{compare::token_streams_eq, convert::into_array};

    // Quip does not validate the token stream inside expression interpolations.
    // Any errors within those interpolated expressions are outside Quip's
    // responsibility.
    //
    // This test verifies that the expressions are captured exactly as written.
    #[test]
    fn extracts_expression_verbatim() {
        let input = quote!(#{[] () for x ? << 0i16 ""});

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input, &mut variables, &mut expressions);

        let [variable] = into_array(variables);
        let [expression] = into_array(expressions);

        let expected = quote!(# #variable);

        assert!(token_streams_eq(
            expression,
            quote!([] () for x ? << 0i16 "")
        ));

        assert!(token_streams_eq(output, expected));
    }

    // This test verifies that expression interpolations are replaced with variable
    // interpolations.
    #[test]
    fn replaces_expression_interpolations() {
        let input = quote! {
            let #{x} = 0;

            impl #{y} for #{z} {}
        };

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input, &mut variables, &mut expressions);

        let [variable_x, variable_y, variable_z] = into_array(variables);
        let [expression_x, expression_y, expression_z] = into_array(expressions);

        let expected = quote! {
            let # #variable_x = 0;

            impl # #variable_y for # #variable_z {}
        };

        assert!(token_streams_eq(expression_x, quote!(x)));
        assert!(token_streams_eq(expression_y, quote!(y)));
        assert!(token_streams_eq(expression_z, quote!(z)));

        assert!(token_streams_eq(output, expected));
    }

    // This test verifies that expression interpolations are replaced with variable
    // interpolations within token tree groups.
    #[test]
    fn replaces_expression_interpolations_in_groups() {
        let input = quote! {
            let Some(#{x}) = #{y} else {
                return Err([#{z}]);
            };
        };

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input, &mut variables, &mut expressions);

        let [variable_x, variable_y, variable_z] = into_array(variables);
        let [expression_x, expression_y, expression_z] = into_array(expressions);

        let expected = quote! {
            let Some(# #variable_x) = # #variable_y else {
                return Err([# #variable_z]);
            };
        };

        assert!(token_streams_eq(expression_x, quote!(x)));
        assert!(token_streams_eq(expression_y, quote!(y)));
        assert!(token_streams_eq(expression_z, quote!(z)));

        assert!(token_streams_eq(output, expected));
    }

    // This test verifies Quip does not replace expression interpolations in the
    // edge cases described below.
    #[test]
    fn skips_invalid_expression_interpolations() {
        let input = quote! {
            // The interpolation contains an empty token stream.
            #{}
            // `#` is not a punctuation token, it belongs to the raw string
            // literal.
            r#""#{x}
            // `#` is not a punctuation token and `{x}` is not a braced group.
            // They're all part of a string literal.
            "#{x}"
        };

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input.clone(), &mut variables, &mut expressions);

        assert!(variables.is_empty());
        assert!(expressions.is_empty());

        let expected = input;

        assert!(token_streams_eq(output, expected));
    }

    // Variable interpolations are handled by the underlying macros. This test
    // verifies that all variable interpolations are skipped.
    #[test]
    fn skips_variable_interpolations() {
        let input = stringify! {
            impl Shape for #shape {
                const SIDES: usize = #sides;
            }
        };

        let input: TokenStream = input.parse().unwrap();

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input.clone(), &mut variables, &mut expressions);

        assert!(variables.is_empty());
        assert!(expressions.is_empty());

        let expected = input;

        assert!(token_streams_eq(output, expected));
    }
}

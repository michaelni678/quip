use met::GroupExt;
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

        TokenStream::from(TokenTree::Ident(variable))
    })
}

fn walk<F>(input: TokenStream, apply: &mut F) -> TokenStream
where
    F: FnMut(TokenStream) -> TokenStream,
{
    let mut output = TokenStream::new();
    let mut token_trees = input.into_iter().peekable();

    while let Some(token_tree) = token_trees.next() {
        match token_tree {
            TokenTree::Punct(punct) => {
                let character = punct.as_char();
                output.append(punct);

                if character == '#'
                    && let Some(TokenTree::Group(group)) = token_trees.peek()
                    && group.delimiter() == Delimiter::Brace
                    && let expression = group.stream()
                    && !expression.is_empty()
                {
                    output.extend(apply(expression));
                    token_trees.next();
                }
            }
            TokenTree::Group(group) => {
                output.append(Group::new_spanned(
                    group.span(),
                    group.delimiter(),
                    walk(group.stream(), apply),
                ));
            }
            _ => output.append(token_tree),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::replace;

    use met::{TokenStreamExt, tokens};
    use quote::quote;

    fn into_array<T, const N: usize>(value: impl TryInto<[T; N]>) -> [T; N] {
        let Ok(array) = value.try_into() else {
            panic!("failed to convert value into an array of size {N}");
        };

        array
    }

    // Quip does not validate the token stream inside expression interpolations.
    // Any errors within those interpolated expressions are outside Quip's
    // responsibility.
    //
    // This test verifies that the expressions are captured exactly as written.
    #[test]
    fn extracts_expression_verbatim() {
        let input = tokens!(#{[] () for x ? << 0i16 ""});

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input, &mut variables, &mut expressions);

        let [variable] = into_array(variables);
        let [expression] = into_array(expressions);

        let expected = quote!(# #variable);

        assert!(expression.equals(&tokens! { [] () for x ? << 0i16 "" }));
        assert!(output.equals(&expected));
    }

    // This test verifies that expression interpolations are replaced with variable
    // interpolations.
    #[test]
    fn replaces_expression_interpolations() {
        let input = tokens! {
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

        assert!(expression_x.equals(&tokens!(x)));
        assert!(expression_y.equals(&tokens!(y)));
        assert!(expression_z.equals(&tokens!(z)));

        assert!(output.equals(&expected));
    }

    // This test verifies that expression interpolations are replaced with variable
    // interpolations within token tree groups.
    #[test]
    fn replaces_expression_interpolations_in_groups() {
        let input = tokens! {
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

        assert!(expression_x.equals(&tokens!(x)));
        assert!(expression_y.equals(&tokens!(y)));
        assert!(expression_z.equals(&tokens!(z)));

        assert!(output.equals(&expected));
    }

    // This test verifies Quip does not replace expression interpolations in the
    // edge cases described below.
    #[test]
    fn skips_invalid_expression_interpolations() {
        let input = tokens! {
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

        assert!(output.equals(&expected));
    }

    // Variable interpolations are handled by the underlying macros. This test
    // verifies that all variable interpolations are skipped.
    #[test]
    fn skips_variable_interpolations() {
        let input = tokens! {
            impl Shape for #shape {
                const SIDES: usize = #sides;
            }
        };

        let mut variables = Vec::new();
        let mut expressions = Vec::new();

        let output = replace(input.clone(), &mut variables, &mut expressions);

        assert!(variables.is_empty());
        assert!(expressions.is_empty());

        let expected = input;

        assert!(output.equals(&expected));
    }
}

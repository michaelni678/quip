use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{format_ident, quote};
use tout::extension::{GroupExt, PunctExt, TokenStreamExt};

pub fn expand(path: TokenStream, input: TokenStream) -> TokenStream {
    let Replaced {
        variables,
        expressions,
        output,
    } = replace(input);

    quote! {
        match (#(&#expressions,)*) {
            (#(#variables,)*) => {
                #path { #output }
            }
        }
    }
}

struct Replaced {
    variables: Vec<Ident>,
    expressions: Vec<TokenStream>,
    output: TokenStream,
}

fn replace(input: TokenStream) -> Replaced {
    let mut variables = Vec::new();
    let mut expressions = Vec::new();

    let mut counter = 0;

    let output = walk(input, &mut |expression| {
        let variable = format_ident!("__interpolation{counter}");

        variables.push(variable.clone());
        expressions.push(expression);

        counter += 1;

        TokenStream::token(variable)
    });

    Replaced {
        variables,
        expressions,
        output,
    }
}

fn walk<F>(input: TokenStream, apply: &mut F) -> TokenStream
where
    F: FnMut(TokenStream) -> TokenStream,
{
    let mut output = TokenStream::new();
    let mut token_trees = input.into_iter().peekable();

    while let Some(token_tree) = token_trees.next() {
        match token_tree {
            TokenTree::Punct(punct)
                if punct.is_char('#')
                    && let Some(TokenTree::Group(group)) = token_trees.peek()
                    && group.is_braced()
                    && let expression = group.stream()
                    && !expression.is_empty() =>
            {
                output.append(punct);
                output.extend(apply(expression));
                token_trees.next();
            }
            TokenTree::Group(group) => output.append(Group::new_spanned(
                group.span(),
                group.delimiter(),
                walk(group.stream(), apply),
            )),
            _ => output.append(token_tree),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use tout::{assert::assert_stream_eq, quasi::stream};

    use super::replace;

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
        let input = quote! { #{ [] () for x ? << 0i16 "" } };

        let replaced = replace(input);

        let [variable] = into_array(replaced.variables);
        let [expression] = into_array(replaced.expressions);

        let expected = quote! { # #variable };

        assert_stream_eq!(expression, quote! { [] () for x ? << 0i16 "" });
        assert_stream_eq!(replaced.output, expected);
    }

    // This test verifies that expression interpolations are replaced with variable
    // interpolations.
    #[test]
    fn replaces_expression_interpolations() {
        let input = quote! {
            let #{x} = 0;

            impl #{y} for #{z} {}
        };

        let replaced = replace(input);

        let [variable_x, variable_y, variable_z] = into_array(replaced.variables);
        let [expression_x, expression_y, expression_z] = into_array(replaced.expressions);

        let expected = quote! {
            let # #variable_x = 0;

            impl # #variable_y for # #variable_z {}
        };

        assert_stream_eq!(expression_x, quote!(x));
        assert_stream_eq!(expression_y, quote!(y));
        assert_stream_eq!(expression_z, quote!(z));
        assert_stream_eq!(replaced.output, expected);
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

        let replaced = replace(input);

        let [variable_x, variable_y, variable_z] = into_array(replaced.variables);
        let [expression_x, expression_y, expression_z] = into_array(replaced.expressions);

        let expected = quote! {
            let Some(# #variable_x) = # #variable_y else {
                return Err([# #variable_z]);
            };
        };

        assert_stream_eq!(expression_x, quote!(x));
        assert_stream_eq!(expression_y, quote!(y));
        assert_stream_eq!(expression_z, quote!(z));
        assert_stream_eq!(replaced.output, expected);
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

        let replaced = replace(input.clone());

        assert!(replaced.variables.is_empty());
        assert!(replaced.expressions.is_empty());

        let expected = input;

        assert_stream_eq!(replaced.output, expected);
    }

    // Variable interpolations are handled by the underlying macros. This test
    // verifies that all variable interpolations are skipped.
    #[test]
    fn skips_variable_interpolations() {
        let input = stream! {
            impl Shape for #shape {
                const SIDES: usize = #sides;
            }
        };

        let replaced = replace(input.clone());

        assert!(replaced.variables.is_empty());
        assert!(replaced.expressions.is_empty());

        let expected = input;

        assert_stream_eq!(replaced.output, expected);
    }
}

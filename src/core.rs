use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{TokenStreamExt, format_ident, quote};

pub fn expand(path: TokenStream, input: TokenStream) -> TokenStream {
    let mut counter = 0;

    let mut idents = Vec::new();
    let mut values = Vec::new();

    let output = for_each_quip_interpolation(input, &mut |block| {
        let ident = format_ident!("interpolation{counter}");

        idents.push(ident.clone());
        values.push(block);

        counter += 1;

        quote!(# #ident)
    });

    quote! {
        {
            #(let #idents = &#values;)*
            #path { #output }
        }
    }
}

fn for_each_quip_interpolation<F>(input: TokenStream, apply: &mut F) -> TokenStream
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
        {
            output.extend(apply(group.stream()));

            tts.next();
        } else if let TokenTree::Group(group) = tt {
            output.append(Group::new(
                group.delimiter(),
                for_each_quip_interpolation(group.stream(), apply),
            ));
        } else {
            output.append(tt);
        }
    }

    output
}

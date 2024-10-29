use proc_macro::{quote, Spacing, TokenStream, TokenTree};

pub fn span_impl(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();
    let mut span_start = TokenStream::new();

    while let Some(tt) = input.next() {
        // the ".." syntax in terms of TokenTrees is represented as two
        // TokenTree::Punct tokens with .char == '.', with the first dot
        // having .spacing == Joint (see Spacing's docs for what that means)
        // and the latter having .spacing == Alone. below, we check exactly
        // for that

        if is_dot(Some(&tt), Spacing::Joint) {
            assert!(is_dot(input.next().as_ref(), Spacing::Alone));

            break;
        } else {
            span_start.extend_one(tt);
        }
    }

    let span_end = TokenStream::from_iter(input);

    quote! {
        ::core::range::Range { start: $span_start, end: $span_end }
    }
}

fn is_dot(tt: Option<&TokenTree>, spacing: Spacing) -> bool {
    if let Some(TokenTree::Punct(p)) = tt
        && p.as_char() == '.'
        && p.spacing() == spacing
    {
        return true;
    }

    false
}

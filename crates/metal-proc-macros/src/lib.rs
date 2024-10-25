use proc_macro::TokenStream;
use proc_macro_error2::ResultExt;
use syn::parse_macro_input;

mod spans;

#[proc_macro_derive(Spanned)]
pub fn derive_spanned(input: TokenStream) -> TokenStream {
    crate::spans::derive::derive_spanned_impl(parse_macro_input!(input as syn::DeriveInput)).into()
}

/// Add a `span: metal_lexer::Span` field to a named struct.
#[proc_macro_attribute]
pub fn spanned(_: TokenStream, item: TokenStream) -> TokenStream {
    crate::spans::attribute::spanned_impl(item.into())
        .unwrap_or_abort()
        .into()
}

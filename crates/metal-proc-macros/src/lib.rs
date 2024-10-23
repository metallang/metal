use proc_macro::TokenStream;

#[proc_macro_derive(Spanned)]
pub fn spanned_derive(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

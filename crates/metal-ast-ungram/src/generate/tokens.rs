use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::Engram;

mod enum_;
mod struct_;

pub fn generate_token_items(grammar: &Engram) -> TokenStream {
    let items = struct_::generate_struct_item(grammar).chain(enum_::generate_enum_item(grammar));

    quote! {
        use crate::{
            AstToken,
            SyntaxKind,
            SyntaxToken,
        };

        #(#items)*
    }
}

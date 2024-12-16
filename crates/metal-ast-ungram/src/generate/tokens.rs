// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::Engram;

mod token_alt_nodes;
mod token_structs;

/// Generates the `tokens.rs` file.
pub fn generate_tokens_file(grammar: &Engram) -> TokenStream {
    let token_structs = token_structs::generate_token_structs(grammar);
    let token_alt_nodes = token_alt_nodes::generate_token_alt_nodes(grammar);

    quote! {
        use crate::{
            AstToken,
            SyntaxKind,
            SyntaxToken,
        };

        #(#token_structs)*
        #(#token_alt_nodes)*
    }
}

use proc_macro2::TokenStream;
use quote::quote;

use crate::{engram::Engram, grammar_item::GrammarItem};

pub fn generate_token_kind(grammar: &Engram) -> TokenStream {
    let variants = grammar.tokens().map(|token| {
        let item_name = token.variant_info().ident;
        let item_doc = token.item_info().doc;

        quote! {
            #[doc = #item_doc]
            #item_name,
        }
    });

    quote! {
        /// The kind of a token.
        pub enum TokenKind {
            #(#variants)*
            /// A special token representing an unknown token.
            Unknown,
            /// A special token representing the end of an input.
            EndOfFile,
        }
    }
}

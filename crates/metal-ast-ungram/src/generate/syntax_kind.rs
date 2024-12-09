use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::{Engram, NodeExt, TokenExt};

pub fn generate_syntax_kind(grammar: &Engram) -> TokenStream {
    let node_variants = grammar.nodes().map(|node| node.as_syntax_kind_name());
    let token_variants = grammar.tokens().map(|token| token.as_syntax_kind_name());

    let variants = node_variants.chain(token_variants);

    quote! {
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum SyntaxKind {
            #(#variants),*,
            __LAST,
        }
    }
}

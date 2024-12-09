use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::Rule;

use crate::{
    engram::Engram,
    generate::nodes::{enum_::generate_enum_item, struct_::generate_struct_item},
};

mod enum_;
mod rule;
mod struct_;

pub fn generate_node_items(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        if matches!(node.rule, Rule::Alt(_)) {
            generate_enum_item(grammar, node)
        } else {
            generate_struct_item(grammar, node)
        }
    });

    quote! {
        use crate::{
            AstNode,
            SyntaxKind,
            SyntaxNode,
            SyntaxNodeExt,
            SyntaxToken,
        };

        #(#token_items)*
    }
}

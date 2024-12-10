use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::Rule;

use crate::{
    engram::Engram,
    generate::nodes::{enum_::generate_enum_item, node_struct::generate_node_struct},
};

mod enum_;
mod node_struct;

/// Generates the `nodes.rs` file.
pub fn generate_nodes_file(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        if matches!(node.rule, Rule::Alt(_)) {
            generate_enum_item(grammar, node)
        } else {
            generate_node_struct(grammar, node)
        }
    });

    quote! {
        use rowan::NodeOrToken;

        use crate::{
            AstNode,
            AstToken,
            SyntaxKind,
            SyntaxNode,
            SyntaxNodeExt,
            SyntaxToken,
            tokens::*,
        };

        #(#token_items)*
    }
}

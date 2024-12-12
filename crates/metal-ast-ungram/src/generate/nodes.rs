use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::Rule;

use crate::{
    engram::Engram,
    generate::nodes::{node_enums::generate_node_enums, node_struct::generate_node_struct},
};

mod node_enums;
mod node_struct;

/// Generates the `nodes.rs` file.
pub fn generate_nodes_file(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        if matches!(node.rule, Rule::Alt(_)) {
            generate_node_enums(grammar, node)
        } else {
            generate_node_struct(grammar, node)
        }
    });

    quote! {
        use either::Either;
        use rowan::NodeOrToken;

        use crate::{
            AstNode,
            AstToken,
            SyntaxKind,
            SyntaxNode,
            SyntaxToken,
            tokens::*,
            utils::SyntaxNodeExt,
        };

        #(#token_items)*
    }
}

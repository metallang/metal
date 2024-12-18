// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::Engram,
    generate::{rules::generate_top_rule, utils::generate_grammar_item_struct},
    grammar_item::{GrammarItem, NodeDataExt},
};

/// Generates a node struct.
pub fn generate_node_struct(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item = generate_grammar_item_struct(node);
    let item_impl = generate_node_struct_impl(grammar, node);

    quote! {
        #item

        #item_impl
    }
}

/// Generates an `impl` block consisting of accessor methods for a node struct.
fn generate_node_struct_impl(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.item_info().ident;

    // a special case for token alt nodes
    // note: cannot be nicely moved to generate_rule because this needs a NodeData,
    // and since we're calling generate_rule *from inside generate_rule* (i.e., recursion),
    // that means we'll have to pass the NodeData through all the functions. besides,
    // Rule::Alts are only supported at the top-level anyway
    if matches!(node.rule, Rule::Alt(_)) {
        let token_enum_name = node.token_enum_info().ident;

        return quote! {
            impl #item_name {
                /// Returns the inner token associated with this node.
                pub fn token(&self) -> Option<#token_enum_name> {
                    self.syntax
                        .children_with_tokens()
                        .find_map(|node_or_token| match node_or_token {
                            NodeOrToken::Node(_) => None,
                            NodeOrToken::Token(token) => #token_enum_name::cast(token),
                        })
                }
            }
        };
    }

    let impl_ = generate_top_rule(grammar, &node.rule);

    quote! {
        impl #item_name {
            #impl_
        }
    }
}

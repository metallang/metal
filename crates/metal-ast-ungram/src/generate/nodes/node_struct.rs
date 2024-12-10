use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::{Engram, GrammarItem, NodeDataExt},
    generate::rules::generate_rule,
};

/// Generates a node struct.
pub fn generate_node_struct(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.item_name();
    let syntax_kind_name = node.syntax_kind_name();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let item_impl = generate_node_struct_impl(grammar, node, &node.rule);

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #item_name {
            syntax: SyntaxNode,
        }

        impl AstNode for #item_name {
            fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_kind_name }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &SyntaxNode { &self.syntax }
        }

        #item_impl
    }
}

/// Generates an `impl` block consisting of accessor methods for a node struct.
fn generate_node_struct_impl(grammar: &Engram, node: &NodeData, rule: &Rule) -> TokenStream {
    let item_name = node.item_name();

    if matches!(rule, Rule::Alt(_)) {
        let token_enum_name = node.as_token_enum_name();

        return quote! {
            impl #item_name {
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

    let impl_ = generate_rule(grammar, rule, None);

    quote! {
        impl #item_name {
            #impl_
        }
    }
}

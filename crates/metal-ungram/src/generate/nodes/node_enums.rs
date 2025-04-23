// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::Engram,
    generate::nodes::node_struct::generate_node_struct,
    grammar_item::{GrammarItem, GrammarItemInfo, RuleExt},
};

/// Finds and generates enums of nodes for nodes that are an alteration of nodes (the terminology is unfortunate),
/// such as `Expr`.
pub fn generate_node_enums(grammar: &Engram, node: &NodeData) -> TokenStream {
    let Rule::Alt(alt_rules) = &node.rule else {
        unreachable!()
    };

    if node.rule.is_alt_of_nodes() {
        generate_node_enum(grammar, alt_rules.as_slice(), node)
    } else if alt_rules.iter().all(|rule| matches!(rule, Rule::Token(_))) {
        generate_node_struct(grammar, node)
    } else {
        let item_name = node.item_info().ident;

        panic!("enum node {item_name} must be either all-nodes or all-tokens")
    }
}

/// Generates a single node alteration node, such as `Expr`.
fn generate_node_enum(grammar: &Engram, rules: &[Rule], node: &NodeData) -> TokenStream {
    let GrammarItemInfo {
        ident: item_name,
        doc: item_doc,
    } = node.item_info();

    let mut enum_variants = TokenStream::new();
    let mut can_cast_arms = TokenStream::new();
    let mut cast_arms = TokenStream::new();
    let mut syntax_arms = TokenStream::new();

    for rule in rules {
        match rule {
            Rule::Node(node) => {
                let node = &grammar[node];
                let GrammarItemInfo {
                    ident: variant_name,
                    doc: variant_doc,
                } = node.variant_info();
                let data_name = node.item_info().ident;

                let enum_variant = quote! {
                    #[doc = #variant_doc]
                    #variant_name(#data_name),
                };
                let can_cast_arm = quote! {
                    #data_name::can_cast(kind) ||
                };
                let cast_arm = quote! {
                    if #data_name::can_cast(syntax.kind()) {
                        return Some(#item_name::#variant_name(#data_name::cast(syntax)?));
                    }
                };
                let syntax_arm = quote! {
                    #item_name::#variant_name(it) => it.syntax(),
                };

                enum_variants.extend(Some(enum_variant));
                can_cast_arms.extend(Some(can_cast_arm));
                cast_arms.extend(Some(cast_arm));
                syntax_arms.extend(Some(syntax_arm));
            }
            _ => unreachable!(),
        }
    }

    /*

    fn can_cast(kind: SyntaxKind) -> bool {
        ItemNode::can_cast(kind) || ExprNode::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if ItemNode::can_cast(syntax.kind()) {
            return Some(StmtKindNode::Item(ItemNode::cast(syntax)?));
        }

        if ExprNode::can_cast(syntax.kind()) {
            return Some(StmtKindNode::Expr(ExprNode::cast(syntax)?));
        }

        None
    }

    */

    quote! {
        #[doc = #item_doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #item_name {
            #enum_variants
        }

        impl AstNode for #item_name {
            fn can_cast(kind: SyntaxKind) -> bool {
                #can_cast_arms false
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                #cast_arms

                None
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    #syntax_arms
                }
            }
        }
    }
}

use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::Engram,
    generate::nodes::node_struct::generate_node_struct,
    grammar_item::{GrammarItem, GrammarItemInfo},
};

/// Finds and generates enums of nodes for nodes that are an alteration of nodes (the terminology is unfortunate),
/// such as `Expr`.
pub fn generate_node_enums(grammar: &Engram, node: &NodeData) -> TokenStream {
    let Rule::Alt(alt_rules) = &node.rule else {
        unreachable!()
    };

    if alt_rules.iter().all(|rule| matches!(rule, Rule::Node(_))) {
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
                let syntax_kind_name = node.syntax_kind_info().ident;

                let enum_variant = quote! {
                    #[doc = #variant_doc]
                    #variant_name(#data_name),
                };
                let can_cast_arm = quote! {
                    SyntaxKind::#syntax_kind_name => true,
                };
                let cast_arm = quote! {
                    SyntaxKind::#syntax_kind_name => Some(#item_name::#variant_name(#data_name::cast(syntax)?)),
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

    quote! {
        #[doc = #item_doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #item_name {
            #enum_variants
        }

        impl AstNode for #item_name {
            #[allow(clippy::match_like_matches_macro)]
            #[allow(clippy::wildcard_enum_match_arm)]
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    #can_cast_arms
                    _ => false,
                }
            }

            #[allow(clippy::wildcard_enum_match_arm)]
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    #cast_arms
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    #syntax_arms
                }
            }
        }
    }
}

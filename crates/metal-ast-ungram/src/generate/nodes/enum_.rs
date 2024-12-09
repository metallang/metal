use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::{Engram, NodeExt},
    generate::nodes::struct_::generate_struct_item,
};

#[allow(clippy::wildcard_enum_match_arm)]
pub fn generate_enum_item(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.as_item_name();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let Rule::Alt(alt_rules) = &node.rule else {
        unreachable!()
    };

    if alt_rules.iter().all(|rule| matches!(rule, Rule::Node(_))) {
        generate_node_enum_item(grammar, alt_rules.as_slice(), &item_name, &doc)
    } else if alt_rules.iter().all(|rule| matches!(rule, Rule::Token(_))) {
        generate_struct_item(grammar, node)
    } else {
        panic!("enum node {item_name} must be either all-nodes or all-tokens")
    }
}

#[allow(clippy::wildcard_enum_match_arm)]
fn generate_node_enum_item(
    grammar: &Engram,
    rules: &[Rule],
    item_name: &syn::Ident,
    doc: &str,
) -> TokenStream {
    let mut enum_variants = TokenStream::new();
    let mut can_cast_arms = TokenStream::new();
    let mut cast_arms = TokenStream::new();
    let mut syntax_arms = TokenStream::new();

    for rule in rules {
        match rule {
            Rule::Node(node) => {
                let node = &grammar[node];
                let variant_name = node.as_variant_name();
                let data_name = node.as_item_name();
                let syntax_kind_name = node.as_syntax_kind_name();

                let enum_variant_doc = format!(" See [{}].", &data_name);

                let enum_variant = quote! {
                    #[doc = #enum_variant_doc]
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
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[doc = #doc]
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

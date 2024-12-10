use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::engram::{Engram, NodeExt, TokenExt};

/// Finds and generates enums of tokens for nodes that are an alteration of tokens, such as `BinaryOp`.
pub fn generate_token_alt_nodes(grammar: &Engram) -> impl Iterator<Item = TokenStream> + use<'_> {
    grammar.nodes().filter_map(|token_node| {
        // Check that the node is an alteration
        let Rule::Alt(rules) = &token_node.rule else {
            return None;
        };

        // Check that the alteration consists purely of tokens
        if !rules.iter().all(|rule| matches!(rule, Rule::Token(_))) {
            return None;
        }

        Some(generate_token_alt_node(
            grammar,
            token_node,
            rules.as_slice(),
        ))
    })
}

/// Generate a single token alteration node, such as `BinaryOp`.
fn generate_token_alt_node(grammar: &Engram, token_node: &NodeData, rules: &[Rule]) -> TokenStream {
    let item_name = token_node.as_token_enum_name();

    let doc = format!(" Represents the `{}` token.", &token_node.name);

    let mut enum_variants = TokenStream::new();
    let mut can_cast_arms = TokenStream::new();
    let mut cast_arms = TokenStream::new();
    let mut syntax_arms = TokenStream::new();

    for rule in rules {
        match rule {
            Rule::Token(token) => {
                let token = &grammar[token];
                let variant_name = token.as_variant_name();
                let data_name = token.as_item_name();
                let syntax_kind_name = token.as_syntax_kind_name();

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
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum #item_name {
            #enum_variants
        }

        impl AstToken for #item_name {
            #[allow(clippy::match_like_matches_macro)]
            #[allow(clippy::wildcard_enum_match_arm)]
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    #can_cast_arms
                    _ => false,
                }
            }

            #[allow(clippy::wildcard_enum_match_arm)]
            fn cast(syntax: SyntaxToken) -> Option<Self> {
                match syntax.kind() {
                    #cast_arms
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxToken {
                match self {
                    #syntax_arms
                }
            }
        }
    }
}

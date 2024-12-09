use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::{Engram, NodeExt},
    generate::nodes::rule::generate_rule,
};

pub fn generate_struct_item(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.as_item_name();
    let syntax_kind_name = node.as_syntax_kind_name();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let item_impl = generate_struct_item_impl(grammar, node, &node.rule);

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

fn generate_struct_item_impl(grammar: &Engram, node: &NodeData, rule: &Rule) -> TokenStream {
    let item_name = node.as_item_name();

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

use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule};

use crate::{
    engram::{Engram, NodeExt},
    generate::nodes::rule::generate_rule,
};

pub fn generate_struct_item(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.as_item_ident();
    let syntax_kind_name = node.as_syntax_kind_ident();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let item_impl = generate_struct_item_impl(grammar, &item_name, &node.rule);

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[doc = #doc]
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

fn generate_struct_item_impl(grammar: &Engram, item_name: &syn::Ident, rule: &Rule) -> TokenStream {
    if matches!(rule, Rule::Alt(_)) {
        return TokenStream::new(); // TODO
    }

    let impl_ = generate_rule(grammar, rule, None);

    quote! {
        impl #item_name {
            #impl_
        }
    }
}

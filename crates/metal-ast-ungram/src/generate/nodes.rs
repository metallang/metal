use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::{Engram, NodeExt};

pub fn generate_node_items(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        let item_name = node.as_item_ident();
        let syntax_kind_name = node.as_syntax_kind_ident();

        let doc = format!(" Represents the `{}` node.", &node.name);

        quote! {
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
        }
    });

    quote! {
        use crate::{
            AstNode,
            SyntaxKind,
            SyntaxNode,
            // SyntaxNodeExt,
            // SyntaxToken,
        };

        #(#token_items)*
    }
}

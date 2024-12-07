use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::{Engram, TokenExt};

pub fn generate_token_items(grammar: &Engram) -> TokenStream {
    let items = grammar.tokens().map(|token| {
        let item_name = token.as_item_ident();
        let syntax_kind_name = token.as_syntax_kind_ident();

        let doc = format!(" Represents the `{}` token.", &token.name);

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            #[doc = #doc]
            pub struct #item_name {
                syntax: SyntaxToken,
            }

            impl AstToken for #item_name {
                fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_kind_name }

                fn cast(syntax: SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) {
                        Some(Self { syntax })
                    } else {
                        None
                    }
                }

                fn syntax(&self) -> &SyntaxToken { &self.syntax }
            }
        }
    });

    quote! {
        use crate::{
            AstToken,
            SyntaxKind,
            SyntaxToken,
        };

        #(#items)*
    }
}

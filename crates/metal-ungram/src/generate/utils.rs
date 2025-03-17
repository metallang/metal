// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar_item::{GrammarItem, GrammarItemInfo};

/// Generates a struct representation of a grammar item.
pub fn generate_grammar_item_struct(item: &impl GrammarItem) -> TokenStream {
    let GrammarItemInfo {
        ident: item_name,
        doc: item_doc,
    } = item.item_info();
    let syntax_kind_name = item.syntax_kind_info().ident;
    let ast_trait_name = item.ast_trait_name();
    let syntax_type_name = item.syntax_type_name();

    quote! {
        #[doc = #item_doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #item_name {
            syntax: #syntax_type_name,
        }

        impl #ast_trait_name for #item_name {
            fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_kind_name }

            fn cast(syntax: #syntax_type_name) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            fn syntax(&self) -> &#syntax_type_name { &self.syntax }
        }
    }
}

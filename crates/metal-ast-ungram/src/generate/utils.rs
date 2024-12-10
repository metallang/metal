use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::GrammarItem;

pub fn generate_item_struct(item: impl GrammarItem) -> TokenStream {
    let doc = item.struct_doc();
    let item_name = item.item_name();
    let syntax_kind_name = item.syntax_kind_name();
    let ast_trait_name = item.ast_trait_name();
    let syntax_type_name = item.syntax_type_name();

    quote! {
        #[doc = #doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct #item_name {
            syntax: SyntaxToken,
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

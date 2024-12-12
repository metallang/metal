use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::Engram;
use crate::grammar_item::{GrammarItem, GrammarItemInfo};

/// Generates the `syntax_kind.rs` file.
pub fn generate_syntax_kind_file(grammar: &Engram) -> TokenStream {
    let syntax_kind = generate_syntax_kind(grammar);
    let t = generate_t_macro(grammar);

    quote! {
        #syntax_kind
        #t
    }
}

/// Generates the `SyntaxKind` enum.
fn generate_syntax_kind(grammar: &Engram) -> TokenStream {
    let node_variants = grammar.nodes().map(|node| {
        let GrammarItemInfo {
            ident: syntax_kind_name,
            doc: syntax_kind_doc,
        } = node.syntax_kind_info();

        quote! {
            #[doc = #syntax_kind_doc]
            #syntax_kind_name,
        }
    });

    let token_variants = grammar.tokens().map(|token| {
        let GrammarItemInfo {
            ident: syntax_kind_name,
            doc: syntax_kind_doc,
        } = token.syntax_kind_info();

        quote! {
            #[doc = #syntax_kind_doc]
            #syntax_kind_name,
        }
    });

    let variants = node_variants.chain(token_variants);

    quote! {
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum SyntaxKind {
            #(#variants)*
            /// A special syntax kind used for transmute safety checks. You shouldn't worry
            /// about (and even less rely on) this.
            __LAST,
        }
    }
}

/// Generates the `T!` macro.
fn generate_t_macro(grammar: &Engram) -> TokenStream {
    let arms = grammar.tokens().map(|token| {
        let token_name = token.name.as_str();
        let syntax_kind_name = token.syntax_kind_info().ident;

        quote! {
            [#token_name] => { $crate::SyntaxKind::#syntax_kind_name },
        }
    });

    quote! {
        /// Returns the [SyntaxKind] variants corresponding to the provided token
        /// as written in the grammar.
        ///
        /// # Example
        ///
        /// ```
        /// if binary_op_node.token() == T![+] {
        ///     // The binary operator is plus!
        /// }
        /// ```
        pub macro T {
            #(#arms)*
        }
    }
}

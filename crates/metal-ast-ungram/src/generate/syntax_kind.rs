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
        let token_name = token.name.as_str().parse().unwrap_or_else(|_| {
            // rust requires braces to be always paired regardless of where they appear,
            // so we can't make e.g. `T![}]` work. instead, we'll wrap such tokens in quotes
            let token_name = token.name.as_str();

            assert_eq!(token_name.len(), 1);

            let token_name = token_name.chars().next().unwrap();

            quote! { #token_name }
        });

        let syntax_kind_name = token.syntax_kind_info().ident;

        quote! {
            [#token_name] => { $crate::SyntaxKind::#syntax_kind_name },
        }
    });

    quote! {
        /// Returns the [SyntaxKind] variants corresponding to the provided token
        /// as written in the grammar.
        ///
        /// Note that certain tokens such as parentheses, braces, and brackets need
        /// to be wrapped in single quotes (like you would spell a character), e.g.
        /// `T!['{']`. This is a limitation imposed by Rust.
        ///
        /// # Example
        ///
        /// ```no_run,
        /// # use metal_ast_ng::{T, BinaryOpNode};
        /// # fn example(binary_op_node: BinaryOpNode) {
        /// if binary_op_node.token() == T![+] {
        ///     // The binary operator is plus!
        /// }
        /// # }
        /// ```
        pub macro T {
            #(#arms)*
        }
    }
}

// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;

use crate::engram::Engram;
use crate::grammar_item::{GrammarItem, GrammarItemInfo, PAREN_TOKENS};

/// Generates the `syntax_kind.rs` file.
pub fn generate_syntax_kind_file(grammar: &Engram) -> TokenStream {
    let syntax_kind = generate_syntax_kind(grammar);
    let t = generate_t_macro(grammar);
    let n = generate_n_macro(grammar);

    quote! {
        #syntax_kind
        #t
        #n
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
            /// Represents a multi- or single-line comment.
            COMMENT_TOKEN,
            /// Represents a whitespace token, such as a space or a tab, among others.
            WHITESPACE_TOKEN,
            /// A special token representing an unknown token.
            UNKNOWN_TOKEN,
            /// A special syntax kind used for transmute safety checks. You shouldn't worry
            /// about (and even less rely on) this.
            __LAST,
        }

        impl From<rowan::SyntaxKind> for SyntaxKind {
            fn from(value: rowan::SyntaxKind) -> Self {
                let d = value.0 as u8;

                assert!(d <= (SyntaxKind::__LAST as u8));

                unsafe { std::mem::transmute::<u8, SyntaxKind>(d) }
            }
        }

        impl From<SyntaxKind> for rowan::SyntaxKind {
            fn from(val: SyntaxKind) -> Self {
                rowan::SyntaxKind(val as u16)
            }
        }

        impl SyntaxKind {
            pub fn is_whitespace(&self) -> bool {
                matches!(self, T![@comment] | T![@whitespace] | T![@unknown])
            }

            pub fn is_item_start(&self) -> bool {
                matches!(
                    self,
                    T![abstract]
                    | T![const]
                    | T![enum]
                    | T![def]
                    | T![import]
                    | T![struct]
                    | T![type]
                    | T![pub] // non-items don't have visibility
                )
            }

            pub fn is_prefix_op(&self) -> bool {
                matches!(
                    self,
                    T![+]
                    | T![-]
                    | T![!]
                    | T![~]
                    | T![*]
                )
            }

            pub fn is_expr_start(&self) -> bool {
                matches!(
                    self,
                    T![@ident]
                    T!['{']
                    | T![@number]
                    | T![@string]
                    | T!['(']
                    | T![return]
                )
                || self.is_prefix_op()
            }
        }
    }
}

/// Generates the `T!` macro.
fn generate_t_macro(grammar: &Engram) -> TokenStream {
    let arms = grammar.tokens().map(|token| {
        let token_name = token.name.as_str();
        let token_name = if PAREN_TOKENS.contains(&token_name) {
            // rust requires braces to be always paired regardless of where they appear,
            // so we can't make e.g. `T![}]` work. instead, we'll wrap such tokens in quotes

            let token_name = token_name.chars().next().unwrap();

            quote! { #token_name }
        } else {
            token_name.parse().unwrap()
        };

        let syntax_kind_name = token.syntax_kind_info().ident;

        quote! {
            [#token_name] => { $crate::SyntaxKind::#syntax_kind_name },
        }
    });

    quote! {
        /// Returns the [SyntaxKind] variant corresponding to the provided token
        /// as written in the grammar.
        ///
        /// Note that certain tokens such as parentheses, braces, and brackets need
        /// to be wrapped in single quotes (like you would spell a character), e.g.
        /// `T!['{']`. This is a limitation imposed by Rust.
        ///
        /// # Example
        ///
        /// ```no_run,
        /// # use metal_ast_ng::{T, AstToken, BinaryExprOpNode};
        /// # fn example(binary_op_node: BinaryExprOpNode) {
        /// if binary_op_node.token().is_some_and(|token| token.syntax().kind() == T![+]) {
        ///     // The binary operator is plus!
        /// }
        /// # }
        /// ```
        pub macro T {
            #(#arms)*
            [@comment] => { $crate::SyntaxKind::COMMENT_TOKEN },
            [@whitespace] => { $crate::SyntaxKind::WHITESPACE_TOKEN },
            [@unknown] => { $crate::SyntaxKind::UNKNOWN_TOKEN },
        }
    }
}

/// Generates the `N!` macro.
fn generate_n_macro(grammar: &Engram) -> TokenStream {
    let arms = grammar.nodes().map(|node| {
        let node_name: TokenStream = node.name.as_str().parse().unwrap();

        let syntax_kind_name = node.syntax_kind_info().ident;

        quote! {
            [#node_name] => { $crate::SyntaxKind::#syntax_kind_name },
        }
    });

    quote! {
        /// Returns the [SyntaxKind] variant corresponding to the provided node
        /// as written in the grammar.
        ///
        /// # Example
        ///
        /// ```no_run,
        /// # use metal_ast_ng::{N, AstNode, BinaryExprOpNode};
        /// # fn example(binary_op_node: BinaryExprOpNode) {
        /// assert!(binary_op_node.syntax().kind() == N![BinaryExprOp]);
        /// # }
        /// ```
        pub macro N {
            #(#arms)*
        }
    }
}

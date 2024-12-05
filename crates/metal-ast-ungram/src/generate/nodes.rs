use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule, TokenData};

use crate::engram::{Engram, NodeExt, TokenExt};

pub fn generate_node_items(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        let item_name = node.as_item_ident();
        let syntax_kind_name = node.as_syntax_kind_ident();

        let doc = format!(" Represents the `{}` node.", &node.name);

        let impl_ = generate_rule(grammar, &node.rule, None);

        quote! {
            #[doc = #doc]
            pub struct #item_name {
                syntax: SyntaxNode,
            }

            impl #item_name {
                #impl_
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
            SyntaxNodeExt,
            SyntaxToken,
        };

        #(#token_items)*
    }
}

fn generate_rule(grammar: &Engram, rule: &Rule, label: Option<&str>) -> TokenStream {
    match rule {
        Rule::Labeled { label, rule } => {
            generate_rule(grammar, rule.as_ref(), Some(label.as_str()))
        }
        Rule::Node(node) => generate_node_rule(&grammar[node], label),
        Rule::Token(token) => generate_token_rule(&grammar[token], label),
        Rule::Seq(rules) => generate_seq_rule(grammar, rules.as_slice()),
        Rule::Alt(_vec) => TokenStream::new(),
        Rule::Opt(_rule) => TokenStream::new(),
        Rule::Rep(_rule) => TokenStream::new(),
    }
}

fn generate_node_rule(node: &NodeData, label: Option<&str>) -> TokenStream {
    let fn_name = node.as_fn_ident(label);
    let item_name = node.as_item_ident();

    let doc = format!(" Find a child node of type [{}].", item_name);

    quote! {
        #[doc = #doc]
        pub fn #fn_name(&self) -> Option<#item_name> {
            self.syntax.child()
        }
    }
}

fn generate_token_rule(token: &TokenData, label: Option<&str>) -> TokenStream {
    let fn_name = token.as_fn_ident(label);
    let syntax_kind_name = token.as_syntax_kind_ident();

    let doc = format!(
        " Find a child token of variant [SyntaxKind::{}].",
        syntax_kind_name
    );

    quote! {
        #[doc = #doc]
        pub fn #fn_name(&self) -> Option<SyntaxToken> {
            self.syntax.find_child_token(SyntaxKind::#syntax_kind_name)
        }
    }
}

fn generate_seq_rule(grammar: &Engram, rules: &[Rule]) -> TokenStream {
    rules
        .iter()
        .map(|rule| generate_rule(grammar, rule, None))
        .collect()
}

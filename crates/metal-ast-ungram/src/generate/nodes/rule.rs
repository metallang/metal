use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule, TokenData};

use crate::engram::{Engram, NodeExt, TokenExt};

pub fn generate_rule(grammar: &Engram, rule: &Rule, label: Option<&str>) -> TokenStream {
    match rule {
        Rule::Labeled { label, rule } => {
            generate_rule(grammar, rule.as_ref(), Some(label.as_str()))
        }
        Rule::Node(node) => generate_node_rule(&grammar[node], label),
        Rule::Token(token) => generate_token_rule(&grammar[token], label),
        Rule::Seq(rules) => generate_seq_rule(grammar, rules.as_slice()),
        Rule::Alt(_) => unreachable!(),
        Rule::Opt(_rule) => TokenStream::new(),
        Rule::Rep(_rule) => TokenStream::new(),
    }
}

fn generate_node_rule(node: &NodeData, label: Option<&str>) -> TokenStream {
    let fn_name = node.as_fn_name(label);
    let item_name = node.as_item_name();

    let doc = format!(" Find a child node of type [{}].", item_name);

    quote! {
        #[doc = #doc]
        pub fn #fn_name(&self) -> Option<#item_name> {
            self.syntax.child()
        }
    }
}

fn generate_token_rule(token: &TokenData, label: Option<&str>) -> TokenStream {
    let fn_name = token.as_fn_name(label);
    let syntax_kind_name = token.as_syntax_kind_name();

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

use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule, TokenData};

use crate::{
    engram::Engram,
    grammar_item::{GrammarItem, GrammarItemInfo},
};

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
    let GrammarItemInfo {
        ident: fn_name,
        doc: fn_doc,
    } = node.fn_info(label);

    let item_name = node.item_info().ident;

    quote! {
        #[doc = #fn_doc]
        pub fn #fn_name(&self) -> Option<#item_name> {
            self.syntax.child()
        }
    }
}

fn generate_token_rule(token: &TokenData, label: Option<&str>) -> TokenStream {
    let GrammarItemInfo {
        ident: fn_name,
        doc: fn_doc,
    } = token.fn_info(label);

    let syntax_kind_name = token.syntax_kind_info().ident;

    quote! {
        #[doc = #fn_doc]
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

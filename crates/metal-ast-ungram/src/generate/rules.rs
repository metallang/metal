use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule, TokenData};

use crate::{
    debug::debug_rule,
    engram::Engram,
    grammar_item::{GrammarItem, GrammarItemInfo},
    utils::call_site_ident,
};

/// Generates the body of an impl block of a struct corresponding to a node.
pub fn generate_top_rule(grammar: &Engram, rule: &Rule) -> TokenStream {
    match rule {
        Rule::Labeled { label, rule } => match rule.as_ref() {
            Rule::Seq(rules) => generate_seq_rule(grammar, rules.as_slice(), Some(label.as_str())),
            other => generate_rule(grammar, other, Some(label.as_str())),
        },
        Rule::Seq(rules) => generate_seq_rule(grammar, rules.as_slice(), None),
        other => generate_rule(grammar, other, None),
    }
}

/// Same as [generate_top_rule], but rejects top-level-only rules.
fn generate_rule(grammar: &Engram, rule: &Rule, label: Option<&str>) -> TokenStream {
    match rule {
        Rule::Node(node) => generate_node_rule(&grammar[node], label),
        Rule::Token(token) => generate_token_rule(&grammar[token], label),
        Rule::Labeled {
            label: new_label,
            rule: new_rule,
        } => {
            // provide a more helpful error message than silently ignoring the outer label
            if matches!(new_rule.as_ref(), Rule::Labeled { .. }) {
                panic!("nested labels detected when evaluating rule {:?}. nested labels have no effect and are not supported", debug_rule(grammar, rule));
            }

            generate_rule(grammar, new_rule.as_ref(), Some(new_label.as_str()))
        }
        Rule::Opt(new_rule) => {
            // count this as an error, just for correctness
            if matches!(new_rule.as_ref(), Rule::Opt(_)) {
                panic!("nested optionals detected when evaluating rule {:?}. nested optionals have no effect and are not supported", debug_rule(grammar, rule))
            }

            generate_rule(grammar, new_rule.as_ref(), label)
        }
        other => {
            let suggestion = if matches!(other, Rule::Rep(_)) {
                "if you wanted to represent a delimited sequence of nodes, extract it into a separate node"
            } else if matches!(other, Rule::Seq(_)) {
                "nested rule sequences are not supported"
            } else {
                ""
            };

            panic!(
                "{:?} in unsupported position. {suggestion}",
                debug_rule(grammar, other)
            )
        }
    }
}

/// Generates an accessor method corresponding to a node rule.
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

/// Generates an accessor method corresponding to a token rule.
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

/// Generates an accessor method corresponding to a sequence rule.
fn generate_seq_rule(grammar: &Engram, rules: &[Rule], label: Option<&str>) -> TokenStream {
    let otherwise = || generate_simple_seq_rule(grammar, rules);

    // this monstrosity exactly matches a "repetition-sequence" rule, see
    // [generate_rep_seq_rule]'s docs. this would've been much easier if deref
    // patterns were a thing (they technically exist, but are insufficient)
    match rules {
        [Rule::Node(lnode), Rule::Rep(rep_rule), Rule::Opt(opt_rule)] => {
            match (rep_rule.as_ref(), opt_rule.as_ref()) {
                (Rule::Seq(seq_rules), Rule::Token(rtoken)) => match seq_rules.as_slice() {
                    [Rule::Token(mtoken), Rule::Node(mnode)] => {
                        if lnode == mnode && mtoken == rtoken {
                            let node = &grammar[mnode];
                            let token = &grammar[mtoken];

                            generate_rep_seq_rule(node, token, label)
                        } else {
                            otherwise()
                        }
                    }
                    _ => otherwise(),
                },
                _ => otherwise(),
            }
        }
        _ => otherwise(),
    }
}

/// Generates an accessor method corresponding to a "simple" sequence rule, i.e.
/// a sequence of nodes and tokens.
fn generate_simple_seq_rule(grammar: &Engram, rules: &[Rule]) -> TokenStream {
    rules
        .iter()
        .map(|rule| generate_rule(grammar, rule, None))
        .collect()
}

/// Generates an accessor method corresponding to a "repetition-sequence" rule, i.e.
/// a sequence node that:
/// - is 3 rules long
/// - it's first rule is a node (#N1)
/// - it's second rule is a repetition(sequence), where
///   - the sequence is 2 rules long
///   - it's first rule is a token (#T1)
///   - it's second rule is a node (#N2)
/// - it's third rule is an optional token (#T2)
///
/// where T1 == T2 and N1 == N2.
///
/// What this means on practice is a rule that represents one or more instances of
/// a node (#N1/#N2) delimited by a token (#T1/#T2), with an optional trailing token.
fn generate_rep_seq_rule(node: &NodeData, token: &TokenData, label: Option<&str>) -> TokenStream {
    let base_fn_name = label.unwrap_or("children");
    let normal_fn_name = call_site_ident(base_fn_name);
    let delimited_fn_name = call_site_ident(format!("{base_fn_name}_with_delimiters"));
    let node_name = node.item_info().ident;
    let token_name = token.item_info().ident;

    quote! {
        /// Returns an iterator over the children nodes of this node.
        pub fn #normal_fn_name(&self) -> impl Iterator<Item = #node_name> {
            self.#delimited_fn_name()
                .filter_map(|either| match either {
                    Either::Left(node) => Some(node),
                    Either::Right(_) => None,
                })
        }

        /// Returns an iterator over the children nodes and token of this node.
        pub fn #delimited_fn_name(
            &self,
        ) -> impl Iterator<Item = Either<#node_name, #token_name>> {
            self.syntax
                .children_with_tokens()
                .filter_map(|node_or_token| match node_or_token {
                    NodeOrToken::Node(node) => #node_name::cast(node).map(Either::Left),
                    NodeOrToken::Token(token) => #token_name::cast(token).map(Either::Right),
                })
        }
    }
}

use std::{borrow::Borrow, collections::HashSet, error::Error, fmt::Debug, str::FromStr};

use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{Grammar, NodeData, Rule, TokenData};

const GRAMMAR: &str = include_str!("./metal.ungram");

mod immutable;
mod names;

type LocalResult<T = ()> = Result<T, Box<dyn Error>>;

fn main() -> LocalResult {
    rerun_if();

    let grammar = Grammar::from_str(GRAMMAR)?;

    save_generated(generate_syntax_kind(&grammar), "./src/syntax_kind.rs")?;
    save_generated(generate_token_items(&grammar), "./src/tokens.rs")?;
    save_generated(generate_node_items(&grammar), "./src/nodes.rs")?;

    Ok(())
}

fn generate_syntax_kind(grammar: &Grammar) -> TokenStream {
    let node_variants = grammar
        .iter()
        .map(|node| &grammar[node])
        .map(|data| RawGrammarName(data.name.as_str()))
        .map(node_visual_name)
        .map(syntax_kind_name)
        .map(into_ident);

    let token_variants = grammar
        .tokens()
        .map(|token| &grammar[token])
        .map(|data| RawGrammarName(data.name.as_str()))
        .map(token_visual_name)
        .map(syntax_kind_name)
        .map(into_ident);

    let variants = node_variants.chain(token_variants);

    quote! {
        #[allow(non_camel_case_types)]
        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum SyntaxKind {
            #(#variants),*,
            __LAST,
        }
    }
}

fn generate_token_items(grammar: &Grammar) -> TokenStream {
    let token_items = grammar.tokens().map(|token| &grammar[token]).map(|data| {
        let raw_token_name = RawGrammarName(&data.name);
        let token_visual_name = token_visual_name(raw_token_name);
        let identable_token_name = ast_item_name(token_visual_name);
        let identable_syntax_kind = syntax_kind_name(token_visual_name);

        let doc = format!(" Represents the `{}` token.", raw_token_name.0);
        let ident = into_ident(identable_token_name);
        let syntax_kind_variant = into_ident(identable_syntax_kind);

        quote! {
            #[doc = #doc]
            pub struct #ident {
                syntax: SyntaxToken,
            }

            impl AstToken for #ident {
                fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_kind_variant }
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

        #(#token_items)*
    }
}

fn generate_node_items(grammar: &Grammar) -> TokenStream {
    let token_items = grammar.iter().map(|token| &grammar[token]).map(|data| {
        let raw_node_name = RawGrammarName(&data.name);
        let node_visual_name = node_visual_name(raw_node_name);
        let identable_node_name = ast_item_name(node_visual_name);
        let identable_syntax_kind = syntax_kind_name(node_visual_name);

        let doc = format!(" Represents the `{}` node.", raw_node_name.0);
        let ident = into_ident(identable_node_name);
        let syntax_kind_variant = into_ident(identable_syntax_kind);

        let node_impl = generate_node_impl_body(grammar, &data.rule, &mut HashSet::new());

        println!("{:<15} = {:?}", &data.name, debug_rule(grammar, &data.rule));

        quote! {
            #[doc = #doc]
            pub struct #ident {
                syntax: SyntaxNode,
            }

            impl #ident {
                #node_impl
            }

            impl AstNode for #ident {
                fn can_cast(kind: SyntaxKind) -> bool { kind == SyntaxKind::#syntax_kind_variant }
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

fn generate_node_impl_body(
    grammar: &Grammar,
    rule: &Rule,
    visited: &mut HashSet<IdentableGrammarName>,
) -> TokenStream {
    generate_node_rule(grammar, rule, None)
}

fn generate_node_rule(grammar: &Grammar, rule: &Rule, label: Option<&String>) -> TokenStream {
    let tokens = match rule {
        Rule::Labeled { label, rule } => generate_node_rule(grammar, rule, Some(label)),
        Rule::Node(node) => todo!(),
        Rule::Token(token) => todo!(),
        Rule::Seq(vec) => match vec.as_slice() {
            [Rule::Node(left_node), Rule::Rep(rep), Rule::Opt(opt)] => {
                match (rep.as_ref(), opt.as_ref()) {
                    (Rule::Seq(vec), Rule::Token(right_token)) => match vec.as_slice() {
                        [Rule::Token(middle_token), Rule::Node(middle_node)]
                            if left_node == middle_node && right_token == middle_token =>
                        {
                            codegen_delimited(grammar, middle_node, middle_token)
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
            rules => rules
                .iter()
                .map(|rule| generate_node_rule(grammar, rule, None))
                .collect(),
        },
        Rule::Alt(vec) => codegen_alt(grammar, vec.as_slice()),
        Rule::Opt(rule) => codegen_opt(grammar, rule.as_ref()),
        Rule::Rep(rule) => codegen_rep(grammar, rule.as_ref()),
        _ => panic!("unrecognized rule pattern: {:?}", debug_rule(grammar, rule)),
    };

    fn is_simple_rule(rule: &Rule) -> bool {
        match &rule {
            Rule::Labeled { label: _, rule } => is_simple_rule(rule.as_ref()),
            Rule::Node(_) => true,
            Rule::Token(_) => true,
            Rule::Opt(rule) => is_simple_rule(rule.as_ref()),
            _ => false,
        }
    }

    tokens
}

fn into_ident(s: impl Borrow<IdentableGrammarName>) -> proc_macro2::Ident {
    proc_macro2::Ident::new(s.borrow().0.as_str(), proc_macro2::Span::call_site())
}

fn save_generated(tokens: TokenStream, to: &str) -> LocalResult {
    let token_string = quote! {
        //! This file is @generated by the build script, do not edit by hand.

        #tokens
    }
    .to_string();

    let parsed = syn::parse_file(token_string.as_str())?;
    let formatted = prettyplease::unparse(&parsed);

    std::fs::write(to, formatted)?;

    Ok(())
}

fn debug_rule<'a>(grammar: &'a Grammar, rule: &'a Rule) -> DebugRule<'a> {
    DebugRule { grammar, rule }
}

fn debug_multiple_rules<'a>(grammar: &'a Grammar, rules: &'a Vec<Rule>) -> DebugMultipleRules<'a> {
    DebugMultipleRules { grammar, rules }
}

fn debug_token_data(data: &TokenData) -> DebugTokenData<'_> {
    DebugTokenData(data)
}

fn debug_node_data(data: &NodeData) -> DebugNodeData<'_> {
    DebugNodeData(data)
}

struct DebugTokenData<'a>(&'a TokenData);

impl Debug for DebugTokenData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.name.fmt(f)
    }
}

struct DebugNodeData<'a>(&'a NodeData);

impl Debug for DebugNodeData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.name.fmt(f)
    }
}

struct DebugMultipleRules<'a> {
    grammar: &'a Grammar,
    rules: &'a Vec<Rule>,
}

impl Debug for DebugMultipleRules<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.rules.iter().map(|rule| debug_rule(self.grammar, rule)))
            .finish()
    }
}

struct DebugRule<'a> {
    grammar: &'a Grammar,
    rule: &'a Rule,
}

impl Debug for DebugRule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rule {
            Rule::Labeled { label, rule } => f
                .debug_struct("Rule::Labeled")
                .field("label", label)
                .field("rule", &debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Seq(rules) => f
                .debug_tuple("Rule::Seq")
                .field(&debug_multiple_rules(self.grammar, rules))
                .finish(),
            Rule::Alt(rules) => f
                .debug_tuple("Rule::Alt")
                .field(&debug_multiple_rules(self.grammar, rules))
                .finish(),
            Rule::Opt(rule) => f
                .debug_tuple("Rule::Opt")
                .field(&debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Rep(rule) => f
                .debug_tuple("Rule::Rep")
                .field(&debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Node(node) => f
                .debug_tuple("Rule::Node")
                .field(&debug_node_data(&self.grammar[*node]))
                .finish(),
            Rule::Token(token) => f
                .debug_tuple("Rule::Token")
                .field(&debug_token_data(&self.grammar[*token]))
                .finish(),
        }
    }
}

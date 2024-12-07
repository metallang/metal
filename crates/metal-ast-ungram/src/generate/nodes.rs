use proc_macro2::TokenStream;
use quote::quote;
use ungrammar::{NodeData, Rule, TokenData};

use crate::engram::{Engram, NodeExt, TokenExt};

pub fn generate_node_items(grammar: &Engram) -> TokenStream {
    let token_items = grammar.nodes().map(|node| {
        if matches!(node.rule, Rule::Alt(_)) {
            generate_enum_item(grammar, node)
        } else {
            generate_struct_item(grammar, node, true)
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

fn generate_struct_item(grammar: &Engram, node: &NodeData, generate_impl: bool) -> TokenStream {
    let item_name = node.as_item_ident();
    let syntax_kind_name = node.as_syntax_kind_ident();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let item_impl =
        generate_impl.then(|| generate_struct_item_impl(grammar, &item_name, &node.rule));

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[doc = #doc]
        pub struct #item_name {
            syntax: SyntaxNode,
        }

        #item_impl

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
}

fn generate_struct_item_impl(grammar: &Engram, item_name: &syn::Ident, rule: &Rule) -> TokenStream {
    let impl_ = generate_rule(grammar, rule, None);

    quote! {
        impl #item_name {
            #impl_
        }
    }
}

#[allow(clippy::wildcard_enum_match_arm)]
fn generate_enum_item(grammar: &Engram, node: &NodeData) -> TokenStream {
    let item_name = node.as_item_ident();

    let doc = format!(" Represents the `{}` node.", &node.name);

    let Rule::Alt(alt_rules) = &node.rule else {
        unreachable!()
    };

    if alt_rules.iter().all(|rule| matches!(rule, Rule::Node(_))) {
        generate_node_enum_item(grammar, alt_rules.as_slice(), &item_name, &doc)
    } else if alt_rules.iter().all(|rule| matches!(rule, Rule::Token(_))) {
        generate_token_enum_item(grammar, node)
    } else {
        panic!("enum node {item_name} must be either all-nodes or all-tokens")
    }
}

#[allow(clippy::wildcard_enum_match_arm)]
fn generate_node_enum_item(
    grammar: &Engram,
    rules: &[Rule],
    item_name: &syn::Ident,
    doc: &str,
) -> TokenStream {
    let mut enum_variants = TokenStream::new();
    let mut can_cast_arms = TokenStream::new();
    let mut cast_arms = TokenStream::new();
    let mut syntax_arms = TokenStream::new();

    for rule in rules {
        match rule {
            Rule::Node(node) => {
                let node = &grammar[node];
                let variant_name = node.as_item_ident(); // TODO: better naming
                let syntax_kind_name = node.as_syntax_kind_ident();

                let enum_variant_doc = format!(" See [{}].", &variant_name);

                let enum_variant = quote! {
                    #[doc = #enum_variant_doc]
                    #variant_name(#variant_name),
                };
                let can_cast_arm = quote! {
                    SyntaxKind::#syntax_kind_name => true,
                };
                let cast_arm = quote! {
                    SyntaxKind::#syntax_kind_name => Some(#item_name::#variant_name(#variant_name::cast(syntax)?)),
                };
                let syntax_arm = quote! {
                    #item_name::#variant_name(it) => it.syntax(),
                };

                enum_variants.extend(Some(enum_variant));
                can_cast_arms.extend(Some(can_cast_arm));
                cast_arms.extend(Some(cast_arm));
                syntax_arms.extend(Some(syntax_arm));
            }
            // Rule::Token(token) => {
            //     let token = &grammar[token];
            //     let variant_name = token.as_item_ident(); // TODO: better naming
            //     let syntax_kind_name = token.as_syntax_kind_ident();

            //     let enum_variant = quote! {
            //         #variant_name(#variant_name),
            //     };
            //     let can_cast_arm = quote! {
            //         SyntaxKind::#syntax_kind_name => true,
            //     };
            //     let cast_arm = quote! {
            //         SyntaxKind::#syntax_kind_name => Some(#item_name::#variant_name(#variant_name::cast(syntax)?)),
            //     };
            //     let syntax_arm = quote! {
            //         #item_name::#variant_name(it) => it.syntax(),
            //     };

            //     enum_variants.extend(Some(enum_variant));
            //     can_cast_arms.extend(Some(can_cast_arm));
            //     cast_arms.extend(Some(cast_arm));
            //     syntax_arms.extend(Some(syntax_arm));
            // }
            _ => unreachable!(),
        }
    }

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[doc = #doc]
        pub enum #item_name {
            #enum_variants
        }

        impl AstNode for #item_name {
            #[allow(clippy::match_like_matches_macro)]
            #[allow(clippy::wildcard_enum_match_arm)]
            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    #can_cast_arms
                    _ => false,
                }
            }

            #[allow(clippy::wildcard_enum_match_arm)]
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    #cast_arms
                    _ => None,
                }
            }

            fn syntax(&self) -> &SyntaxNode {
                match self {
                    #syntax_arms
                }
            }
        }
    }
}

fn generate_token_enum_item(grammar: &Engram, node: &NodeData) -> TokenStream {
    generate_struct_item(grammar, node, false)
}

fn generate_rule(grammar: &Engram, rule: &Rule, label: Option<&str>) -> TokenStream {
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

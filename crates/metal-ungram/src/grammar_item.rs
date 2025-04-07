// SPDX-License-Identifier: MIT

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use syn::Ident;
use ungrammar::{NodeData, Rule, TokenData};

use crate::utils::call_site_ident;

pub const PAREN_TOKENS: &[&str] = &["{", "}", "(", ")", "[", "]"];

/// An alias for [String] to improve type readability.
// note: we want this to be a newtype as RA "sees through" normal type aliases
pub struct DocString(String);

impl quote::ToTokens for DocString {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// A "named tuple" of `(syn::Ident, DocString)`.
pub struct GrammarItemInfo {
    /// The identifier. What the identifier represents will depend on
    /// where you got this [GrammarItemInfo] from.
    pub ident: Ident,
    /// The docstring. What the identifier represents will depend on
    /// where you got this [GrammarItemInfo] from, but it is generally
    /// associated with the item referred to by the [GrammarItemInfo.ident].
    pub doc: DocString,
}

impl From<(Ident, DocString)> for GrammarItemInfo {
    fn from((ident, doc): (Ident, DocString)) -> Self {
        Self { ident, doc }
    }
}

/// Shared utility methods between [TokenData] and [NodeData].
pub trait GrammarItem {
    /// Returns the name of the "AST trait" that this grammar item implements/should
    /// implement.
    fn ast_trait_name(&self) -> Ident;
    /// Returns the name of the "syntax" type that this grammar item wraps/should wrap.
    fn syntax_type_name(&self) -> Ident;
    /// Returns the name and docstring of the struct/enum corresponding to this grammar item.
    fn item_info(&self) -> GrammarItemInfo;
    /// Returns the name of the accessor method corresponding to this grammar item.
    fn fn_info(&self, label: Option<&str>) -> GrammarItemInfo;
    /// Returns the name and docstring of the `SyntaxKind` variant corresponding to this grammar item.
    fn syntax_kind_info(&self) -> GrammarItemInfo;
    /// Returns the name and docstring for enum variants that house this grammar item as data.
    fn variant_info(&self) -> GrammarItemInfo;
}

impl GrammarItem for TokenData {
    fn ast_trait_name(&self) -> Ident {
        call_site_ident("AstToken")
    }

    fn syntax_type_name(&self) -> Ident {
        call_site_ident("SyntaxToken")
    }

    fn item_info(&self) -> GrammarItemInfo {
        let name = token_name(&self.name);
        let ident = call_site_ident(name.to_upper_camel_case());
        let doc = format_docstring!("Represents the `{}` token.", self.name.as_str());

        (ident, doc).into()
    }

    fn fn_info(&self, label: Option<&str>) -> GrammarItemInfo {
        let name = token_name(label.unwrap_or(self.name.as_str()));
        let ident = call_site_ident(name.to_snake_case());

        let doc = format_docstring!(
            "Find a child token of variant [SyntaxKind::{}].",
            self.syntax_kind_info().ident,
        );

        (ident, doc).into()
    }

    fn syntax_kind_info(&self) -> GrammarItemInfo {
        let name = token_name(&self.name);
        let ident = call_site_ident(name.to_shouty_snake_case());
        let usage = if PAREN_TOKENS.contains(&self.name.as_str()) {
            format!("T!['{}']", self.name.as_str())
        } else {
            format!("T![{}]", self.name.as_str())
        };
        let doc = format_docstring!("Don't try to remember this! Use [`{}`](T) instead.", usage);

        (ident, doc).into()
    }

    fn variant_info(&self) -> GrammarItemInfo {
        let name = grammar_item_name(&self.name);
        let ident = call_site_ident(name.to_upper_camel_case());
        let doc = format_docstring!("See [{}].", self.item_info().ident);

        (ident, doc).into()
    }
}

impl GrammarItem for NodeData {
    fn ast_trait_name(&self) -> Ident {
        call_site_ident("AstNode")
    }

    fn syntax_type_name(&self) -> Ident {
        call_site_ident("SyntaxNode")
    }

    fn item_info(&self) -> GrammarItemInfo {
        let name = node_name(&self.name);
        let ident = call_site_ident(name.to_upper_camel_case());
        let doc = format_docstring!("Represents the `{}` node.", self.name.as_str());

        (ident, doc).into()
    }

    fn fn_info(&self, label: Option<&str>) -> GrammarItemInfo {
        let name = node_name(label.unwrap_or(&self.name));
        let ident = call_site_ident(name.to_snake_case());

        let doc = format_docstring!("Find a child node of type [{}].", self.item_info().ident);

        (ident, doc).into()
    }

    fn syntax_kind_info(&self) -> GrammarItemInfo {
        let name = node_name(&self.name);
        let ident = call_site_ident(name.to_shouty_snake_case());
        let doc = format_docstring!(
            "Don't try to remember this! Use [`N![{}]`](N) instead.",
            &self.name
        );

        (ident, doc).into()
    }

    fn variant_info(&self) -> GrammarItemInfo {
        let name = grammar_item_name(&self.name);
        let ident = call_site_ident(name.to_upper_camel_case());
        let doc = format_docstring!("See [{}].", self.item_info().ident);

        (ident, doc).into()
    }
}

/// Additional methods for [NodeData].
#[extend::ext]
pub impl NodeData {
    /// Returns the name and docstring of the token enum corresponding to this node.
    fn token_enum_info(&self) -> GrammarItemInfo {
        let self_as_token = TokenData {
            name: self.name.clone(),
        };

        self_as_token.item_info()
    }

    /// Same as [GrammarItem::fn_info], but pluralized.
    fn plural_fn_info(&self, label: Option<&str>) -> GrammarItemInfo {
        let name = format!("{}s", node_name(label.unwrap_or(&self.name)));
        let ident = call_site_ident(name.to_snake_case());

        let doc = format_docstring!(
            "Find all children nodes of type [{}].",
            self.item_info().ident
        );

        (ident, doc).into()
    }
}

/// See [RuleExt::simple_index].
#[derive(Hash, Eq, PartialEq)]
pub enum NodeOrTokenIndex {
    NodeIndex(ungrammar::Node),
    TokenIndex(ungrammar::Token),
}

/// Additional methods for [Rule].
#[extend::ext]
pub impl Rule {
    /// Returns the [NodeOrTokenIndex] of a simple rule (or `None` if it's not simple).
    fn simple_index(&self) -> Option<NodeOrTokenIndex> {
        match self {
            Rule::Labeled { label: _, rule } => rule.simple_index(),
            Rule::Opt(rule) => rule.simple_index(),
            Rule::Node(node) => Some(NodeOrTokenIndex::NodeIndex(*node)),
            Rule::Token(token) => Some(NodeOrTokenIndex::TokenIndex(*token)),
            _ => None,
        }
    }
}

/// Appends `"_token"` to the end of the result of [grammar_item_name].
fn token_name(token: &str) -> String {
    let name = grammar_item_name(token);

    format!("{}_token", name)
}

/// Appends `"_node"` to the end of the result of [grammar_item_name].
fn node_name(token: &str) -> String {
    let name = grammar_item_name(token);

    format!("{}_node", name)
}

/// Returns a name (identifier) for a "grammar item" (a token or a node, as spelled in the grammar).
fn grammar_item_name(item: &str) -> &str {
    match item {
        // tokens
        "@ident" => "lit_ident",
        "@number" => "lit_num",
        "@string" => "lit_str",
        "=" => "eq",
        ":" => "colon",
        "{" => "l_brace",
        "}" => "r_brace",
        "(" => "l_paren",
        ")" => "r_paren",
        "[" => "l_bracket",
        "]" => "r_bracket",
        "," => "comma",
        ";" => "semicolon",
        "." => "dot",
        "+" => "plus",
        "-" => "minus",
        "!" => "bang",
        "~" => "tilde",
        "+=" => "plus_eq",
        "-=" => "minus_eq",
        "/=" => "slash_eq",
        "*=" => "star_eq",
        "**=" => "star2_eq",
        "%=" => "percent_eq",
        "^=" => "caret_eq",
        "&=" => "amp_eq",
        "|=" => "pipe_eq",
        "<<=" => "lt2_eq",
        ">>=" => "gt2_eq",
        "/" => "slash",
        "*" => "star",
        "**" => "star2",
        "%" => "percent",
        "&&" => "amp2",
        "||" => "pipe2",
        "==" => "eq2",
        "!=" => "bang_eq",
        ">" => "gt",
        ">=" => "gt_eq",
        "<" => "lt",
        "<=" => "lt_eq",
        "^" => "caret",
        "&" => "amp",
        "|" => "pipe",
        "<<" => "lt2",
        ">>" => "gt2",
        ".." => "dot2",
        "@" => "at",
        "|>" => "pipe_gt",
        // nodes
        "Visibility" => "Vis",
        "Mutability" => "Mutness",
        "ExprSpecifier" => "ExprSpec",
        "TypeQualifier" => "TypeQual",
        // does not require special handling
        other => other,
    }
}

macro format_docstring($template:literal $($rest:tt)*) {
    DocString(format!(concat!(" ", $template) $($rest)*))
}

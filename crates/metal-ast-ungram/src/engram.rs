use std::ops::Index;

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use syn::Ident;
use ungrammar::{Grammar, Node, NodeData, Rule, Token, TokenData};

/// A convenience wrapper around [Grammar].
pub struct Engram(Grammar);

impl Engram {
    /// Returns an iterator over the inner grammar's tokens.
    pub fn tokens(&self) -> impl Iterator<Item = &TokenData> {
        self.0.tokens().map(|token| &self.0[token])
    }

    /// Returns an iterator over the inner grammar's nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &NodeData> {
        self.0.iter().map(|node| &self.0[node])
    }
}

impl From<Grammar> for Engram {
    fn from(value: Grammar) -> Self {
        Self(value)
    }
}

impl Index<&Token> for Engram {
    type Output = TokenData;

    fn index(&self, index: &Token) -> &Self::Output {
        &self.0[*index]
    }
}

impl Index<&Node> for Engram {
    type Output = NodeData;

    fn index(&self, index: &Node) -> &Self::Output {
        &self.0[*index]
    }
}

/// Additional methods for [Rule].
#[extend::ext]
pub impl Rule {
    /// "Unwraps" potential "layers" around the inner [simple rule](is_simple)
    /// and returns it. If this rule is not a simple rule, returns `None`.
    #[allow(clippy::wildcard_enum_match_arm)]
    fn unwrap_simple(&self) -> Option<&Rule> {
        match self {
            rule @ (Rule::Node(_) | Rule::Token(_)) => Some(rule),
            Rule::Opt(rule) => rule.as_ref().unwrap_simple(),
            Rule::Labeled { rule, .. } => rule.as_ref().unwrap_simple(),
            _ => None,
        }
    }
}

/// Additional methods for [TokenData].
#[extend::ext(name = TokenExt)]
pub impl TokenData {
    /// Returns an identifier to be used as the name of the item (struct/enum)
    /// corresponding to this token.
    fn as_item_name(&self) -> Ident {
        let name = token_name(&self.name);
        let ident = name.to_upper_camel_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name of the accessor method
    /// corresponding to this token.
    fn as_fn_name(&self, label: Option<&str>) -> Ident {
        let name = token_name(label.unwrap_or(&self.name));
        let ident = name.to_snake_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name of the `SyntaxKind` variant
    /// corresponding to this token.
    fn as_syntax_kind_name(&self) -> Ident {
        let name = token_name(&self.name);
        let ident = name.to_shouty_snake_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name for enum variants that house
    /// this node as data.
    fn as_variant_name(&self) -> Ident {
        let name = grammar_item_name(&self.name);
        let ident = name.to_upper_camel_case();

        call_site_ident(ident)
    }
}

/// Additional methods for [NodeData].
#[extend::ext(name = NodeExt)]
pub impl NodeData {
    /// Returns an identifier to be used as the name of the item (struct/enum)
    /// corresponding to this node.
    fn as_item_name(&self) -> Ident {
        let name = node_name(&self.name);
        let ident = name.to_upper_camel_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name of the accessor method
    /// corresponding to this node.
    fn as_fn_name(&self, label: Option<&str>) -> Ident {
        let name = node_name(label.unwrap_or(&self.name));
        let ident = name.to_snake_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name of the `SyntaxKind` variant
    /// corresponding to this node.
    fn as_syntax_kind_name(&self) -> Ident {
        let name = node_name(&self.name);
        let ident = name.to_shouty_snake_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name for enum variants that house
    /// this node as data.
    fn as_variant_name(&self) -> Ident {
        let name = grammar_item_name(&self.name);
        let ident = name.to_upper_camel_case();

        call_site_ident(ident)
    }

    /// Returns an identifier to be used as the name of the token enum item
    /// corresponding to this node.
    fn as_token_enum_name(&self) -> Ident {
        let name = token_name(&self.name);
        let ident = name.to_upper_camel_case();

        call_site_ident(ident)
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
        "," => "comma",
        "(" => "l_paren",
        ")" => "r_paren",
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
        "<<=" => "shiftl_eq",
        ">>=" => "shiftr_eq",
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
        // nodes
        "Visibility" => "Vis",
        "Mutability" => "Mutness",
        "ExprSpecifier" => "ExprSpec",
        "TypeQualifier" => "TypeQual",
        // does not require special handling
        other => other,
    }
}

fn call_site_ident(ident: String) -> Ident {
    Ident::new(&ident, proc_macro2::Span::call_site())
}

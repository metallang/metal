use heck::{ToShoutySnakeCase, ToUpperCamelCase};

pub struct IdentableGrammarName(String);

fn syntax_kind_name(s: &str) -> IdentableGrammarName {
    IdentableGrammarName(s.to_shouty_snake_case())
}

fn ast_item_name(s: &str) -> IdentableGrammarName {
    IdentableGrammarName(s.to_upper_camel_case())
}

fn token_fn_item_name(s: &str) -> IdentableGrammarName {
    IdentableGrammarName(format!("{}_token", s))
}

fn node_visual_name(n: &str) -> &str {
    match n {
        "Visibility" => "Vis",
        "Mutability" => "Mutness",
        "ExprSpecifier" => "ExprSpec",
        "TypeQualifier" => "TypeQual",
        other => other,
    }
}

fn token_visual_name(t: &str) -> &str {
    match t {
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
        other => other,
    }
}

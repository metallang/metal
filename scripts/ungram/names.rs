struct Name<'a> {
    inner: Cow<'a, String>,
    generation: u8,
}

// fn syntax_kind_name(s: FormattedGrammarName<'_>) -> IdentableGrammarName {
//     IdentableGrammarName(s.0.to_shouty_snake_case())
// }

// fn ast_item_name(s: FormattedGrammarName<'_>) -> IdentableGrammarName {
//     IdentableGrammarName(s.0.to_upper_camel_case())
// }

// fn token_fn_item_name(s: FormattedGrammarName<'_>) -> IdentableGrammarName {
//     IdentableGrammarName(format!("{}_token", s.0))
// }

// fn node_visual_name(n: RawGrammarName<'_>) -> FormattedGrammarName<'_> {
//     FormattedGrammarName(match n {
//         RawGrammarName("Visibility") => "Vis",
//         RawGrammarName("Mutability") => "Mutness",
//         RawGrammarName("ExprSpecifier") => "ExprSpec",
//         RawGrammarName("TypeQualifier") => "TypeQual",
//         other => other.0,
//     })
// }

// fn token_visual_name(t: RawGrammarName<'_>) -> FormattedGrammarName<'_> {
//     FormattedGrammarName(match t {
//         RawGrammarName("@ident") => "lit_ident",
//         RawGrammarName("@number") => "lit_num",
//         RawGrammarName("@string") => "lit_str",
//         RawGrammarName("=") => "eq",
//         RawGrammarName(":") => "colon",
//         RawGrammarName("{") => "l_brace",
//         RawGrammarName("}") => "r_brace",
//         RawGrammarName(",") => "comma",
//         RawGrammarName("(") => "l_paren",
//         RawGrammarName(")") => "r_paren",
//         RawGrammarName(";") => "semicolon",
//         RawGrammarName(".") => "dot",
//         RawGrammarName("+") => "plus",
//         RawGrammarName("-") => "minus",
//         RawGrammarName("!") => "bang",
//         RawGrammarName("~") => "tilde",
//         RawGrammarName("+=") => "plus_eq",
//         RawGrammarName("-=") => "minus_eq",
//         RawGrammarName("/=") => "slash_eq",
//         RawGrammarName("*=") => "star_eq",
//         RawGrammarName("**=") => "star2_eq",
//         RawGrammarName("%=") => "percent_eq",
//         RawGrammarName("^=") => "caret_eq",
//         RawGrammarName("&=") => "amp_eq",
//         RawGrammarName("|=") => "pipe_eq",
//         RawGrammarName("<<=") => "shiftl_eq",
//         RawGrammarName(">>=") => "shiftr_eq",
//         RawGrammarName("/") => "slash",
//         RawGrammarName("*") => "star",
//         RawGrammarName("**") => "star2",
//         RawGrammarName("%") => "percent",
//         RawGrammarName("&&") => "amp2",
//         RawGrammarName("||") => "pipe2",
//         RawGrammarName("==") => "eq2",
//         RawGrammarName("!=") => "bang_eq",
//         RawGrammarName(">") => "gt",
//         RawGrammarName(">=") => "gt_eq",
//         RawGrammarName("<") => "lt",
//         RawGrammarName("<=") => "lt_eq",
//         RawGrammarName("^") => "caret",
//         RawGrammarName("&") => "amp",
//         RawGrammarName("|") => "pipe",
//         RawGrammarName("<<") => "lt2",
//         RawGrammarName(">>") => "gt2",
//         RawGrammarName("..") => "dot2",
//         other => other.0,
//     })
// }

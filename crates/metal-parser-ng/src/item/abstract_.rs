use metal_ast_ng::SyntaxKind;

use super::fn_::parse_fn_signature;
use crate::common::parse_name;
use crate::common::parse_visibility;

pub fn parse_abstract_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ABSTRACT_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::ABSTRACT_TOKEN);
    parse_name(parser);
    parser.maybe_eat(SyntaxKind::L_BRACE_TOKEN); // TODO: fix L_BRACE_TOKEN docs
    parse_abstract_body(parser);
    parser.maybe_eat(SyntaxKind::R_BRACE_TOKEN);

    parser.end_node();
}

pub fn parse_abstract_body(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ABSTRACT_BODY_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACE_TOKEN) || parser.is_eof()) {
        parse_abstract_fn_item(parser);
    }

    parser.end_node();
}

pub fn parse_abstract_fn_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ABSTRACT_FN_ITEM_NODE);

    parser.maybe_eat(SyntaxKind::DEF_TOKEN);
    parse_name(parser);
    parse_fn_signature(parser);
    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

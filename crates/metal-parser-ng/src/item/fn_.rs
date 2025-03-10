use metal_ast_ng::SyntaxKind;

use crate::block::parse_block;
use crate::common::parse_expr_specifier;
use crate::common::parse_mutability;
use crate::common::parse_name;
use crate::common::parse_type_qualifier;
use crate::common::parse_visibility;

pub fn parse_fn_item(parser: &mut crate::parser::parser_type!(), at: rowan::Checkpoint) {
    parser.start_node_at(SyntaxKind::FN_ITEM_NODE, at);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::DEF_TOKEN);
    parse_name(parser);
    parse_fn_signature(parser);
    parse_block(parser);

    parser.end_node();
}

pub fn parse_fn_signature(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::FN_SIGNATURE_NODE);

    parser.maybe_eat(SyntaxKind::L_PAREN_TOKEN);
    parse_fn_inputs(parser);
    parser.maybe_eat(SyntaxKind::R_PAREN_TOKEN);

    parse_type_qualifier(parser);

    parser.end_node();
}

pub fn parse_fn_inputs(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::FN_INPUTS_NODE);

    while !(parser.peek_is(SyntaxKind::R_PAREN_TOKEN) || parser.is_eof()) {
        parse_fn_input(parser);
        parser.maybe_eat(SyntaxKind::COMMA_TOKEN);
    }

    parser.end_node();
}

pub fn parse_fn_input(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::FN_INPUT_NODE);

    parse_mutability(parser);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);

    parser.end_node();
}

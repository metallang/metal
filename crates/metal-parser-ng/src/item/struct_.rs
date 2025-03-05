use metal_ast_ng::SyntaxKind;

use crate::common::parse_name;
use crate::common::parse_type_qualifier;
use crate::common::parse_visibility;
use crate::item::fn_::parse_fn_item;

pub fn parse_struct_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::STRUCT_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::STRUCT_TOKEN);
    parse_name(parser);
    parser.maybe_eat(SyntaxKind::L_BRACE_TOKEN);
    parse_struct_body(parser);
    parser.maybe_eat(SyntaxKind::R_BRACE_TOKEN);

    parser.end_node();
}

pub fn parse_struct_body(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::STRUCT_BODY_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACE_TOKEN) || parser.is_eof()) {
        parse_struct_body_item(parser);
    }

    parser.end_node();
}

pub fn parse_struct_body_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::STRUCT_BODY_ITEM_NODE);

    let at = parser.checkpoint();

    parse_visibility(parser);

    match parser.peek().expect("expected a struct body item").kind {
        SyntaxKind::LIT_IDENT_TOKEN => parse_struct_field(parser, at),
        SyntaxKind::DEF_TOKEN => parse_fn_item(parser, at),
        other => todo!("{other:#?}"),
    }

    parser.end_node();
}

pub fn parse_struct_field(parser: &mut crate::parser::parser_type!(), at: rowan::Checkpoint) {
    parser.start_node_at(SyntaxKind::STRUCT_FIELD_NODE, at);

    parse_name(parser);
    parse_type_qualifier(parser);
    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

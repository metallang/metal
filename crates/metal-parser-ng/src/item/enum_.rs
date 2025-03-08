use metal_ast_ng::SyntaxKind;

use crate::common::parse_name;
use crate::common::parse_visibility;
use crate::item::fn_::parse_fn_item;
use crate::type_::parse_type;

pub fn parse_enum_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ENUM_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::ENUM_TOKEN);
    parse_name(parser);
    parser.maybe_eat(SyntaxKind::L_BRACE_TOKEN);
    parse_enum_body(parser);
    parser.maybe_eat(SyntaxKind::R_BRACE_TOKEN);

    parser.end_node();
}

pub fn parse_enum_body(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ENUM_BODY_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACE_TOKEN) || parser.is_eof()) {
        parse_enum_body_item(parser);
    }

    parser.end_node();
}

pub fn parse_enum_body_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ENUM_BODY_ITEM_NODE);

    let at = parser.checkpoint();

    match parser.peek().expect("expected an enum body item").kind {
        SyntaxKind::LIT_IDENT_TOKEN => parse_enum_variant(parser),
        SyntaxKind::DEF_TOKEN => parse_fn_item(parser, at),
        _ => todo!(),
    }

    parser.end_node();
}

pub fn parse_enum_variant(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ENUM_VARIANT_NODE);

    parse_name(parser);
    parse_enum_variant_data_type(parser);

    parser.end_node();
}

pub fn parse_enum_variant_data_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ENUM_VARIANT_DATA_TYPE_NODE);

    if parser.maybe_eat(SyntaxKind::L_PAREN_TOKEN) {
        parse_type(parser);
        parser.maybe_eat(SyntaxKind::R_PAREN_TOKEN);
    }

    parser.end_node();
}

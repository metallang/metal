use metal_ast_ng::SyntaxKind;

use crate::type_::parse_type;

pub fn parse_name_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::NAME_TYPE_NODE);

    parser.maybe_eat(SyntaxKind::LIT_IDENT_TOKEN);

    if parser.peek_is(SyntaxKind::L_BRACKET_TOKEN) {
        parse_name_type_generics(parser);
    }

    parser.end_node();
}

pub fn parse_name_type_generics(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::NAME_TYPE_GENERICS_NODE);

    parser.maybe_eat(SyntaxKind::L_BRACKET_TOKEN);
    parse_name_type_generics_inner(parser);
    parser.maybe_eat(SyntaxKind::R_BRACKET_TOKEN);

    parser.end_node();
}

pub fn parse_name_type_generics_inner(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::NAME_TYPE_GENERICS_INNER_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACKET_TOKEN) || parser.is_eof()) {
        parse_type(parser);
        parser.maybe_eat(SyntaxKind::COMMA_TOKEN);
    }

    parser.end_node();
}

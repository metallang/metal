use metal_ast_ng::SyntaxKind;

use crate::common::parse_expr_specifier;
use crate::common::parse_name;
use crate::common::parse_type_qualifier;
use crate::common::parse_visibility;

pub fn parse_const_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::CONST_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::CONST_TOKEN);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);
    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

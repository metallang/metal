use metal_ast_ng::SyntaxKind;

use crate::common::parse_name;
use crate::common::parse_visibility;
use crate::type_::parse_type;

pub fn parse_type_alias_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::TYPE_ALIAS_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::TYPE_TOKEN);
    parse_name(parser);
    parser.maybe_eat(SyntaxKind::EQ_TOKEN);
    parse_type(parser);
    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

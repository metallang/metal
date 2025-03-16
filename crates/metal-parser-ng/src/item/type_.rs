use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::common::parse_name;
use crate::type_::parse_type;

pub fn parse_type_alias_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![TypeAliasItem]);

    parser.maybe_eat(T![type]);
    parse_name(parser);
    parser.maybe_eat(T![=]);
    parse_type(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

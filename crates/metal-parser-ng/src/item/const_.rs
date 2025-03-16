// SPDX-License-Identifier: MIT

use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::common::parse_expr_specifier;
use crate::common::parse_name;
use crate::common::parse_type_qualifier;

pub fn parse_const_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ConstItem]);

    parser.maybe_eat(T![const]);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

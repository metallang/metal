// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::{parse_expr_specifier, parse_name, parse_type_qualifier};

pub fn parse_const_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![ConstItem]);

    parser.maybe_eat(T![const]);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

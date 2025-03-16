// SPDX-License-Identifier: MIT

use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::expr::parse_expr;

pub fn parse_return_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ReturnItem]);

    parser.maybe_eat(T![return]);

    if !parser.peek_is(T![;]) {
        parse_expr(parser);
    }

    parser.maybe_eat(T![;]);

    parser.end_node();
}

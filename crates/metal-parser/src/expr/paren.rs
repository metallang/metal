// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_paren_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![ParenExpr]);

    parser.maybe_eat(T!['(']);
    parse_expr(parser);
    parser.maybe_eat(T![')']);

    parser.end_node();
}

// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_defer_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![DeferExpr]);

    parser.maybe_eat(T![defer]);
    parse_expr(parser);

    parser.end_node();
}

// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_return_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![ReturnExpr]);

    parser.maybe_eat(T![return]);

    if parser
        .peek(0)
        .is_some_and(|token| token.kind.is_expr_start())
    {
        parse_expr(parser);
    }

    parser.end_node();
}

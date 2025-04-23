// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_call_expr(parser: &mut crate::parser::Parser, checkpoint: rowan::Checkpoint) {
    parser.start_node_at(N![CallExpr], checkpoint);

    // the callee is now here

    parser.maybe_eat(T!['(']);
    parse_call_expr_args(parser);
    parser.maybe_eat(T![')']);

    parser.end_node();
}

pub fn parse_call_expr_args(parser: &mut crate::parser::Parser) {
    parser.start_node(N![CallExprArgs]);

    while parser.is_not_at_eof_or(T![')']) {
        parse_expr(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

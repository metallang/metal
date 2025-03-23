// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_call_expr(parser: &mut crate::parser::parser_type!(), checkpoint: rowan::Checkpoint) {
    parser.start_node_at(N![Expr], checkpoint);
    parser.start_node_at(N![CallExpr], checkpoint);

    // the callee is now here

    parser.maybe_eat(T!['(']);
    parse_call_expr_args(parser);
    parser.maybe_eat(T![')']);

    parser.end_node();
    parser.end_node();
}

pub fn parse_call_expr_args(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![CallExprArgs]);

    while !(parser.peek_is(T![')']) || parser.is_eof()) {
        parse_expr(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

use metal_ast::{N, T};

use crate::block::parse_block;

pub fn parse_loop_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![LoopExpr]);

    parser.maybe_eat(T![loop]);

    parse_block(parser);

    parser.end_node();
}

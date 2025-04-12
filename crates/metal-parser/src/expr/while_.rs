// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::block::parse_block;
use crate::expr::parse_expr;
use crate::restrictions::RestrictionFlags;

pub fn parse_while_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![WhileExpr]);

    parser.maybe_eat(T![while]);
    parser.with_restrictions(|parser| {
        parser
            .restrictions_mut()
            .add(RestrictionFlags::NO_STRUCT_EXPR);

        parse_expr(parser);
    });
    parse_block(parser);

    parser.end_node();
}

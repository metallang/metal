use metal_ast::{N, T};

use crate::expr::parse_expr;
use crate::restrictions::RestrictionFlags;
use crate::{block::parse_block, common::parse_name};

pub fn parse_for_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![ForExpr]);

    parser.maybe_eat(T![for]);
    parse_name(parser);
    parser.maybe_eat(T![in]);
    parser.with_restrictions(|parser| {
        parser
            .restrictions_mut()
            .add(RestrictionFlags::NO_STRUCT_EXPR);

        parse_expr(parser);
    });
    parse_block(parser);

    parser.end_node();
}

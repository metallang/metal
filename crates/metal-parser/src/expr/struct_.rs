// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::{parse_expr_specifier, parse_name};

pub fn parse_struct_expr(parser: &mut crate::parser::Parser, checkpoint: rowan::Checkpoint) {
    parser.start_node_at(N![StructExpr], checkpoint);

    // the struct name/path expr is now here

    parser.maybe_eat(T!['{']);
    parse_struct_expr_fields(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_struct_expr_fields(parser: &mut crate::parser::Parser) {
    parser.start_node(N![StructExprFields]);

    while parser.is_not_at_eof_or(T!['}']) {
        parse_struct_expr_field(parser);
        parser.maybe_eat(T![;]);
    }

    parser.end_node();
}

pub fn parse_struct_expr_field(parser: &mut crate::parser::Parser) {
    parser.start_node(N![StructExprField]);

    parse_name(parser);
    parse_expr_specifier(parser);

    parser.end_node();
}

// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::stmt::parse_stmt;

pub fn parse_block(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Block]);

    parser.maybe_eat(T!['{']);
    parse_block_stmts(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_block_stmts(parser: &mut crate::parser::Parser) {
    parser.start_node(N![BlockStmts]);

    while parser.is_not_at_eof_or(T!['}']) {
        parse_stmt(parser);
    }

    parser.end_node();
}

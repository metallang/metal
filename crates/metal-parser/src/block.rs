// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::parser::parser_type;
use crate::stmt::parse_stmt;

pub fn parse_block(parser: &mut parser_type!()) {
    parser.start_node(N![Block]);

    parser.maybe_eat(T!['{']);
    parse_block_stmts(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_block_stmts(parser: &mut parser_type!()) {
    parser.start_node(N![BlockStmts]);

    while !(parser.peek_is(T!['}']) || parser.is_eof()) {
        parse_stmt(parser);
    }

    parser.end_node();
}

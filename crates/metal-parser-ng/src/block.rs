// SPDX-License-Identifier: MIT

use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::item::parse_item;
use crate::parser::parser_type;

pub fn parse_block(parser: &mut parser_type!()) {
    parser.start_node(N![Block]);

    parser.maybe_eat(T!['{']);
    parse_block_items(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_block_items(parser: &mut parser_type!()) {
    parser.start_node(N![BlockItems]);

    while !(parser.peek_is(T!['}']) || parser.is_eof()) {
        parse_item(parser);
    }

    parser.end_node();
}

// SPDX-License-Identifier: MIT

use metal_ast_ng::{N, T};

use crate::common::parse_name;
use crate::item::fn_::parse_fn_signature;

pub fn parse_abstract_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![AbstractItem]);

    parser.maybe_eat(T![abstract]);
    parse_name(parser);
    parser.maybe_eat(T!['{']);
    parse_abstract_body(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_abstract_body(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![AbstractBody]);

    while !(parser.peek_is(T!['}']) || parser.is_eof()) {
        parse_abstract_fn_item(parser);
    }

    parser.end_node();
}

pub fn parse_abstract_fn_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![AbstractFnItem]);

    parser.maybe_eat(T![def]);
    parse_name(parser);
    parse_fn_signature(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

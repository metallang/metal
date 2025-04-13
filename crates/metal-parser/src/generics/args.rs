// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::type_::parse_type;

pub fn parse_generic_arg_list(parser: &mut crate::parser::Parser) {
    parser.start_node(N![GenericArgList]);

    parser.maybe_eat(T!['[']);
    parse_generic_args(parser);
    parser.maybe_eat(T![']']);

    parser.end_node();
}

pub fn parse_generic_args(parser: &mut crate::parser::Parser) {
    parser.start_node(N![GenericArgs]);

    while parser.is_not_at_eof_or(T![']']) {
        parse_type(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

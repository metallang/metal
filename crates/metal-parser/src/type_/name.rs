// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::generics::args::parse_generic_arg_list;

pub fn parse_name_type(parser: &mut crate::parser::Parser) {
    parser.start_node(N![NameType]);

    parser.maybe_eat(T![@ident]);

    if parser.peek_is(0, T!['[']) {
        parse_generic_arg_list(parser);
    }

    parser.end_node();
}

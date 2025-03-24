// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::generics::args::parse_generic_arg_list;

pub fn parse_name_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![NameType]);

    parser.maybe_eat(T![@ident]);

    if parser.peek_is(T!['[']) {
        parse_generic_arg_list(parser);
    }

    parser.end_node();
}

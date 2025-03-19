// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::type_::parse_type;

pub fn parse_name_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![NameType]);

    parser.maybe_eat(T![@ident]);

    if parser.peek_is(T!['[']) {
        parse_name_type_generics(parser);
    }

    parser.end_node();
}

pub fn parse_name_type_generics(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![NameTypeGenerics]);

    parser.maybe_eat(T!['[']);
    parse_name_type_generics_inner(parser);
    parser.maybe_eat(T![']']);

    parser.end_node();
}

pub fn parse_name_type_generics_inner(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![NameTypeGenericsInner]);

    while !(parser.peek_is(T![']']) || parser.is_eof()) {
        parse_type(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

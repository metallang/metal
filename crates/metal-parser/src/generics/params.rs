// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::{parse_name, parse_type_qualifier, parse_type_specifier};

pub fn parse_generic_param_list(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![GenericParamList]);

    parser.maybe_eat(T!['[']);
    parse_generic_params(parser);
    parser.maybe_eat(T![']']);

    parser.end_node();
}

pub fn parse_generic_params(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![GenericParams]);

    while !(parser.peek_is(T![']']) || parser.is_eof()) {
        parse_generic_param(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

pub fn parse_generic_param(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![GenericParam]);

    parse_name(parser);
    parse_type_qualifier(parser);
    parse_type_specifier(parser);

    parser.end_node();
}

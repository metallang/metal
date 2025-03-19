// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::generics::params::parse_generic_name;
use crate::type_::parse_type;

pub fn parse_type_alias_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![TypeAliasItem]);

    parser.maybe_eat(T![type]);
    parse_generic_name(parser);
    parser.maybe_eat(T![=]);
    parse_type(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

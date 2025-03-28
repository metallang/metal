// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::{generics::utils::parse_name_generics, type_::parse_type};

pub fn parse_type_alias_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![TypeAliasItem]);

    parser.maybe_eat(T![type]);
    parse_name_generics(parser);
    parser.maybe_eat(T![=]);
    parse_type(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

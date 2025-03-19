// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::parse_mutability;
use crate::type_::parse_type;

pub fn parse_ref_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![RefType]);

    parser.maybe_eat(T![&]);
    parse_mutability(parser);
    parse_type(parser);

    parser.end_node();
}

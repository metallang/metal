// SPDX-License-Identifier: MIT

use metal_ast::T;

use crate::common::parse_name;
use crate::generics::params::parse_generic_param_list;

/// Parse a Name and, if followed by a [, parse a GenericParamList. Does NOT create a node.
pub fn parse_name_generics(parser: &mut crate::parser::parser_type!()) {
    parse_name(parser);

    if parser.peek_is(T!['[']) {
        parse_generic_param_list(parser);
    }
}

// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::parse_name;
use crate::common::parse_type_qualifier;
use crate::common::parse_expr_specifier;

pub fn parse_let_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![LetExpr]);

    parser.maybe_eat(T![let]);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);

    parser.end_node();
}

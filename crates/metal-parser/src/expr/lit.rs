// SPDX-License-Identifier: MIT

use metal_ast::T;

pub fn parse_lit_expr(parser: &mut crate::parser::Parser) {
    assert!(parser
        .peek(0)
        .is_some_and(|token| token.kind == T![@number] || token.kind == T![@string]));

    parser.eat_any();
}

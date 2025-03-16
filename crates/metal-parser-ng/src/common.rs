// SPDX-License-Identifier: MIT

use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::expr::parse_expr;
use crate::type_::parse_type;

pub fn parse_name(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![Name]);

    parser.maybe_eat(T![@ident]);

    parser.end_node();
}

pub fn parse_visibility(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![Visibility]);

    parser.maybe_eat(T![pub]);

    parser.end_node();
}

pub fn parse_mutability(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![Mutability]);

    parser.maybe_eat(T![mut]);

    parser.end_node();
}

pub fn parse_type_qualifier(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![TypeQualifier]);

    if parser.maybe_eat(T![:]) {
        parse_type(parser);
    }

    parser.end_node();
}

pub fn parse_expr_specifier(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ExprSpecifier]);

    if parser.maybe_eat(T![=]) {
        parse_expr(parser);
    }

    parser.end_node();
}

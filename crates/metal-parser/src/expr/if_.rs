// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::{expr::parse_expr, parser::ParsingContext};

pub fn parse_if_expr(parser: &mut crate::parser::Parser) {
    parser.start_node(N![IfExpr]);

    parser.maybe_eat(T![if]);

    parser.in_cx(ParsingContext::IfExprCond, |parser| parse_expr(parser)); // condition
    parse_expr(parser); // 'true' branch

    if parser.peek_is(0, T![else]) {
        parser.start_node(N![IfExprElseClause]);

        parser.eat_any();
        parse_expr(parser); // 'false' branch

        parser.end_node();
    }

    parser.end_node();
}

use metal_ast::{N, T};

use crate::expr::parse_expr;

pub fn parse_if_expr(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![IfExpr]);

    parser.maybe_eat(T![if]);

    parse_expr(parser); // condition
    parse_expr(parser); // 'true' branch

    if parser.peek_is(T![else]) {
        parser.start_node(N![IfExprElseClause]);

        parser.eat_any();
        parse_expr(parser); // 'false' branch

        parser.end_node();
    }

    parser.end_node();
}

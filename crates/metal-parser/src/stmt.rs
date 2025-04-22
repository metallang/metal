// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::expr::parse_expr;
use crate::item::parse_item;

pub fn parse_stmt(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Stmt]);

    parse_stmt_kind(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

pub fn parse_stmt_kind(parser: &mut crate::parser::Parser) {
    // IMPORTANT: Check for items first, as otherwise `def hello() {}`
    // will get parsed as a lambda function expression instead of an item
    if parser
        .peek(0)
        .is_some_and(|token| token.kind.is_item_start())
    {
        parse_item(parser);
    } else {
        parse_expr(parser);
    }
}

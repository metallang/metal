// SPDX-License-Identifier: MIT

use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::expr::parse_expr;
use crate::item::parse_item;

pub fn parse_stmt(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![Stmt]);

    parse_stmt_kind(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

pub fn parse_stmt_kind(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![StmtKind]);

    if parser
        .peek()
        .is_some_and(|token| token.kind.is_item_start())
    {
        parse_item(parser);
    } else {
        parse_expr(parser);
    }

    parser.end_node();
}

use metal_ast_ng::SyntaxKind;

use crate::expr::parse_expr;

pub fn parse_return_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::RETURN_ITEM_NODE);

    parser.maybe_eat(SyntaxKind::RETURN_TOKEN);

    if !parser.peek_is(SyntaxKind::SEMICOLON_TOKEN) {
        parse_expr(parser);
    }

    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

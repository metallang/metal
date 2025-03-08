use metal_ast_ng::SyntaxKind;

use crate::item::parse_item;
use crate::parser::parser_type;

pub fn parse_block(parser: &mut parser_type!()) {
    parser.start_node(SyntaxKind::BLOCK_NODE);

    parser.maybe_eat(SyntaxKind::L_BRACE_TOKEN);
    parse_block_items(parser);
    parser.maybe_eat(SyntaxKind::R_BRACE_TOKEN);

    parser.end_node();
}

pub fn parse_block_items(parser: &mut parser_type!()) {
    parser.start_node(SyntaxKind::BLOCK_ITEMS_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACE_TOKEN) || parser.is_eof()) {
        parse_item(parser);
    }

    parser.end_node();
}

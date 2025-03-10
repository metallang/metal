use metal_ast_ng::SyntaxKind;

use crate::common::parse_name;
use crate::common::parse_visibility;

pub fn parse_import_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_ITEM_NODE);

    parse_visibility(parser);
    parser.maybe_eat(SyntaxKind::IMPORT_TOKEN);
    parse_import_tree(parser);
    parser.maybe_eat(SyntaxKind::SEMICOLON_TOKEN);

    parser.end_node();
}

pub fn parse_import_tree(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_TREE_NODE);

    match parser.peek().expect("expected an import tree").kind {
        SyntaxKind::LIT_IDENT_TOKEN => parse_import_leaf(parser),
        SyntaxKind::L_BRACE_TOKEN => parse_import_branch(parser),
        _ => todo!(),
    }

    parser.end_node();
}

pub fn parse_import_leaf(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_LEAF_NODE);

    parse_name(parser);

    if parser.peek_is(SyntaxKind::DOT_TOKEN) {
        parse_import_segment(parser);
    }

    parser.end_node();
}

pub fn parse_import_segment(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_SEGMENT_NODE);

    parser.maybe_eat(SyntaxKind::DOT_TOKEN);
    parse_import_tree(parser);

    parser.end_node();
}

pub fn parse_import_branch(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_BRANCH_NODE);

    parser.maybe_eat(SyntaxKind::L_BRACE_TOKEN);
    parse_import_branch_subtrees(parser);
    parser.maybe_eat(SyntaxKind::R_BRACE_TOKEN);

    parser.end_node();
}

pub fn parse_import_branch_subtrees(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::IMPORT_BRANCH_SUBTREES_NODE);

    while !(parser.peek_is(SyntaxKind::R_BRACE_TOKEN) || parser.is_eof()) {
        parse_import_tree(parser);
        parser.maybe_eat(SyntaxKind::COMMA_TOKEN);
    }

    parser.end_node();
}

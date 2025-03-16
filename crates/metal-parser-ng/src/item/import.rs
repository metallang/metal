use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::common::parse_name;

pub fn parse_import_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportItem]);

    parser.maybe_eat(T![import]);
    parse_import_tree(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

pub fn parse_import_tree(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportTree]);

    match parser.peek().expect("expected an import tree").kind {
        T![@ident] => parse_import_leaf(parser),
        T!['{'] => parse_import_branch(parser),
        _ => todo!(),
    }

    parser.end_node();
}

pub fn parse_import_leaf(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportLeaf]);

    parse_name(parser);

    if parser.peek_is(T![.]) {
        parse_import_segment(parser);
    }

    parser.end_node();
}

pub fn parse_import_segment(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportSegment]);

    parser.maybe_eat(T![.]);
    parse_import_tree(parser);

    parser.end_node();
}

pub fn parse_import_branch(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportBranch]);

    parser.maybe_eat(T!['{']);
    parse_import_branch_subtrees(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_import_branch_subtrees(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![ImportBranchSubtrees]);

    while !(parser.peek_is(T!['}']) || parser.is_eof()) {
        parse_import_tree(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

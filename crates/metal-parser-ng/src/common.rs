use metal_ast_ng::SyntaxKind;

use crate::expr::parse_expr;
use crate::type_::parse_type;

pub fn parse_name(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::NAME_NODE);

    parser.maybe_eat(SyntaxKind::LIT_IDENT_TOKEN);

    parser.end_node();
}

pub fn parse_visibility(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(metal_ast_ng::SyntaxKind::VIS_NODE);

    parser.maybe_eat(SyntaxKind::PUB_TOKEN);

    parser.end_node();
}

pub fn parse_mutability(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(metal_ast_ng::SyntaxKind::MUTNESS_NODE);

    parser.maybe_eat(SyntaxKind::MUT_TOKEN);

    parser.end_node();
}

pub fn parse_type_qualifier(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(metal_ast_ng::SyntaxKind::TYPE_QUAL_NODE);

    if parser.maybe_eat(SyntaxKind::COLON_TOKEN) {
        parse_type(parser);
    }

    parser.end_node();
}

pub fn parse_expr_specifier(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(metal_ast_ng::SyntaxKind::EXPR_SPEC_NODE);

    if parser.maybe_eat(SyntaxKind::EQ_TOKEN) {
        parse_expr(parser);
    }

    parser.end_node();
}

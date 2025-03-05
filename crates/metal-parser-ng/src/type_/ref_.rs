use metal_ast_ng::SyntaxKind;

use crate::common::parse_mutability;
use crate::type_::parse_type;

pub fn parse_ref_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::REF_TYPE_NODE);

    parser.maybe_eat(SyntaxKind::AMP_TOKEN);
    parse_mutability(parser);
    parse_type(parser);

    parser.end_node();
}

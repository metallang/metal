use crate::parser::ParserMode;
use crate::type_::name::parse_name_type;
use crate::type_::ref_::parse_ref_type;

use metal_ast_ng::SyntaxKind;

mod name;
mod ref_;

pub fn parse_type(parser: &mut crate::parser::parser_type!()) {
    let at = parser.checkpoint();

    parser.start_node(SyntaxKind::TYPE_NODE);
    let old_mode = parser.enter_mode(ParserMode::Type);

    match parser.peek().expect("expected a type").kind {
        SyntaxKind::LIT_IDENT_TOKEN => parse_name_type(parser),
        SyntaxKind::AMP_TOKEN => parse_ref_type(parser),
        other => todo!("{other:#?}"),
    }

    parser.end_node();

    if parser.peek().is_some_and(|t| is_binary_type_op(t.kind)) {
        parser.start_node_at(SyntaxKind::BINARY_TYPE_OP_NODE, at);

        parser.eat_any();
        parse_type(parser);

        parser.end_node();
    }

    parser.enter_mode(old_mode);
}

fn is_binary_type_op(kind: SyntaxKind) -> bool {
    kind == SyntaxKind::AMP_TOKEN
}

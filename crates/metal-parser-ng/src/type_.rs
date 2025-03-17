// SPDX-License-Identifier: MIT

use crate::parser::ParserMode;
use crate::type_::name::parse_name_type;
use crate::type_::ref_::parse_ref_type;

use metal_ast_ng::SyntaxKind;
use metal_ast_ng::N;
use metal_ast_ng::T;

mod name;
mod ref_;

pub fn parse_type(parser: &mut crate::parser::parser_type!()) {
    let at = parser.checkpoint();

    parser.start_node(N![Type]);
    let old_mode = parser.enter_mode(ParserMode::Type);

    match parser.peek().expect("expected a type").kind {
        T![@ident] => parse_name_type(parser),
        T![&] => parse_ref_type(parser),
        other => todo!("{other:#?}"),
    }

    parser.end_node();

    if parser.peek().is_some_and(|t| is_binary_type_op(t.kind)) {
        parser.start_node_at(N![BinaryTypeOp], at);

        parser.eat_any();
        parse_type(parser);

        parser.end_node();
    }

    parser.enter_mode(old_mode);
}

fn is_binary_type_op(kind: SyntaxKind) -> bool {
    kind == T![&]
}

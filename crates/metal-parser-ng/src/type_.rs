// SPDX-License-Identifier: MIT

use metal_ast_ng::{SyntaxKind, N, T};

use crate::parser::ParserMode;
use crate::type_::name::parse_name_type;
use crate::type_::ref_::parse_ref_type;

mod name;
mod ref_;

pub fn parse_type(parser: &mut crate::parser::parser_type!()) {
    let at = parser.checkpoint();

    parser.start_node(N![Type]);

    parser.in_mode(ParserMode::Type, |parser| {
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
    });
}

fn is_binary_type_op(kind: SyntaxKind) -> bool {
    kind == T![&]
}

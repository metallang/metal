// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::type_::name::parse_name_type;
use crate::type_::ref_::parse_ref_type;

pub mod name;
pub mod ref_;

pub fn parse_type(parser: &mut crate::parser::Parser) {
    let at = parser.checkpoint();

    parser.start_node(N![Type]);

    match parser.peek(0).expect("expected a type").kind {
        T![@ident] => parse_name_type(parser),
        T![&] => parse_ref_type(parser),
        other => todo!("{other:#?}"),
    }

    parser.end_node();

    if parser.peek(0).is_some_and(|t| t.kind.is_binary_type_op()) {
        parser.start_node_at(N![BinaryTypeOp], at);

        parser.eat_any();
        parse_type(parser);

        parser.end_node();
    }
}

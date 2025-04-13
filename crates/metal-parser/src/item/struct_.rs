// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::{parse_expr_specifier, parse_name, parse_type_qualifier, parse_visibility};
use crate::generics::utils::parse_name_generics;
use crate::item::fn_::parse_fn_item;

pub fn parse_struct_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![StructItem]);

    parser.maybe_eat(T![struct]);
    parse_name_generics(parser);
    parser.maybe_eat(T!['{']);
    parse_struct_body(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_struct_body(parser: &mut crate::parser::Parser) {
    parser.start_node(N![StructBody]);

    while parser.is_not_at_eof_or(T!['}']) {
        parse_struct_body_item(parser);
    }

    parser.end_node();
}

pub fn parse_struct_body_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![StructBodyItem]);

    let at = parser.checkpoint();

    parse_visibility(parser);

    match parser.peek(0).expect("expected a struct body item").kind {
        T![@ident] => parse_struct_field(parser, at),
        T![def] => parse_struct_fn(parser, at),
        other => todo!("{other:#?}"),
    }

    parser.end_node();
}

pub fn parse_struct_field(parser: &mut crate::parser::Parser, at: rowan::Checkpoint) {
    parser.start_node_at(N![StructField], at);

    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

pub fn parse_struct_fn(parser: &mut crate::parser::Parser, at: rowan::Checkpoint) {
    parser.start_node_at(N![StructFn], at);

    // FIXME: visibility is missing
    parse_fn_item(parser);

    parser.end_node();
}

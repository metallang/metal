// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::{parse_name, parse_visibility};
use crate::generics::utils::parse_name_generics;
use crate::item::fn_::parse_fn_item;
use crate::type_::parse_type;

pub fn parse_enum_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumItem]);

    parse_visibility(parser);
    parser.maybe_eat(T![enum]);
    parse_name_generics(parser);
    parser.maybe_eat(T!['{']);
    parse_enum_body(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_enum_body(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumBody]);

    while parser.is_not_at_eof_or(T!['}']) {
        parse_enum_body_item(parser);
    }

    parser.end_node();
}

pub fn parse_enum_body_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumBodyItem]);

    match parser.peek(0).expect("expected an enum body item").kind {
        T![@ident] => parse_enum_variant(parser),
        T![pub] | T![def] => parse_enum_fn(parser),
        _ => todo!(),
    }

    parser.end_node();
}

pub fn parse_enum_variant(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumVariant]);

    parse_name(parser);
    parse_enum_variant_data_type(parser);
    parser.maybe_eat(T![;]);

    parser.end_node();
}

pub fn parse_enum_variant_data_type(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumVariantDataType]);

    if parser.maybe_eat(T!['(']) {
        parse_type(parser);
        parser.maybe_eat(T![')']);
    }

    parser.end_node();
}

pub fn parse_enum_fn(parser: &mut crate::parser::Parser) {
    parser.start_node(N![EnumFn]);

    parse_visibility(parser);
    parse_fn_item(parser);

    parser.end_node();
}

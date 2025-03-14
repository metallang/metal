use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::common::parse_name;
use crate::common::parse_visibility;
use crate::item::fn_::parse_fn_item;
use crate::type_::parse_type;

pub fn parse_enum_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![EnumItem]);

    parse_visibility(parser);
    parser.maybe_eat(T![enum]);
    parse_name(parser);
    parser.maybe_eat(T!['{']);
    parse_enum_body(parser);
    parser.maybe_eat(T!['}']);

    parser.end_node();
}

pub fn parse_enum_body(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![EnumBody]);

    while !(parser.peek_is(T!['}']) || parser.is_eof()) {
        parse_enum_body_item(parser);
    }

    parser.end_node();
}

pub fn parse_enum_body_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![EnumBodyItem]);

    let at = parser.checkpoint();

    match parser.peek().expect("expected an enum body item").kind {
        T![@ident] => parse_enum_variant(parser),
        T![def] => parse_fn_item(parser, at),
        _ => todo!(),
    }

    parser.end_node();
}

pub fn parse_enum_variant(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![EnumVariant]);

    parse_name(parser);
    parse_enum_variant_data_type(parser);

    parser.end_node();
}

pub fn parse_enum_variant_data_type(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![EnumVariantDataType]);

    if parser.maybe_eat(T!['(']) {
        parse_type(parser);
        parser.maybe_eat(T![')']);
    }

    parser.end_node();
}

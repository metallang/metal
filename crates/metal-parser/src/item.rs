// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::common::parse_visibility;
use crate::expr::parse_expr;
use crate::item::abstract_::parse_abstract_item;
use crate::item::const_::parse_const_item;
use crate::item::enum_::parse_enum_item;
use crate::item::fn_::parse_fn_item;
use crate::item::import::parse_import_item;
use crate::item::struct_::parse_struct_item;
use crate::item::type_::parse_type_alias_item;

mod abstract_;
mod const_;
mod enum_;
pub mod fn_; // TODO: make all other modules pub as well
mod import;
mod struct_;
mod type_;

pub fn parse_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Item]);

    parse_annotations(parser);
    parse_visibility(parser);
    parse_item_kind(parser);

    parser.end_node();
}

pub fn parse_item_kind(parser: &mut crate::parser::Parser) {
    parser.start_node(N![ItemKind]);

    match parser.peek(0).expect("expected an item").kind {
        T![abstract] => parse_abstract_item(parser),
        T![const] => parse_const_item(parser),
        T![def] => parse_fn_item(parser),
        T![enum] => parse_enum_item(parser),
        T![import] => parse_import_item(parser),
        T![struct] => parse_struct_item(parser),
        T![type] => parse_type_alias_item(parser),
        other => todo!("{other:#?}"),
    }

    parser.end_node();
}

pub fn parse_annotations(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Annotations]);

    while parser.peek_is(0, T![@]) {
        parse_annotation(parser);
    }

    parser.end_node();
}

pub fn parse_annotation(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Annotation]);

    parser.maybe_eat(T![@]);
    parse_expr(parser);

    parser.end_node();
}

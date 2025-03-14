use metal_ast_ng::SyntaxKind;
use metal_ast_ng::T;

use crate::expr::parse_expr;
use crate::item::abstract_::parse_abstract_item;
use crate::item::const_::parse_const_item;
use crate::item::enum_::parse_enum_item;
use crate::item::fn_::parse_fn_item;
use crate::item::import::parse_import_item;
use crate::item::return_::parse_return_item;
use crate::item::struct_::parse_struct_item;
use crate::item::type_::parse_type_alias_item;

mod abstract_;
mod const_;
mod enum_;
mod fn_;
mod import;
mod return_;
mod struct_;
mod type_;

pub fn parse_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(SyntaxKind::ITEM_NODE);

    let at = parser.checkpoint();

    match parser.peek().expect("expected an item").kind {
        T![abstract] => parse_abstract_item(parser),
        T![const] => parse_const_item(parser),
        T![def] => parse_fn_item(parser, at),
        T![enum] => parse_enum_item(parser),
        T![import] => parse_import_item(parser),
        T![return] => parse_return_item(parser),
        T![struct] => parse_struct_item(parser),
        T![type] => parse_type_alias_item(parser),
        _ => parse_expr(parser),
    }

    parser.end_node();
}

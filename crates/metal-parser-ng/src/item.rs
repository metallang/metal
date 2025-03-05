use metal_ast_ng::SyntaxKind;

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
        SyntaxKind::ABSTRACT_TOKEN => parse_abstract_item(parser),
        SyntaxKind::CONST_TOKEN => parse_const_item(parser),
        SyntaxKind::DEF_TOKEN => parse_fn_item(parser, at),
        SyntaxKind::ENUM_TOKEN => parse_enum_item(parser),
        SyntaxKind::IMPORT_TOKEN => parse_import_item(parser),
        SyntaxKind::RETURN_TOKEN => parse_return_item(parser),
        SyntaxKind::STRUCT_TOKEN => parse_struct_item(parser),
        SyntaxKind::TYPE_TOKEN => parse_type_alias_item(parser),
        other => todo!("{other:#?}"),
    }

    parser.end_node();
}

use metal_ast::{Item, Visibility};
use metal_lexer::TokenKind;

use crate::{
    parse_const_item, parse_enum_item, parse_expr, parse_fn_item, parse_import_item,
    parse_struct_item, parse_vis, parser_type,
};

pub mod const_item;
pub mod enum_item;
pub mod fn_item;
pub mod import;
pub mod struct_item;

pub fn parse_item<'src>(parser: parser_type!('src)) -> Item<'src> {
    match parser.peek().expect("an item").kind {
        TokenKind::Const
        | TokenKind::Enum
        | TokenKind::Struct
        | TokenKind::Def
        | TokenKind::Import
        | TokenKind::Pub => {
            let vis = parse_vis(parser);

            parse_item_with_vis(parser, vis)
        }
        _ => Item::Expr(parse_expr(parser)),
    }
}

pub fn parse_item_with_vis<'src>(parser: parser_type!('src), vis: Visibility) -> Item<'src> {
    match parser.peek().expect("an item").kind {
        TokenKind::Const => Item::Const(Box::new(parse_const_item(parser, vis))),
        TokenKind::Enum => Item::Enum(Box::new(parse_enum_item(parser, vis))),
        TokenKind::Struct => Item::Struct(Box::new(parse_struct_item(parser, vis))),
        TokenKind::Def => Item::Fn(Box::new(parse_fn_item(parser, vis))),
        TokenKind::Import => Item::Import(Box::new(parse_import_item(parser, vis))),
        _ => panic!("expected an item that's valid with vis quals"),
    }
}

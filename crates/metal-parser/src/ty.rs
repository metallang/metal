// SPDX-License-Identifier: MIT

use metal_ast::Ty;

use crate::{parse_ident, parser_type};

pub fn parse_ty<'src>(parser: parser_type!('src)) -> Ty<'src> {
    Ty::Ident(Box::new(parse_ident(parser)))
}

pub fn parse_ty_qualifier<'src>(parser: parser_type!('src)) -> Option<Ty<'src>> {
    if parser.peek().is_some_and(|t| t.kind.is_colon()) {
        parser.next();

        Some(parse_ty(parser))
    } else {
        None
    }
}

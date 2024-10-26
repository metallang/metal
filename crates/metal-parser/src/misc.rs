use metal_ast::{Ident, Mutability, Visibility};
use metal_lexer::TokenKind;

use crate::{expect, parser_type};

pub fn parse_ident<'src>(parser: parser_type!('src)) -> Ident<'src> {
    let (inner, span) = expect!(parser, TokenKind::Ident(inner), inner);

    Ident { inner, span }
}

pub fn parse_mutness<'src>(parser: parser_type!('src)) -> Mutability {
    if parser.peek().is_some_and(|t| t.kind.is_mut()) {
        Mutability::Mut(parser.next().unwrap().span)
    } else {
        Mutability::Immut
    }
}

pub fn parse_vis<'src>(parser: parser_type!('src)) -> Visibility {
    if parser.peek().is_some_and(|t| t.kind.is_pub()) {
        Visibility::Pub(parser.next().unwrap().span)
    } else {
        Visibility::Local
    }
}

// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_ast::{ConstItem, Visibility};
use metal_lexer::{span, MaybeSpanned, TokenKind};

use crate::{expect, parse_expr_specifier, parse_ident, parse_ty_qualifier, parser_type};

pub fn parse_const_item<'src>(parser: parser_type!('src), vis: Visibility) -> ConstItem<'src> {
    let const_span = expect!(parser, TokenKind::Const);
    let ident = parse_ident(parser);
    let ty = parse_ty_qualifier(parser);
    let value = parse_expr_specifier(parser).expect("the constant's value");
    let end_span = expect!(parser, TokenKind::Semicolon);

    let span_start = vis.maybe_span().unwrap_or(&const_span).start;
    let span_end = end_span.end;

    ConstItem {
        vis,
        ident,
        ty,
        value,
        span: span!(span_start..span_end),
    }
}

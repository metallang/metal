// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_ast::{FnInput, FnItem, Visibility};
use metal_lexer::{span, MaybeSpanned, Spanned, TokenKind};

use crate::{
    expect, parse_block, parse_delimited, parse_expr_specifier, parse_ident, parse_mutness,
    parse_ty_qualifier, parser_type, Delimiter,
};

pub fn parse_fn_item<'src>(parser: parser_type!('src), vis: Visibility) -> FnItem<'src> {
    let start_span = expect!(parser, TokenKind::Def);

    let ident = parse_ident(parser);

    expect!(parser, TokenKind::OpeningParenthesis);

    let inputs = parse_delimited(
        parser,
        Delimiter::Comma,
        [Delimiter::ClosingParenthesis],
        &mut parse_fn_input,
    );

    expect!(parser, TokenKind::ClosingParenthesis);

    let return_type = parse_ty_qualifier(parser);

    let body = parse_block(parser);

    let span_end = body.span.end;

    FnItem {
        vis,
        ident,
        inputs,
        return_type,
        body,
        span: span!(start_span.start..span_end),
    }
}

pub fn parse_fn_input<'src>(parser: parser_type!('src)) -> FnInput<'src> {
    let mutness = parse_mutness(parser);
    let ident = parse_ident(parser);
    let ty = parse_ty_qualifier(parser); // `self` can omit the type
    let default = parse_expr_specifier(parser);

    let span_start = mutness.maybe_span().unwrap_or(&ident.span).start;
    let span_end = default
        .maybe_span()
        .or(ty.maybe_span())
        .unwrap_or(ident.span())
        .end;

    FnInput {
        mutness,
        ident,
        ty,
        default: None,
        span: span!(span_start..span_end),
    }
}

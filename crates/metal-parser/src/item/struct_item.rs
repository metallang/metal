use metal_ast::{StructBody, StructBodyItem, StructField, StructItem, Visibility};
use metal_lexer::{span, MaybeSpanned, Spanned, TokenKind};

use crate::{expect, parse_fn_item, parse_ident, parse_ty_qualifier, parse_vis, parser_type};

pub fn parse_struct_item<'src>(parser: parser_type!('src), vis: Visibility) -> StructItem<'src> {
    let start_span = expect!(parser, TokenKind::Struct);
    let ident = parse_ident(parser);
    let body = parse_struct_body(parser);

    let span_end = body.span().end;

    StructItem {
        vis,
        ident,
        body,
        span: span!(start_span.start..span_end),
    }
}

pub fn parse_struct_body<'src>(parser: parser_type!('src)) -> StructBody<'src> {
    let start_span = expect!(parser, TokenKind::OpeningBrace);

    let mut items = vec![];

    while parser.peek().is_some_and(|t| !t.kind.is_closing_brace()) {
        let vis = parse_vis(parser);

        items.push(match parser.peek().unwrap().kind {
            TokenKind::Ident(_) => StructBodyItem::Field(parse_struct_field(parser, vis)),
            TokenKind::Def => StructBodyItem::FnItem(parse_fn_item(parser, vis)),
            _ => panic!("expected a struct item, found {:?}", parser.next().unwrap()),
        });
    }

    let end_span = expect!(parser, TokenKind::ClosingBrace);

    StructBody {
        items,
        span: span!(start_span.start..end_span.end),
    }
}

pub fn parse_struct_field<'src>(parser: parser_type!('src), vis: Visibility) -> StructField<'src> {
    let ident = parse_ident(parser);
    let ty = parse_ty_qualifier(parser).expect("expected a type qualifier");

    expect!(parser, TokenKind::Comma);

    let span_start = vis.maybe_span().unwrap_or(ident.span()).start;
    let span_end = ty.span().end;

    StructField {
        vis,
        ident,
        ty,
        span: span!(span_start..span_end),
    }
}

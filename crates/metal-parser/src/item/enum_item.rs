use metal_ast::{EnumBody, EnumBodyItem, EnumItem, EnumVariant, Visibility};
use metal_lexer::{span, MaybeSpanned, Spanned, TokenKind};

use crate::{expect, parse_fn_item, parse_ident, parse_ty, parse_vis, parser_type};

pub fn parse_enum_item<'src>(parser: parser_type!('src), vis: Visibility) -> EnumItem<'src> {
    let start_span = expect!(parser, TokenKind::Enum);
    let ident = parse_ident(parser);
    let body = parse_enum_body(parser);

    let span_end = body.span().end;

    EnumItem {
        vis,
        ident,
        body,
        span: span!(start_span.start..span_end),
    }
}

pub fn parse_enum_body<'src>(parser: parser_type!('src)) -> EnumBody<'src> {
    let start_span = expect!(parser, TokenKind::OpeningBrace);

    let mut items = vec![];

    while parser.peek().is_some_and(|t| !t.kind.is_closing_brace()) {
        items.push(match parser.peek().unwrap().kind {
            TokenKind::Ident(_) => EnumBodyItem::Variant(parse_enum_variant(parser)),
            TokenKind::Def | TokenKind::Pub => EnumBodyItem::FnItem({
                let vis = parse_vis(parser);

                parse_fn_item(parser, vis)
            }),
            _ => panic!("expected a enum item, found {:?}", parser.next().unwrap()),
        });
    }

    let end_span = expect!(parser, TokenKind::ClosingBrace);

    EnumBody {
        items,
        span: span!(start_span.start..end_span.end),
    }
}

pub fn parse_enum_variant<'src>(parser: parser_type!('src)) -> EnumVariant<'src> {
    let ident = parse_ident(parser);

    let (datatype, closing_paren_span) = if parser
        .peek()
        .is_some_and(|t| t.kind.is_opening_parenthesis())
    {
        parser.next();

        let datatype = if !parser
            .peek()
            .is_some_and(|t| t.kind.is_closing_parenthesis())
        {
            Some(parse_ty(parser))
        } else {
            None
        };

        let end_span = expect!(parser, TokenKind::ClosingParenthesis);

        (datatype, Some(end_span))
    } else {
        (None, None)
    };

    expect!(parser, TokenKind::Comma);

    let span_start = ident.span().start;
    let span_end = closing_paren_span
        .as_ref()
        .or(datatype.maybe_span())
        .unwrap_or(ident.span())
        .end;

    EnumVariant {
        ident,
        datatype,
        span: span!(span_start..span_end),
    }
}

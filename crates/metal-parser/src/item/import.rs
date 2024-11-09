use metal_ast::{ImportItem, ImportTree, MultiImport, SegmentImport, Visibility};
use metal_lexer::{span, MaybeSpanned, Spanned, TokenKind};

use crate::{expect, parse_delimited, parse_ident, parser_type, Delimiter};

pub fn parse_import_item<'src>(parser: parser_type!('src), vis: Visibility) -> ImportItem<'src> {
    let import_span = expect!(parser, TokenKind::Import);
    let tree = parse_import_tree(parser);
    let end_span = expect!(parser, TokenKind::Semicolon);

    let span_start = vis.maybe_span().unwrap_or(&import_span).start;
    let span_end = end_span.end;

    ImportItem {
        vis,
        tree,
        span: span!(span_start..span_end),
    }
}

pub fn parse_import_tree<'src>(parser: parser_type!('src)) -> ImportTree<'src> {
    match parser.peek().unwrap().kind {
        TokenKind::Ident(_) => parse_name_or_segment_import_tree(parser),
        TokenKind::OpeningBrace => ImportTree::Multiple(Box::new(parse_multi_import(parser))),
        _ => panic!("expected an import tree, found {:?}", parser.next()),
    }
}

pub fn parse_name_or_segment_import_tree<'src>(parser: parser_type!('src)) -> ImportTree<'src> {
    let segment = parse_ident(parser);

    if parser.peek().is_some_and(|t| t.kind.is_period()) {
        parser.next(); // guaranteed to be a TokenKind::Period

        let rest = parse_import_tree(parser);

        let span_start = segment.span().start;
        let span_end = rest.span().end;

        ImportTree::Segment(Box::new(SegmentImport {
            segment,
            rest,
            span: span!(span_start..span_end),
        }))
    } else {
        ImportTree::Name(Box::new(segment))
    }
}

pub fn parse_multi_import<'src>(parser: parser_type!('src)) -> MultiImport<'src> {
    let start_span = expect!(parser, TokenKind::OpeningBrace);

    let subtrees = parse_delimited(
        parser,
        Delimiter::Comma,
        [Delimiter::ClosingBrace],
        &mut parse_import_tree,
    );

    let end_span = expect!(parser, TokenKind::ClosingBrace);

    MultiImport {
        subtrees,
        span: span!(start_span.start..end_span.end),
    }
}

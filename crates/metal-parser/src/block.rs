// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_ast::{Block, Item};
use metal_lexer::{span, TokenKind};

use crate::{expect, parse_delimited, parse_item, parser_type, Delimiter};

pub fn parse_block<'src>(parser: parser_type!('src)) -> Block<'src> {
    let start_span = expect!(parser, TokenKind::OpeningBrace);

    let items = parse_block_raw(parser);

    let end_span = expect!(parser, TokenKind::ClosingBrace);

    Block {
        items,
        span: span!(start_span.start..end_span.end),
    }
}

pub fn parse_block_raw<'src>(parser: parser_type!('src)) -> Vec<Item<'src>> {
    parse_delimited(
        parser,
        Delimiter::Semicolon,
        [Delimiter::ClosingBrace],
        &mut parse_item,
    )
}

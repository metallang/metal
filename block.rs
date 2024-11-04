mod block {
    use metal_ast::{Block, Item};
    use metal_lexer::{span, TokenKind};
    use crate::{expect, parse_delimited, parse_item, parser_type, Delimiter};
    pub fn parse_block<'src>(
        parser: &mut dyn ::peekable::Peekable<Item = ::metal_lexer::Token<'src>>,
    ) -> Block<'src> {
        let (_, start_span) = match parser.next() {
            Some(::metal_lexer::Token { kind: TokenKind::OpeningBrace, span }) => {
                ((), span)
            }
            other => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "expected {0}, found {1:?}",
                        "TokenKind::OpeningBrace",
                        other,
                    ),
                );
            }
        };
        let items = parse_block_raw(parser);
        let (_, end_span) = match parser.next() {
            Some(::metal_lexer::Token { kind: TokenKind::ClosingBrace, span }) => {
                ((), span)
            }
            other => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "expected {0}, found {1:?}",
                        "TokenKind::ClosingBrace",
                        other,
                    ),
                );
            }
        };
        Block {
            items,
            span: ::core::range::Range {
                start: start_span.start,
                end: end_span.end,
            },
        }
    }
    pub fn parse_block_raw<'src>(
        parser: &mut dyn ::peekable::Peekable<Item = ::metal_lexer::Token<'src>>,
    ) -> Vec<Item<'src>> {
        parse_delimited(
            parser,
            Delimiter::Semicolon,
            [Delimiter::ClosingBrace],
            &mut parse_item,
        )
    }
    pub fn p<'src>(
        parser: &mut dyn ::peekable::Peekable<Item = ::metal_lexer::Token<'src>>,
    ) -> () {
        let local_parser = parser;
        match local_parser.next() {
            Some(
                ::metal_lexer::Token {
                    kind: ::metal_lexer::TokenKind::OpeningBrace,
                    span,
                },
            ) => ((), span),
            other => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "expected {0}, found {1:?}",
                        ":: metal_lexer :: TokenKind :: OpeningBrace",
                        other,
                    ),
                );
            }
        };
        let local_parser = local_parser;
        match local_parser.next() {
            Some(
                ::metal_lexer::Token {
                    kind: ::metal_lexer::TokenKind::OpeningBrace,
                    span,
                },
            ) => ((), span),
            other => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "expected {0}, found {1:?}",
                        ":: metal_lexer :: TokenKind :: OpeningBrace",
                        other,
                    ),
                );
            }
        };
    }
    /// ```rust,ignore
    /// parser! {
    ///  This doc comment is (for now) purely to tell rust-analyzer which bracket style to use
    ///  in autocompletion.
    /// }
    /// ```
    pub macro parser {
        ($vis : vis fn $ident : ident = { $($body : tt)* }) => { $crate::block::parser!
        { @ priv entry(parser) $vis fn $ident = { $($body)* } } }, (@ priv entry($parser
        : ident) $vis : vis fn $ident : ident = { $($body : tt)* }) => { $vis fn $ident
        <'src > ($parser : $crate::parser_type!('src)) -> () { $crate::block::parser!(@
        priv transform($parser) ~ $($body)*); } }, (@ priv transform($parser : ident) ~ @
        open_brace $($excess : tt)*) => { let local_parser = $parser;
        $crate::expect!(local_parser, ::metal_lexer::TokenKind::OpeningBrace);
        $crate::block::parser!(@ priv transform(local_parser) $($excess)*) }, (@ priv
        transform($parser : ident) ~) => {}, (@ priv transform($parser : ident)) => {},
    }
}

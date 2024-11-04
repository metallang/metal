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

parser! {
    pub extern "pest" fn parse_block_ng<'src> -> Block<'src> {
        span_start: "{"
        ~ items: parse_block_raw
        ~ span_end: "}"

        => Block { items, span: span!(span_start.start..span_end.end) }
    }
}

parser! {
    pub extern "pest" fn parse_block_raw_ng<'src> -> Vec<Item<'src>> {
        items: parse_delimited(
            Delimiter::Semicolon,
            [Delimiter::ClosingBrace],
            &mut parse_item
        )

        => items
    }
}

/// ```rust,ignore
/// parser! {
///  This doc comment is (for now) purely to tell rust-analyzer which bracket style to use
///  in autocompletion.
/// }
/// ```
pub macro parser {
    (
        $vis:vis extern "pest" fn $ident:ident <$src:lifetime> -> $ret:ty {
            $($grammar:tt)*
        }
    ) => {
        $vis fn $ident <$src> (parser: $crate::parser_type!($src)) -> $ret {
            $crate::block::parser!(@priv begin_transform(parser) ~ $($grammar)*);
        }
    },

    // some grammar
    (@priv begin_transform($parser:ident) ~ $($excess:tt)+) => {
        $crate::block::parser!(@priv transform($parser,) ~ $($excess)+)
    },
    // fully empty grammar
    (@priv begin_transform($parser:ident) ~) => {},

    (@priv transform($parser:ident,) ~ $bind:ident : $($excess:tt)+ ) => {
        $crate::block::parser!(@priv transform($parser, let $bind =) ~ $($excess)+ )
    },

    // literal token transforms
    (@priv transform($parser:ident, $($binder:tt)*) ~ "{" $($excess:tt)* ) => {
        $($binder)* $crate::expect!($parser, ::metal_lexer::TokenKind::OpeningBrace);
        $crate::block::parser!(@priv transform($parser,) $($excess)*)
    },
    (@priv transform($parser:ident, $($binder:tt)*) ~ "}" $($excess:tt)* ) => {
        $($binder)* $crate::expect!($parser, ::metal_lexer::TokenKind::ClosingBrace);
        $crate::block::parser!(@priv transform($parser,) $($excess)*)
    },

    // external parse fn call transforms
    (@priv transform($parser:ident, $($binder:tt)*) ~ $parser_func:ident ( $($arg:expr),+ ) $($excess:tt)* ) => {
        $($binder)* $parser_func ($parser, $($arg),+ );
        $crate::block::parser!(@priv transform($parser,) $($excess)*)
    },
    (@priv transform($parser:ident, $($binder:tt)*) ~ $parser_func:ident $($excess:tt)* ) => {
        $($binder)* $parser_func ($parser);
        $crate::block::parser!(@priv transform($parser,) $($excess)*)
    },

    // valid invalid inputs
    (@priv transform($parser:ident,) ~) => {
        compile_error!("expected a rule after `~`")
    },

    // return with something
    (@priv transform($parser:ident,) => $ret_expr:expr) => {
        return $ret_expr
    },
    // return omitted
    (@priv transform($parser:ident,)) => {},
}

// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_lexer::Token;
use peekable::Peekable;

use crate::Delimiter;

pub macro parser_type($lt:lifetime) {
    &mut impl ::peekable::Peekable<Item = ::metal_lexer::Token<$lt>>
}

/// Asserts that the next token is of the given variant, and returns a tuple
/// of `(<return expr>, metal_lexer::Span)`. If the return expression is empty, only
/// the span is return
pub macro expect {
    ($iter:expr, $variant:pat) => {{
        let (_, span) = $crate::utils::expect!($iter, $variant, ());
        span
    }},
    ($iter:expr, $variant:pat, $ret:expr) => {
        match $iter.next() {
            Some(::metal_lexer::Token { kind: $variant, span }) => ($ret, span),
            other => panic!("expected {}, found {other:?}", stringify!($variant)),
        }
    }
}

// TODO: Should we make this return an iterator? Should be fine with a `#[must_use]`
/// Parses a list of values (as defined by the passed callback) separated by the specified delimiter.
/// ``stop_at`` is used to know when to stop looking for new values.
pub fn parse_delimited<
    'src,
    V,
    Parser: Peekable<Item = Token<'src>>,
    ParserFn: FnMut(&mut Parser) -> V,
    const N: usize,
>(
    parser: &mut Parser,
    delimiter: Delimiter,
    stop_at: [Delimiter; N], // TODO: see if making this a callback improves perf
    callback: &mut ParserFn,
) -> Vec<V> {
    let mut values = vec![];

    while parser
        .peek()
        .is_some_and(|t| !(stop_at.iter().any(|k| k.same_as(&t.kind))))
    {
        values.push(callback(parser));

        if parser.peek().is_some_and(|t| delimiter.same_as(&t.kind)) {
            parser.next();
        }
    }

    values
}

use metal_ast::{BoolLit, LitExpr, NumLit, StrLit};
use metal_lexer::{Token, TokenKind};

use crate::parser_type;

pub fn parse_lit_expr<'src>(parser: parser_type!('src)) -> LitExpr<'src> {
    let Token { kind, span } = parser.next().unwrap();

    match kind {
        TokenKind::NumLit(v) | TokenKind::HexNumLit(v) | TokenKind::BinaryNumLit(v) => {
            LitExpr::Num(NumLit {
                inner: v as f64,
                span,
            })
        }
        TokenKind::FloatLit(inner) | TokenKind::EFloatLit(inner) => {
            LitExpr::Num(NumLit { inner, span })
        }
        TokenKind::BoolLit(inner) => LitExpr::Bool(BoolLit { inner, span }),
        TokenKind::StrLit(inner) => LitExpr::Str(StrLit { inner, span }),
        kind => panic!("expected a literal, found {:?}", Token { kind, span }),
    }
}

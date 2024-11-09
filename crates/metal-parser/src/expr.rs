use metal_ast::Expr;
use metal_lexer::TokenKind;

use crate::{parse_ident, parse_lit_expr, parser_type};

pub mod lit;

pub fn parse_expr<'src>(parser: parser_type!('src)) -> Expr<'src> {
    match parser.peek().unwrap().kind {
        TokenKind::Ident(_) => Expr::Ident(Box::new(parse_ident(parser))),
        TokenKind::NumLit(_)
        | TokenKind::BinaryNumLit(_)
        | TokenKind::HexNumLit(_)
        | TokenKind::FloatLit(_)
        | TokenKind::EFloatLit(_)
        | TokenKind::StrLit(_)
        | TokenKind::BoolLit(_) => Expr::Lit(Box::new(parse_lit_expr(parser))),
        _ => panic!("expected an expr, found {:?}", parser.next().unwrap()),
    }
}

pub fn parse_expr_specifier<'src>(parser: parser_type!('src)) -> Option<Expr<'src>> {
    if parser.peek().is_some_and(|t| t.kind.is_assignment()) {
        parser.next(); // guaranteed to be TokenKind::Assignment

        Some(parse_expr(parser))
    } else {
        None
    }
}

use crate::{Span, TokenKind, token::Token};

pub struct Lexer<'src> {
    bytes: &'src [u8],
    pos: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Self {
        Self {
            bytes: text.as_bytes(),
            pos: 0,
        }
    }

    fn bump(&mut self) {
        if self.pos < self.bytes.len() {
            self.pos += 1;
        }
    }

    fn bump_token(&mut self, kind: TokenKind) -> TokenKind {
        self.bump();
        kind
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;

        let kind = match self.bytes.get(start)? {
            b'{' => self.bump_token(TokenKind::LBrace),
            b'}' => self.bump_token(TokenKind::RBrace),
            b'=' => self.bump_token(TokenKind::Eq),
            b':' => self.bump_token(TokenKind::Colon),
            b';' => self.bump_token(TokenKind::Semicolon),
            b'(' => self.bump_token(TokenKind::LParen),
            b')' => self.bump_token(TokenKind::RParen),
            b',' => self.bump_token(TokenKind::Comma),
            b'.' => self.bump_token(TokenKind::Dot),
            b'[' => self.bump_token(TokenKind::LBracket),
            b']' => self.bump_token(TokenKind::RBracket),
            b'&' => self.bump_token(TokenKind::Amp),
            b'+' => self.bump_token(TokenKind::Plus),
            _ => self.ident_token(),
        };

        let end = self.pos;

        Some(Token::new(kind, Span::new(start, end)))
    }
}

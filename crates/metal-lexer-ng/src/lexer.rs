use core::str;

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

    fn peek0(&self) -> Option<&u8> {
        self.bytes.get(self.pos)
    }

    fn peek1(&self) -> Option<&u8> {
        self.bytes.get(self.pos + 1)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;

        let kind = match self.bytes.get(start)? {
            b'*' => self.star_token(),
            b'&' => self.amp_token(),
            b'/' => self.slash_token(), // TODO: comments
            b'.' => self.dot_token(),
            b'|' => self.pipe_token(),
            b'>' => self.gt_token(),
            b'<' => self.lt_token(),
            b'!' => self.bang_token(),
            b'=' => self.eq_token(),
            b'{' => self.bump_token(TokenKind::LBrace),
            b'}' => self.bump_token(TokenKind::RBrace),
            b'(' => self.bump_token(TokenKind::LParen),
            b')' => self.bump_token(TokenKind::RParen),
            b'[' => self.bump_token(TokenKind::LBracket),
            b']' => self.bump_token(TokenKind::RBracket),
            b':' => self.bump_token(TokenKind::Colon),
            b';' => self.bump_token(TokenKind::Semicolon),
            b',' => self.bump_token(TokenKind::Comma),
            b'~' => self.bump_token(TokenKind::Tilde),
            b'+' => self.plus_token(),
            b'-' => self.minus_token(),
            b'%' => self.percent_token(),
            b'^' => self.caret_token(),
            b'"' => self.str_token(),
            b if b.is_ascii_digit() => self.num_token(),
            b if b.is_ascii_alphabetic() || b == &b'_' => self.ident_token(),
            b if b.is_ascii_whitespace() => self.whitespace_token(),
            _ => self.bump_token(TokenKind::Unknown),
        };

        let end = self.pos;

        Some(Token::new(kind, Span::new(start, end)))
    }
}

#[rustfmt::skip]
impl Lexer<'_> {
    fn star_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'*') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                TokenKind::Star2Eq
            } else {
                TokenKind::Star2
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::StarEq
        } else {
            TokenKind::Star
        }
    }

    fn amp_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'&') {
            self.bump();

            TokenKind::Amp2
        } else {
            TokenKind::Amp
        }
    }

    fn dot_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'.') {
            self.bump();

            TokenKind::Dot2
        } else {
            TokenKind::Dot
        }
    }

    fn pipe_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'|') {
            self.bump();

            TokenKind::Pipe2
        } else {
            TokenKind::Pipe
        }
    }

    fn gt_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'>') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                TokenKind::Gt2Eq
            } else {
                TokenKind::Gt2
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::GtEq
        } else {
            TokenKind::Gt
        }
    }

    fn lt_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'<') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                TokenKind::Lt2Eq
            } else {
                TokenKind::Lt2
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::LtEq
        } else {
            TokenKind::Lt
        }
    }

    fn str_token(&mut self) -> TokenKind {
        self.bump(); // consume the initial "

        loop {
            if self.peek0().is_some_and(|b| b == &b'"') {
                self.bump();
                break;
            }

            if self.peek0().is_some_and(|b| b == &b'\\') {
                self.bump();
                continue;
            }

            self.bump();
        }

        TokenKind::LitStr
    }

    fn num_token(&mut self) -> TokenKind {
        while self.peek0().is_some_and(|b| b.is_ascii_digit()) {
            self.bump();
        }

        TokenKind::LitNum
    }

    fn ident_token(&mut self) -> TokenKind {
        let start = self.pos;

        while self.peek0().is_some_and(|b| b.is_ascii_alphanumeric() || b == &b'_') {
            self.bump();
        }

        match &self.bytes[start..self.pos] {
            b"pub" => TokenKind::Pub,
            b"mut" => TokenKind::Mut,
            b"abstract" => TokenKind::Abstract,
            b"def" => TokenKind::Def,
            b"const" => TokenKind::Const,
            b"enum" => TokenKind::Enum,
            b"import" => TokenKind::Import,
            b"return" => TokenKind::Return,
            b"struct" => TokenKind::Struct,
            b"type" => TokenKind::Type,
            _ => TokenKind::LitIdent,
        }
    }

    fn whitespace_token(&mut self) -> TokenKind {
        while self.peek0().is_some_and(|b| b.is_ascii_whitespace()) {
            self.bump();
        }

        TokenKind::Whitespace
    }

    fn bang_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::BangEq
        } else {
            TokenKind::Bang
        }
    }

    fn eq_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::Eq2
        } else {
            TokenKind::Eq
        }
    }

    fn plus_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::PlusEq
        } else {
            TokenKind::Plus
        }
    }

    fn minus_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::MinusEq
        } else {
            TokenKind::Minus
        }
    }
    
    fn percent_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::PercentEq
        } else {
            TokenKind::Percent
        }
    }
    
    fn caret_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            TokenKind::CaretEq
        } else {
            TokenKind::Caret
        }
    }

    fn slash_token(&mut self) -> TokenKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            TokenKind::SlashEq
        } else if self.peek0().is_some_and(|b| b == &b'/') {
            // single-line comment, just like this one
            
            self.bump();

            while self.peek0().is_some_and(is_not_newline) {
                self.bump();
            }

            TokenKind::Comment
        } else if self.peek0().is_some_and(|b| b == &b'*') {
            /* multi-line comment, just like this one */

            self.bump();

            loop {
                if self.peek0().is_some_and(|b| b == &b'*') && self.peek1().is_some_and(|b| b == &b'/') {
                    self.bump();
                    self.bump();
                    break;
                }

                self.bump();
            }

            TokenKind::Comment
        } else {
            TokenKind::Slash
        }
    }
}

fn is_eq(b: &u8) -> bool {
    b == &b'='
}

fn is_not_newline(b: &u8) -> bool {
    !(b == &b'\n' || b == &b'\r')
}

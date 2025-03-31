// SPDX-License-Identifier: MIT

use core::str;

use metal_ast::{SyntaxKind, T};

use crate::{token::Token, Span};

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

    fn bump_token(&mut self, kind: SyntaxKind) -> SyntaxKind {
        self.bump();
        kind
    }

    fn peek(&self, n: usize) -> Option<&u8> {
        self.bytes.get(self.pos + n)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let start: usize = self.pos;

        let kind = match self.bytes.get(start)? {
            b'*' => self.bump_token(T![*]),
            b'&' => self.bump_token(T![&]),
            b'.' => self.bump_token(T![.]),
            b'|' => self.bump_token(T![|]),
            b'>' => self.bump_token(T![>]),
            b'<' => self.bump_token(T![<]),
            b'!' => self.bump_token(T![!]),
            b'=' => self.bump_token(T![=]),
            b'{' => self.bump_token(T!['{']),
            b'}' => self.bump_token(T!['}']),
            b'(' => self.bump_token(T!['(']),
            b')' => self.bump_token(T![')']),
            b'[' => self.bump_token(T!['[']),
            b']' => self.bump_token(T![']']),
            b':' => self.bump_token(T![:]),
            b';' => self.bump_token(T![;]),
            b',' => self.bump_token(T![,]),
            b'~' => self.bump_token(T![~]),
            b'@' => self.bump_token(T![@]),
            b'+' => self.bump_token(T![+]),
            b'-' => self.bump_token(T![-]),
            b'%' => self.bump_token(T![%]),
            b'^' => self.bump_token(T![^]),
            b'/' => self.slash_token(),
            b'"' => self.str_token(),
            b if b.is_ascii_digit() => self.num_token(),
            b if b.is_ascii_alphabetic() || b == &b'_' => self.ident_token(),
            b if b.is_ascii_whitespace() => self.whitespace_token(),
            _ => self.bump_token(T![@unknown]),
        };

        let end = self.pos;

        Some(Token::new(kind, Span::new(start, end)))
    }
}

#[rustfmt::skip]
impl Lexer<'_> {
    fn str_token(&mut self) -> SyntaxKind {
        self.bump(); // consume the initial "

        loop {
            if self.peek(0).is_some_and(|b| b == &b'"') {
                self.bump();
                break;
            }

            if self.peek(0).is_some_and(|b| b == &b'\\') {
                self.bump();
                self.bump();
                continue;
            }

            if self.is_eof() {
                break;
            }

            self.bump();
        }

        T![@string]
    }

    fn num_token(&mut self) -> SyntaxKind {
        while self.peek(0).is_some_and(|b| b.is_ascii_digit()) {
            self.bump();
        }

        T![@number]
    }

    fn ident_token(&mut self) -> SyntaxKind {
        let start = self.pos;

        while self.peek(0).is_some_and(|b| b.is_ascii_alphanumeric() || b == &b'_') {
            self.bump();
        }

        // TODO: generate this
        match &self.bytes[start..self.pos] {
            b"pub" => T![pub],
            b"mut" => T![mut],
            b"abstract" => T![abstract],
            b"def" => T![def],
            b"const" => T![const],
            b"enum" => T![enum],
            b"import" => T![import],
            b"return" => T![return],
            b"struct" => T![struct],
            b"type" => T![type],
            b"if" => T![if],
            b"else" => T![else],
            b"defer" => T![defer],
            b"let" => T![let],
            _ => T![@ident],
        }
    }

    fn whitespace_token(&mut self) -> SyntaxKind {
        while self.peek(0).is_some_and(|b| b.is_ascii_whitespace()) {
            self.bump();
        }

        T![@whitespace]
    }

    fn slash_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek(0).is_some_and(|b| b == &b'/') {
            // single-line comment, just like this one
            
            self.bump();

            while self.peek(0).is_some_and(is_not_newline) {
                self.bump();
            }

            T![@comment]
        } else if self.peek(0).is_some_and(|b| b == &b'*') {
            /* multi-line comment, just like this one */

            self.bump();

            loop {
                if self.peek(0).is_some_and(|b| b == &b'*') && self.peek(1).is_some_and(|b| b == &b'/') {
                    self.bump();
                    self.bump();
                    break;
                }

                if self.is_eof() {
                    break;
                }

                self.bump();
            }

            T![@comment]
        } else {
            T![/]
        }
    }

    fn is_eof(&self) -> bool {
        self.peek(0).is_none()
    }
}

fn is_not_newline(b: &u8) -> bool {
    !(b == &b'\n' || b == &b'\r')
}

// SPDX-License-Identifier: MIT

use core::str;

use metal_ast::SyntaxKind;

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
            b'/' => self.slash_token(),
            b'.' => self.dot_token(),
            b'|' => self.pipe_token(),
            b'>' => self.gt_token(),
            b'<' => self.lt_token(),
            b'!' => self.bang_token(),
            b'=' => self.eq_token(),
            b'{' => self.bump_token(SyntaxKind::L_BRACE_TOKEN),
            b'}' => self.bump_token(SyntaxKind::R_BRACE_TOKEN),
            b'(' => self.bump_token(SyntaxKind::L_PAREN_TOKEN),
            b')' => self.bump_token(SyntaxKind::R_PAREN_TOKEN),
            b'[' => self.bump_token(SyntaxKind::L_BRACKET_TOKEN),
            b']' => self.bump_token(SyntaxKind::R_BRACKET_TOKEN),
            b':' => self.bump_token(SyntaxKind::COLON_TOKEN),
            b';' => self.bump_token(SyntaxKind::SEMICOLON_TOKEN),
            b',' => self.bump_token(SyntaxKind::COMMA_TOKEN),
            b'~' => self.bump_token(SyntaxKind::TILDE_TOKEN),
            b'+' => self.plus_token(),
            b'-' => self.minus_token(),
            b'%' => self.percent_token(),
            b'^' => self.caret_token(),
            b'"' => self.str_token(),
            b if b.is_ascii_digit() => self.num_token(),
            b if b.is_ascii_alphabetic() || b == &b'_' => self.ident_token(),
            b if b.is_ascii_whitespace() => self.whitespace_token(),
            _ => self.bump_token(SyntaxKind::UNKNOWN_TOKEN),
        };

        let end = self.pos;

        Some(Token::new(kind, Span::new(start, end)))
    }
}

#[rustfmt::skip]
impl Lexer<'_> {
    fn star_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'*') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                SyntaxKind::STAR2_EQ_TOKEN
            } else {
                SyntaxKind::STAR2_TOKEN
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::STAR_EQ_TOKEN
        } else {
            SyntaxKind::STAR_TOKEN
        }
    }

    fn amp_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'&') {
            self.bump();

            SyntaxKind::AMP2_TOKEN
        } else {
            SyntaxKind::AMP_TOKEN
        }
    }

    fn dot_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'.') {
            self.bump();

            SyntaxKind::DOT2_TOKEN
        } else {
            SyntaxKind::DOT_TOKEN
        }
    }

    fn pipe_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'|') {
            self.bump();

            SyntaxKind::PIPE2_TOKEN
        } else {
            SyntaxKind::PIPE_TOKEN
        }
    }

    fn gt_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'>') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                SyntaxKind::GT2_EQ_TOKEN
            } else {
                SyntaxKind::GT2_TOKEN
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::GT_EQ_TOKEN
        } else {
            SyntaxKind::GT_TOKEN
        }
    }

    fn lt_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(|b| b == &b'<') {
            self.bump();

            if self.peek0().is_some_and(is_eq) {
                self.bump();

                SyntaxKind::LT2_EQ_TOKEN
            } else {
                SyntaxKind::LT2_TOKEN
            }
        } else if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::LT_EQ_TOKEN
        } else {
            SyntaxKind::LT_TOKEN
        }
    }

    fn str_token(&mut self) -> SyntaxKind {
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

        SyntaxKind::LIT_STR_TOKEN
    }

    fn num_token(&mut self) -> SyntaxKind {
        while self.peek0().is_some_and(|b| b.is_ascii_digit()) {
            self.bump();
        }

        SyntaxKind::LIT_NUM_TOKEN
    }

    fn ident_token(&mut self) -> SyntaxKind {
        let start = self.pos;

        while self.peek0().is_some_and(|b| b.is_ascii_alphanumeric() || b == &b'_') {
            self.bump();
        }

        match &self.bytes[start..self.pos] {
            b"pub" => SyntaxKind::PUB_TOKEN,
            b"mut" => SyntaxKind::MUT_TOKEN,
            b"abstract" => SyntaxKind::ABSTRACT_TOKEN,
            b"def" => SyntaxKind::DEF_TOKEN,
            b"const" => SyntaxKind::CONST_TOKEN,
            b"enum" => SyntaxKind::ENUM_TOKEN,
            b"import" => SyntaxKind::IMPORT_TOKEN,
            b"return" => SyntaxKind::RETURN_TOKEN,
            b"struct" => SyntaxKind::STRUCT_TOKEN,
            b"type" => SyntaxKind::TYPE_TOKEN,
            b"if" => SyntaxKind::IF_TOKEN,
            b"else" => SyntaxKind::ELSE_TOKEN,
            b"defer" => SyntaxKind::DEFER_TOKEN,
            _ => SyntaxKind::LIT_IDENT_TOKEN,
        }
    }

    fn whitespace_token(&mut self) -> SyntaxKind {
        while self.peek0().is_some_and(|b| b.is_ascii_whitespace()) {
            self.bump();
        }

        SyntaxKind::WHITESPACE_TOKEN
    }

    fn bang_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::BANG_EQ_TOKEN
        } else {
            SyntaxKind::BANG_TOKEN
        }
    }

    fn eq_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::EQ2_TOKEN
        } else {
            SyntaxKind::EQ_TOKEN
        }
    }

    fn plus_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::PLUS_EQ_TOKEN
        } else {
            SyntaxKind::PLUS_TOKEN
        }
    }

    fn minus_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::MINUS_EQ_TOKEN
        } else {
            SyntaxKind::MINUS_TOKEN
        }
    }
    
    fn percent_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::PERCENT_EQ_TOKEN
        } else {
            SyntaxKind::PERCENT_TOKEN
        }
    }
    
    fn caret_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            self.bump();

            SyntaxKind::CARET_EQ_TOKEN
        } else {
            SyntaxKind::CARET_TOKEN
        }
    }

    fn slash_token(&mut self) -> SyntaxKind {
        self.bump();

        if self.peek0().is_some_and(is_eq) {
            SyntaxKind::SLASH_EQ_TOKEN
        } else if self.peek0().is_some_and(|b| b == &b'/') {
            // single-line comment, just like this one
            
            self.bump();

            while self.peek0().is_some_and(is_not_newline) {
                self.bump();
            }

            SyntaxKind::COMMENT_TOKEN
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

            SyntaxKind::COMMENT_TOKEN
        } else {
            SyntaxKind::SLASH_TOKEN
        }
    }
}

fn is_eq(b: &u8) -> bool {
    b == &b'='
}

fn is_not_newline(b: &u8) -> bool {
    !(b == &b'\n' || b == &b'\r')
}

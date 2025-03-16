// SPDX-License-Identifier: MIT

use metal_ast_ng::SyntaxKind;
use metal_ast_ng::SyntaxNode;
use metal_ast_ng::T;
use metal_lexer_ng::Span;
use metal_lexer_ng::Token;
use rowan::GreenNodeBuilder;

#[derive(Clone, Copy, Debug)]
pub enum ParserMode {
    /// The default parsing mode.
    Normal,
    /// Indicates that a type is being parsed.
    Type,
}

pub struct Parser<'src, L> {
    builder: GreenNodeBuilder<'static>,
    lexer: L,
    source: &'src str,
    mode: ParserMode,
    current_token: Option<Token>,
    split_token_buffer: Option<Token>,
}

impl<'src, L> Parser<'src, L>
where
    L: Iterator<Item = Token>,
{
    pub fn new(lexer: L, source: &'src str) -> Self {
        let mut parser = Self {
            builder: GreenNodeBuilder::new(),
            lexer,
            source,
            mode: ParserMode::Normal,
            current_token: None,
            split_token_buffer: None,
        };

        parser.compute_next_token();

        parser
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        let next = self.current_token.take();

        self.compute_next_token();

        next
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    pub fn peek_is(&mut self, kind: SyntaxKind) -> bool {
        self.peek().is_some_and(|t| t.kind == kind)
    }

    pub fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn enter_mode(&mut self, mode: ParserMode) -> ParserMode {
        let old_mode = self.mode;

        self.mode = mode;

        old_mode
    }

    fn compute_next_token(&mut self) {
        if self.split_token_buffer.is_some() {
            self.current_token = self.split_token_buffer.take();
            return;
        }

        let Some(mut new_current_token) = self.lexer.next() else {
            return;
        };

        // skip whitespace tokens
        while new_current_token.kind.is_whitespace() {
            self.token(new_current_token.kind, new_current_token.span);

            match self.lexer.next() {
                Some(t) => new_current_token = t,
                None => return,
            }
        }

        if matches!(self.mode, ParserMode::Type) && new_current_token.kind == T![&&] {
            // split && into two &
            let first = Token {
                kind: T![&],
                span: Span {
                    start: new_current_token.span.start,
                    end: new_current_token.span.end - 1,
                },
            };
            let second = Token {
                kind: T![&],
                span: Span {
                    start: new_current_token.span.start + 1,
                    end: new_current_token.span.end,
                },
            };

            self.split_token_buffer = Some(second);
            new_current_token = first;
        }

        self.current_token = Some(new_current_token);
    }

    // green node builder functions

    pub fn checkpoint(&mut self) -> rowan::Checkpoint {
        self.builder.checkpoint()
    }

    pub fn start_node_at(&mut self, kind: SyntaxKind, at: rowan::Checkpoint) {
        self.builder.start_node_at(at, kind.into());
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub fn end_node(&mut self) {
        self.builder.finish_node();
    }

    #[allow(clippy::let_and_return)]
    pub fn finish(self) -> SyntaxNode {
        let green = self.builder.finish();
        let syntax = SyntaxNode::new_root(green);

        syntax
    }

    pub fn eat_any(&mut self) {
        if let Some(token) = self.next() {
            self.token(token.kind, token.span);
        }
    }

    pub fn maybe_eat(&mut self, kind: SyntaxKind) -> bool {
        if self.peek_is(kind) {
            let span = self.next().unwrap().span;

            self.token(kind, span);

            true
        } else {
            false
        }
    }

    fn token(&mut self, kind: SyntaxKind, span: Span) {
        self.builder.token(kind.into(), &self.source[span]);
    }
}

pub macro parser_type() {
    $crate::parser::Parser<impl ::std::iter::Iterator<Item = ::metal_lexer_ng::Token>>
}

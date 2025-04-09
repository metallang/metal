// SPDX-License-Identifier: MIT

use std::collections::VecDeque;

use metal_ast::{SyntaxKind, SyntaxNode};
use metal_lexer::Token;
use rowan::GreenNodeBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParsingContext {
    /// The default parsing mode.
    Normal,
    /// Indicates that the condition of an if expression is being parsed.
    IfExprCond,
}

pub struct Parser<'src> {
    builder: GreenNodeBuilder<'static>,
    tokens: VecDeque<Token>,
    source: &'src str,
    cx: ParsingContext,
}

impl<'src> Parser<'src> {
    pub fn new(tokens: VecDeque<Token>, source: &'src str) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            source,
            cx: ParsingContext::Normal,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        loop {
            let token = self.tokens.pop_front()?;

            if token.kind.is_whitespace() {
                self.token(token);
                continue;
            }

            return Some(dbg!(token));
        }
    }

    pub fn peek(&self, n: usize) -> Option<&Token> {
        dbg!(self.tokens.get(self.peek_impl(n)?))
    }

    pub fn peek_mut(&mut self, n: usize) -> Option<&mut Token> {
        dbg!(self.tokens.get_mut(self.peek_impl(n)?))
    }

    fn peek_impl(&self, n: usize) -> Option<usize> {
        let mut cursor = 0;
        let mut non_ws_tokens_seen = 0;

        loop {
            let token = self.tokens.get(cursor)?;

            if !token.kind.is_whitespace() {
                non_ws_tokens_seen += 1;
            }

            if non_ws_tokens_seen > n {
                break Some(cursor);
            }

            cursor += 1;
        }
    }

    pub fn peek_is(&self, n: usize, kind: SyntaxKind) -> bool {
        self.peek(n).is_some_and(|t| t.kind == kind)
    }

    pub fn is_eof(&mut self) -> bool {
        self.peek(0).is_none()
    }

    pub fn in_cx(&mut self, new_cx: ParsingContext, mut f: impl FnMut(&mut Self)) {
        let old_cx = self.cx.clone(); // cheap

        self.cx = new_cx;

        f(self);

        self.cx = old_cx;
    }

    #[track_caller]
    pub fn is_in_cx(&self, cx: ParsingContext) -> bool {
        self.cx == cx
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
            self.token(token);
        }
    }

    pub fn maybe_eat(&mut self, kind: SyntaxKind) -> bool {
        if self.peek(0).is_some_and(|token| token.kind == kind) {
            self.eat_any();

            true
        } else {
            false
        }
    }

    fn token(&mut self, token: Token) {
        self.builder
            .token(token.kind.into(), &self.source[token.span]);
    }
}

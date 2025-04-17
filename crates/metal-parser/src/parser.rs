// SPDX-License-Identifier: MIT

use std::collections::VecDeque;

use metal_ast::{SyntaxKind, SyntaxNode};
use metal_lexer::Token;
use rowan::GreenNodeBuilder;

use crate::restrictions::Restrictions;

pub struct Parser<'src> {
    builder: GreenNodeBuilder<'static>,
    tokens: VecDeque<Token>,
    source: &'src str,
    restrictions: Restrictions,
}

impl<'src> Parser<'src> {
    pub fn new(tokens: VecDeque<Token>, source: &'src str) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            source,
            restrictions: Restrictions::new(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<Token> {
        let mut next_non_trivia_token = self.nth_non_trivia(0)?;

        while next_non_trivia_token > 0 {
            let trivia_token = self.tokens.pop_front()?;

            self.token(trivia_token);

            next_non_trivia_token -= 1;
        }

        self.tokens.pop_front()
    }

    pub fn next_raw(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn peek(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.nth_non_trivia(n)?)
    }

    pub fn peek_mut(&mut self, n: usize) -> Option<&mut Token> {
        self.tokens.get_mut(self.nth_non_trivia(n)?)
    }

    fn nth_non_trivia(&self, n: usize) -> Option<usize> {
        let mut cursor = 0;
        let mut non_ws_tokens_seen = 0;

        loop {
            let token = self.tokens.get(cursor)?;

            if !token.kind.is_trivia() {
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

    pub fn is_not_at_eof_or(&mut self, kind: SyntaxKind) -> bool {
        !(self.peek_is(0, kind) || self.is_eof())
    }

    // restrictions api

    pub fn restrictions(&self) -> &Restrictions {
        &self.restrictions
    }

    pub fn restrictions_mut(&mut self) -> &mut Restrictions {
        &mut self.restrictions
    }

    /// Calls the given parser function and then restores the restrictions to the
    /// state they were in before the call.
    pub fn with_restrictions(&mut self, mut f: impl FnMut(&mut Self)) {
        let restrictions = self.restrictions.clone(); // cheap

        f(self);

        self.restrictions = restrictions;
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

    pub fn token(&mut self, token: Token) {
        self.builder
            .token(token.kind.into(), &self.source[token.span]);
    }
}

use metal_ast_ng::SyntaxKind;
use metal_ast_ng::SyntaxNode;
use metal_lexer_ng::Span;
use metal_lexer_ng::Token;
use peekable::Peekable;
use rowan::GreenNodeBuilder;

pub struct Parser<'src, L> {
    builder: GreenNodeBuilder<'static>,
    lexer: L,
    source: &'src str,
}

impl<'src, L> Parser<'src, L>
where
    L: Peekable<Item = Token>,
{
    pub fn new(lexer: L, source: &'src str) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexer,
            source,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.lexer.next()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.skip_whitespace();
        self.lexer.peek()
    }

    fn skip_whitespace(&mut self) {
        while self.lexer.peek().is_some_and(|t| {
            t.kind == SyntaxKind::WHITESPACE_TOKEN || t.kind == SyntaxKind::COMMENT_TOKEN
        }) {
            if let Some(token) = self.lexer.next() {
                self.token(token.kind, token.span);
            }
        }
    }

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

    pub fn is_eof(&mut self) -> bool {
        self.peek().is_none()
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

    pub fn peek_is(&mut self, kind: SyntaxKind) -> bool {
        self.peek().is_some_and(|t| t.kind == kind)
    }

    fn token(&mut self, kind: SyntaxKind, span: Span) {
        self.builder.token(kind.into(), &self.source[span]);
    }
}

pub macro parser_type() {
    $crate::parser::Parser<impl ::peekable::Peekable<Item = ::metal_lexer_ng::Token>>
}

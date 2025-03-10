// SPDX-License-Identifier: MIT

use metal_ast_ng::SyntaxKind;

use crate::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: SyntaxKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: SyntaxKind, span: Span) -> Self {
        Self { kind, span }
    }
}

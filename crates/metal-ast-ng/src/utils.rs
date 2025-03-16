// SPDX-License-Identifier: MIT

use std::marker::PhantomData;

use crate::{AstNode, SyntaxKind, SyntaxNode, SyntaxNodeChildren, SyntaxToken};

/// An iterator over `SyntaxNode` children of a particular AST type.
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

#[extend::ext]
pub impl SyntaxNode {
    #[inline]
    fn child_token(&self, kind: SyntaxKind, n: usize) -> Option<SyntaxToken> {
        self.children_with_tokens()
            .filter_map(|it| it.into_token())
            .filter(|it| it.kind() == kind)
            .nth(n)
    }

    #[inline]
    fn child<N: AstNode>(&self, n: usize) -> Option<N> {
        self.children().filter_map(N::cast).nth(n)
    }

    #[inline]
    fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }
}

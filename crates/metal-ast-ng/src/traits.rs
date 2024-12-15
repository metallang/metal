// SPDX-License-Identifier: MIT

use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

/// A trait to be implemented by [SyntaxNode] wrappers to provide rich, typed AST experience.
pub trait AstNode {
    /// Check if it's safe to cast the given `SyntaxKind` to this node type.
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Try to cast the given `SyntaxKind` to this node type.
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    /// Get the [SyntaxNode] this type wraps.
    fn syntax(&self) -> &SyntaxNode;

    // fn clone_for_update(&self) -> Self
    // where
    //     Self: Sized,
    // {
    //     Self::cast(self.syntax().clone_for_update()).unwrap()
    // }
    // fn clone_subtree(&self) -> Self
    // where
    //     Self: Sized,
    // {
    //     Self::cast(self.syntax().clone_subtree()).unwrap()
    // }
}

/// Like [AstNode], but wraps [SyntaxToken]s rather than nodes.
pub trait AstToken {
    /// Check if it's safe to cast the given `SyntaxKind` to this token type.
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Try to cast the given `SyntaxKind` to this token type.
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    /// Get the [SyntaxToken] this type wraps.
    fn syntax(&self) -> &SyntaxToken;

    /// Get the text of this token.
    fn text(&self) -> &str {
        self.syntax().text()
    }
}

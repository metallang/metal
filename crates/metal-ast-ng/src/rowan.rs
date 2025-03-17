// SPDX-License-Identifier: MIT

use crate::SyntaxKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MetalLanguage {}

impl rowan::Language for MetalLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from(raw)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<MetalLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<MetalLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<MetalLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<MetalLanguage>;

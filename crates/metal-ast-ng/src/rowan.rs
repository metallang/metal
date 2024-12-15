// SPDX-License-Identifier: MIT

use crate::SyntaxKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum MetalLanguage {}

impl rowan::Language for MetalLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        let d = raw.0 as u8;

        assert!(d <= (SyntaxKind::__LAST as u8));

        unsafe { std::mem::transmute::<u8, SyntaxKind>(d) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<MetalLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<MetalLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<MetalLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<MetalLanguage>;

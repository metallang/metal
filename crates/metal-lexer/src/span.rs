// SPDX-License-Identifier: MIT

/// A span in a source file.
pub type Span = core::range::Range<usize>;

/// An object with a [Span] attached.
pub trait Spanned {
    /// Get the [Span] of this object.
    fn span(&self) -> &Span;
}

impl<T: Spanned> Spanned for Box<T> {
    fn span(&self) -> &Span {
        self.as_ref().span()
    }
}

pub trait MaybeSpanned {
    fn maybe_span(&self) -> Option<&Span>;
}

impl<T: Spanned> MaybeSpanned for Option<T> {
    fn maybe_span(&self) -> Option<&Span> {
        match self {
            Some(spanned) => Some(spanned.span()),
            None => None,
        }
    }
}

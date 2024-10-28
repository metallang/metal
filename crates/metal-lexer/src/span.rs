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

/// Create a `core::range::Range` using the usual range syntax.
pub macro span {
    ($start:ident..$end:expr) => {
        $crate::span!(@priv $start, $end)
    },
    ($start:literal..$end:expr) => {
        $crate::span!(@priv $start, $end)
    },
    (@priv $start:expr, $end:expr) => {
        ::core::range::Range { start: $start, end: $end }
    }
}

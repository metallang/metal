/// A span in a source file.
pub type Span = std::ops::Range<usize>;

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

impl<T: Spanned> Spanned for Vec<T> {
    fn span(&self) -> &Span {
        let span_start = self.first().unwrap().span().start;
        let span_end = self.last().unwrap().span().end;
        let span_box = Box::new(span_start..span_end);
        let span = Box::leak(span_box);

        span
    }
}

use metal_lexer::Spanned;

/// A literal expression, such as `true` or `"entry"`.
#[derive(Spanned)]
pub enum LitExpr<'src> {
    /// See [BoolLit].
    Bool(BoolLit),
    /// See [NumLit].
    Num(NumLit),
    /// See [StrLit].
    Str(StrLit<'src>),
}

literals! {
    /// A boolean literal, such as `true` or `false`.
    BoolLit: bool,
    /// A number literal, such as `123` or `-3.14`.
    NumLit: f64,
    /// A string literal, such as `"Hello, World!"`.
    StrLit<'src>: &'src str,
}

macro literals($($(#[$attr:meta])* $ident:ident $(<$lt:lifetime>)?: $inner:ty),* $(,)?) {
    $(
        $(#[$attr])*
        #[metal_lexer::spanned]
        #[derive(metal_lexer::Spanned)]
        pub struct $ident <$($lt)?> {
            pub inner: $inner,
        }
    )*
}

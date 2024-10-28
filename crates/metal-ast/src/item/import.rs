use metal_lexer::{spanned, Spanned};

use crate::misc::Ident;

/// An import statement, such as `import std.num`.
#[spanned]
#[derive(Spanned)]
pub struct ImportItem<'src> {
    /// See [ImportTree].
    pub tree: ImportTree<'src>,
}

/// A tree-like representation of an [ImportItem]'s imports.
///
/// # Example
///
/// ```rs
/// // Below is how imports of `import std.{num, fs.{open, close}, rand.random};` would be
/// // represented using this structure (rust-like pseudocode).
///
/// ImportTree::Segment(SegmentImport {
///     segment: "std",
///     rest: ImportTree::Multiple(MultiImport { subtrees: vec![
///         ImportTree::Name("num"),
///         ImportTree::Segment(SegmentImport {
///             segment: "fs",
///             rest: ImportTree::Multiple(MultiImport { subtrees: vec![
///                 ImportTree::Name("open"),
///                 ImportTree::Name("close"),
///             ]}),
///         }),
///         ImportTree::Segment(SegmentImport {
///             segment: "rand",
///             rest: ImportTree::Name("random"),
///         }),
///     ]}),
/// })
///
/// ```
#[derive(Spanned)]
pub enum ImportTree<'src> {
    /// The final component of a path
    Name(Ident<'src>),
    /// An intermediate
    Segment(SegmentImport<'src>),
    Multiple(MultiImport<'src>),
}

#[spanned]
#[derive(Spanned)]
pub struct MultiImport<'src> {
    pub subtrees: Vec<ImportTree<'src>>,
}

#[spanned]
#[derive(Spanned)]
pub struct SegmentImport<'src> {
    pub segment: Ident<'src>,
    pub rest: Box<ImportTree<'src>>,
}

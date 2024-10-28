use metal_lexer::{spanned, Spanned};

use crate::misc::Ident;

/// An import item, such as `import std.num`.
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
    /// The final component of any import tree, such as `fs` in `import std.fs;`.
    Name(Ident<'src>),
    /// See [SegmentImport].
    Segment(SegmentImport<'src>),
    /// See [MultiImport].
    Multiple(MultiImport<'src>),
}

/// A branch in an import tree, such as `{num, fs}` in `import std.{num, fs};`.
#[spanned]
#[derive(Spanned)]
pub struct MultiImport<'src> {
    /// The subtrees that descend from this branch.
    pub subtrees: Vec<ImportTree<'src>>,
}

/// An intermediate component of an import tree, such as `std` in `import std.fs;`.
#[spanned]
#[derive(Spanned)]
pub struct SegmentImport<'src> {
    /// The segment.
    pub segment: Ident<'src>,
    /// The continuation of this import tree.
    pub rest: Box<ImportTree<'src>>,
}

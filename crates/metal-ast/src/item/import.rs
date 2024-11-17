// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use metal_lexer::{spanned, Spanned};

use crate::{Ident, Visibility};

/// An import declaration, such as `import std.num`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct ImportItem<'src> {
    /// See [Visibility]. An `ImportItem` with `.vis = Visibility::Pub` is effectively a re-export.
    pub vis: Visibility,
    /// See [ImportTree].
    pub tree: ImportTree<'src>,
}

/// A tree-like representation of an [ImportItem]'s imports.
///
/// # Example
///
/// ```rust,ignore
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
#[derive(Debug, Spanned)]
pub enum ImportTree<'src> {
    /// The final component of any import tree, such as `fs` in `import std.fs;`.
    Name(Box<Ident<'src>>),
    /// See [SegmentImport].
    Segment(Box<SegmentImport<'src>>),
    /// See [MultiImport].
    Multiple(Box<MultiImport<'src>>),
}

/// A branch in an import tree, such as `{num, fs}` in `import std.{num, fs};`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct MultiImport<'src> {
    /// The subtrees that descend from this branch.
    pub subtrees: Vec<ImportTree<'src>>,
}

/// An intermediate component of an import tree, such as `std` in `import std.fs;`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct SegmentImport<'src> {
    /// The segment.
    pub segment: Ident<'src>,
    /// The continuation of this import tree.
    pub rest: ImportTree<'src>,
}

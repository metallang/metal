use metal_lexer::{spanned, Spanned};

#[spanned]
#[derive(Spanned)]
/// An import statement, like `import std.num`.
pub struct ImportStmt<'src> {
    /// See [Path].
    pub tree: ImportTree<'src>,
}

pub enum ImportTree<'src> {
    Single,
    Multiple,
}

// import {std.num, std.{}}

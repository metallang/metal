//! The most honest-to-God representation of any piece of Metal code.

mod expr;
mod ty;

use metal_lexer::{spanned, Spanned};

pub use crate::expr::*;
pub use crate::ty::*;

/// A block of code.
pub type Block<'src> = Vec<Statement<'src>>;
/// An [ImportStmt]'s path.
pub type Path<'src> = Vec<PathSegment<'src>>;

/// An identifier.
#[spanned]
#[derive(Spanned)]
pub struct Ident<'src> {
    pub inner: &'src str,
}

#[derive(Spanned)]
/// A statement, typically a line of code.
pub enum Statement<'src> {
    /// See [ClassDefStmt].
    ClassDef(Box<ClassDefStmt<'src>>),
    /// See [ExprStmt].
    Expr(Box<ExprStmt<'src>>),
    /// See [FnDefStmt].
    FnDef(Box<FnDefStmt<'src>>),
    /// See [ImportStmt].
    Import(Box<ImportStmt<'src>>),
}

/// A class definition statement, like `class Vec { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct ClassDefStmt<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Block].
    pub body: Block<'src>,
}

/// A standalone expression, like `print("Hello, World!")`.
#[spanned]
#[derive(Spanned)]
pub struct ExprStmt<'src> {
    /// See [Expr].
    pub expr: Expr<'src>,
}

/// A function definition statement, like `main() {}`.
#[spanned]
#[derive(Spanned)]
pub struct FnDefStmt<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [FnInput].
    pub inputs: Vec<FnInput<'src>>,
    /// See [Ty].
    pub return_type: Option<Ty<'src>>,
    /// See [Block].
    pub body: Block<'src>,
}

#[spanned]
#[derive(Spanned)]
/// A function input, also called a parameter, like `yes: bool = True`.
pub struct FnInput<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Ty].
    pub ty: Ty<'src>,
    /// See [Expr].
    pub default: Option<Expr<'src>>,
}

#[spanned]
#[derive(Spanned)]
/// An import statement, like `import std.num`.
pub struct ImportStmt<'src> {
    /// See [Path].
    pub path: Path<'src>,
}

#[derive(Spanned)]
/// A segment of an import [Path].
pub enum PathSegment<'src> {
    /// A simple path segment, like `num` in `std.num`.
    Single(Ident<'src>),
    /// A "branched" import, like the bracketed part in `std.{num, collections.Vec}`.
    Multiple(Vec<Path<'src>>),
}

pub mod enum_def;
pub mod expr;
pub mod fn_def;
pub mod import;
pub mod struct_def;

use metal_lexer::Spanned;

use crate::{ExprItem, FnDef, ImportItem, StructDef};

/// An item, such as a constant definition or an import.
#[derive(Spanned)]
pub enum Item<'src> {
    /// See [StructDefStmt].
    StructDef(Box<StructDef<'src>>),
    /// See [ExprStmt].
    Expr(Box<ExprItem<'src>>),
    /// See [FnDefStmt].
    FnDef(Box<FnDef<'src>>),
    /// See [ImportStmt].
    Import(Box<ImportItem<'src>>),
}

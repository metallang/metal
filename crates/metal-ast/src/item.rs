mod expr;
mod fn_def;
mod import;
mod struct_def;

use metal_lexer::Spanned;

pub use crate::item::{expr::ExprItem, fn_def::FnDef, import::ImportItem, struct_def::StructDef};

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

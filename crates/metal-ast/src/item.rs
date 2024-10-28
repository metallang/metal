use expr::ExprStmt;
use fn_def::FnDefStmt;
use import::ImportItem;
use metal_lexer::Spanned;
use struct_def::StructDef;

pub mod expr;
pub mod fn_def;
pub mod import;
pub mod struct_def;

#[derive(Spanned)]
/// An item, such as a constant definition or an import.
pub enum Item<'src> {
    /// See [StructDefStmt].
    StructDef(Box<StructDef<'src>>),
    /// See [ExprStmt].
    Expr(Box<ExprStmt<'src>>),
    /// See [FnDefStmt].
    FnDef(Box<FnDefStmt<'src>>),
    /// See [ImportStmt].
    Import(Box<ImportItem<'src>>),
}

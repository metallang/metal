use expr::ExprStmt;
use fn_def::FnDefStmt;
use import::ImportStmt;
use metal_lexer::Spanned;
use struct_def::StructDefStmt;

pub mod expr;
pub mod fn_def;
pub mod import;
pub mod struct_def;

#[derive(Spanned)]
/// A statement, typically a line of code.
pub enum Statement<'src> {
    /// See [StructDefStmt].
    StructDef(Box<StructDefStmt<'src>>),
    /// See [ExprStmt].
    Expr(Box<ExprStmt<'src>>),
    /// See [FnDefStmt].
    FnDef(Box<FnDefStmt<'src>>),
    /// See [ImportStmt].
    Import(Box<ImportStmt<'src>>),
}

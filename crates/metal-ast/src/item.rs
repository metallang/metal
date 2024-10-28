pub mod const_def;
pub mod enum_def;
pub mod expr;
pub mod fn_def;
pub mod import;
pub mod struct_def;

use metal_lexer::Spanned;

use crate::{ConstDef, EnumDef, ExprItem, FnDef, ImportItem, StructDef};

/// An item, such as a constant definition or an import.
#[derive(Spanned)]
pub enum Item<'src> {
    /// See [ConstDef].
    Const(Box<ConstDef<'src>>),
    /// See [EnumDef].
    Enum(Box<EnumDef<'src>>),
    /// See [ExprItem].
    Expr(Box<ExprItem<'src>>),
    /// See [FnDef].
    FnDef(Box<FnDef<'src>>),
    /// See [ImportItem].
    Import(Box<ImportItem<'src>>),
    /// See [StructDef].
    StructDef(Box<StructDef<'src>>),
}

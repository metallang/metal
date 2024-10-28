pub mod const_item;
pub mod enum_item;
pub mod expr;
pub mod fn_item;
pub mod import;
pub mod struct_item;

use metal_lexer::Spanned;

use crate::{ConstItem, EnumItem, ExprItem, FnItem, ImportItem, StructItem};

/// An item, such as a constant definition or an import.
#[derive(Spanned)]
pub enum Item<'src> {
    /// See [ConstDef].
    Const(Box<ConstItem<'src>>),
    /// See [EnumDef].
    Enum(Box<EnumItem<'src>>),
    /// See [ExprItem].
    Expr(Box<ExprItem<'src>>),
    /// See [FnDef].
    FnDef(Box<FnItem<'src>>),
    /// See [ImportItem].
    Import(Box<ImportItem<'src>>),
    /// See [StructDef].
    StructDef(Box<StructItem<'src>>),
}

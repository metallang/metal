pub mod const_item;
pub mod enum_item;
pub mod fn_item;
pub mod import;
pub mod struct_item;

use metal_lexer::Spanned;

use crate::{ConstItem, EnumItem, Expr, FnItem, ImportItem, StructItem};

/// An item, such as a constant definition or an import.
#[derive(Debug, Spanned)]
pub enum Item<'src> {
    /// See [ConstItem].
    Const(Box<ConstItem<'src>>),
    /// See [EnumItem].
    Enum(Box<EnumItem<'src>>),
    /// A standalone expression.
    Expr(Expr<'src>), // needs not to be boxed because it's all boxes already
    /// See [FnItem].
    Fn(Box<FnItem<'src>>),
    /// See [ImportItem].
    Import(Box<ImportItem<'src>>),
    /// See [StructItem].
    Struct(Box<StructItem<'src>>),
}

//! The most honest-to-God representation of any piece of Metal code.

#![feature(decl_macro)]

mod expr;
mod item;
mod misc;
mod ty;

pub use crate::expr::{CallExpr, Expr, LitExpr};
pub use crate::item::{ExprItem, FnDef, ImportItem, Item, StructDef};
pub use crate::ty::Ty;

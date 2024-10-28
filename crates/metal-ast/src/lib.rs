//! The most honest-to-God representation of any piece of Metal code.

#![feature(decl_macro)]

mod expr;
mod item;
mod misc;
mod ty;

pub use crate::{
    expr::{
        call::CallExpr,
        lit::{BoolLit, LitExpr, NumLit, StrLit},
        Expr,
    },
    item::{
        enum_def::{EnumDef, EnumItem, EnumItems, EnumVariant},
        expr::ExprItem,
        fn_def::{FnDef, FnInput},
        import::{ImportItem, ImportTree, MultiImport, SegmentImport},
        struct_def::{StructDef, StructField, StructItem, StructItems},
        Item,
    },
    misc::{Block, Ident, Visibility, VisibilityKind},
    ty::Ty,
};

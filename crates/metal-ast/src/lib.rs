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
        const_item::ConstItem,
        enum_item::{EnumBody, EnumBodyItem, EnumItem, EnumVariant},
        fn_item::{FnInput, FnItem},
        import::{ImportItem, ImportTree, MultiImport, SegmentImport},
        struct_item::{StructBody, StructBodyItem, StructField, StructItem},
        Item,
    },
    misc::{Block, Ident, Mutability, Visibility},
    ty::Ty,
};

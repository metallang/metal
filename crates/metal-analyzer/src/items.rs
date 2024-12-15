// SPDX-License-Identifier: MIT

use rkyv::{Archive, Deserialize, Serialize};

use crate::expr::Expr;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub enum Item {
    Abstract(AbstractItem),
    Const(ConstItem),
    Enum(EnumItem),
    Expr(Expr),
    Fn(FnItem),
    Import(ImportItem),
    Return(ReturnItem),
    Struct(StructItem),
    TypeAlias(TypeAliasItem),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct AbstractItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct ConstItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct EnumItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct FnItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct ImportItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct ReturnItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct StructItem {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct TypeAliasItem {}

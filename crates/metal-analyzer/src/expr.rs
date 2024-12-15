// SPDX-License-Identifier: MIT

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub enum Expr {
    Name(NameExpr),
    Prefix(PrefixExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Lit(LitExpr),
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct NameExpr {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct PrefixExpr {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct BinaryExpr {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct CallExpr {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct LitExpr {}

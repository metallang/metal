// SPDX-License-Identifier: MIT
use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    expr::Expr,
    types::{primitives::Primitive, visibility::Visibility},
};

/// An immutable global constant value.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub struct Constant {
    /// The name of this constant.
    pub name: String,
    /// The value of this constant.
    pub expr: Expr,
    /// The type of this constant. This is expected to
    /// be a primitive type, and should panic if not.
    pub ty: Primitive,
    /// Visibility of this constant.
    pub vis: Visibility,
}

// SPDX-License-Identifier: MIT

use crate::{
    expr::Expr,
    types::{primitives::Primitive, visibility::Visibility},
};

/// An immutable global constant value.
#[derive(Debug, Clone)]
pub struct Constant {
    /// The name of this constant.
    pub name: &'static str,
    /// The value of this constant.
    pub expr: Expr,
    /// The type of this constant. This is expected to
    /// be a primitive type, and should panic if not.
    pub ty: Primitive,
    /// Visibility of this constant.
    pub vis: Visibility,
}

use crate::{
    expr::Expr,
    types::{visibility::Visibility, Type},
};

#[derive(Debug, Clone)]
pub struct Constant {
    /// The name of this constant.
    pub name: &'static str,
    /// The value of this constant.
    pub expr: Expr,
    /// The type of this constant. This is expected to
    /// be a primitive type, and should panic if not.
    pub ty: Type,
    /// Visibility of this constant.
    pub vis: Visibility,
}

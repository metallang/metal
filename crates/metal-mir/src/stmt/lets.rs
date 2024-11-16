use crate::{expr::Expr, types::Type};

/// Represents a stack allocation for `name` using `type` and an
/// optional assignment afterwards using `expr`.
#[derive(Debug, Clone)]
pub struct Let {
    /// The variable name to use
    pub name: &'static str,
    /// The type of the variable
    pub ty: Type,
    /// Represents an optional expression to assign to this variable
    pub expr: Option<Expr>,
}

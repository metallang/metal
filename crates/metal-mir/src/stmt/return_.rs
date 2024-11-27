use crate::expr::Expr;

/// Represents a return statement. i.e. `return 15`
#[derive(Debug, Clone)]
pub struct Return(pub Expr);

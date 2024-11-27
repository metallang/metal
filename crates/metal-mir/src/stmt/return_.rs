use crate::expr::Expr;

/// Rerpresents a return statement. i.e. `return 15`
#[derive(Debug, Clone)]
pub struct Return(pub Expr);

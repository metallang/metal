use crate::expr::Expr;
use serde::{Deserialize, Serialize};

/// Represents a return statement. i.e. `return 15`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return(pub Expr);

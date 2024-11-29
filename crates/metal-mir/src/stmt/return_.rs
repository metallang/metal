use serde::{Deserialize, Serialize};

use crate::expr::Expr;

/// Represents a return statement. i.e. `return 15`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return(pub Expr);

// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::expr::Expr;

/// Represents a return statement. i.e. `return 15`
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Return(pub Expr);

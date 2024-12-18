use rkyv::{Archive, Deserialize, Serialize};

use crate::expr::Expr;

/// Represents a return statement. i.e. `return 15`
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub struct Return(pub Expr);

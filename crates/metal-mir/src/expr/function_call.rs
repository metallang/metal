// SPDX-License-Identifier: MIT

use rkyv::{Archive, Deserialize, Serialize};

use super::Expr;
use crate::types;

/// Represents a call to a function.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub struct FunctionCall {
    /// The function signature to call.
    pub signature: types::function::FunctionSignature,
    /// The arguments to call this function with.
    pub arguments: Vec<Expr>,
}

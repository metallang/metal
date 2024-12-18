// SPDX-License-Identifier: MIT
use rkyv::{Archive, Deserialize, Serialize};

use super::Statement;
use crate::types;

/// Represents a definition for a function.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub struct FunctionDefinition {
    /// The function signature/identifier/type.
    pub signature: types::function::FunctionSignature,
    /// The body of this function
    pub body: Vec<Statement>,
}

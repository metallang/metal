// SPDX-License-Identifier: MIT
use serde::{Deserialize, Serialize};

use super::Statement;
use crate::types;

/// Represents a definition for a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// The function signature/identifier/type.
    pub signature: types::function::FunctionSignature,
    /// The body of this function
    pub body: Vec<Statement>,
}

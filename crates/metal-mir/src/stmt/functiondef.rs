// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use super::Statement;
use crate::{parcel, types};

/// Represents a definition for a function.
#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    /// The module of this function.
    pub module: parcel::Module,
    /// The function signature/identifier/type.
    pub signature: types::function::FunctionSignature,
    /// The body of this function.
    pub body: Vec<Statement>,
}

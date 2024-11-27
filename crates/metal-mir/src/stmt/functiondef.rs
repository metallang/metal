// SPDX-License-Identifier: MIT

use super::Statement;
use crate::{parcel, types};

/// Represents a definition for a function.
#[derive(Debug, Clone)]
pub struct FunctionDefinition<'a> {
    /// The module of this function.
    pub module: parcel::Module<'a>,
    /// The function signature/identifier/type.
    pub signature: types::function::FunctionSignature,
    /// The body of this function.
    pub body: Vec<Statement<'a>>,
}

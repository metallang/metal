// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{visibility::Visibility, Type};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    /// The name of this function.
    pub name: String,
    /// The type returned by this function.
    pub return_type: Type,
    /// The inputs this function takes.
    pub inputs: HashMap<String, Type>,
    /// The visibility of this function.
    pub vis: Visibility,
}

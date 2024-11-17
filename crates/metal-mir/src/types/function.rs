// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use super::{visibility::Visibility, Type};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// The name of this function.
    pub name: String,
    /// The type returned by this function.
    pub return_type: Type,
    /// The inputs this function takes.
    pub inputs: Vec<Type>,
    /// The visibility of this function.
    pub vis: Visibility,
}

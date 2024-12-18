// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use rkyv::{Archive, Deserialize, Serialize};

use super::{visibility::Visibility, Type};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub struct FunctionSignature {
    /// The name of this function.
    pub name: String,
    /// The type returned by this function.
    pub return_type: Type,
    /// The inputs this function takes.
    pub inputs: BTreeMap<String, Type>,
    /// The visibility of this function.
    pub vis: Visibility,
}

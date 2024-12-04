// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use super::Type;

/// Represents a basic array type.
///
/// i.e.: [1, 2, 3]
/// Array { item_type: Type(...) }
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Array {
    /// The type which is present in a list.
    pub item_type: Type,
    /// The initially defined size/capacity of the array.
    pub size: u64,
}

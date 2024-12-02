// SPDX-License-Identifier: MIT
use serde::{Deserialize, Serialize};

/// Represents the visibility of an item.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Visibility {
    /// Public for any module to use.
    Public,
    /// Public for the current parcel only.
    Parcel,
    /// Only usable in the current module.
    Private,
}

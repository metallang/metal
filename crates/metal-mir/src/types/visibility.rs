// SPDX-License-Identifier: MIT
use rkyv::{Archive, Deserialize, Serialize};

/// Represents the visibility of an item.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub enum Visibility {
    /// Public for any module to use.
    Public,
    /// Public for the current parcel only.
    Parcel,
    /// Only usable in the current module.
    Private,
}

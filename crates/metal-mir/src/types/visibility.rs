// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

/// Represents the visibility of an item.
#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    /// Public for any module to use.
    Public,
    /// Public for the current parcel only.
    Parcel,
    /// Only usable in the current module.
    Private,
}

// SPDX-License-Identifier: MIT

//! Contains representation of Metal Parcels and modules.

use serde::{Deserialize, Serialize};

use crate::stmt;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Parcel {
    /// Parcel name.
    /// i.e. `std`
    pub name: String,
    /// Modules included in
    /// this parcel.
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Module {
    /// Module name.
    /// i.e. `std.io`
    pub name: String,
    /// Module file name.
    /// i.e. `io.mt`
    pub filename: String,
    /// An exhaustive list of the statements
    /// the module includes
    pub statements: Vec<Box<stmt::Statement>>,
    /// All imports declared within this module.
    pub imports: Vec<Box<stmt::import::Import>>,
}

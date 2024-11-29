// SPDX-License-Identifier: MIT

//! Contains representation of Metal Parcels and modules.

use serde::{Deserialize, Serialize};

use crate::{stmt, types};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parcel {
    /// Parcel name.
    /// i.e. `std`
    pub name: String,
    /// Modules included in
    /// this parcel.
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Module name.
    /// i.e. `io`
    pub name: String,
    /// If this is a submodule
    /// includes the higher-level parent.
    pub parent: Option<Box<Module>>,
    /// Submodules inside of this module.
    /// Should be empty if module isn't a folder.
    pub children: Vec<Box<Module>>,
    /// An exhaustive list of the statements
    /// the module includes
    pub statements: Vec<Box<stmt::Statement>>,
    /// Signatures of all functions in this module.
    /// Used for function calls to avoid relying on
    /// where a function is located in a module.
    pub function_signatures: Vec<Box<types::function::FunctionSignature>>,
    /// All imports declared within this module.
    pub imports: Vec<Box<stmt::import::Import>>,
    /// All of the defined constants in this module.
    pub constants: Vec<Box<stmt::constant::Constant>>,
    /// All of the defined structs in this module.
    pub structs: Vec<Box<super::struct_::Struct>>,
}

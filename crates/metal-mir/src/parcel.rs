// SPDX-License-Identifier: MIT

//! Contains representation of Metal Parcels and modules.

use crate::{stmt, types};

#[derive(Debug, Clone)]
pub struct Parcel<'a> {
    /// Parcel name.
    /// i.e. `std`
    pub name: String,
    /// Modules included in
    /// this parcel.
    pub modules: Vec<Module<'a>>,
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    /// Module name.
    /// i.e. `io`
    pub name: String,
    /// If this is a submodule
    /// includes the higher-level parent.
    pub parent: Option<Box<&'a Module<'a>>>,
    /// Submodules inside of this module.
    /// Should be empty if module isn't a folder.
    pub children: Vec<&'a Module<'a>>,
    /// An exhaustive list of the statements
    /// the module includes.
    pub statements: Vec<&'a stmt::Statement<'a>>,
    /// Signatures of all functions in this module.
    /// Used for function calls to avoid relying on
    /// where a function is located in a module.
    pub function_signatures: Vec<&'a types::function::FunctionSignature>,
    /// All imports declared within this module.
    pub imports: Vec<&'a stmt::import::Import<'a>>,
    /// All of the defined constants in this module.
    pub constants: Vec<&'a stmt::constant::Constant>,
    /// All of the defined structs in this module.
    pub structs: Vec<&'a super::struct_::Struct>,
}

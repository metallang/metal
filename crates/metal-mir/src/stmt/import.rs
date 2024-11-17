// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use crate::{parcel::Module, struct_, types};

/// Represents an import.
/// i.e. `import std.{io.{write, writers.{websocket}}, os}`
/// ```metal
/// Import {
///     module: ..., // std module
///     functions: [],
///     children: [
///         Import {
///             module: ..., // io module
///             functions: [...], // write function signature
///             children: [
///                 Import {
///                     module: ..., // writers module
///                     functions: [...] // websocket function
///                     
///                 }
///             ]
///         }
///     ]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Import {
    /// The module imported.
    pub module: Box<Module>,
    /// The functions imported.
    pub functions: Vec<types::function::FunctionSignature>,
    /// The structs imported.
    pub structs: Vec<struct_::Struct>,
    /// The constants imported.
    pub constants: Vec<super::constant::Constant>,
    /// The next level of imports.
    pub children: Vec<Import>,
}

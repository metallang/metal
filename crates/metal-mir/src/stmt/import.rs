use crate::{parcel::Module, types};

// TODO: support global variables
// TODO: support structs & enums

/// Represents an import.
/// i.e. `import std.{io.{write, writers.{websocket}}, os}`
///
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
///
#[derive(Debug, Clone)]
pub struct Import {
    pub module: Box<Module>,
    pub functions: Vec<types::function::FunctionSignature>,
    pub children: Vec<Import>,
}

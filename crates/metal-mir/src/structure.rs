use crate::stmt::functiondef::FunctionDefinition;
use crate::types::{visibility::Visibility, Type};

/// Represents the properties of a struct.
/// i.e. `a: B`
#[derive(Debug, Clone)]
pub struct StructProperty {
    /// name of the struct property. i.e.: `a`
    pub name: &'static str,
    /// index of the struct property (order,) i.e. `0`
    pub idx: u32,
    /// the type of this struct property. i.e. `String`
    pub ty: Type,
}

/// Represents a struct in Metal.
/// i.e.:
/// ```metal
/// pub struct A {
///     b: String;
///     pub c: i32;
///
///     def d(e: String): i32 {
///         Self {
///             b: e,
///             c: e.len()
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Struct {
    /// The name of this struct.
    pub name: &'static str,
    /// Properties of this struct.
    pub properties: Option<Vec<StructProperty>>,
    /// Module-level struct visibility.
    pub vis: Visibility,
    /// Functions defined within this function.
    pub functions: Vec<FunctionDefinition>,
    /// Whether or not this struct should directly
    /// be created within this module using its `properties`
    /// or only be represented by `impl_type`.
    pub is_impl: bool,
    /// The type which should be given as `self` in
    /// function arguments. Should be None if `is_impl` is false.
    pub impl_type: Option<Box<Type>>,
}

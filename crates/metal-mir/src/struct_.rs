use crate::types::{visibility::Visibility, Type};

/// Represents the fields of a struct.
/// i.e. `a: B`
#[derive(Debug, Clone)]
pub struct StructField {
    /// name of the struct field. i.e.: `a`
    pub name: &'static str,
    /// the type of this struct field. i.e. `String`
    pub ty: Type,
}

/// Represents a struct in Metal.
/// i.e.:
/// ```metal
/// pub struct A {
///     b: String,
///     pub c: i32
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Struct {
    /// The name of this struct.
    pub name: &'static str,
    /// Fields of this struct.
    pub properties: Vec<StructField>,
    /// The visibility of this struct.
    pub vis: Visibility,
}
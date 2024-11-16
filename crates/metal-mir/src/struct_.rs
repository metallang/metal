use crate::types::{visibility::Visibility, Type};

/// Represents the fields of a struct.
/// i.e. `a: B`
#[derive(Debug, Clone)]
pub struct StructField {
    /// name of the struct property. i.e.: `a`
    pub name: &'static str,
    /// the type of this struct property. i.e. `String`
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
    /// Properties of this struct.
    pub properties: Vec<StructField>,
    /// Module-level struct visibility.
    pub vis: Visibility,
}

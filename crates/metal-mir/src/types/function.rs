use super::{visibility::Visibility, Type};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Represents the type returned by this function
    pub return_type: Type,
    /// Represents the inputs this function takes
    pub inputs: Vec<Type>,
    /// Represents the visibility of this function
    pub vis: Visibility,
}

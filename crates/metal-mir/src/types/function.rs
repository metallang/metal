use super::{visibility::Visibility, Type};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Function name
    pub name: String,
    /// Represents the type returned by this function
    pub return_type: Type,
    /// Represents the arguments this function takes
    pub arguments: Vec<Type>,
    /// Represents who can use this function
    pub vis: Visibility,
}

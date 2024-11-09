use super::{visibility::Visibility, Type};

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: Type,
    pub arguments: Vec<Type>,
    pub vis: Visibility,
}

use super::Expr;
use crate::types;

/// Represents a call to a function.
#[derive(Debug, Clone)]
pub struct FunctionCall {
    /// The function signature to call.
    pub signature: types::function::FunctionSignature,
    /// The arguments to call this function with.
    pub arguments: Vec<Expr>,
}

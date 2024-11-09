use super::Expr;
use crate::types;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub signature: types::function::FunctionSignature,
    pub arguments: Vec<Expr>,
}

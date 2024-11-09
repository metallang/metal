use super::Statement;
use crate::{parcel, types};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub module: parcel::Module,
    pub signature: types::function::FunctionSignature,
    pub body: Vec<Statement>,
}
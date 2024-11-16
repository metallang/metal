use crate::types::function::FunctionSignature;

pub mod constant;
pub mod functiondef;
pub mod import;
pub mod lets;

#[derive(Debug, Clone)]
pub struct Extern {
    pub name: &'static str,
    pub sig: FunctionSignature,
}

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
    Let(Box<lets::Let>),
    Extern(Box<Extern>),
}

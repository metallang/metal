pub mod constant;
pub mod functiondef;
pub mod import;

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
}

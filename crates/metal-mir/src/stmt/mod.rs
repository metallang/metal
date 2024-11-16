pub mod constant;
pub mod functiondef;
pub mod import;
pub mod lets;

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
    Let(Box<lets::Let>),
}

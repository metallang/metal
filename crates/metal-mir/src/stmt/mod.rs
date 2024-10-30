pub mod functiondef;
pub mod import;

#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Import(Box<import::Import>),
}

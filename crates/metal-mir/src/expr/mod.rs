use crate::{stmt::lets::Let, types::Type};

pub mod function_call;
pub mod literals;

#[derive(Debug, Clone)]
pub struct MathematicalValue {
    pub a: Expr,
    pub b: Expr,
    pub signed: Option<bool>,
    pub float: Option<bool>,
    pub result_var_name: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub name: &'static str,
    pub expr: Expr,
    pub parent: Let,
}

#[derive(Debug, Clone)]
pub struct Load {
    pub name: &'static str,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(Box<function_call::FunctionCall>),
    Literal(Box<literals::Literal>),
    /// Assign a value to a variable
    Assignment(Box<Assignment>),
    /// Load a variable into memory registry
    Load(Box<Load>),
    // Math
    Add(Box<MathematicalValue>),
    Sub(Box<MathematicalValue>),
    Mul(Box<MathematicalValue>),
    Div(Box<MathematicalValue>),
    Percent(Box<MathematicalValue>),
    Gt(Box<MathematicalValue>),
    Lt(Box<MathematicalValue>),
}

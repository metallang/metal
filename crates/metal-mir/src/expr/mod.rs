use crate::{stmt::lets::Let, types::Type};

pub mod function_call;
pub mod literals;

/// Represents a mathematical value
#[derive(Debug, Clone)]
pub struct MathematicalValue {
    /// Represents the left hand side of an expression
    pub a: Expr,
    /// Represents the right hand side of an expression
    pub b: Expr,
    /// Whether or not this integer is signed or not
    pub signed: Option<bool>,
    /// Whether or not this is a float value
    pub float: Option<bool>,
    /// The variable to put the returned value into
    pub result_var_name: Option<&'static str>,
}

/// Represents a variable assignment
#[derive(Debug, Clone)]
pub struct Assignment {
    /// The name of the variable
    pub name: &'static str,
    /// The value to be assigned to `name`
    pub expr: Expr,
    /// The `let` assignment who parent's this variable
    pub parent: Let,
}

/// Represents loading a pointer into the memory registry
#[derive(Debug, Clone)]
pub struct Load {
    /// The variable/local to load
    pub name: &'static str,
    /// The type of the variable to be loaded
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

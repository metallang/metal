// SPDX-License-Identifier: MIT

use crate::types::Type;

pub mod function_call;
pub mod literals;

/// Represents a mathematical value.
#[derive(Debug, Clone)]
pub struct MathematicalValue {
    /// Represents the left hand side of the expression.
    pub a: Expr,
    /// Represents the right hand side of the expression.
    pub b: Expr,
    /// Whether this integer is signed.
    pub signed: bool,
    /// Whether this is a float value.
    pub float: bool,
    /// The variable to put the returned value into.
    pub result_var_name: Option<&'static str>,
}

/// Represents a stack allocation for `name` using `type` and an
/// optional assignment afterwards using `expr`, or a variable assignment.
#[derive(Debug, Clone)]
pub struct Assignment {
    /// The variable name to use.
    pub name: &'static str,
    /// The type of the variable.
    pub ty: Type,
    /// Represents an optional expression to assign to this variable.
    pub expr: Option<Expr>,
}

/// Represents loading a pointer into the memory register.
#[derive(Debug, Clone)]
pub struct Load {
    /// The variable/local to load.
    pub name: &'static str,
    /// The type of the variable to be loaded.
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(Box<function_call::FunctionCall>),
    Literal(Box<literals::Literal>),
    Assignment(Box<Assignment>),
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

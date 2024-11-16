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
pub enum Expr {
    FunctionCall(Box<function_call::FunctionCall>),
    Literal(Box<literals::Literal>),
    // Math
    Add(Box<MathematicalValue>),
    Sub(Box<MathematicalValue>),
    Mul(Box<MathematicalValue>),
    Div(Box<MathematicalValue>),
    Percent(Box<MathematicalValue>),
    Gt(Box<MathematicalValue>),
    Lt(Box<MathematicalValue>),
}

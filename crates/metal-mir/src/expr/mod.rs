pub mod function_call;
pub mod literals;

#[derive(Debug, Clone)]
pub enum Expr {
    FunctionCall(Box<function_call::FunctionCall>),
    Literal(Box<literals::Literal>),
}

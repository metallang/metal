use crate::types;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(Number),
    String(StringLiteral),
    Boolean(Bool),
}

#[derive(Debug, Clone)]
pub struct Number {
    pub primitive: types::primitives::Primitive,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Bool {
    pub value: bool,
}

// SPDX-License-Identifier: MIT

use crate::types;

/// Represents a literal.
#[derive(Debug, Clone)]
pub enum Literal {
    /// Represents a literal number, i.e. `1`.
    Number(Number),
    /// Represents a literal immutable string, i.e. `"a"`.
    String(StringLiteral),
    /// Represents a boolean value, i.e. `true` or `false`.
    Boolean(Bool),
}

#[derive(Debug, Clone)]
pub struct Number {
    /// The primitive type of this number.
    pub primitive: types::primitives::Primitive,
    /// The literal value.
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    /// The string value.
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Bool {
    /// The boolean value.
    pub value: bool,
}

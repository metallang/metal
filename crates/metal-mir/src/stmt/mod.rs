// SPDX-License-Identifier: MIT

use crate::{expr::Assignment, types::function::FunctionSignature};

pub mod constant;
pub mod functiondef;
pub mod import;

/// Represents a statement in Metal code.
#[derive(Debug, Clone)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
    Let(Box<Assignment>),
    Extern(Box<FunctionSignature>),
}

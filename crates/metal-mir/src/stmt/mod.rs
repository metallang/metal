// SPDX-License-Identifier: MIT

use crate::{expr::Assignment, types::function::FunctionSignature};

pub mod constant;
pub mod functiondef;
pub mod import;
pub mod return_;

/// Represents a statement in Metal code.
#[derive(Debug, Clone)]
pub enum Statement<'a> {
    FunctionDefine(Box<functiondef::FunctionDefinition<'a>>),
    Constant(Box<constant::Constant>),
    Let(Box<Assignment>),
    Extern(Box<FunctionSignature>),
    Return(Option<Box<return_::Return>>),
}

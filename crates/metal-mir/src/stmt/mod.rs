// SPDX-License-Identifier: MIT
use serde::{Deserialize, Serialize};

use crate::{expr::Assignment, types::function::FunctionSignature};

pub mod constant;
pub mod functiondef;
pub mod import;
pub mod return_;

/// Represents a statement in Metal code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
    Let(Box<Assignment>),
    Extern(Box<FunctionSignature>),
    Return(Option<Box<return_::Return>>),
}

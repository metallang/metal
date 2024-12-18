// SPDX-License-Identifier: MIT
use rkyv::{Archive, Deserialize, Serialize};

use crate::{expr::Assignment, types::function::FunctionSignature};

pub mod constant;
pub mod functiondef;
pub mod import;
pub mod return_;

/// Represents a statement in Metal code.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub enum Statement {
    FunctionDefine(Box<functiondef::FunctionDefinition>),
    Constant(Box<constant::Constant>),
    Let(Box<Assignment>),
    Extern(Box<FunctionSignature>),
    Return(Option<Box<return_::Return>>),
}

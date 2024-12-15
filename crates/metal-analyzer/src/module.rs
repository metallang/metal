// SPDX-License-Identifier: MIT

//! Modules constructed using AST nodes

use rkyv::{Archive, Deserialize, Serialize};

/// A general AST detail of a Metal module
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct ASTModule {}

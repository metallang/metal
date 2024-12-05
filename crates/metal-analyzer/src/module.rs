//! Modules constructed using AST nodes

use rkyv::{Archive, Deserialize, Serialize};

// TODO(ast): add ast nodes or other such here.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[rkyv(compare(PartialEq), derive(Debug, Clone, Copy))]
pub struct Module;

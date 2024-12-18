// SPDX-License-Identifier: MIT
use rkyv::{Archive, Deserialize, Serialize};

/// Represents the basic types of the language.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub enum Primitive {
    // Integers (signed)
    I8,
    I16,
    I32,
    I64,
    I128,

    // Integers (unsigned)
    U8,
    U16,
    U32,
    U64,
    U128,

    // Floats
    F32,
    F64,
    F128,

    // Strings
    Literal(u64),

    // Void
    Void,
}

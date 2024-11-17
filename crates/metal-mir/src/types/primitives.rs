// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

/// Represents the basic types of the language.
#[derive(Debug, Clone, Copy)]
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

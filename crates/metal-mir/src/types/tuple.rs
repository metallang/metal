use super::Type;

// NOTE: The definitions here for a tuple simply tell the backend
// what types to add for the struct to be created.

/// Represents a tuple type.
/// i.e. (1, 2, 3)
/// Tuple { id: "...", size: 3, types: [Int, Int, Int] }
#[derive(Debug, Clone)]
pub struct Tuple {
    /// A unique identifier for this tuple.
    /// Used for naming inside of LLVM.
    pub id: String,
    pub size: i16,
    pub types: Vec<Type>,
}

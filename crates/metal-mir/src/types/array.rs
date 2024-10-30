use super::Type;

/// Represents a basic array type.
/// Unlike Tuple, Array will always be allocated
/// to the Heap instead of Stack by default.
///
/// May change once we support memory management.
///
/// i.e.: [1, 2, 3]
/// Array { item_type: Type(...) }
#[derive(Debug, Clone)]
pub struct Array {
    pub item_type: Type,
    pub default_size: u64,
}

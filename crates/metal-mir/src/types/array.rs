use super::Type;

/// Represents a basic array type.
///
/// i.e.: [1, 2, 3]
/// Array { item_type: Type(...) }
#[derive(Debug, Clone)]
pub struct Array {
    pub item_type: Type,
    pub default_size: u64,
}

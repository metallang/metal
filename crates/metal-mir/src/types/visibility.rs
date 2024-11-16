/// Represents the visibility of an item
#[derive(Debug, Clone, Copy)]
pub enum Visibility {
    /// Public for any module to use
    Public,
    /// Public for only this current parcel
    Parcel,
    /// Only usable in the current module
    Private,
}

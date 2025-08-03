use crate::{COLUMN_LIMIT, INDENT};

#[derive(Default)]
pub struct PositionTracker {
    pub indent_level: usize,
    pub column: usize,
}

impl PositionTracker {
    pub fn indent_string(&self) -> String {
        INDENT.repeat(self.indent_level)
    }

    pub fn is_overboard(&self, flat_width: usize) -> bool {
        self.column + self.indent_level * INDENT.len() + flat_width > COLUMN_LIMIT
    }
}

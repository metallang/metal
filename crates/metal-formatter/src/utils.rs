pub const INDENT: &str = "    ";
pub const COLUMN_LIMIT: usize = 100;
pub const NEWLINE: &str = "\n";

#[derive(Default)]
pub struct PositionTracker {
    pub indent_level: usize,
    pub column: usize,
}

impl PositionTracker {
    pub fn indent_string(&self) -> String {
        INDENT.repeat(self.indent_level)
    }

    pub fn is_overboard(&self, width: usize) -> bool {
        self.column + self.indent_level * INDENT.len() + width > COLUMN_LIMIT
    }
}

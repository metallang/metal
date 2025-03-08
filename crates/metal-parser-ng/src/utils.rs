#[extend::ext]
pub(crate) impl bool {
    fn if_false(self, f: impl FnOnce()) {
        if !self {
            f()
        }
    }
}

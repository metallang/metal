// SPDX-License-Identifier: MIT

/// A set of restrictions to impose on the [crate::parser::Parser].
#[derive(Clone, Debug)]
pub struct Restrictions(RestrictionFlags);

impl Restrictions {
    /// Create a new empty set of restrictions.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(RestrictionFlags::empty())
    }

    /// Impose additional restrictions on the parser.
    pub fn add(&mut self, restrictions: RestrictionFlags) {
        self.0.insert(restrictions);
    }

    /// Relax some of the restrictions on the parser.
    pub fn relax(&mut self, restrictions: RestrictionFlags) {
        self.0.remove(restrictions);
    }

    /// Check if the parser has the given restrictions imposed on it.
    pub fn include(&self, restrictions: RestrictionFlags) -> bool {
        self.0.contains(restrictions)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Debug)]
    pub struct RestrictionFlags: u8 {
        const NO_STRUCT_EXPR = 1 << 0;
    }
}

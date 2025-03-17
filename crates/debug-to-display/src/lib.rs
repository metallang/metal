// SPDX-License-Identifier: MIT

use std::fmt::{Debug, Display};

/// Usually, returning a Result::Err will use error's Debug impl
/// to display the error to the user. This type forwards Debug impl
/// to Display, allowing to circumvent that.
pub struct ForwardDebugToDisplay<T>(T);

impl<T: Display> Display for ForwardDebugToDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display> Debug for ForwardDebugToDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl<T: Display> From<T> for ForwardDebugToDisplay<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

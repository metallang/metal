// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

/// Trait version of [std::iter::Peekable].
pub trait Peekable: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}

impl<I: Iterator> Peekable for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        self.peek()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.peek_mut()
    }
}

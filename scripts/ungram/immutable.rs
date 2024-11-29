use std::ops::Deref;

pub struct Immutable<T>(T);

impl<T> Immutable<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }

    pub fn consume(self) -> T {
        self.0
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
